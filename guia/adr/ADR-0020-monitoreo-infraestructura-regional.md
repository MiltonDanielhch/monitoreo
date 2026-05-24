# ADR 0020: Módulo de Monitoreo de Infraestructura Regional

| Campo | Valor |
| --- | --- |
| **Autores** | Milton Hipamo / Laboratorio 3030 |
| **Relacionado con** | ADR 0001 (Hexagonal), ADR 0003 (Axum), ADR 0004 (MySQL 8.0), ADR 0006 (Sea-ORM), ADR 0008 (PASETO), ADR 0015 (Apalis), ADR 0017 (SvelteKit), ADR 0019 (Coolify), ADR 0021 (Local-First), ADR 0022 (Agentes Distribuidos) |

---

## 📋 Contexto y Decisión

**Problema:** La Gobernación del Beni requiere monitorear de forma centralizada la infraestructura de red de sus sedes institucionales regionales. Estas dependencias operan bajo condiciones de conectividad altamente inestables, caracterizadas por latencias elevadas, microcortes frecuentes y la necesidad mandatoria de mantener capacidades operativas en modo offline.

**Decisión:** Diseñar e implementar un módulo nativo de monitoreo de red con enfoque *Local-First*. El sistema será capaz de inventariar dispositivos, mapear topologías dinámicas, auditar el consumo de ancho de banda, detectar intrusiones en caliente y despachar alertas tempranas, utilizando un almacenamiento centralizado unificado y tolerancia a fallos local en el cliente.

---

## 🏗️ Arquitectura General del Flujo de Datos

```
Sedes Regionales (Trinidad, Riberalta, Guayaramerín, etc.)
                          ↓
        Agentes de Captura Ligeros (apps/agent) [ADR 0022]
                          ↓
             API en Axum 0.8 (apps/api) [ADR 0003]
                          ↓
       Colas de Procesamiento Async (tokio Jobs) [ADR 0015]
                          ↓
    Persistencia de Datos Unificada (MySQL 8.0 / InnoDB) [ADR 0004]
                          ↓
     Dashboard Web Realtime (Svelte 5 / Runes) [ADR 0017]

```

> 📌 **Nota de Persistencia:** El backend del servidor opera exclusivamente sobre el motor unificado MySQL 8.0. La base de datos SQLite se confina de manera estricta al entorno del navegador (WebAssembly) para la persistencia del estado e indexación en modo offline (ADR 0021).

---

## 📦 Organización de Componentes

| Módulo de Dominio | Responsabilidad Técnica | Ubicación en el Monorepo |
| --- | --- | --- |
| `inventory` | Registro analítico y ciclo de vida de activos físicos. | `crates/inventory` |
| `topology` | Cálculo de adyacencias y grafos de conexión de red. | `crates/topology` |
| `metrics` | Procesamiento de telemetría de interfaces de red. | `crates/database` |
| `alerts` | Heurística de evaluación de umbrales y anomalías. | `crates/jobs` |
| `agents` | Demonios de recolección distribuida mediante SNMP/ICMP. | `apps/agent` |
| `sync` | Mecanismo de conciliación de estados diferidos. | `crates/sync` |

---

## 🎯 Funcionalidades Principales

### 1 — Inventario Físico de Dispositivos

* **Entidades Soportadas:** Switches, Access Points, Routers, Firewalls, Servidores, Sistemas de Energía Ininterrumpida (UPS), Cámaras IP y Enlaces Inalámbricos de larga distancia.
* **Metadata de Control:** Hostname, dirección IPv4 (validada en objeto de valor), dirección MAC (IEEE 802), identificador de sede, proveedor, modelo, versión de firmware y estado operativo actual (`Active`, `Offline`, `Maintenance`).

### 2 — Topología de Red y Dependencias Jerárquicas

* Mapeo visual dinámico de enlaces físicos e interconexiones lógicas (Uplinks WAN, switches secundarios y APs dependientes).
* Cálculo inmediato de Puntos Únicos de Fallo (SPOF) para alertar de forma preventiva el aislamiento potencial de una sede completa.

### 3 — Auditoría de Ancho de Banda y Telemetría

* **Métricas Clave:** Tráfico de entrada/salida (Bytes RX/TX), saturación de enlaces WAN, porcentaje de pérdida de paquetes (*packet loss*), latencia media (ms) y fluctuación del retraso (*jitter*).
* **Streaming de Datos:** Implementación de Eventos Enviados por Servidor (SSE) nativos en Axum como canal primario de actualización en tiempo real; WebSocket reservado únicamente para controles bidireccionales explícitos.

### 4 — Detección en Caliente de Dispositivos Intrusos

* Descubrimiento automatizado mediante barridos ICMP asíncronos, consultas de tablas ARP via SNMP y auditoría de asignaciones DHCP.
* **Flujo de Evaluación:**

| Estado de Coincidencia | Acción Ejecutada por el Sistema |
| --- | --- |
| **MAC en Lista Blanca** | Actualiza marca de tiempo (`last_seen_at`) en el inventario. |
| **MAC Desconocida** | Registra evento de intrusión inmediato y eleva alerta crítica. |
| **IP Duplicada (IP Spoofing)** | Clasifica como anomalía grave y bloquea visualmente el nodo en el mapa. |

---

## 🗄️ Estructura del Modelo de Dominio (Rust)

### Entidad Dispositivo

```rust
pub enum DeviceType { Switch, AccessPoint, Router, Firewall, Server, Ups, Camera, WirelessLink }
pub enum DeviceStatus { Active, Offline, Maintenance }

pub struct Device {
    pub id: DeviceId,
    pub hostname: String,
    pub ip_address: String,      // Objeto de valor con validación regex estricta IPv4
    pub mac_address: String,     // Objeto de valor normalizado a mayúsculas (IEEE 802)
    pub device_type: DeviceType,
    pub status: DeviceStatus,
    pub sede_id: SedeId,
    pub last_seen_at: Option<OffsetDateTime>,
    pub deleted_at: Option<OffsetDateTime>,  // Soporte nativo para borrado suave
}

```

### Lectura de Telemetría

```rust
pub struct MetricReading {
    pub id: ReadingId,
    pub device_id: DeviceId,
    pub bandwidth_rx_bytes: i64,
    pub bandwidth_tx_bytes: i64,
    pub latency_ms: Option<i32>,           // Entero para evitar la imprecisión de punto flotante en cálculos
    pub packet_loss_percent: Option<f64>,  // Precisión doble para agregaciones estadísticas
    pub anomaly_detected: bool,
    pub created_at: OffsetDateTime,
}

```

---

## 📊 Estrategia de Retención y Purgado de Datos

Para evitar la degradación del rendimiento del almacenamiento en InnoDB dentro de nuestro entorno VPS optimizado, se aplica una política estricta de compactación y Rollup:

| Tipo de Registro | Ventana de Retención Activa | Acción de Limpieza / Agregación |
| --- | --- | --- |
| **Métricas Crudas** | 30 Días | Remoción física permanente mediante `CleanupMetricsJob`. |
| **Métricas Agregadas** | 1 Año | Compactadas en resúmenes por hora y día vía `MetricsAggregationJob`. |
| **Alertas Históricas** | Permanente | Preservadas para auditorías de la Gobernación. |
| **Eventos de Intrusión** | Permanente | Almacenadas con firmas de auditoría inmutables. |

---

## 🔄 Orquestación de Tareas en Segundo Plano (tokio)

* `MetricsAggregationJob` (Cada hora): Reduce lecturas de telemetría cruda a promedios históricos horarios, minimizando el impacto de lecturas indexadas.
* `IntrusionDetectionJob` (Cada 5 minutos): Contrasta las tablas ARP recolectadas por los agentes contra la lista blanca institucional (`device_whitelist`).
* `SyncOfflineJob` (Cada 2 minutos): Consume y concilia la cola de sincronización remitida por los clientes que recuperaron conectividad WAN.

---

## 🛠️ Toolchain de Dependencias e Infraestructura

Las herramientas se alinean de manera inmutable con el compilador **Rust 1.95.0** y el ecosistema del **Laboratorio 3030**:

* **`sea-orm` (`v1.1.x`):** ORM oficial de abstracción de datos utilizando el driver asíncrono para MySQL sin sentencias SQL crudas en la capa de infraestructura.
* **`tokio` (`v1.43.0`):** Runtime asíncrono de alto rendimiento (rama estable de soporte extendido).
| `tokio::sync` | Canales nativos `mpsc` y primitivas de sincronización asíncrona. | `1.45.x` | ✅ Activa (Estable) |
* **`async-snmp` (`v0.12.0`):** Crate reactivo no bloqueante para el consumo de OIDs de red.
* **`surge-ping` (`v0.8.4`):** Implementación de pings ICMP asíncronos de alta velocidad basados en Tokio.
* **`pasetors` (`v0.7.8`):** Generación segura de tokens PASETO v4, erradicando por completo el uso de JWT.

---

## 🛡️ Seguridad Perimetral y Reglas RBAC

El acceso a los endpoints expuestos en la API de Axum queda restringido por las siguientes firmas de permisos:

* **`Admin`:** Control total de inventario, edición de listas blancas y configuraciones globales del sistema.
* **`Operator`:** Gestión de incidentes, reconocimiento de alertas activas (`Acknowledged`) y registro de bitácoras de investigación de intrusos.
* **`Viewer`:** Acceso exclusivo de lectura a la topología y reportes ejecutivos.
* **`Agent`:** Permiso restringido por token PASETO de larga duración con alcance único de escritura de métricas y lectura de perfiles de dispositivos asignados.

---
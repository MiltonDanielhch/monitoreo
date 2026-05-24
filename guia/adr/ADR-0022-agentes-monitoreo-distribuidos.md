# ADR 0022: Agentes de Monitoreo Distribuidos

| Campo | Valor |
| --- | --- |
| **Autores** | Milton Hipamo / Laboratorio 3030 |
| **Relacionado con** | ADR 0001 (Hexagonal), ADR 0004 (MySQL 8.0), ADR 0006 (Sea-ORM), ADR 0008 (PASETO), ADR 0015 (tokio jobs), ADR 0017 (SvelteKit), ADR 0020 (Monitoreo Regional), ADR 0021 (Local-First), ADR 0023 (Migración docker) |

---

## 📋 Contexto y Decisión

**Problema:** La Gobernación del Beni cuenta con múltiples sedes regionales con conectividad WAN inestable, ancho de banda severamente limitado y hardware restrictivo en campo. Exponer los dispositivos de infraestructura de red (switches, routers, APs) directamente a internet para su monitoreo centralizado representa un riesgo de seguridad inaceptable y una ineficiencia operativa crítica.

**Decisión:** Desplegar de forma nativa un **agente de monitoreo ligero en Rust** (`apps/agent/`) en cada sede regional. Este agente se encarga de escanear la red local de forma aislada, recolectar métricas de rendimiento, detectar intrusos mediante análisis de direccionamiento MAC, almacenar temporalmente la información en caliente y sincronizar los lotes mediante ráfagas HTTPS controladas hacia la API central cuando la conectividad lo permita.

---

## 🛠️ Stack Tecnológico del Agente

| Componente | Tecnología | Versión | Justificación Operativa |
| --- | --- | --- | --- |
| **Lenguaje** | Rust (Edition 2024) | **1.95.0** | Ausencia de recolector de basura, huella de memoria mínima y binario estático único. |
| **DB Local** | SQLite (`sqlx`) | **0.8.6** | Persistencia transaccional embebida para buffering offline sin demonios adicionales. |
| **HTTP Client** | `reqwest` | **0.13.2** | Cliente asíncrono optimizado con soporte para `hyper-rustls`. |
| **Async Runtime** | `tokio` | **1.52.3** | Motor asíncrono con soporte LTS hasta marzo de 2027 configurado en modo `current_thread` para minimizar consumo. |
| **Configuración** | `toml` + `env` | **0.8.22** | Análisis rígido en tiempo de arranque, tipado fuerte y fail-fast. |
| **Logging** | `tracing` + `-subscriber` | **0.1.44** / **0.3.23** | Estructuración de logs para auditoría local y diagnóstico de red. |
| **Protocolo SNMP** | `snmp2` | **0.5.0** | Implementación nativa para recolección activa en capas de core switching. |
| **Protocolo ICMP** | `surge-ping` | **0.8.4** | Sondeo de latencia y disponibilidad mediante sockets asíncronos puros. |
| **Núcleo Sync** | `crates/sync/` (Rust) | Workspace | Reutilización de estructuras y tipos compartidos en el monorepo. |
| **Autenticación** | `pasetors` | **0.7.8** | Validación local asimétrica mediante tokens de seguridad PASETO v4.local. |

---

## 🏗️ Arquitectura de Red y Flujo Distribuidos

```
Sede Regional (Beni)
 ┌─────────┐    ┌─────────┐    ┌─────────┐
 │ Switch  │◄──►│ Router  │◄──►│   APs   │  (Infraestructura Local)
 │ (SNMP)  │    │ (SNMP)  │    │ (SNMP)  │
 └────┬────┘    └────┬────┘    └────┬────┘
      └───────────────┼──────────────┘
                      ▼
 ┌─────────────────────────────────────────┐
 │  Agente Rust (apps/agent)               │
 │  ├─ SNMP poller (cada 60s)              │
 │  ├─ ICMP discovery (cada 300s)          │
 │  ├─ ARP scan (cada 600s)                │
 │  ├─ SQLite local (buffering)            │
 │  ├─ Sync engine (crates/sync/)          │
 │  └─ HTTP client (reqwest 0.13.2)        │
 └──────────────────┬──────────────────────┘
                    │ HTTPS Outbound (Puerto 443 — Cuando hay WAN)
                    ▼
 ┌─────────────────────────────────────────┐
 │  API Central (apps/api)                 │
 │  ├─ Capa de Autenticación (PASETO v4)   │
 │  ├─ Endpoints de Estado (/api/v1/agent)  │
 │  ├─ Sea-ORM 1.1.x                       │
 │  └─ Persistencia Unificada (MySQL 8.0)   │
 └─────────────────────────────────────────┘

```

---

## 🔐 Autenticación y Seguridad del Agente

* **Restricción de Privilegios (Scopes):** El agente opera bajo un token firmado con el scope exclusivo `agent`. Tiene prohibido interactuar con endpoints destinados a usuarios (`user`) o administradores (`admin`).
* **Ciclo de Vida del Token:** Los tokens son emitidos desde el panel central de administración (`POST /admin/agents/{id}/rotate-token`), almacenándose el hash de control en la tabla centralizada de **MySQL 8.0**. Cuentan con una expiración rígida de 365 días.
* **Revocación en Caliente:** Cualquier anomalía física en la sede permite al administrador marcar el token como revocado (`revoked_at IS NOT NULL`) en la base de datos central, invalidando el middleware de la API de forma inmediata.

---

## 📡 Catálogo de Endpoints de Control

| Método | Endpoint | Propósito Técnico |
| --- | --- | --- |
| **POST** | `/api/v1/agent/heartbeat` | Reporte de estado de salud del hardware receptor + extracción de comandos encolados. |
| **POST** | `/api/v1/agent/metrics` | Ingesta masiva en bloques (*batching*) de telemetría e interfaces de red. |
| **POST** | `/api/v1/agent/devices` | Inventariado de hardware activo descubierto mediante escaneo pasivo y activo. |
| **POST** | `/api/v1/agent/intrusions` | Alertas tempranas sobre colisión de direcciones MAC no registradas en la lista blanca. |
| **GET** | `/api/v1/agent/config` | Descarga de directivas operativas específicas para el segmento de red asignado. |

---

## 🗄️ Esquema de Almacenamiento Local (SQLite)

```sql
-- Buffering de telemetría local ante cortes WAN
CREATE TABLE metric_readings_local (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    device_ip TEXT NOT NULL,
    device_id TEXT,
    bandwidth_rx BIGINT,
    bandwidth_tx BIGINT,
    latency_ms REAL,
    packet_loss_percent REAL,
    anomaly_detected BOOLEAN DEFAULT FALSE,
    collected_at INTEGER NOT NULL,
    synced_at INTEGER,
    sync_attempts INTEGER DEFAULT 0,
    api_response TEXT
);
CREATE INDEX idx_metrics_synced ON metric_readings_local(synced_at) WHERE synced_at IS NULL;

-- Tabla de control para mitigación de intrusos locales
CREATE TABLE discovered_devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ip TEXT NOT NULL,
    mac TEXT NOT NULL,
    hostname TEXT,
    vendor TEXT,
    device_type TEXT,
    first_seen_at INTEGER NOT NULL,
    last_seen_at INTEGER NOT NULL,
    status TEXT DEFAULT 'unknown' CHECK(status IN ('unknown','active','offline','new','intrusion')),
    synced BOOLEAN DEFAULT FALSE,
    is_whitelisted BOOLEAN DEFAULT FALSE
);
CREATE INDEX idx_discovered_mac ON discovered_devices(mac);
CREATE INDEX idx_discovered_status ON discovered_devices(status) WHERE status IN ('new','intrusion');

```

---

## 🚀 Despliegue de Infraestructura Inmutable

Alineado con nuestras directrices de aislamiento y la remoción total de Docker de los servidores de desarrollo, el empaquetado del agente se realiza de manera nativa mediante **Podman en modo Rootless**:

### Podmanfile

```dockerfile
FROM rust:1.95-alpine AS builder
RUN apk add --no-cache musl-dev openssl-dev sqlite-dev
WORKDIR /app
COPY . .
RUN cargo build --release --bin agent --profile release-size

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/agent /redes-agent
VOLUME ["/var/lib/redes-agent/data"]
ENV DATA_DIR=/var/lib/redes-agent/data
ENTRYPOINT ["/redes-agent"]

```

### Endurecimiento del Servicio Systemd (Podman integration)

```ini
[Service]
Type=simple
User=redes-agent
Group=redes-agent
Restart=always
RestartSec=5
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/lib/redes-agent/data
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true

```

---

## ✅ Pros vs. ⚠️ Trade-offs

### Ventajas Centrales

* **Aislamiento Perimetral Absoluto:** El agente funciona exclusivamente mediante peticiones salientes (*outbound*) HTTPS. No requiere la apertura de puertos entrantes en los firewalls regionales, neutralizando vectores de ataque externos.
* **Eficiencia Extrema de Recursos:** Consumo controlado en campo (**menor a 50MB de RAM** y **menor a 20MB de almacenamiento base**), ideal para terminales antiguos o mini-PCs reutilizadas en las gobernaciones provinciales.
* **Resiliencia Operativa:** Ante una caída total de los enlaces de telecomunicaciones en el Beni, el agente continúa reteniendo de forma segura las métricas locales gracias a los índices parciales configurados en SQLite.

### Limitaciones del Enfoque

* **Gestión de Configuración Distribuida:** Cada agente requiere el aprovisionamiento inicial explícito de sus variables de entorno locales (`sede_id`, llaves locales y comunidades SNMP).
* *Mitigación:* Centralización de directivas mediante el endpoint `/api/v1/agent/config` gobernado desde el panel de control del administrador en la capital.



---

## 📜 Decisiones Derivadas Consolidadas

* **Persistencia Centralizada Segura:** Se revoca cualquier mención histórica a PostgreSQL. La verificación de tokens de agentes (`agent_tokens`) se ejecuta transaccionalmente sobre **MySQL 8.0** mediante la capa de abstracción de **Sea-ORM 1.1.x**.
* **Motor Asíncrono Restringido:** El runtime de `tokio` en `apps/agent` se compila bajo la directiva de un único hilo de ejecución para evitar la penalización de rendimiento en arquitecturas ARMv7 y procesadores de bajo costo de las sedes.
* **Separación Estricta de Contextos de Sincronización:** El transporte de datos técnicos e intrusiones se procesa mediante las estructuras estricta de Rust en `crates/sync/`, manteniendo un desacoplamiento lógico total con las estrategias de sincronización del frontend escritas en TypeScript dentro de `apps/web/src/lib/sync/`.
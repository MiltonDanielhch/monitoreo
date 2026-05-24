# Resumen — ADR 0014: Monitoreo: Healthchecks.io + Patrón Dead Man's Switch
**Autores:** Milton Hipamo / Laboratorio 3030

**Relacionado con:** ADR 0004 (MySQL Backups), ADR 0012 (Tooling), ADR 0013 (Docker Compose), ADR 0015 ( tokio Jobs), ADR 0020 (Monitoreo Regional)

---

## 📋 Contexto y Decisión

**Problema:** Monitorear un VPS económico sin la sobrecarga de un equipo de operaciones dedicado 24/7. El monitoreo tradicional basado en agentes residentes que corren dentro del servidor sufre de un punto ciego fatal: si el VPS se apaga por completo, se queda sin memoria (OOM) o pierde conectividad de red, el agente interno queda inhabilitado y no puede emitir alertas de fallo.

**Decisión:** Adoptar oficialmente **Healthchecks.io** explotando el patrón de diseño estricto **"Dead Man's Switch" (Interruptor de Hombre Muerto)**. Bajo este modelo, es el servidor el que tiene la obligación de notificar externamente que completó una tarea en un intervalo de tiempo específico; si la notificación remota no llega dentro de la ventana de tiempo esperada, la plataforma externa asume el colapso del servicio y dispara alertas automáticas inmediatas hacia los canales de comunicación activos.

**Ventajas Operativas del Código 3026:**

* **Detección de Caída Total:** Si el servidor físico o VPS muere, la alerta se dispara de forma externa.
* **Aislamiento Absoluto de Recursos:** Consumo de recursos locales prácticamente inexistente (RAM ≈ 0, CPU ≈ 0), preservando la potencia del VPS para los binarios de Axum.
* **Monitoreo de Procesos en Segundo Plano:** Supervisa de forma efectiva la ejecución de tareas asíncronas y scripts de mantenimiento.
* **Eficiencia de Costos:** Se implementa sobre la capa gratuita del proveedor, cubriendo hasta 20 puntos de control (*checks*) independientes sin costo operativo.

---

## 🔄 Patrón Dead Man's Switch

El flujo operativo se basa en solicitudes HTTP rápidas condicionadas al éxito de la operación. Si el proceso falla, la cadena se rompe, se omite el ping de respuesta y la plataforma remota activa el protocolo de emergencia:

```bash
# Envío de ping condicionado estrictamente al éxito de la tarea
./respaldo_mysql_3026.sh && curl -fsS -m 10 --retry 5 https://hc-ping.com/{uuid}

# Si la lógica de negocio o la infraestructura fallan:
# -> El comando '&&' interrumpe la ejecución.
# -> No se emite la señal HTTP.
# -> Healthchecks.io detecta la ausencia y genera la alerta.

```

---

## 📊 Puntos de Control Configurados (Checks)

| Check Operativo | Intervalo Esperado | Período de Gracia | Severidad | Vector de Detección Crítico |
| --- | --- | --- | --- | --- |
| **MySQL Backup** | 1 hora | 15 minutos | Alta | Respaldo detenido, almacenamiento lleno, corrupción del contenedor MySQL o VPS apagado. |
| **Worker tokio** | 5 minutos | 2 minutos | Crítica | Hilo del worker asíncrono congelado, colas bloqueadas en memoria o caída total del runtime de Rust. |
| **TLS / SSL** | 24 horas | 2 horas | Media | Fallos en la renovación automática de certificados Let's Encrypt administrados por el proxy de Coolify. |
| **Deploy Pipeline** | Evento Manual | — | Informativa | Historial y auditoría temporal de despliegues exitosos en producción. |

---

## 🔧 Variables de Entorno del Toolchain

Las siguientes variables de entorno se gestionan de forma centralizada en el `mise.toml` de producción e inyectadas a los contenedores mediante el `docker-compose.yml`:

```bash
HC_API_KEY=                   # Clave de API opcional para la administración y provisión de nuevos checks.
HC_MYSQL_BACKUP_UUID=         # Identificador único para el script de respaldo automatizado de MySQL.
HC_DEPLOY_UUID=               # Identificador para la notificación de despliegue del Puente.
HC_TLS_UUID=                  # Identificador del monitor de certificados TLS.
HC_WORKER_UUID=               # Identificador del latido (heartbeat) del Worker de Apalis.

```

> **Principio de Tolerancia a Fallos:** El sistema operativo local y los binarios de Rust deben arrancar de manera limpia y continuar con sus tareas normales de negocio incluso si estas variables de entorno se encuentran ausentes o vacías.

---

## 🏗️ Arquitectura de Observabilidad Externa

```
┌──────────────────────────────────────────────┐
│             VPS / Entorno Docker             │
│                                              │
│  ┌────────────────────────────────────────┐  │
│  │               Rust API                 │──┼──── HTTP Ping ───▶ Healthchecks.io
│  ├────────────────────────────────────────┤  │                     │
│  │           tokio Async Worker          │──┤                     │ (Si falta el ping)
│  ├────────────────────────────────────────┤  │                     ▼
│  │      MySQL Backup Script (ADR 0004)    │──┤             Alertas Inmediatas:
│  └────────────────────────────────────────┘  │             [Telegram, WhatsApp, Email]
│                                              │
└──────────────────────────────────────────────┘

```

---

## 🛠️ Herramientas de Red y Observabilidad Aprobadas

| Crate / Herramienta | Función en el Ecosistema | Versión Mínima | Estado |
| --- | --- | --- | --- |
| `reqwest` | Cliente HTTP no bloqueante encargado de despachar los pings con políticas estrictas de timeout y reintentos. | `0.13.x` | ✅ Activa |
| `tokio` | Runtime asíncronono encargado de orquestar los intervalos de los temporizadores de monitoreo. | `1.45.x` | ✅ Activa |
| `tracing` | Recolección de logs estructurados y trazas de diagnóstico en el backend de Axum. | Ecosistema Workspace | ✅ Activa |
| Sentry Crons | Consolidación avanzada de errores y monitoreo visual de cronjobs en un dashboard unificado. | `0.48.x` | ⏳ Postergado |

---
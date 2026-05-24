# ADR 0015: Jobs Asíncronos mediante Canales Nativos Tokio (MPSC)
**Autores:** Milton Hipamo / Laboratorio 3030

**Relacionado con:** ADR 0003 (Backend Axum), ADR 0004 (MySQL), ADR 0012 (Tooling), ADR 0014 (Monitoreo)

---

## 📋 Contexto y Decisión

**Problema:** El sistema del Laboratorio 3030 requiere procesar operaciones costosas en tiempo y cómputo (envío de correos, procesamiento de métricas, sincronización de datos y alertas) en segundo plano, sin bloquear los hilos (*handlers*) de solicitudes HTTP de Axum. El uso de frameworks externos en fase Release Candidate introducía inestabilidad en la compilación, dependencias de bases de datos duplicadas y una complejidad innecesaria en el mantenimiento del monorepo.

**Decisión:** Rechazar el uso de Apalis y **adoptar de forma oficial canales asíncronos nativos `tokio::sync::mpsc` (Multi-Producer, Single-Consumer)** como el motor de colas y jobs del sistema. Las tareas se despachan desde los endpoints de Axum hacia hilos de ejecución (*workers*) dedicados que corren de manera segura en el trasfondo del mismo binario, utilizando nuestra base de datos **MySQL 8.0** existente como capa opcional para persistencia de auditoría si una tarea requiere tolerancia a fallos catastróficos.

**Ventajas del Enfoque Nativo (Código 3026):**

* **Cero Dependencias Inestables:** Eliminación total de crates en estado beta o RC. Se utiliza el núcleo maduro y estable de Tokio.
* **Rendimiento en Memoria:** El paso de mensajes a través de canales MPSC ocurre a velocidad de memoria nativa, sin llamadas de red ni escrituras obligatorias en disco por cada evento.
* **Control Total del Middleware:** El manejo de reintentos (*retries*), límites de concurrencia y límites de tasa (*rate limiting*) se implementa de forma directa con estructuras simples de Rust.
* **Aislamiento y Eficiencia:** Consumo de recursos de infraestructura nulo en frío.

---

## ⚙️ Arquitectura y Flujo de los Workers Nativos

El sistema inicializa los canales en el arranque del servidor, distribuyendo los extremos de envío (`Sender`) a los controladores de Axum y los extremos de recepción (`Receiver`) a bucles asíncronos persistentes:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        Ecosistema Binario Axum                         │
│                                                                        │
│   Endpoints HTTP Axum             Canal Tokio MPSC     Async Workers   │
│  ┌────────────────────┐  .send()  ┌──────────────┐    ┌──────────────┐ │
│  │ /api/v1/auth/login │──────────▶│ Email Buffer │───▶│ Email Loop   │ │
│  └────────────────────┘           └──────────────┘    └──────────────┘ │
│  ┌────────────────────┐  .send()  ┌──────────────┐    ┌──────────────┐ │
│  │ /api/v1/telemetry  │──────────▶│ Metrics Buf  │───▶│ Metrics Loop │ │
│  └────────────────────┘           └──────────────┘    └──────────────┘ │
└────────────────────────────────────────────────────────────────────────┘

```

### Matriz de Concurrencia Estructurada:

| Nombre del Worker | Capacidad del Buffer | Estrategia de Control | Propósito de Negocio |
| --- | --- | --- | --- |
| `email-worker` | 1000 mensajes | Límite de tasa (Rate Limit) | Despacho de notificaciones y tokens. |
| `metrics-worker` | 5000 mensajes | Agrupamiento (*Batching*) | Procesamiento y volcado de telemetría hacia MySQL. |
| `alert-worker` | 500 mensajes | Prioridad inmediata | Envío de alertas críticas de infraestructura. |
| `sync-worker` | 100 mensajes | Ejecución Secuencial | Sincronización e integridad de datos (ADR 0021). |

---

## 💻 Implementación de Referencia (Rust Puro)

La abstracción de un worker nativo se resuelve de forma limpia mediante un bucle supervisado por Tokio:

```rust
use tokio::sync::mpsc;
use tracing::{info, error, instrument};

// Definición estricta de los tipos de tareas
#[derive(Debug)]
pub enum JobCommand {
    SendWelcomeEmail { email: String, name: String },
    ProcessMetrics { payload: Vec<u8> },
}

// Inicializador del Worker Autónomo
pub fn spawn_job_worker(mut receiver: mpsc::Receiver<JobCommand>) {
    tokio::spawn(async move {
        info!("Worker asíncronono del Laboratorio iniciado con éxito");
        
        while let Some(command) = receiver.recv().await {
            execute_job(command).await;
        }
        
        info!("Canal cerrado de forma limpia. Finalizando Worker.");
    });
}

#[instrument(skip_all)]
async fn execute_job(command: JobCommand) {
    match command {
        JobCommand::SendWelcomeEmail { email, name } => {
            // Lógica nativa de reintentos con bucles estructurados
            let mut attempts = 0;
            while attempts < 3 {
                match email::send(&email, &name).await {
                    Ok(_) => break,
                    Err(e) => {
                        attempts += 1;
                        error!("Fallo en envío de correo (Intento {}): {:?}", attempts, e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }
        }
        JobCommand::ProcessMetrics { payload } => {
            // Volcado directo utilizando las conexiones de Sea-ORM hacia MySQL
            if let Err(e) = db::save_metrics(payload).await {
                error!("Error crítico al persistir métricas en MySQL: {:?}", e);
            }
        }
    }
}

```

---

## 🛡️ Resiliencia y Ciclo de Vida del Software

### 1 — Cierre Sincronizado y Confiable (Graceful Shutdown)

Cuando el servidor recibe una señal de apagado (`SIGTERM` o `SIGINT`), el backend de Axum deja de aceptar nuevas conexiones HTTP y los transmisores (`Sender`) de los canales se destruyen de forma ordenada. Los bucles de los workers continúan procesando los mensajes remanentes que ya se encontraban alojados en la memoria del buffer del canal, garantizando que ninguna tarea a medio procesar sea interrumpida de golpe.

### 2 — Gestión del Desbordamiento de Memoria (Backpressure)

Al utilizar canales con límites fijos de tamaño (`bounded channels`), si la cola de tareas se satura debido a un pico masivo de tráfico, el comando `.send().await` del endpoint suspende temporalmente la recepción de esa petición HTTP específica en lugar de agotar descontroladamente la memoria RAM del servidor.

### 3 — Observabilidad Integrada con Tracing

Cada ciclo de lectura del canal genera un bloque de diagnóstico (*tracing span*) que inyecta automáticamente metadatos en las salidas de logs, permitiendo auditar con exactitud el tiempo neto de ejecución de cada tarea en segundo plano.

---

## 🛠️ Herramientas y Crates Estables Autorizados

| Crate / Módulo | Rol en el Ecosistema del Laboratorio | Versión | Estado |
| --- | --- | --- | --- |
| `tokio::sync` | Canales nativos `mpsc` y primitivas de sincronización asíncrona. | `1.45.x` | ✅ Activa (Estable) |
| `tracing` | Instrumentación y logs estructurados de los ciclos de los workers. | Workspace | ✅ Activa (Estable) |
| `futures-util` | Herramientas de control de flujos de streams si se requiere procesamiento en lotes. | `0.3.x` | ✅ Activa (Estable) |

---
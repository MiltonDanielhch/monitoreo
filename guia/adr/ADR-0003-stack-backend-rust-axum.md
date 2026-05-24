# # Resumen — ADR 0003: Stack Backend: Rust 2024 + Axum 0.8 + Tokio
**Autores:** Milton Hipamo / Laboratorio 3030
---

## 📋 Contexto y Decisión

**Problema:** El sistema de monitoreo de infraestructura de red distribuida requiere un backend corporativo de alta eficiencia, capaz de procesar ráfagas masivas de métricas concurrentes (SNMP/ICMP) y mantener conexiones en tiempo real sin disparar el consumo de CPU ni requerir escalado horizontal costoso en servidores VPS limitados.

**Decisión:** Adoptar **Rust (Edition 2024) + Axum 0.8 + Tokio Runtime** como el núcleo tecnológico inmutable para el desarrollo del ecosistema backend.

### Protocolos Oficialmente Aprobados

* **REST (JSON):** Interfaz API primaria para operaciones síncronas, comandos de administración y mutaciones de estado.
* **SSE (Server-Sent Events):** Canal unidireccional nativo para el empuje de telemetría y alertas en tiempo real hacia el dashboard (Svelte 5).
* **HTTP/1.1 (Payloads Compactos):** Protocolo base y ultraligero para la ingesta de datos desde los agentes remotos distribuidos hacia el servidor central.

> **Tecnologías Excluidas (Baneadas):** Se prohíbe el uso de gRPC, WebSockets (salvo bypass explícito por control de hardware reactivo), GraphQL, y brokers pesados como Kafka o NATS, priorizando la simplicidad operacional y la compatibilidad con firewalls institucionales estrictos.

---

## 📦 Stack Tecnológico y Matriz de Dependencias

| Categoría | Crates Aprobados | Versión Base | Propósito Específico en el Sistema |
| --- | --- | --- | --- |
| **Runtime** | `tokio` | `1.x` (Estable) | Motor asíncrono multihilo con flags `rt-multi-thread`, `macros`, `signal` y `time`. |
| **Web Framework** | `axum` + `axum-extra` | `0.8.x` / `0.12.x` | Enrutamiento basado en macros avanzadas, extracción tipada de headers, queries y cookies seguras. |
| **Middleware** | `tower` + `tower-http` | `0.5.x` / `0.6.x` | Capa modular de orquestación HTTP: control estricto de CORS, compresión Gzip/Brotli, timeouts y trazabilidad de solicitudes (`request-id`). |
| **Serialización** | `serde` + `serde_json` | `1.0.x` | Deserialización segura en tiempo de compilación y manipulación eficiente de estructuras JSON de red. |
| **Observabilidad** | `tracing` + `tracing-subscriber` | `0.1.x` / `0.3.x` | Telemetría estructurada, filtrado dinámico por nivel de entorno (`env-filter`) y formateo JSON para auditorías. |
| **Persistencia** | `sea-orm` + `sqlx` | `1.1.x` / `0.8.x` | ORM asíncrono nativo configurado con el driver `runtime-tokio-rustls` exclusivo para **MySQL 8.0+**. |
| **Caché Local** | `moka` | `0.12.x` | Memoria caché asíncrona de alto rendimiento y concurrencia para mitigar consultas repetitivas a la base de datos. |
| **Seguridad / Auth** | `pasetors` + `argon2` | `0.7.x` / `0.5.x` | Generación de tokens criptográficos cripto-seguros PASETO v4 Local y hashing adaptativo de contraseñas con Argon2id. |
| **Gestión de Datos** | `secrecy` + `validator` | `0.10.x` / `0.20.x` | Enmascaramiento inmutable de credenciales en memoria y validaciones estructurales mediante macros distributivas. |
| **Cliente HTTP** | `reqwest` | `0.13.x` | Cliente asíncrono configurado con TLS nativo para comunicaciones salientes y sincronización externa. |
| **Documentación** | `utoipa` + `utoipa-scalar` | `5.x` / `0.4.x` | Generación automática de especificaciones OpenAPI 3.1 desde el código de Rust y visualizador interactivo Scalar. |
| **Tareas de Fondo** | `tokio` | `1.x` | Planificador de procesos asíncronos y colas de trabajo integradas nativamente sobre el motor MySQL. |
| **Pruebas (Test)** | `cargo-nextest` + `axum-test` | `0.9.x` / `17.x` | Suite industrial de pruebas paralelas, simulación integrada de Handlers y aislamiento de entornos lógicos. |

---

## 📜 Principios Arquitectónicos Adoptados

* **Binario Único (Monolito Compacto):** El backend se compila en un único artefacto ejecutable hiperficiente que contiene la API, las tareas en segundo plano y los adaptadores, reduciendo drásticamente la fricción en infraestructura.
* **Eficiencia Operacional Extrema:** Consumo garantizado en reposo inferior a **512 MB de RAM** para el servicio de API general, asegurando viabilidad total en VPS de prestaciones moderadas.
* **Asincronía Nativa Real:** Aprovechamiento total del modelo de hilos verdes (*Green Threads*) de Tokio para gestionar decenas de miles de conexiones concurrentes de red de manera simultánea sin bloqueos de kernel.
* **Garantía de Compilación:** Todo error de ruta, tipos de datos en la base de datos o contratos mal enlazados se intercepta durante la fase de compilación, impidiendo regresiones críticas en entornos vivos.

---

## 🔄 Protocolo de Apagado Controlado (Graceful Shutdown)

Ante la detección de señales del sistema operativo (`Ctrl+C` / `SIGINT` o `SIGTERM`), el motor Axum detendrá inmediatamente la aceptación de nuevas solicitudes entrantes y ejecutará de forma secuencial y atómica los siguientes pasos:

```
[Señal SIGTERM Detectada]
           │
           ▼
┌─────────────────────────────────────┐
│ 1. Drenar Conexiones HTTP y SSE     │ -> Finaliza respuestas pendientes limpiamente
└──────────┬──────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│ 2. Detener Workers Activos (Apalis) │ -> Cancela gracefully las colas de procesamiento
└──────────┬──────────────────────────┘
           │
           ▼
┌─────────────────────────────────────┐
│ 3. Cerrar Pool de Conexiones        │ -> Libera de forma ordenada las conexiones MySQL/Sea-ORM
└──────────┬──────────────────────────┘
           │
           ▼
   [Apagado Seguro del Proceso]

```

Este procedimiento garantiza un estado de consistencia absoluta en el disco, evitando la corrupción de métricas recolectadas a medias o la interrupción abrupta de transacciones operativas.
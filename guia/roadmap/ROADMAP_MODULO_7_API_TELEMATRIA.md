# 🗺️ Roadmap — Módulo 7: API de Telemetría e Integración de Agentes

> **Propósito:** Proveer la capa de ingestión de métricas de alta velocidad y bajo consumo mediante HTTP/2, permitiendo que los agentes de las sedes envíen reportes de rendimiento y estado de la red de forma segura y compacta.
> **Entregable:** Endpoints optimizados en Axum 0.8 que reciben lotes (*batches*) de telemetría, validación criptográfica de tokens de agentes, cola de procesamiento en memoria con canales concurrentes (`tokio::sync::mpsc`) y persistencia agregada mediante Sea-ORM.
> **Regla de Pureza:** El dominio no sabe qué es un JSON, un paquete HTTP o un puerto TCP. El dominio recibe estructuras puras de telemetría (Métricas de CPU, memoria, estado de interfaces, latencia, pérdidas de paquetes) y decide si violan los umbrales de salud para disparar alertas.
> **Stack:** Rust 2024 · Axum 0.8 (con soporte HTTP/2 via Rustls) · Tokio Channels · Sea-ORM 1.1 · Serde (De/Serialización ultra rápida) · Svelte 5 (Runes) · docker
> **Última Revisión:** Mayo 2026

---

## Estados

```
[ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

---

## Progreso General

| Slice | Nombre | Progreso |
| --- | --- | --- |
| 7.1 | Esquema de Muestreos y Registro de Agentes Regionales | [ ] |
| 7.2 | Modelos de Telemetría y Contratos de Ingestión (`crates/domain`) | [ ] |
| 7.3 | Repositorio de Métricas e Inserción Masiva (`crates/database`) | [ ] |
| 7.4 | Motor de Ingestión Asíncrona (Tokio Channels / Workers) | [ ] |
| 7.5 | Endpoints HTTP/2 Seguros con Validación de Token en Axum | [ ] |
| 7.6 | Dashboard de Estado de Conectividad de Agentes (Svelte 5 Runes) | [ ] |
| 7.7 | Gráficos de Telemetría en Tiempo Real por Sede | [ ] |
| 7.8 | Pruebas de Carga (Stress Testing), Pérdida de Enlace y Reintento | [ ] |
| **Módulo 7 Total** |  | [ ] |

---

## Slice 7.1: Esquema de Muestreos y Registro de Agentes Regionales 🔥

> **Objetivo:** Diseñar el almacenamiento relacional optimizado para series de tiempo cortas y la gestión de identidades de los agentes remotos.

```
[ ] Crear archivo de migración plano en `data/migrations/0007_agent_telemetry.sql`
    [ ] Definir tabla `remote_agents` (id, name, sede_id, auth_token_hash, status, last_seen_at, created_at)
    [ ] Definir tabla `agent_metrics_batch` (id, agent_id, cpu_usage, memory_usage, disk_usage, network_latency_ms, packet_loss_percentage, created_at)
    [ ] *Nota de Rendimiento:* Añadir índice compuesto de ordenación temporal: `idx_metrics_agent_date` en `(agent_id, created_at DESC)`. Las tablas de métricas deben limpiarse periódicamente o mantenerse optimizadas para evitar un crecimiento desmedido.

[ ] Ejecutar la migración dentro de tu contenedor de base de datos:
    [ ] Comando: docker exec -i redes-db-dev mysql -u redes -predes redes_dev < data/migrations/0007_agent_telemetry.sql

```

---

## Slice 7.2: Modelos de Telemetría y Contratos de Ingestión (`crates/domain`) 🔥

> **Objetivo:** Definir las estructuras de datos puras del negocio para validar la telemetría entrante de forma estricta.

```
[ ] Crear el archivo `crates/domain/src/models/telemetry.rs`
    [ ] Definir el struct de dominio `MetricBatch` que valide límites lógicos (ej. el uso de CPU no puede ser menor a 0 ni mayor a 100%; la pérdida de paquetes debe estar entre 0 y 100).
    [ ] Implementar lógica de dominio para evaluar la salud del agente: `MetricBatch::evaluate_health(&self) -> AgentHealthStatus` (Si la pérdida de paquetes es mayor al 15%, marcar como estado `Degraded`).
    [ ] Definir el trait/puerto `TelemetryRepositoryPort` para desacoplar la persistencia masiva de métricas.

```

---

## Slice 7.3: Repositorio de Métricas e Inserción Masiva (`crates/database`) 🔥

> **Objetivo:** Implementar operaciones de base de datos de alto rendimiento utilizando las capacidades de inserción por lotes (*Batch Insert*) de Sea-ORM.

```
[ ] Mapear las entidades en `crates/database/src/entities/`
    [ ] Crear `remote_agent_entity.rs` y `metrics_batch_entity.rs`.

[ ] Crear `crates/database/src/repositories/telemetry_repository.rs`
    [ ] Implementar `register_agent_heartbeat(agent_id)` para actualizar instantáneamente la columna `last_seen_at`.
    [ ] Implementar `insert_metrics_batch(batches: Vec<MetricBatch>)` utilizando la inserción masiva estructurada de Sea-ORM para minimizar los viajes (*round-trips*) a la base de datos de desarrollo.

```

---

## Slice 7.4: Motor de Ingestión Asíncrona (Tokio Channels / Workers) 🔥

> **Objetivo:** Evitar que las peticiones HTTP de los agentes bloqueen el servidor central, delegando el procesamiento a hilos secundarios eficientes.

```
[ ] Implementar un gestor de colas en memoria en `crates/infrastructure/src/telemetry/queue.rs`
    [ ] Definir un canal multiproductor, un solo consumidor (`tokio::sync::mpsc::channel`) global inicializado en el arranque del servidor.
    [ ] Escribir un Worker asíncrono (`tokio::spawn`) que escuche el canal continuamente, agrupe las métricas entrantes en ráfagas de memoria y dispare la inserción en la base de datos cada 2 segundos o al alcanzar 100 registros (patrón *Buffer/Flush*).

```

---

## Slice 7.5: Endpoints HTTP/2 Seguros con Validación de Token en Axum 🔥

> **Objetivo:** Exponer la API receptora usando las ventajas de rendimiento de HTTP/2 y protegiéndola contra accesos no autorizados.

```
[ ] Crear el controlador en `crates/infrastructure/src/handlers/telemetry_handler.rs`
    [ ] Implementar el endpoint `POST /api/v1/agent/telemetry`
        - Configurar Axum para requerir la cabecera `X-Agent-Token`.
        - Validar el token contra la caché de agentes o la base de datos de manera eficiente.
        - Extraer el payload JSON binario optimizado con Serde, validar e inyectar el lote directamente en el canal de Tokio (`mpsc_tx.send(...)`). Retornar inmediatamente un código HTTP 202 Accepted.
    [ ] Validar en el arranque de la infraestructura que Axum use TLS (via `axum-server::tls_rustls`) para forzar la negociación nativa de HTTP/2 multiplexado en el puerto seguro.

```

---

## Slice 7.6: Dashboard de Estado de Conectividad de Agentes (Svelte 5 Runes) 🔥

> **Objetivo:** Construir el panel de control central de telemetría que muestre el estado de comunicación con cada sede en tiempo real.

```
[ ] Crear la interfaz de infraestructura en `apps/web/src/routes/dashboard/telemetry/+page.svelte`
    [ ] Diseñar tarjetas de estado regionales utilizando Tailwind v4 (Verde = Conectado, Amarillo = Degradado, Rojo = Desconectado).
    [ ] Implementar la rune `$state` para mantener el inventario de agentes remotos y sus últimas estampas de tiempo (`last_seen_at`).
    [ ] Utilizar la rune `$derived` para calcular de manera dinámica cuántos agentes en total se encuentran caídos en el departamento del Beni sin necesidad de reprocesar el array manualmente.

```

---

## 7.7: Gráficos de Telemetría en Tiempo Real por Sede 🔥

> **Objetivo:** Mostrar de forma visual e intuitiva la fluctuación de métricas de red críticas para el operador de la Gobernación.

```
[ ] Crear el componente visual en `apps/web/src/lib/components/TelemetryChart.svelte`
    [ ] Implementar una gráfica lineal de rendimiento utilizando elementos SVG nativos controlados por Svelte, alimentada por los datos de rendimiento que el backend entrega de forma paginada o mediante consultas periódicas.
    [ ] Vincular propiedades reactivas con las Runes de Svelte 5 para actualizar el trazado del gráfico de manera suave cada vez que se actualice el estado local de datos.

```

Para darte una idea clara del comportamiento visual del monitoreo del sistema central, la UI mostrará las ráfagas de datos históricos con este nivel de detalle descriptivo:

* 14:15:00: Batch Recibido - Sede Riberalta
Ingreso exitoso de **120 métricas** de interfaces mediante HTTP/2. Latencia promedio del enlace regional: `42ms`. Pérdida de paquetes: `0%`. Estado del Agente: **Óptimo**.


* 14:14:32: Batch Recibido - Sede Guayaramerín
Ingreso de lote de telemetría. Latencia detectada: `118ms`. Pérdida de paquetes: `4.2%`. Estado del Agente: **Estable con degradación menor**.


* 14:10:15: Alerta de Latido - Sede San Borja
El agente central de San Borja no ha reportado un lote válido en los últimos 5 minutos. Marcando nodo regional de manera preventiva como **Desconectado / Inalcanzable**.


---

## Slice 7.8: Pruebas de Carga (Stress Testing), Pérdida de Enlace y Reintento 🔥

> **Objetivo:** Validar la robustez del receptor ante ráfagas masivas de tráfico de red concurrente.

```
[ ] Prueba 1 (Simulación de Concurrencia Extrema): Ejecutar un script de prueba en Rust que simule 50 agentes remotos enviando lotes de métricas simultáneamente cada 500ms sobre el canal HTTP/2. Comprobar que el consumo de memoria RAM en el servidor de Axum permanece plano y que la cola interna no desborda gracias al sistema de Tokio Workers.
[ ] Prueba 2 (Seguridad de Tokens): Intentar enviar un lote de métricas falsas alterando el token en la cabecera `X-Agent-Token`. El backend debe rechazar de inmediato la petición devolviendo un código HTTP 401 Unauthorized sin procesar el cuerpo del mensaje.

```

---

## Entregable del Módulo 7

Al culminar las tareas de este módulo, tu infraestructura contará con una pasarela de telemetría de nivel industrial. Los agentes de las sedes provinciales podrán reportar constantemente el estado de sus servidores y routers consumiendo el mínimo ancho de banda, dándote visibilidad analítica completa en tu centro de operaciones en Trinidad.

¡El motor de comunicaciones e integraciones queda estructurado bajo las directrices del **Código 3026**! Si deseas revisar o profundizar en la lógica de algún módulo anterior o refinar especificaciones, dime y nos ponemos en marcha de inmediato.
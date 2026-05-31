# 🗺️ Roadmap Actualizado — Módulo 7: API de Telemetría

### ⚡ Ingestión de Métricas de Alta Velocidad y Conectividad Provincial

```text
Propósito: Proveer la capa de ingestión de métricas de alta velocidad y bajo consumo mediante HTTP/2, permitiendo que los agentes de las sedes envíen reportes de rendimiento de red de forma segura y compacta.
Entregable: Endpoints optimizados en Axum 0.8 que reciben lotes (batches) de telemetría, cola concurrente en memoria con canales de Tokio (tokio::sync::mpsc) y persistencia masiva agregada en MySQL Workbench.
Regla de Pureza: El dominio no sabe de JSONs o sockets TCP. Recibe estructuras puras de telemetría (CPU, latencia, pérdida de paquetes) y evalúa si violan los umbrales para disparar alertas.
Estados: [ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

### 📊 Matriz de Progreso General

| Slice | Nombre | Referencia ADR | Progreso |
| --- | --- | --- | --- |
| **7.1** | Esquema de Muestreos y Series de Tiempo (Workbench) | `ADR-0004`, `ADR-0005` | [x] |
| **7.2** | Modelos de Telemetría y Contratos de Ingestión | `ADR-0001` | [x] |
| **7.3** | Repositorio de Métricas e Inserción Masiva (Sea-ORM) | `ADR-0004` | [x] |
| **7.4** | Motor de Ingestión Asíncrona (Tokio Channels) | `ADR-0015` | [x] |
| **7.5** | Endpoints HTTP/2 Seguros con Validación de Token | `ADR-0003`, `ADR-0006` | [x] |
| **7.6** | Dashboard de Conectividad de Agentes (Svelte 5 + UI) | `ADR-0017` [AGENTS] | [x] |
| **7.7** | Gráficos de Rendimiento SVG en Tiempo Real | `ADR-0017` [CHARTS] | [x] |
| **7.8** | Pruebas de Carga Concurrente y Validación de Token | `ADR-0010` | [x] |
| **M7** | **Módulo 7 Total** |  | **[x]** |

---

## Slice 7.1: Esquema de Muestreos y Agentes (MySQL Workbench) 🗄️

> **Objetivo:** Diseñar el almacenamiento relacional local en tu Workbench optimizado para series de tiempo y gestión de credenciales criptográficas de los agentes.

* [ ] **7.1.1 — Diseño del Archivo SQL:**
* Crear el archivo plano en `data/migrations/0007_agent_telemetry.sql`.
* Diseñar la tabla `remote_agents` (registro e identidad de las antenas/nodos provinciales) y la tabla `agent_metrics_batch` para capturar el rendimiento (CPU, memoria, latencia en ms y porcentaje de pérdida de paquetes).


* [ ] **7.1.2 — Optimización Temporal en Workbench:**
* Ejecutar el script directamente desde la consola de comandos de tu **MySQL Workbench**.
* Crear el índice compuesto temporal `idx_metrics_agent_date` en `(agent_id, created_at DESC)` para garantizar que las gráficas del frontend carguen al instante sin importar los millones de filas acumuladas.



---

## Slice 7.2: Modelos de Telemetría y Contratos de Ingestión 🧠

> **Objetivo:** Definir las estructuras puras del negocio en Rust para validar la consistencia lógica de las métricas que llegan de las provincias.

* [ ] **7.2.1 — Reglas de Salud del Dominio:**
* Crear la estructura `MetricBatch` limitando los valores lógicos (ej. los porcentajes de pérdida de paquetes deben oscilar rígidamente entre 0 y 100).
* Programar la función de dominio que evalúe el estado del enlace: si la pérdida de paquetes supera el 15%, marcar automáticamente el estado de la sede como `DEGRADED`.



---

## Slice 7.3: Repositorio de Métricas e Inserción Masiva (Sea-ORM) 🔌

> **Objetivo:** Desarrollar los accesos de datos usando la técnica de inserción por lotes (*Batch Insert*) de Sea-ORM para evitar saturar el disco local.

* [ ] **7.3.1 — Operaciones de Alto Rendimiento:**
* Mapear las nuevas entidades de telemetría hacia las tablas físicas creadas en Workbench.
* Implementar el método del repositorio que ejecute un único viaje (*round-trip*) insertando colecciones completas de métricas en un solo comando SQL estructurado.



---

## Slice 7.4: Motor de Ingestión Asíncrona (Tokio Channels) ⚙️

> **Objetivo:** Evitar que los reportes recurrentes de los ruteadores bloqueen el servidor web de Axum, derivando la carga a canales concurrentes en memoria RAM.

* [ ] **7.4.1 — Arquitectura Buffer/Flush:**
* Inicializar un canal asíncrono multiproductor (`tokio::sync::mpsc::channel`) en el arranque de la infraestructura.
* Programar el Worker en segundo plano que retenga las métricas entrantes en memoria RAM y aplique una descarga masiva hacia la base de datos local cada 2 segundos o al alcanzar un límite preestablecido de registros.



---

## Slice 7.5: Endpoints HTTP/2 Seguros con Validación de Token 🛣️

> **Objetivo:** Exponer la pasarela web de recepción aprovechando la multiplexación de HTTP/2 y blindándola contra suplantaciones.

* [ ] **7.5.1 — Handler de Ingestión Rápida:**
* Crear el endpoint `POST /api/v1/agent/telemetry` exigiendo la cabecera de seguridad `X-Agent-Token`.
* Extraer el JSON con Serde, enviarlo directamente al canal de Tokio en memoria y retornar de inmediato un estado HTTP `202 Accepted` al agente remoto para liberar la conexión en milisegundos.



---

## Slice 7.6: Dashboard de Estado de Agentes (Svelte 5 + TanStack Query) 🎨

> **Objetivo:** Construir la visualización de control de telemetría provincial en la ruta `/dashboard/telemetry`, mimetizándose con los paneles oscuros Zinc de tu captura de pantalla de "Redes Beni".

* [ ] **7.6.1 — Sincronización Inmediata con TanStack Query:**
* Configurar un `createQuery` que consulte el estado de actividad de los agentes remotos mediante un sondeo continuo y veloz.


* [ ] **7.6.2 — Grid de Estados Regionales:**
* Diseñar tarjetas con componentes de **shadcn-svelte** aplicando colores tácticos de Tailwind v4 basados en la última señal recibida: verde para canales óptimos, amarillo para enlaces degradados y rojo para desconexiones en provincias.


* [ ] **7.6.3 — Runes de Cómputo Derivado:**
* Utilizar la herramienta `$derived` de Svelte 5 para calcular al vuelo cuántos nodos del Beni se encuentran caídos en total, actualizando el banner de la cabecera al instante sin reprocesar variables manualmente.



---

## Slice 7.7: Gráficos de Rendimiento SVG en Tiempo Real (Svelte 5 + Zod) 📊

> **Objetivo:** Dibujar la fluctuación de rendimiento de forma gráfica utilizando componentes nativos ultraligeros de baja huella de procesamiento.

* [ ] **7.7.1 — Esquemas Estrictos de Validación:**
* Implementar contratos en el frontend con **Zod** para validar que las colecciones históricas de rendimiento que envía Axum contengan los valores numéricos precisos antes de alimentar el motor gráfico.


* [ ] **7.7.2 — Trazado Gráfico Dinámico con SVG:**
* Crear el componente visual utilizando polilíneas nativas de HTML/SVG manipuladas directamente por las Runes de Svelte 5, mostrando el flujo dinámico de la red de la Gobernación:
* **[Sede Riberalta]:** Ingreso exitoso de telemetría. Latencia: `42ms`. Pérdida: `0%`. Estado: **Óptimo**.
* **[Sede Guayaramerín]:** Latencia detectada: `118ms`. Pérdida: `4.2%`. Estado: **Estable con degradación menor**.
* **[Sede San Borja]:** Sin reportes en los últimos 5 minutos. Estado: **Desconectado / Inalcanzable**.





---

## Slice 7.8: Pruebas de Carga Concurrente y Validación de Token 🏁

> **Objetivo:** Forzar estrés controlado en el entorno local para asegurar que la cola en memoria sea invulnerable a ráfagas masivas.

* [ ] **7.8.1 — Simulación de Ráfaga Provincial:**
* Ejecutar una prueba concurrente desde la terminal para simular múltiples agentes remotos bombardeando el endpoint local cada 500ms. Validar desde tus consolas de Bacon que el consumo de memoria del backend permanece plano gracias al colchón de los canales de Tokio.


* [ ] **7.8.2 — Bloqueo de Token Falso:**
* Intentar inyectar métricas alterando de forma deliberada el valor de la cabecera `X-Agent-Token`. Verificar que Axum corta la transmisión de inmediato y responde con un código HTTP `401 Unauthorized` protegiendo tu servidor Workbench local.
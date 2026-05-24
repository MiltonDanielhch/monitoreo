# 🗺️ Roadmap — Módulo 8: Sistema de Tareas en Segundo Plano y Automatización

> **Propósito:** Orquestar de forma concurrente, tolerante a fallos y asíncrona los trabajos recurrentes de la infraestructura: pings de alta frecuencia, descubrimiento de red y mantenimiento del almacenamiento.
> **Entregable:** Cola de tareas configurada con Apalis, trabajadores distribuidos para sondeo ICMP/Ping, orquestador cron para el barrido SNMPv3 cada 6 horas y un ejecutor de pruning automático para mantener compactas las tablas de métricas.
> **Regla de Pureza:** Los workers de Apalis son simples adaptadores de infraestructura. Invocan casos de uso puros del dominio (ej. `PingHostUseCase`, `DiscoverNetworkUseCase`, `PruneMetricsUseCase`). Ningún componente del motor de tareas interactúa de forma directa con SQL crudo o sockets de red sin pasar por los puertos definidos.
> **Stack:** Rust 2024 · Apalis (Multi-storage Queue) · Tokio · Sea-ORM 1.1 · SNMPv3 Protocol Crate · docker
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
| 8.1 | Inicialización del Motor Apalis y Contexto Compartido | [ ] |
| 8.2 | Worker de Alta Frecuencia: Monitoreo ICMP / Ping Resiliente | [ ] |
| 8.3 | Worker de Descubrimiento Automático SNMPv3 (Cada 6 horas) | [ ] |
| 8.4 | Worker de Mantenimiento y Pruning de Datos Históricos | [ ] |
| 8.5 | Orquestador de Tareas Cron y Planificación de Horarios | [ ] |
| 8.6 | Dashboard de Monitoreo de Colas y Estado de Tareas (Svelte 5) | [ ] |
| 8.7 | Panel de Configuración de Umbrales de Depuración | [ ] |
| 8.8 | Pruebas de Recuperación ante Caídas de Base de Datos y Overload | [ ] |
| **Módulo 8 Total** |  | [ ] |

---

## Slice 8.1: Inicialización del Motor Apalis y Contexto Compartido 🔥

> **Objetivo:** Configurar la infraestructura base de Apalis compartiendo el pool de conexiones de Sea-ORM para la persistencia de las colas de trabajo.

```
[ ] Configurar dependencias en `crates/infrastructure/Cargo.toml` (`apalis`, `apalis-sql` o `apalis-redis`).
[ ] Crear el inicializador del motor en `crates/infrastructure/src/background/mod.rs`
    [ ] Definir el tipo `BackgroundContext` que herede de forma segura las instancias de tus repositorios y adaptadores de red.
    [ ] Configurar el arranque del Monitor de Apalis en el hilo principal de Tokio dentro de `apps/api/src/main.rs`.

```

---

## Slice 8.2: Worker de Alta Frecuencia: Monitoreo ICMP / Ping Resiliente 🔥

> **Objetivo:** Ejecutar ráfagas periódicas de pings hacia los routers y switches de las sedes provinciales para detectar pérdidas de enlace en menos de 30 segundos.

```
[ ] Crear la tarea en `crates/infrastructure/src/background/jobs/ping_job.rs`
    [ ] Definir la estructura `PingPayload { host_id: Uuid, ip_address: String }`.
    [ ] Implementar el trait `Job` de Apalis para esta estructura.
    [ ] Enlazar el flujo con el puerto del dominio encargado de medir la latencia. Si el host no responde 3 pings consecutivos, el worker debe invocar de inmediato el dominio de alertas para disparar el flujo del Módulo 6/7.

```

---

## Slice 8.3: Worker de Descubrimiento Automático SNMPv3 (Cada 6 horas) 🔥

> **Objetivo:** Escanear de forma segura los rangos de red asignados a las sedes (Riberalta, Guayaramerín, etc.) usando credenciales cifradas SNMPv3 para mapear nuevos dispositivos o interfaces.

```
[ ] Crear el job en `crates/infrastructure/src/background/jobs/snmp_discovery_job.rs`
    [ ] Definir `DiscoveryPayload { subred: String, Sede_id: Uuid }`.
    [ ] Integrar una biblioteca nativa de Rust para el manejo seguro de paquetes SNMPv3 (con soporte para autenticación SHA y privacidad AES).
    [ ] Programar el bucle de escaneo: interrogar las MIBs estándar (`sysName`, `sysDescr`, `ifTable`) de las IPs activas y registrar de forma segura los nuevos equipos mapeados en el inventario del sistema.

```

---

## Slice 8.4: Worker de Mantenimiento y Pruning de Datos Históricos 🔥

> **Objetivo:** Proteger el espacio en disco de los servidores en Trinidad eliminando o compactando métricas antiguas que superen las ventanas de retención operativas.

```
[ ] Crear la tarea en `crates/infrastructure/src/background/jobs/pruning_job.rs`
    [ ] Definir la regla de negocio inyectada desde el dominio: Retener métricas crudas por un máximo de 30 días, compactar promedios por hora hasta 90 días, y eliminar definitivamente registros que excedan ese umbral.
    [ ] Implementar la consulta optimizada de borrado masivo por lotes en Sea-ORM: `DELETE FROM agent_metrics_batch WHERE created_at < X LIMIT 5000` para evitar bloqueos prolongados en las tablas activas.

```

---

## Slice 8.5: Orquestador de Tareas Cron y Planificación de Horarios 🔥

> **Objetivo:** Definir las frecuencias temporales exactas de ejecución utilizando extensiones basadas en expresiones Cron para Apalis.

```
[ ] Enlazar las tareas recurrentes en `crates/infrastructure/src/background/scheduler.rs`:
    [ ] Configurar la ejecución del `PingJob` en intervalos continuos de alta frecuencia (ej. cada 60 segundos por dispositivo).
    [ ] Configurar la expresión cron del `SnmpDiscoveryJob` para ejecutarse estrictamente cada 6 horas (`0 */6 * * *`).
    [ ] Configurar la ejecución del `PruningJob` para correr de forma silenciosa en la madrugada, idealmente a las 02:00 AM hora local (`0 2 * * *`).

```

---

## Slice 8.6: Dashboard de Monitoreo de Colas y Estado de Tareas (Svelte 5) 🔥

> **Objetivo:** Crear una vista de control operativa que permita a los administradores verificar la salud de los trabajadores en segundo plano y el volumen de tareas procesadas.

```
[ ] Diseñar la interfaz en `apps/web/src/routes/dashboard/jobs/+page.svelte`
    [ ] Estructurar una tabla analítica que desglose las colas activas de Apalis (Nombre de la cola, tareas exitosas, fallidas, en ejecución, latencia de procesamiento).
    [ ] Usar las Runes `$state` y `$derived` para actualizar los contadores reactivos globales en el cliente a medida que el pool de hilos procesa los lotes de tareas.

```

Para asegurar el control y visibilidad del procesamiento en segundo plano del nodo central, la interfaz presentará el flujo de ejecución de los workers de la siguiente manera:

* 14:16:00: Ejecución de Rutina de Sondeo
El motor **Apalis** procesó de forma exitosa **45 tareas de Ping simultáneas** hacia las subredes regionales. Tiempo promedio de respuesta del worker: `12ms`. Cero fallos reportados.


* 12:00:00: Ciclo SNMPv3 Completado
Finalizó el barrido periódico de descubrimiento automático en la sede **Riberalta**. Se identificaron `2 nuevos switches` perimetrales y se actualizaron `24 descripciones de interfaces`.


* 02:00:15: Mantenimiento de Datos (Pruning)
El optimizador de almacenamiento ejecutó la depuración programada. Se purgaron de forma segura `145,230 registros de métricas obsoletas` de más de 30 días de antigüedad. Espacio liberado en disco: `48 MB`.


---

## Slice 8.7: Panel de Configuración de Umbrales de Depuración 🔥

> **Objetivo:** Permitir a los ingenieros ajustar los tiempos de retención de datos en caliente para equilibrar rendimiento y espacio de almacenamiento.

```
[ ] Crear la sección en `apps/web/src/routes/dashboard/jobs/settings/+page.svelte`
    [ ] Diseñar controles interactivos utilizando componentes estilizados de Tailwind para modificar los valores de retención (días de métricas crudas, días de logs de auditoría, etc.).
    [ ] Asegurar que al guardar los cambios, se actualice el estado persistente en la configuración del backend, impactando directamente el comportamiento del próximo ciclo del `PruningJob`.

```

---

## Slice 8.8: Pruebas de Recuperación ante Caídas de Base de Datos y Overload 🔥

> **Objetivo:** Forzar condiciones de falla extrema para garantizar que el sistema de tareas recupere su estado de manera automática sin corromper datos.

```
[ ] Prueba 1 (Persistencia y Reintento de Tareas): Simular una falla de red bloqueando temporalmente el acceso a la base de datos mientras hay 500 tareas de ping en cola. Validar que Apalis retiene los trabajos en memoria o en su storage temporal y los reintenta de forma progresiva con una estrategia de retraso exponencial (*Exponential Backoff*) una vez se restablece la conexión.
[ ] Prueba 2 (Aislamiento de Recursos de CPU): Monitorear la ejecución del escaneo SNMPv3 masivo junto al proceso de pruning. Comprobar que los workers de Apalis corren en hilos de Tokio limitados (`worker_threads`), evitando que un pico de carga en segundo plano congele las respuestas HTTP de los endpoints de la API web de Axum.

```

---

## Entregable del Módulo 8

Al completar este módulo, tu núcleo de software tendrá automatizadas todas sus rutinas de mantenimiento e inspección. El servidor central en Trinidad operará de manera autónoma, sondeando constantemente los enlaces provinciales, detectando hardware nuevo y auto-limpiándose diariamente para garantizar estabilidad por años sin intervención manual.

¡El motor asíncrono y distribuido del **Código 3026** queda completamente estructurado! Dime, Milton, ¿cómo procedemos a partir de aquí o qué componente refinamos a continuación?
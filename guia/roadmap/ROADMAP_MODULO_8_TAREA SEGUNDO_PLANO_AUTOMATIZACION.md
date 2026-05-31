# 🗺️ Roadmap Actualizado — Módulo 8: Automatización y Tareas

### ⚙️ Orquestador de Tareas en Segundo Plano y Mantenimiento Autónomo

```text
Propósito: Orquestar de forma concurrente, tolerante a fallos y asíncrona los trabajos recurrentes de la infraestructura: pings de alta frecuencia, descubrimiento de red y mantenimiento del almacenamiento.
Entregable: Cola de tareas configurada con Apalis, trabajadores de sondeo ICMP/Ping, orquestador cron para barrido SNMPv3 cada 6 horas y ejecutor de pruning automático integrado con MySQL Workbench.
Regla de Pureza: Los workers de Apalis son adaptadores de infraestructura. Invocan casos de uso puros del dominio (Ping, Discovery, Prune). Ningún componente interactúa con SQL crudo o sockets directamente.
Estados: [ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

### 📊 Matriz de Progreso General

| Slice | Nombre | Referencia ADR | Progreso |
| --- | --- | --- | --- |
| **8.1** | Inicialización del Motor Apalis y Pool de Datos (Workbench) | `ADR-0004`, `ADR-0015` | [ ] |
| **8.2** | Worker de Alta Frecuencia: Monitoreo ICMP / Ping Resiliente | `ADR-0001`, `ADR-0012` | [ ] |
| **8.3** | Worker de Descubrimiento Automático SNMPv3 (Cada 6 horas) | `ADR-0001`, `ADR-0014` | [ ] |
| **8.4** | Worker de Mantenimiento y Pruning de Datos Históricos | `ADR-0004`, `ADR-0009` | [ ] |
| **8.5** | Orquestador de Tareas Cron y Planificación de Horarios | `ADR-0015` | [ ] |
| **8.6** | Dashboard de Monitoreo de Colas y Estado (Svelte 5 + UI) | `ADR-0017` [JOBS] | [ ] |
| **8.7** | Panel de Configuración de Umbrales de Depuración | `ADR-0017` [SETTINGS] | [ ] |
| **8.8** | Pruebas de Recuperación ante Cortes de Base de Datos y Overload | `ADR-0010` | [ ] |
| **M8** | **Módulo 8 Total** |  | **[ ]** |

---

## Slice 8.1: Inicialización de Apalis y Contexto Compartido 🗄️

> **Objetivo:** Configurar la infraestructura base de Apalis en Rust conectando el pool de almacenamiento nativo de tu base de datos para gestionar las colas de trabajo pendientes.

* [ ] **8.1.1 — Migración de Tablas de Control:**
* Crear el archivo de migración `data/migrations/0008_apalis_jobs.sql`.
* Definir la estructura de tablas que requiere Apalis para el control de estados de tareas (`pending`, `running`, `failed`, `done`) y ejecutarlas en tu **MySQL Workbench**.


* [ ] **8.1.2 — Contexto del Servidor:**
* Configurar en `crates/infrastructure/src/background/mod.rs` el `BackgroundContext` que compartirá de manera segura el pool de conexiones del backend hacia los hilos asíncronos de Tokio dentro de `apps/api/src/main.rs`.



---

## Slice 8.2: Worker de Alta Frecuencia: Monitoreo ICMP / Ping Resiliente 🧠

> **Objetivo:** Ejecutar ráfagas periódicas de pings en segundo plano hacia los ruteadores provinciales del Beni para actualizar el Dashboard en tiempo real.

* [ ] **8.2.1 — Job de Sondeo ICMP:**
* Definir en Rust la estructura `PingPayload { host_id: String, ip_address: String }` e implementar el trait `Job` de Apalis.
* Conectar el flujo con el puerto del dominio: si un equipo falla 3 pings seguidos, invocar de inmediato el canal de alertas para notificar la caída en la interfaz web de forma instantánea.



---

## Slice 8.3: Worker de Descubrimiento Automático SNMPv3 (Cada 6 horas) 🔌

> **Objetivo:** Escanear las subredes de la Gobernación de forma segura usando credenciales cifradas SNMPv3 para catalogar hardware nuevo de manera automática.

* [ ] **8.3.1 — Barrido de Equipos por Red:**
* Programar el payload `DiscoveryPayload { subnet: String, sede_id: String }` utilizando librerías nativas de Rust con soporte para cifrado SHA/AES.
* Mapear el bucle de escaneo para interrogar las MIBs estándar e inyectar los nuevos dispositivos descubiertos en los repositorios del Módulo 3.



---

## Slice 8.4: Worker de Mantenimiento y Pruning de Datos Históricos ⚙️

> **Objetivo:** Resguardar la salud del almacenamiento local en Trinidad depurando de forma programada los millones de registros antiguos de telemetría.

* [ ] **8.4.1 — Rutina de Depuración por Lotes:**
* Implementar las reglas de retención: mantener métricas crudas por 30 días, promedios compactados por 90 días y purgar el excedente de forma permanente.
* Programar la sentencia de borrado optimizado por lotes en Sea-ORM (`DELETE FROM agent_metrics_batch WHERE ... LIMIT 5000`) para que tu base de datos en Workbench no sufra bloqueos de tablas mientras el sistema opera.



---

## Slice 8.5: Orquestador de Tareas Cron y Planificación de Horarios 🛣️

> **Objetivo:** Configurar las expresiones de tiempo exactas para activar los hilos de ejecución automáticos en el backend de Rust.

* [ ] **8.5.1 — Configuración Cron de Infraestructura:**
* Programar en `crates/infrastructure/src/background/scheduler.rs` los intervalos de tiempo:
* `PingJob`: Ejecución continua de alta frecuencia (ej. cada 30 o 60 segundos por equipo).
* `SnmpDiscoveryJob`: Expresión cron para ejecutarse rígidamente cada 6 horas (`0 */6 * * *`).
* `PruningJob`: Activación silenciosa en la madrugada a las 02:00 AM hora de Bolivia (`0 2 * * *`).





---

## Slice 8.6: Dashboard de Monitoreo de Colas (Svelte 5 + TanStack Query) 🎨

> **Objetivo:** Diseñar la consola de visualización operativa de tareas en la ruta `/dashboard/jobs` utilizando componentes oscuros de **shadcn-svelte**.

* [ ] **8.6.1 — Estado de Colas con TanStack Query:**
* Implementar `createQuery` para monitorear el rendimiento del motor Apalis, capturando cuántas tareas se procesan con éxito o fallan por minuto.


* [ ] **8.6.2 — Consola de Control Reactiva:**
* Utilizar las Runes `$state` y `$derived` para pintar indicadores de rendimiento suaves. El panel presentará el flujo con descripciones quirúrgicas de rendimiento técnico:
* **[Sondeo de Red]:** Apalis procesó exitosamente *45 tareas de Ping simultáneas*. Tiempo de respuesta promedio: `12ms`.
* **[Ciclo SNMPv3]:** Barrido completado en *Sede Riberalta*. Descubiertos `2 nuevos switches` perimetrales.
* **[Mantenimiento de Almacenamiento]:** Depuración ejecutada a las 02:00 AM. Purgados `145,230 registros obsoletos`. Espacio liberado en disco: `48 MB`.





---

## Slice 8.7: Panel de Configuración de Umbrales de Depuración 📊

> **Objetivo:** Proveer un panel técnico interactivo para que los ingenieros ajusten los tiempos de retención de datos en caliente.

* [ ] **8.7.1 — Contratos de Entrada con Zod:**
* Usar esquemas de **Zod** en el frontend para validar que los valores numéricos ingresados por el operador (días de retención, límites de reintentos) cumplan con los límites de seguridad antes de ser enviados a la API.


* [ ] **8.7.2 — Guardado Dinámico en Caliente:**
* Diseñar la UI interactiva en la ruta `/dashboard/jobs/settings`. Al guardar, disparar una mutación hacia Axum que actualice los parámetros de configuración, alterando el comportamiento del próximo ciclo del `PruningJob` inmediatamente sin reiniciar el servidor.



---

## Slice 8.8: Pruebas de Recuperación ante Cortes de Base de Datos 🏁

> **Objetivo:** Forzar fallos controlados a nivel local para certificar la resiliencia absoluta de tus colas de hilos de trabajo.

* [ ] **8.8.1 — Prueba de Retraso Exponencial (Backoff):**
* Simular una desconexión interrumpiendo brevemente el acceso al servidor de datos mientras hay tareas en ejecución. Certificar que Apalis retiene los payloads de ping de forma segura y activa reintentos progresivos con *Exponential Backoff* una vez que Workbench vuelve a estar en línea.


* [ ] **8.8.2 — Control de Hilos de Tokio:**
* Lanzar en paralelo el descubrimiento SNMPv3 masivo y la limpieza de datos. Verificar desde tus consolas de desarrollo que los procesos se ejecutan de manera aislada en hilos secundarios limitados (`worker_threads`), evitando que el consumo de hardware congele el tráfico HTTP de las rutas web del Dashboard principal.

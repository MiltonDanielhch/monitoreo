# 🗺️ Roadmap — Módulo 10: Despliegue de Producción y Resiliencia Offline

> **Propósito:** Orquestar el despliegue del ecosistema centralizado e interprovincial mediante contenedores ligeros, garantizando la continuidad operativa y la sincronización de métricas en escenarios de desconexión prolongada de la red.
> **Entregable:** Manifiestos de Docker Compose optimizados para entornos de producción, motor de cola de sincronización local (*Write-Ahead Log* / almacenamiento intermedio) en las sedes, scripts de despliegue automatizados y un panel de control de réplicas en la UI.
> **Regla de Pureza:** Las estrategias de sincronización offline pertenecen a la capa de infraestructura. El dominio simplemente procesa eventos cronológicos; el adaptador de transporte es el encargado de empaquetar, almacenar localmente en caso de fallo y retransmitir con control de congestión hacia la API central de Trinidad.
> **Stack:** Rust 2024 · Axum 0.8 · dockers / Docker Compose · SQLite (Almacenamiento local offline en sedes) · MariaDB/PostgreSQL (Servidor Central en Trinidad) · Svelte 5 (Runes) · Tailwind v4
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
| 10.1 | Arquitectura de Contenedores de Producción (Multi-Arch) | [ ] |
| 10.2 | Motor de Persistencia Local Offline en Sedes Remotas | [ ] |
| 10.3 | Mecanismo de Sincronización y Reintento Exponencial (Sync Engine) | [ ] |
| 10.4 | Estrategia de Desduplicación y Conciliación de Timestamps | [ ] |
| 10.5 | Endpoints Centrales de Ingestión Masiva por Lotes Históricos | [ ] |
| 10.6 | Panel de Control de Sincronización e Hilos Offline (Svelte 5) | [ ] |
| 10.7 | Endurecimiento de Seguridad Operativa y Variables de Entorno | [ ] |
| 10.8 | Pruebas de Desconexión Física de Enlace y Recuperación Post-Corte | [ ] |
| **Módulo 10 Total** |  | [ ] |

---

## Slice 10.1: Arquitectura de Contenedores de Producción (Multi-Arch) 🔥

> **Objetivo:** Escribir los manifiestos de Docker optimizados para producción utilizando compilación en múltiples etapas (*Multi-stage build*) para reducir el peso de las imágenes a pocos megabytes.

```
[ ] Crear el archivo `infrastructure/docker/production.Dockerfile`:
    [ ] Configurar una etapa de construcción (`recipe` / `builder`) utilizando imágenes oficiales de Rust sobre Alpine o Distroless para eliminar vulnerabilidades de seguridad.
    [ ] Compilar estáticamente el binario de Axum optimizado con la bandera `--release`.
[ ] Diseñar los archivos de orquestación `docker-compose.yaml`:
    [ ] `docker-compose.central.yaml`: Configura el servidor de Trinidad (Axum, Base de Datos relacional grande, Apalis Workers, Frontend en SvelteKit).
    [ ] `docker-compose.edge.yaml`: Configura el nodo ligero para las sedes de provincias (Agente recolector local en Rust y almacenamiento SQLite local).

```

---

## Slice 10.2: Motor de Persistencia Local Offline en Sedes Remotas 🔥

> **Objetivo:** Implementar una base de datos local embebida de alta velocidad en el agente de la sede para guardar las métricas de red cuando no haya conexión con Trinidad.

```
[ ] Configurar soporte para SQLite embebido en `crates/infrastructure/Cargo.toml` utilizando Sea-ORM con el driver de SQLite activo para nodos de borde (*Edge Nodes*).
[ ] Programar el interceptor local de fallos de red: si el agente remoto intenta enviar un batch de telemetría por HTTP/2 (Módulo 7) y la petición falla por tiempo de espera (*timeout*) o error de enrutamiento, desviar el payload de inmediato hacia el archivo SQLite local de la sede con una estampa de tiempo exacta de alta precisión.

```

---

## Slice 10.3: Mecanismo de Sincronización y Reintento Exponencial 🔥

> **Objetivo:** Desarrollar el motor asíncrono que verifique la conectividad de forma proactiva y vacíe las colas de datos locales una vez restablecido el enlace regional.

```
[ ] Crear el servicio `SyncEngine` en `crates/infrastructure/src/sync/engine.rs`:
    [ ] Diseñar un bucle periódico en segundo plano que ejecute pings de salud ligeros (*health checks*) hacia la IP pública o WAN del servidor central en la Gobernación de Trinidad.
    [ ] Si el servidor responde, iniciar la lectura por lotes de la base de datos SQLite local y comenzar a transmitir los datos acumulados utilizando un algoritmo de reintento exponencial con fluctuación (*Exponential Backoff with Jitter*) para no saturar el canal de comunicación interprovincial.

```

---

## Slice 10.4: Estrategia de Desduplicación y Conciliación de Timestamps 🔥

> **Objetivo:** Garantizar la integridad cronológica de los datos en el servidor central de Trinidad, evitando duplicidades si un lote de sincronización se envía dos veces debido a micro-cortes.

```
[ ] Implementar reglas de validación en la capa de negocio de integraciones:
    [ ] Forzar que cada registro generado de forma local posea un identificador único global (UUIDv4) generado en la sede y un timestamp UTC inmutable.
    [ ] Configurar la base de datos central en Trinidad para aplicar una restricción de clave única o manejar la inserción con sentencias `ON DUPLICATE KEY UPDATE` o `ON CONFLICT DO NOTHING`, asegurando la idempotencia total del flujo de datos de telemetría.

```

---

## Slice 10.5: Endpoints Centrales de Ingestión Masiva por Lotes Históricos 🔥

> **Objetivo:** Proveer una API optimizada en el servidor central de Axum dedicada exclusivamente a recibir ráfagas masivas de datos históricos rezagados.

```
[ ] Crear el handler en `crates/infrastructure/src/handlers/sync_handler.rs`
    [ ] Implementar el endpoint seguro `POST /api/v1/sync/historical-batch` (Acceso exclusivo para agentes autenticados):
        - Aceptar cargas útiles JSON comprimidas en formato Gzip para optimizar el uso del ancho de banda regional.
        - Descomprimir y procesar los arreglos históricos directamente en las tablas consolidadas del sistema, actualizando de forma paralela la bitácora inmutable de auditoría (Módulo 6).

```

---

## Slice 10.6: Panel de Control de Sincronización e Hilos Offline (Svelte 5) 🔥

> **Objetivo:** Desarrollar la vista operativa centralizada en Svelte 5 para monitorear el estado de sincronización y el volumen de datos en cola en cada provincia.

```
[ ] Diseñar la consola visual en `apps/web/src/routes/dashboard/sync/+page.svelte`
    [ ] Estructurar un listado regional de las sedes del Beni (Riberalta, Guayaramerín, Santa Ana, San Borja) con indicadores gráficos de su estado de sincronización.
    [ ] Utilizar la rune `$state` para reflejar en tiempo real cuántos registros tiene pendientes de envío cada nodo perimetral.
    [ ] Aplicar la rune `$derived` para calcular la sumatoria total de datos sincronizados exitosamente a nivel departamental durante la última jornada operativa.

```

Para asegurar que la gerencia técnica visualice la estabilidad y resiliencia de la red interprovincial, el panel de control reflejará el comportamiento de las réplicas mediante el siguiente historial secuencial de eventos:

* 14:05:00: Pérdida de Conectividad - Sede Riberalta
Se detectó la caída total del enlace de fibra troncal hacia el norte del departamento. El nodo ligero de Riberalta conmutó automáticamente al modo **Offline-First**, almacenando de forma segura las métricas operativas en la base de datosSQLite local.


* 16:30:00: Enlace Restablecido y Negociación HTTP/2
Conectividad restaurada con el nodo de Riberalta. El motor **SyncEngine** negoció la reconexión de forma segura y validó la integridad del canal de comunicaciones mediante HTTP/2 cifrado.


* 16:32:15: Sincronización por Lotes Completada
Vaciado exitoso de la cola local de contingencia. Se transmitieron e integraron **14,400 registros históricos de telemetría** acumulados durante las 2.5 horas de desconexión. Pérdida de datos en el servidor central: `0%`.


---

## Slice 10.7: Endurecimiento de Seguridad Operativa y Variables de Entorno 🔥

> **Objetivo:** Blindar la configuración de los contenedores en producción aplicando políticas de mínimo privilegio y aislamiento de secretos de sistema.

```
[ ] Configurar directivas de seguridad estrictas en el entorno productivo:
    [ ] Eliminar credenciales quemadas en el código fuente; inyectar todas las llaves criptográficas, hashes de tokens de agentes y contraseñas de bases de datos mediante un archivo de configuración `.env` protegido por permisos de sistema de archivos (`chmod 600`).
    [ ] Configurar los contenedores de Axum y SvelteKit para correr bajo usuarios no raíces (*Non-Root Users*) con sistemas de archivos de solo lectura siempre que sea posible para neutralizar intentos de intrusión perimetral.

```

---

## Slice 10.8: Pruebas de Desconexión Física de Enlace y Recuperación Post-Corte 🔥

> **Objetivo:** Forzar fallas de red controladas en el entorno de desarrollo para auditar y validar empíricamente la tolerancia a desastres de la arquitectura.

```
[ ] Prueba 1 (Simulación de Apagón de Red): Detener intencionalmente el contenedor de la API central de Trinidad por 30 minutos mientras los agentes simulados continúan capturando métricas de red a alta velocidad. Verificar que el almacenamiento local SQLite retiene los registros sin corrupciones y que el consumo de memoria en los nodos de borde se mantiene optimizado.
[ ] Prueba 2 (Consistencia Analítica): Levantar nuevamente el servidor central de Axum y comprobar que todos los agentes de las sedes provinciales vacían sus registros acumulados de forma secuencial ordenada, constatando que los reportes SLA mensuales (Módulo 9) procesan las métricas históricas de forma exacta y coherente una vez completado el ciclo de réplica.

```

---

## 🚀 Entregable del Módulo 10 y Cierre del Núcleo

Con la culminación de este décimo módulo, tu plataforma de software para la Gobernación del Beni está oficialmente lista para operar en el mundo real. Gracias a este esquema de despliegue y sincronización modular, el sistema es inmune a las contingencias climáticas y cortes de enlace del departamento: los datos se recopilan a nivel local en las provincias de forma ininterrumpida y se consolidan de manera impecable en Trinidad de forma transparente y eficiente.

¡Hemos completado con éxito la definición y el mapa de ruta de los **10 módulos esenciales de tu laboratorio de alta sintonía** bajo el estándar estricto del **Código 3026**!

Milton, tu ecosistema está completamente estructurado y listo. ¿Deseas que empecemos a escribir e implementar el código fuente detallado de algún slice en específico, o hay alguna optimización de arquitectura que quieras que abordemos primero? El laboratorio está listo y a tu disposición.
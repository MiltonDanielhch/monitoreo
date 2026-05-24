# 🗺️ Roadmap — Módulo 4: Notificaciones y Alertas en Segundo Plano

> **Propósito:** Construir el motor de procesamiento y despacho asíncrono de alertas críticas del sistema de monitoreo, garantizando que el envío de correos o mensajería no bloquee el hilo de ejecución web y aislando el dominio de los protocolos de red (SMTP/HTTP).
> **Entregable:** Panel de historial de alertas reactivo en Svelte 5, procesamiento asíncrono con Apalis (u otra cola respaldada por la base de datos) y despacho de notificaciones mediante plantillas dinámicas parametrizables.
> **Regla de Pureza:** El dominio no conoce qué es un cliente SMTP, qué es `lettre`, qué es la API de Telegram o cómo se serializa un JSON. El dominio solo entiende eventos de red que requieren difusión, plantillas de texto y contratos de despacho.
> **Stack:** Rust 2024 · Axum 0.8 · Apalis (o SQL-Queue alternative) · Sea-ORM 1.1 · Lettre (SMTP) · SvelteKit 2 · Svelte 5 (Runes) · Tailwind v4 · Docker
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
| 4.1 | Esquema y Migración SQL de Alertas | [ ] |
| 4.2 | Eventos de Notificación y Errores del Dominio (`crates/domain`) | [ ] |
| 4.3 | Entidades y Repositorios de Persistencia (`crates/database`) | [ ] |
| 4.4 | Adaptadores de Salida (SMTP/Lettre) y Lógica de Colas (`crates/infrastructure`) | [ ] |
| 4.5 | Endpoints de Historial y Desencadenadores en Axum | [ ] |
| 4.6 | UI de Historial de Alertas y Estado Reactivo (Svelte 5 Runes) | [ ] |
| 4.7 | Formulario de Configuración de Canales y Test-Email | [ ] |
| 4.8 | Pruebas de Carga de la Cola, Reintentos y Fallo de Red | [ ] |
| **Módulo 4 Total** |  | [ ] |

---

## Slice 4.1: Esquema y Migración SQL de Alertas 🔥

> **Objetivo:** Diseñar el modelo de datos relacional para persistir los canales, las plantillas preconfiguradas y el histórico de ejecuciones/reintentos de notificaciones.

```
[ ] Crear archivo de migración plano en `data/migrations/0004_notification_engine.sql`
    [ ] Definir tabla `notification_channels` (id, name, type [EMAIL/TELEGRAM], credentials_secure_json, is_active, created_at)
    [ ] Definir tabla `notification_templates` (slug [PRIMARY KEY], title_template, body_template, description, updated_at)
    [ ] Definir tabla `notification_logs` (id, channel_id, recipient, template_slug, context_json, status [PENDING/SENT/FAILED], attempts, error_message, created_at, processed_at)
    [ ] Configurar llaves foráneas indexadas hacia `notification_channels` y `notification_templates` aplicando ON DELETE RESTRICT

[ ] Ejecutar la migración dentro del contenedor de la base de datos:
    [ ] Comando: docker exec -i redes-db-dev mysql -u redes -predes redes_dev < data/migrations/0004_notification_engine.sql

[ ] Insertar Semillas (Seeds) iniciales para las alertas institucionales:
    [ ] INSERT INTO notification_templates para los slugs: 'critical_node_down', 'unauthorized_mac_detected', 'high_latency_warning' con variables en formato dual `{{host}}` o `{{latency}}`.

[ ] Verificar integridad física:
    [ ] Comando: docker exec -it redes-db-dev mysql -u redes -predes redes_dev -e "DESCRIBE notification_logs;"

```

---

## Slice 4.2: Eventos de Notificación y Errores del Dominio (`crates/domain`) 🔥

> **Objetivo:** Modelar de forma pura las necesidades del negocio de alertas, aislando el dominio de los mecanismos de transporte.

```
[ ] Actualizar `crates/domain/src/errors.rs`
    [ ] Añadir variantes a `DomainError`: `TemplateNotFound`, `InvalidRecipient`, `NotificationDispatchFailed(String)`, `MaxRetriesExceeded`

[ ] Crear `crates/domain/src/models/notification.rs`
    [ ] Definir enum puro `NotificationPayload` con variantes estructuradas (ej. `NodeDown { host: String, ip: String, latency: u32 }`)
    [ ] Definir struct de dominio `NotificationDispatcher` y contratos de validación de destinatarios (expresiones regulares puras para emails corporativos o IDs de chat)
    [ ] Definir el trait/puerto `NotificationQueuePort` que declare la operación `enqueue_notification(log_id: &str) -> Result<(), DomainError>`

[ ] Validar compilación aislada:
    [ ] Ejecutar: cargo check -p domain

```

---

## Slice 4.3: Entidades y Repositorios de Persistencia (`crates/database`) 🔥

> **Objetivo:** Implementar los accesos de datos para leer el contexto de la alerta, procesar plantillas y persistir transiciones de estado de la cola.

```
[ ] Mapear entidades con Sea-ORM en `crates/database/src/entities/`
    [ ] Generar/crear `notification_channel_entity.rs`, `notification_template_entity.rs` y `notification_log_entity.rs`
    [ ] Definir relaciones estructurales directas (un log pertenece a un canal y a una plantilla)

[ ] Crear `crates/database/src/repositories/notification_repository.rs`
    [ ] Implementar método `NotificationRepository::create_log(...)` para registrar el disparo inicial en estado `PENDING`
    [ ] Implementar método `NotificationRepository::update_status(id, status, error_msg)` para transicionar estados en la BD
    [ ] Implementar consulta paginada de logs optimizada con orden descendente por `created_at` para la UI

```

---

## Slice 4.4: Adaptadores de Salida (SMTP/Lettre) y Lógica de Colas (`crates/infrastructure`) 🔥

> **Objetivo:** Programar el software de infraestructura que interactúa con la red física y el motor de tareas asíncronas en background (Apalis).

```
[ ] Configurar dependencias en `crates/infrastructure/Cargo.toml`
    [ ] Agregar: `lettre` (con features de TLS nativo), `apalis` (o tu abstracción de workers sobre DB) y `handlebars` (para compilación ligera de texto)

[ ] Crear el servicio de renderizado y transporte en `crates/infrastructure/src/services/mailer.rs`
    [ ] Implementar la integración de `lettre::SmtpTransport` alimentada por las credenciales descifradas del canal
    [ ] Utilizar `Handlebars` interno para inyectar el `context_json` del log dentro del texto crudo de la plantilla física de la BD

[ ] Configurar el Worker de Background en `crates/infrastructure/src/jobs/notification_job.rs`
    [ ] Definir la estructura de la tarea asíncrona (`NotificationTask { log_id: String }`)
    [ ] Programar la función ejecutora del worker: consume de la cola, extrae el log, compila la plantilla, dispara mediante `Lettre` y actualiza el estado de la BD a `SENT`
    [ ] Implementar política de captura de pánicos y reintentos: si el servidor SMTP de la Gobernación rebota temporalmente, incrementar `attempts` y posponer la ejecución

```

---

## Slice 4.5: Endpoints de Historial y Desencadenadores en Axum 🔥

> **Objetivo:** Exponer la telemetría del estado de los envíos a la API y proveer un mecanismo manual de prueba para los administradores.

```
[ ] Crear `crates/infrastructure/src/handlers/notification_handler.rs`
    [ ] Definir DTOs de salida: `NotificationLogResponse` y esquemas de paginación estructurados
    [ ] Implementar el endpoint `GET /api/v1/notifications/logs` (Protegido con el extractor RBAC del Módulo 1 en rol `ADMIN` u `OPERATOR`)
    [ ] Implementar el endpoint `POST /api/v1/notifications/test-smtp` que fuerce de forma síncrona el envío de una alerta fantasma a un correo de pruebas para validar la conexión del servidor local de Trinidad

[ ] Montar las rutas en el enrutador general `crates/infrastructure/src/router.rs` bajo el prefijo correspondiente

```

---

## Slice 4.6: UI de Historial de Alertas y Estado Reactivo (Svelte 5 Runes) 🔥

> **Objetivo:** Crear el centro de monitoreo visual de notificaciones, mostrando fallos de envío de forma proactiva.

```
[ ] Crear/Modificar la ruta de interfaz en `apps/web/src/routes/dashboard/notifications/+page.svelte`
    [ ] Diseñar la tabla principal del historial usando Tailwind v4 (columnas: Destinatario, Tipo Alerta, Canal, Intentos, Estado, Fecha)
    [ ] Implementar la rune `$state` para manejar la lista de logs del servidor y el estado de paginación
    [ ] Utilizar una lógica reactiva para aplicar estilos de Tailwind basados en el estado (`status`): badge verde suave para `sent`, amarillo animado para `pending`, y rojo con tooltip de error para `failed`
    [ ] Agregar un botón de refresco manual que limpie el estado y vuelva a llamar al API usando `fetch` con cabeceras Bearer

```

---

## Slice 4.7: Formulario de Configuración de Canales y Test-Email 🔥

> **Objetivo:** Proveer interfaz interactiva para alterar los parámetros de red de los servidores de envío sin tocar código fuente.

```
[ ] Diseñar componente de configuración en `apps/web/src/routes/dashboard/notifications/settings/+page.svelte`
    [ ] Crear el formulario reactivo en Svelte 5 para los parámetros SMTP (Host, Puerto, Usuario, Contraseña, Encriptación TLS/SSL)
    [ ] Enlazar las variables del formulario usando la sintaxis nativa de bindeo de Runes
    [ ] Implementar un botón secundario "Probar Conexión" que dispare una petición al endpoint `/test-smtp`, capture los errores crudos devuelvos por Rust y los renderice en un bloque de código monospaciado en caso de error de red o credenciales

```

---

## Slice 4.8: Pruebas de Carga de la Cola, Reintentos y Fallo de Red 🔥

> **Objetivo:** Garantizar que el subsistema de notificaciones es inmune a las interrupciones imprevistas y no satura los recursos del servidor del Beni.

```
[ ] Prueba 1 (Simulación offline): Desactivar localmente el acceso al puerto SMTP de pruebas, disparar una alerta y confirmar mediante base de datos que el log pasa a estado `FAILED` reflejando el mensaje exacto del socket caído de red.
[ ] Prueba 2 (Concurrencia masiva): Inyectar 100 alertas simultáneas simuladas en la base de datos. Verificar con `bacon` o telemetría que el servidor de Axum sigue respondiendo peticiones HTTP en menos de 5ms mientras la cola procesa en background de forma lineal sin bloquear la API principal.
[ ] Prueba 3 (Flujo feliz E2E): Activar credenciales válidas, simular un evento de red, validar almacenamiento inmediato del log, encolamiento automático, despacho limpio de correo y actualización visual asíncrona reflejada en la UI de Svelte 5.

```

---

## Entregable del Módulo 4

Al finalizar este slice, tu software de control regional procesará cualquier contingencia crítica de infraestructura de manera desacoplada. Ninguna lentitud o caída del servidor de correos institucional congelará el backend de Axum ni afectará la experiencia del operador en la interfaz de Svelte 5. El motor de notificaciones operará de forma autónoma y con control total de auditoría.
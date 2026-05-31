# Módulo 11: Detección de Intrusiones y Seguridad 🔒

> **Objetivo:** Implementar un sistema de detección de intrusiones y eventos de seguridad para monitorear amenazas en la red interprovincial de la Gobernación del Beni.
>
> **Entregable:** Sistema de detección de intrusiones con tabla de eventos de seguridad, modelos de dominio, repositorio, endpoints HTTP y dashboard de visualización de amenazas.
>
> **Regla de Pureza:** El dominio no sabe de HTTP o JSONs. Recibe estructuras puras de eventos de seguridad (IP origen, MAC, tipo de intrusión, severidad) y evalúa si requieren acción inmediata.
>
> **Estados:** [ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

---

## 📊 Matriz de Progreso General

| Slice | Nombre | Referencia ADR | Progreso |
| --- | --- | --- | --- |
| **11.1** | Esquema de Eventos de Seguridad (Workbench) | `ADR-0004`, `ADR-0005` | [ ] |
| **11.2** | Modelos de Seguridad y Contratos de Detección | `ADR-0001` | [ ] |
| **11.3** | Repositorio de Eventos de Seguridad (Sea-ORM) | `ADR-0004` | [ ] |
| **11.4** | Motor de Detección de Patrones Anómalos | `ADR-0015` | [ ] |
| **11.5** | Endpoints HTTP/2 Seguros para Eventos de Seguridad | `ADR-0003`, `ADR-0006` | [ ] |
| **11.6** | Dashboard de Intrusiones (Svelte 5 + UI) | `ADR-0017` [INTRUSIONS] | [ ] |
| **11.7** | Correlación de Eventos con Telemetría | `ADR-0017` [CORRELATION] | [ ] |
| **11.8** | Pruebas de Detección y Validación de Alertas | `ADR-0010` | [ ] |
| **M11** | **Módulo 11 Total** |  | **[ ]** |

---

## Slice 11.1: Esquema de Eventos de Seguridad (MySQL Workbench) 🗄️

> **Objetivo:** Diseñar el almacenamiento relacional local en tu Workbench optimizado para eventos de seguridad y detección de intrusiones.

* [ ] **11.1.1 — Diseño del Archivo SQL:**
  * Crear el archivo plano en `data/migrations/0011_security_events.sql`.
  * Diseñar la tabla `security_events` (registro de eventos de intrusión) con campos:
    * `id` (VARCHAR(255) PRIMARY KEY)
    * `event_type` (VARCHAR(100) - tipo de intrusión: port_scan, ddos, unauthorized_access, etc.)
    * `severity` (VARCHAR(20) - critical, high, medium, low)
    * `status` (VARCHAR(50) - detected, investigating, resolved, false_positive)
    * `source_ip` (VARCHAR(45) - IP origen del ataque)
    * `source_mac` (VARCHAR(17) - MAC address origen)
    * `target_device_id` (VARCHAR(255) - dispositivo objetivo)
    * `target_sede_id` (VARCHAR(255) - sede objetivo)
    * `description` (TEXT - descripción del evento)
    * `metadata` (TEXT - JSON con detalles adicionales)
    * `detected_at` (DATETIME - timestamp de detección)
    * `resolved_at` (DATETIME NULL - timestamp de resolución)
    * `resolved_by` (VARCHAR(255) NULL - usuario que resolvió)
    * `created_at` (DATETIME - timestamp de creación)
  * Crear índices en:
    * `idx_security_events_severity` (severity)
    * `idx_security_events_status` (status)
    * `idx_security_events_detected_at` (detected_at)
    * `idx_security_events_source_ip` (source_ip)
    * `idx_security_events_target_device` (target_device_id)
  * Ejecutar el script en MySQL Workbench y verificar que se creen las tablas correctamente.

---

## Slice 11.2: Modelos de Seguridad y Contratos de Detección 📦

> **Objetivo:** Definir los modelos de dominio para eventos de seguridad, tipos de intrusión, severidad y el trait SecurityPort con métodos asincrónicos.

* [ ] **11.2.1 — Definir Enums y Estructuras en `crates/domain/src/models/security.rs`:**
  * Crear el archivo `crates/domain/src/models/security.rs`.
  * Definir el enum `IntrusionType` con variantes:
    * `PortScan` (escaneo de puertos)
    * `DDoS` (ataque de denegación de servicio)
    * `UnauthorizedAccess` (acceso no autorizado)
    * `MalwareDetection` (detección de malware)
    * `Phishing` (intento de phishing)
    * `DataExfiltration` (exfiltración de datos)
    * `Other(String)` (otro tipo)
  * Definir el enum `Severity` con variantes:
    * `Critical` (crítico)
    * `High` (alto)
    * `Medium` (medio)
    * `Low` (bajo)
  * Definir el enum `SecurityStatus` con variantes:
    * `Detected` (detectado)
    * `Investigating` (investigando)
    * `Resolved` (resuelto)
    * `FalsePositive` (falso positivo)
  * Definir la struct `SecurityEvent` con campos:
    * `id: String`
    * `event_type: IntrusionType`
    * `severity: Severity`
    * `status: SecurityStatus`
    * `source_ip: String`
    * `source_mac: Option<String>`
    * `target_device_id: Option<String>`
    * `target_sede_id: Option<String>`
    * `description: String`
    * `metadata: Option<serde_json::Value>`
    * `detected_at: String` (RFC3339)
    * `resolved_at: Option<String>` (RFC3339)
    * `resolved_by: Option<String>`
  * Implementar métodos en `SecurityEvent`:
    * `new()` - constructor que genera ID único
    * `validate_ip()` - valida formato de IP
    * `validate_mac()` - valida formato de MAC
    * `is_critical()` - retorna true si severity es Critical
    * `mark_as_resolved()` - cambia estado a Resolved
    * `mark_as_false_positive()` - cambia estado a FalsePositive
  * Definir el trait `SecurityPort` con métodos asincrónicos:
    * `async fn log_event(&self, event: SecurityEvent) -> Result<(), DomainError>`
    * `async fn get_events(&self, filters: SecurityFilters) -> Result<Vec<SecurityEvent>, DomainError>`
    * `async fn get_event_by_id(&self, id: String) -> Result<Option<SecurityEvent>, DomainError>`
    * `async fn resolve_event(&self, id: String, resolved_by: String) -> Result<(), DomainError>`
    * `async fn mark_false_positive(&self, id: String) -> Result<(), DomainError>`
    * `async fn get_events_by_severity(&self, severity: Severity) -> Result<Vec<SecurityEvent>, DomainError>`
    * `async fn get_events_by_status(&self, status: SecurityStatus) -> Result<Vec<SecurityEvent>, DomainError>`
    * `async fn get_events_by_device(&self, device_id: String) -> Result<Vec<SecurityEvent>, DomainError>`
  * Definir la struct `SecurityFilters` con campos opcionales para filtros:
    * `severity: Option<Severity>`
    * `status: Option<SecurityStatus>`
    * `source_ip: Option<String>`
    * `target_device_id: Option<String>`
    * `target_sede_id: Option<String>`
    * `date_from: Option<String>`
    * `date_to: Option<String>`
  * Agregar el módulo `security` en `crates/domain/src/models/mod.rs` y exportar los tipos.

---

## Slice 11.3: Repositorio de Eventos de Seguridad (Sea-ORM) 🗄️

> **Objetivo:** Crear las entidades Sea-ORM para la tabla `security_events` e implementar el repositorio SecurityRepository que implementa el trait SecurityPort.

* [ ] **11.3.1 — Crear Entidad Sea-ORM en `crates/database/src/entities/security_event_entity.rs`:**
  * Crear el archivo `crates/database/src/entities/security_event_entity.rs`.
  * Definir la entidad `SecurityEvent` con los campos correspondientes a la tabla `security_events`.
  * Usar `Decimal` para campos numéricos si aplica.
  * Usar `DateTime` para timestamps.
  * Almacenar `metadata` como `Option<String>` (JSON como string).
  * Implementar `ActiveModelBehavior` para la entidad.
* [ ] **11.3.2 — Registrar la Entidad en `crates/database/src/entities/mod.rs`:**
  * Agregar `pub mod security_event_entity;`
  * Agregar `pub use security_event_entity::Entity as SecurityEvent;`
* [ ] **11.3.3 — Crear Repositorio en `crates/database/src/repositories/security_repository.rs`:**
  * Crear el archivo `crates/database/src/repositories/security_repository.rs`.
  * Definir la struct `SecurityRepository` con `db: DatabaseConnection`.
  * Implementar el trait `SecurityPort` para `SecurityRepository`.
  * Implementar `log_event()`:
    * Convertir `SecurityEvent` a `security_event_entity::ActiveModel`.
    * Serializar `metadata` a JSON string.
    * Insertar en la base de datos.
    * Manejar errores y mapear a `DomainError`.
  * Implementar `get_events()`:
    * Construir query con filtros opcionales.
    * Usar `QueryFilter` para aplicar filtros.
    * Ordenar por `detected_at` descendente.
    * Convertir modelos de Sea-ORM a entidades de dominio.
    * Deserializar `metadata` de JSON string.
  * Implementar `get_event_by_id()`:
    * Buscar por ID usando `find_by_id()`.
    * Convertir a entidad de dominio o retornar None.
  * Implementar `resolve_event()`:
    * Buscar evento por ID.
    * Actualizar `status` a `Resolved`, `resolved_at` a timestamp actual, `resolved_by` al usuario.
    * Guardar cambios.
  * Implementar `mark_false_positive()`:
    * Buscar evento por ID.
    * Actualizar `status` a `FalsePositive`.
    * Guardar cambios.
  * Implementar `get_events_by_severity()`:
    * Filtrar por severidad.
    * Retornar lista de eventos.
  * Implementar `get_events_by_status()`:
    * Filtrar por estado.
    * Retornar lista de eventos.
  * Implementar `get_events_by_device()`:
    * Filtrar por `target_device_id`.
    * Retornar lista de eventos.
* [ ] **11.3.4 — Registrar el Repositorio en `crates/database/src/repositories/mod.rs`:**
  * Agregar `pub mod security_repository;`
  * Agregar `pub use security_repository::SecurityRepository;`
* [ ] **11.3.5 — Exportar desde `crates/database/src/lib.rs`:**
  * Agregar `pub use repositories::SecurityRepository;`
* [ ] **11.3.6 — Verificar Compilación:**
  * Ejecutar `cargo check -p database` y corregir errores.

---

## Slice 11.4: Motor de Detección de Patrones Anómalos 🔍

> **Objetivo:** Implementar un motor de detección de patrones anómalos usando Tokio Channels para procesar eventos de seguridad de forma asíncrona.

* [ ] **11.4.1 — Crear Motor de Detección en `crates/infrastructure/src/security/detection_engine.rs`:**
  * Crear el archivo `crates/infrastructure/src/security/detection_engine.rs`.
  * Definir el enum `DetectionMessage`:
    * `AnalyzeTraffic(TrafficData)` - analizar tráfico de red
    * `CheckDevice(DeviceData)` - verificar dispositivo
    * `Shutdown` - detener motor
  * Definir la struct `DetectionEngine` con:
    * `tx: mpsc::Sender<DetectionMessage>`
    * `_handle: tokio::task::JoinHandle<()>`
  * Implementar `new()` que crea el motor con canales de Tokio.
  * Implementar el worker principal que:
    * Recibe mensajes del canal.
    * Analiza patrones anómalos:
      * Escaneo de puertos (múltiples conexiones a diferentes puertos desde misma IP)
      * DDoS (alto volumen de conexiones desde múltiples IPs)
      * Acceso no autorizado (intentos de login fallidos)
      * Tráfico anómalo (picos inusuales de ancho de banda)
    * Genera eventos de seguridad cuando detecta patrones sospechosos.
    * Envía eventos al repositorio de seguridad.
  * Implementar `analyze_traffic()` para enviar datos de tráfico al motor.
  * Implementar `check_device()` para verificar estado de dispositivo.
  * Implementar `shutdown()` para detener el motor.
* [ ] **11.4.2 — Crear Módulo en `crates/infrastructure/src/security/mod.rs`:**
  * Agregar `pub mod detection_engine;`
  * Exportar tipos.
* [ ] **11.4.3 — Agregar Módulo a `crates/infrastructure/src/lib.rs`:**
  * Agregar `pub mod security;`
* [ ] **11.4.4 — Verificar Compilación:**
  * Ejecutar `cargo check -p infrastructure` y corregir errores.

---

## Slice 11.5: Endpoints HTTP/2 Seguros para Eventos de Seguridad 🌐

> **Objetivo:** Implementar los endpoints HTTP optimizados en Axum 0.8 para recibir y gestionar eventos de seguridad con validación de token.

* [ ] **11.5.1 — Crear Handler en `crates/infrastructure/src/handlers/security_handler.rs`:**
  * Crear el archivo `crates/infrastructure/src/handlers/security_handler.rs`.
  * Definir DTOs para request/response:
    * `SecurityEventRequest` - para crear evento de seguridad
    * `SecurityEventResponse` - respuesta de éxito
    * `SecurityEventsPaginatedResponse` - lista paginada de eventos
    * `SecurityEventFilters` - filtros para query
  * Implementar `log_security_event()`:
    * Extraer y validar token de autorización.
    * Validar permisos de usuario.
    * Convertir request a `SecurityEvent`.
    * Validar IP y MAC.
    * Llamar a `security_repo.log_event()`.
    * Retornar respuesta de éxito.
  * Implementar `get_security_events()`:
    * Extraer y validar token.
    * Aplicar filtros de query.
    * Llamar a `security_repo.get_events()`.
    * Retornar lista paginada de eventos.
  * Implementar `get_security_event_by_id()`:
    * Extraer y validar token.
    * Llamar a `security_repo.get_event_by_id()`.
    * Retornar evento o 404.
  * Implementar `resolve_security_event()`:
    * Extraer y validar token.
    * Validar permisos de usuario.
    * Llamar a `security_repo.resolve_event()`.
    * Retornar respuesta de éxito.
  * Implementar `mark_false_positive()`:
    * Extraer y validar token.
    * Validar permisos de usuario.
    * Llamar a `security_repo.mark_false_positive()`.
    * Retornar respuesta de éxito.
  * Implementar `get_events_by_severity()`:
    * Extraer y validar token.
    * Llamar a `security_repo.get_events_by_severity()`.
    * Retornar lista de eventos.
* [ ] **11.5.2 — Registrar Handler en `crates/infrastructure/src/handlers/mod.rs`:**
  * Agregar `pub mod security_handler;`
  * Exportar funciones.
* [ ] **11.5.3 — Agregar Rutas en `crates/infrastructure/src/lib.rs`:**
  * Agregar rutas:
    * `POST /api/v1/security/events` - log_security_event
    * `GET /api/v1/security/events` - get_security_events
    * `GET /api/v1/security/events/{id}` - get_security_event_by_id
    * `PUT /api/v1/security/events/{id}/resolve` - resolve_security_event
    * `PUT /api/v1/security/events/{id}/false-positive` - mark_false_positive
    * `GET /api/v1/security/events/severity/{severity}` - get_events_by_severity
* [ ] **11.5.4 — Verificar Compilación:**
  * Ejecutar `cargo check -p infrastructure` y corregir errores.

---

## Slice 11.6: Dashboard de Intrusiones (Svelte 5 + UI) 🎨

> **Objetivo:** Crear el dashboard de visualización de intrusiones con Svelte 5 y TanStack Query para mostrar eventos de seguridad en tiempo real.

* [ ] **11.6.1 — Crear Página en `apps/web/src/routes/dashboard/intrusions/+page.svelte`:**
  * Crear el archivo `apps/web/src/routes/dashboard/intrusions/+page.svelte`.
  * Implementar query con `createQuery` para obtener eventos de seguridad.
  * Configurar `refetchInterval` para actualización en tiempo real (cada 30 segundos).
  * Implementar filtros por severidad, estado, IP origen, dispositivo.
  * Mostrar estadísticas:
    * Total de eventos
    * Eventos críticos
    * Eventos en investigación
    * Eventos resueltos
  * Mostrar tabla de eventos con:
    * Tipo de intrusión
    * Severidad (con colores)
    * Estado (con badges)
    * IP origen
    * MAC
    * Dispositivo objetivo
    - Timestamp de detección
  * Implementar acciones:
    - Resolver evento
    - Marcar como falso positivo
    - Ver detalles
  * Usar componentes UI: Card, Badge, Button, Table.
  * Implementar paginación.
* [ ] **11.6.2 — Crear Página de Detalles en `apps/web/src/routes/dashboard/intrusions/[id]/+page.svelte`:**
  * Crear el archivo `apps/web/src/routes/dashboard/intrusions/[id]/+page.svelte`.
  * Implementar query para obtener evento por ID.
  * Mostrar detalles completos del evento.
  - Mostrar metadata JSON formateado.
  - Mostrar timeline de acciones (detección, investigación, resolución).
  - Implementar botones de acción.
* [ ] **11.6.3 — Agregar Ruta al Sidebar en `apps/web/src/lib/components/layout/Sidebar.svelte`:**
  * Agregar `{ href: '/dashboard/intrusions', label: 'Intrusiones', icon: ShieldAlert, description: 'Detección de intrusiones y eventos de seguridad' }`
  - Agregar icono `ShieldAlert` a los imports.
* [ ] **11.6.4 — Verificar Compilación:**
  * Ejecutar `npm run check` en `apps/web` y corregir errores.

---

## Slice 11.7: Correlación de Eventos con Telemetría 🔗

> **Objetivo:** Implementar la correlación de eventos de seguridad con métricas de telemetría para detectar patrones complejos.

* [ ] **11.7.1 — Crear Servicio de Correlación en `crates/infrastructure/src/security/correlation_service.rs`:**
  * Crear el archivo `crates/infrastructure/src/security/correlation_service.rs`.
  * Definir la struct `CorrelationService` con:
    * `security_repo: SecurityRepository`
    * `telemetry_repo: TelemetryRepository`
  * Implementar `correlate_events_with_metrics()`:
    - Obtener eventos de seguridad recientes.
    - Obtener métricas de telemetría para los mismos dispositivos.
    - Buscar correlaciones:
      - Picos de latencia con intrusiones
      - Alto uso de CPU con DDoS
      - Pérdida de paquetes con escaneo de puertos
    - Retornar eventos correlacionados con métricas.
  * Implementar `detect_anomalous_patterns()`:
    - Analizar patrones históricos de métricas.
    - Detectar desviaciones significativas.
    - Generar eventos de seguridad si se detectan anomalías.
* [ ] **11.7.2 — Integrar con Motor de Detección:**
  - Agregar `CorrelationService` al `DetectionEngine`.
  - Ejecutar correlación periódicamente.
  - Generar eventos de seguridad cuando se detecten correlaciones.
* [ ] **11.7.3 — Verificar Compilación:**
  - Ejecutar `cargo check -p infrastructure` y corregir errores.

---

## Slice 11.8: Pruebas de Detección y Validación de Alertas 🧪

> **Objetivo:** Implementar pruebas de carga concurrente y validación de alertas para asegurar que el sistema de detección funciona correctamente.

* [ ] **11.8.1 — Crear Tests de Integración en `crates/infrastructure/src/tests/security_tests.rs`:**
  * Crear el archivo `crates/infrastructure/src/tests/security_tests.rs`.
  * Implementar test `test_log_security_event()`:
    - Crear evento de seguridad.
    - Llamar a endpoint POST /api/v1/security/events.
    - Verificar que se guarde en la base de datos.
  * Implementar test `test_get_security_events()`:
    - Crear múltiples eventos.
    - Llamar a endpoint GET /api/v1/security/events.
    - Verificar que se retornen los eventos.
  * Implementar test `test_resolve_security_event()`:
    - Crear evento con estado Detected.
    - Llamar a endpoint PUT /api/v1/security/events/{id}/resolve.
    - Verificar que cambie a Resolved.
  * Implementar test `test_mark_false_positive()`:
    - Crear evento con estado Detected.
    - Llamar a endpoint PUT /api/v1/security/events/{id}/false-positive.
    - Verificar que cambie a FalsePositive.
* [ ] **11.8.2 — Crear Tests de Carga Concurrente:**
  * Implementar test `test_concurrent_security_events()`:
    - Enviar 100 eventos concurrentemente.
    - Verificar que todos se guarden correctamente.
    - Verificar que no haya duplicados.
  * Implementar test `test_detection_engine_performance()`:
    - Simular alto volumen de tráfico.
    - Verificar que el motor de detección procese eventos en tiempo real.
    - Verificar latencia de detección.
* [ ] **11.8.3 — Crear Tests de Correlación:**
  * Implementar test `test_event_correlation()`:
    - Crear evento de seguridad y métricas correlacionadas.
    - Llamar a servicio de correlación.
    - Verificar que se detecte la correlación.
* [ ] **11.8.4 — Ejecutar Tests:**
  - Ejecutar `cargo test --package infrastructure security_tests` y corregir errores.

---

## 🎯 Criterios de Aceptación del Módulo 11

- [ ] La tabla `security_events` existe en MySQL con los índices correctos.
- [ ] Los modelos de dominio `SecurityEvent`, `IntrusionType`, `Severity`, `SecurityStatus` están definidos.
- [ ] El trait `SecurityPort` está implementado con todos los métodos requeridos.
- [ ] El repositorio `SecurityRepository` implementa `SecurityPort` correctamente.
- [ ] El motor de detección `DetectionEngine` procesa eventos de forma asíncrona.
- [ ] Los endpoints HTTP `/api/v1/security/events*` funcionan correctamente.
- [ ] El dashboard de intrusiones muestra eventos en tiempo real.
- [ ] La correlación de eventos con telemetría funciona correctamente.
- [ ] Las pruebas de carga concurrente pasan exitosamente.
- [ ] El sistema detecta patrones anómalos y genera alertas automáticamente.

---

## 📝 Notas Técnicas

- **ADR-0004:** Persistencia con Sea-ORM y MySQL.
- **ADR-0005:** Diseño de esquema relacional.
- **ADR-0006:** Seguridad con JWT y RBAC.
- **ADR-0015:** Ingestión asíncrona con Tokio Channels.
- **ADR-0017:** Frontend Svelte 5 con TanStack Query.
- **ADR-0020:** Monitoreo regional de infraestructura.

---

## 🔗 Dependencias

- Módulo 1: Autenticación Core (para validación de token)
- Módulo 7: API de Telemetría (para correlación de eventos)
- Módulo 6: Auditoría Inmutable (para log de acciones de seguridad)

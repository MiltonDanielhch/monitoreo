# Módulo 12: Descubrimiento de Red y Asset Discovery 🔍

> **Objetivo:** Implementar un sistema de descubrimiento automático de red para escanear rangos de IPs, detectar dispositivos, obtener sus características y mantener un inventario actualizado de la infraestructura.
>
> **Entregable:** Sistema de escaneo de red con detección de dispositivos, recolección de datos (IP, MAC, hostname, OS, puertos), clasificación de dispositivos y dashboard de visualización.
>
> **Regla de Pureza:** El dominio no sabe de protocolos de red o sockets. Recibe estructuras puras de dispositivos descubiertos (IP, MAC, hostname, tipo) y evalúa si son autorizados o requieren atención.
>
> **Estados:** [ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

---

## 📊 Matriz de Progreso General

| Slice | Nombre | Referencia ADR | Progreso |
| --- | --- | --- | --- |
| **12.1** | Esquema de Dispositivos Descubiertos (Workbench) | `ADR-0004`, `ADR-0005` | [ ] |
| **12.2** | Modelos de Descubrimiento y Contratos de Escaneo | `ADR-0001` | [ ] |
| **12.3** | Repositorio de Dispositivos (Sea-ORM) | `ADR-0004` | [ ] |
| **12.4** | Motor de Escaneo de Red (pnet, rayon) | `ADR-0015` | [ ] |
| **12.5** | Endpoints HTTP/2 Seguros para Escaneo | `ADR-0003`, `ADR-0006` | [ ] |
| **12.6** | Dashboard de Descubrimiento (Svelte 5 + UI) | `ADR-0017` [DISCOVERY] | [ ] |
| **12.7** | Clasificación de Dispositivos y OUI Lookup | `ADR-0017` [CLASSIFICATION] | [ ] |
| **12.8** | Pruebas de Escaneo y Validación de Datos | `ADR-0010` | [ ] |
| **M12** | **Módulo 12 Total** |  | **[ ]** |

---

## Slice 12.1: Esquema de Dispositivos Descubiertos (MySQL Workbench) 🗄️

> **Objetivo:** Diseñar el almacenamiento relacional local en tu Workbench optimizado para dispositivos descubiertos en la red.

* [ ] **12.1.1 — Diseño del Archivo SQL:**
  * Crear el archivo plano en `data/migrations/0012_network_discovery.sql`.
  * Diseñar la tabla `discovered_devices` (registro de dispositivos descubiertos) con campos:
    * `id` (VARCHAR(255) PRIMARY KEY)
    * `ip_address` (VARCHAR(45) - dirección IP del dispositivo)
    * `mac_address` (VARCHAR(17) - dirección MAC)
    * `hostname` (VARCHAR(255) - nombre del host)
    * `device_type` (VARCHAR(50) - router, switch, server, pc, mobile, iot, unknown)
    * `os_fingerprint` (VARCHAR(100) - sistema operativo detectado)
    * `manufacturer` (VARCHAR(100) - fabricante del dispositivo)
    * `open_ports` (TEXT - JSON con lista de puertos abiertos)
    * `services` (TEXT - JSON con servicios detectados)
    * `status` (VARCHAR(50) - online, offline, unknown)
    * `is_authorized` (BOOLEAN - si el dispositivo está autorizado)
    * `last_seen` (DATETIME - última vez que se vio el dispositivo)
    * `first_seen` (DATETIME - primera vez que se vio el dispositivo)
    * `scan_id` (VARCHAR(255) - ID del escaneo que lo descubrió)
    * `sede_id` (VARCHAR(255) - sede donde se descubrió el dispositivo)
    * `metadata` (TEXT - JSON con detalles adicionales)
    * `created_at` (DATETIME - timestamp de creación)
    * `updated_at` (DATETIME - timestamp de actualización)
  * Diseñar la tabla `network_scans` (registro de escaneos de red) con campos:
    * `id` (VARCHAR(255) PRIMARY KEY)
    * `scan_type` (VARCHAR(50) - full, partial, targeted)
    * `ip_range` (VARCHAR(100) - rango de IPs escaneado)
    * `status` (VARCHAR(50) - pending, running, completed, failed)
    * `devices_found` (INT - cantidad de dispositivos encontrados)
    * `started_at` (DATETIME - timestamp de inicio)
    * `completed_at` (DATETIME NULL - timestamp de finalización)
    * `duration_seconds` (INT NULL - duración en segundos)
    * `sede_id` (VARCHAR(255) - sede del escaneo)
    * `created_by` (VARCHAR(255) - usuario que inició el escaneo)
    * `created_at` (DATETIME - timestamp de creación)
  * Crear índices en:
    * `idx_discovered_devices_ip` (ip_address)
    * `idx_discovered_devices_mac` (mac_address)
    * `idx_discovered_devices_status` (status)
    * `idx_discovered_devices_device_type` (device_type)
    * `idx_discovered_devices_last_seen` (last_seen)
    * `idx_discovered_devices_sede` (sede_id)
    * `idx_network_scans_status` (status)
    * `idx_network_scans_sede` (sede_id)
  * Ejecutar el script en MySQL Workbench y verificar que se creen las tablas correctamente.

---

## Slice 12.2: Modelos de Descubrimiento y Contratos de Escaneo 📦

> **Objetivo:** Definir los modelos de dominio para dispositivos descubiertos, tipos de dispositivos, resultados de escaneo y el trait DiscoveryPort con métodos asincrónicos.

* [ ] **12.2.1 — Definir Enums y Estructuras en `crates/domain/src/models/discovery.rs`:**
  * Crear el archivo `crates/domain/src/models/discovery.rs`.
  * Definir el enum `DeviceType` con variantes:
    * `Router` (router)
    * `Switch` (switch)
    * `Server` (servidor)
    * `PC` (computadora personal)
    * `Mobile` (dispositivo móvil)
    * `IoT` (dispositivo IoT)
    * `Printer` (impresora)
    * `Unknown` (desconocido)
  * Definir el enum `ScanStatus` con variantes:
    * `Pending` (pendiente)
    * `Running` (en ejecución)
    * `Completed` (completado)
    * `Failed` (fallido)
  * Definir el enum `DeviceStatus` con variantes:
    * `Online` (en línea)
    * `Offline` (fuera de línea)
    * `Unknown` (desconocido)
  * Definir la struct `DiscoveredDevice` con campos:
    * `id: String`
    * `ip_address: String`
    * `mac_address: Option<String>`
    * `hostname: Option<String>`
    * `device_type: DeviceType`
    * `os_fingerprint: Option<String>`
    * `manufacturer: Option<String>`
    * `open_ports: Vec<u16>`
    * `services: Vec<String>`
    * `status: DeviceStatus`
    * `is_authorized: bool`
    * `last_seen: String` (RFC3339)
    * `first_seen: String` (RFC3339)
    * `scan_id: String`
    * `sede_id: Option<String>`
    * `metadata: Option<serde_json::Value>`
  * Definir la struct `NetworkScan` con campos:
    * `id: String`
    * `scan_type: String`
    * `ip_range: String`
    * `status: ScanStatus`
    * `devices_found: i32`
    * `started_at: String` (RFC3339)
    * `completed_at: Option<String>` (RFC3339)
    * `duration_seconds: Option<i32>`
    * `sede_id: Option<String>`
    * `created_by: String`
  * Implementar métodos en `DiscoveredDevice`:
    * `new()` - constructor que genera ID único
    * `validate_ip()` - valida formato de IP
    * `validate_mac()` - valida formato de MAC
    * `is_online()` - retorna true si status es Online
    * `mark_as_offline()` - cambia estado a Offline
    * `mark_as_authorized()` - marca como autorizado
    * `mark_as_unauthorized()` - marca como no autorizado
  * Implementar métodos en `NetworkScan`:
    * `new()` - constructor que genera ID único
    * `start()` - cambia estado a Running
    * `complete()` - cambia estado a Completed
    * `fail()` - cambia estado a Failed
  * Definir el trait `DiscoveryPort` con métodos asincrónicos:
    * `async fn log_device(&self, device: DiscoveredDevice) -> Result<(), DomainError>`
    * `async fn get_devices(&self, filters: DiscoveryFilters) -> Result<Vec<DiscoveredDevice>, DomainError>`
    * `async fn get_device_by_ip(&self, ip: String) -> Result<Option<DiscoveredDevice>, DomainError>`
    * `async fn get_device_by_mac(&self, mac: String) -> Result<Option<DiscoveredDevice>, DomainError>`
    * `async fn update_device_status(&self, id: String, status: DeviceStatus) -> Result<(), DomainError>`
    * `async fn mark_device_authorized(&self, id: String) -> Result<(), DomainError>`
    * `async fn mark_device_unauthorized(&self, id: String) -> Result<(), DomainError>`
    * `async fn create_scan(&self, scan: NetworkScan) -> Result<(), DomainError>`
    * `async fn get_scan(&self, id: String) -> Result<Option<NetworkScan>, DomainError>`
    * `async fn get_scans(&self, filters: ScanFilters) -> Result<Vec<NetworkScan>, DomainError>`
    * `async fn update_scan_status(&self, id: String, status: ScanStatus) -> Result<(), DomainError>`
  * Definir la struct `DiscoveryFilters` con campos opcionales para filtros:
    * `device_type: Option<DeviceType>`
    * `status: Option<DeviceStatus>`
    * `is_authorized: Option<bool>`
    * `sede_id: Option<String>`
    * `manufacturer: Option<String>`
    * `date_from: Option<String>`
    * `date_to: Option<String>`
  * Definir la struct `ScanFilters` con campos opcionales para filtros:
    * `status: Option<ScanStatus>`
    * `sede_id: Option<String>`
    * `scan_type: Option<String>`
    * `date_from: Option<String>`
    * `date_to: Option<String>`
  * Agregar el módulo `discovery` en `crates/domain/src/models/mod.rs` y exportar los tipos.

---

## Slice 12.3: Repositorio de Dispositivos (Sea-ORM) 🗄️

> **Objetivo:** Crear las entidades Sea-ORM para las tablas `discovered_devices` y `network_scans` e implementar el repositorio DiscoveryRepository que implementa el trait DiscoveryPort.

* [ ] **12.3.1 — Crear Entidad Sea-ORM en `crates/database/src/entities/discovered_device_entity.rs`:**
  * Crear el archivo `crates/database/src/entities/discovered_device_entity.rs`.
  * Definir la entidad `DiscoveredDevice` con los campos correspondientes a la tabla `discovered_devices`.
  * Usar `Decimal` para campos numéricos si aplica.
  * Usar `DateTime` para timestamps.
  * Almacenar `open_ports` y `services` como `Option<String>` (JSON como string).
  * Implementar `ActiveModelBehavior` para la entidad.
* [ ] **12.3.2 — Crear Entidad Sea-ORM en `crates/database/src/entities/network_scan_entity.rs`:**
  * Crear el archivo `crates/database/src/entities/network_scan_entity.rs`.
  * Definir la entidad `NetworkScan` con los campos correspondientes a la tabla `network_scans`.
  * Usar `DateTime` para timestamps.
  * Implementar `ActiveModelBehavior` para la entidad.
* [ ] **12.3.3 — Registrar las Entidades en `crates/database/src/entities/mod.rs`:**
  * Agregar `pub mod discovered_device_entity;`
  * Agregar `pub mod network_scan_entity;`
  * Agregar `pub use discovered_device_entity::Entity as DiscoveredDevice;`
  * Agregar `pub use network_scan_entity::Entity as NetworkScan;`
* [ ] **12.3.4 — Crear Repositorio en `crates/database/src/repositories/discovery_repository.rs`:**
  * Crear el archivo `crates/database/src/repositories/discovery_repository.rs`.
  * Definir la struct `DiscoveryRepository` con `db: DatabaseConnection`.
  * Implementar el trait `DiscoveryPort` para `DiscoveryRepository`.
  * Implementar `log_device()`:
    * Convertir `DiscoveredDevice` a `discovered_device_entity::ActiveModel`.
    * Serializar `open_ports` y `services` a JSON string.
    * Insertar o actualizar en la base de datos.
    * Manejar errores y mapear a `DomainError`.
  * Implementar `get_devices()`:
    * Construir query con filtros opcionales.
    * Usar `QueryFilter` para aplicar filtros.
    * Ordenar por `last_seen` descendente.
    * Convertir modelos de Sea-ORM a entidades de dominio.
    * Deserializar `open_ports` y `services` de JSON string.
  * Implementar `get_device_by_ip()`:
    * Buscar por IP usando filtro.
    * Convertir a entidad de dominio o retornar None.
  * Implementar `get_device_by_mac()`:
    * Buscar por MAC usando filtro.
    * Convertir a entidad de dominio o retornar None.
  * Implementar `update_device_status()`:
    * Buscar dispositivo por ID.
    - Actualizar `status` y `last_seen`.
    - Guardar cambios.
  * Implementar `mark_device_authorized()`:
    - Buscar dispositivo por ID.
    - Actualizar `is_authorized` a true.
    - Guardar cambios.
  * Implementar `mark_device_unauthorized()`:
    - Buscar dispositivo por ID.
    - Actualizar `is_authorized` a false.
    - Guardar cambios.
  * Implementar `create_scan()`:
    - Convertir `NetworkScan` a `network_scan_entity::ActiveModel`.
    - Insertar en la base de datos.
  * Implementar `get_scan()`:
    - Buscar por ID.
    - Convertir a entidad de dominio o retornar None.
  * Implementar `get_scans()`:
    - Construir query con filtros opcionales.
    - Ordenar por `started_at` descendente.
    - Convertir modelos de Sea-ORM a entidades de dominio.
  * Implementar `update_scan_status()`:
    - Buscar escaneo por ID.
    - Actualizar `status`, `completed_at`, `duration_seconds`.
    - Guardar cambios.
* [ ] **12.3.5 — Registrar el Repositorio en `crates/database/src/repositories/mod.rs`:**
  * Agregar `pub mod discovery_repository;`
  * Agregar `pub use discovery_repository::DiscoveryRepository;`
* [ ] **12.3.6 — Exportar desde `crates/database/src/lib.rs`:**
  * Agregar `pub use repositories::DiscoveryRepository;`
* [ ] **12.3.7 — Verificar Compilación:**
  * Ejecutar `cargo check -p database` y corregir errores.

---

## Slice 12.4: Motor de Escaneo de Red (pnet, rayon) 🔍

> **Objetivo:** Implementar un motor de escaneo de red usando crates como `pnet`, `tokio`, `rayon` para escanear rangos de IPs de forma paralela y detectar dispositivos.

* [ ] **12.4.1 — Agregar Dependencias a `Cargo.toml`:**
  * Agregar `pnet` para manipulación de paquetes de red.
  * Agregar `rayon` para procesamiento paralelo.
  * Agregar `trust-dns-resolver` para resolución DNS.
  * Agregar `mac_address` para manipulación de direcciones MAC.
* [ ] **12.4.2 — Crear Motor de Escaneo en `crates/infrastructure/src/discovery/scan_engine.rs`:**
  * Crear el archivo `crates/infrastructure/src/discovery/scan_engine.rs`.
  * Definir el enum `ScanMessage`:
    * `StartScan(ScanConfig)` - iniciar escaneo
    * `ScanProgress(ScanProgress)` - progreso del escaneo
    * `DeviceFound(DiscoveredDevice)` - dispositivo encontrado
    * `ScanCompleted(String)` - escaneo completado
    * `Shutdown` - detener motor
  * Definir la struct `ScanConfig` con:
    * `ip_range: String` (ej: "192.168.1.0/24")
    * `scan_type: String` (full, partial, targeted)
    * `ports_to_scan: Vec<u16>`
    * `timeout_ms: u64`
    * `max_concurrent: usize`
  * Definir la struct `ScanProgress` con:
    * `scan_id: String`
    * `total_ips: usize`
    * `scanned_ips: usize`
    * `devices_found: usize`
    * `percentage: f64`
  * Definir la struct `ScanEngine` con:
    * `tx: mpsc::Sender<ScanMessage>`
    * `_handle: tokio::task::JoinHandle<()>`
  * Implementar `new()` que crea el motor con canales de Tokio.
  * Implementar el worker principal que:
    * Recibe mensajes del canal.
    - Escanea IPs en paralelo usando `rayon`.
    - Realiza ping ICMP para detectar hosts activos.
    - Realiza ARP scan para obtener direcciones MAC.
    - Realiza port scanning TCP/UDP.
    - Realiza OS fingerprinting.
    - Realiza DNS reverse lookup para obtener hostname.
    - Realiza OUI lookup para obtener fabricante.
    - Clasifica dispositivos según puertos y servicios.
    - Genera eventos de dispositivo encontrado.
    - Envía dispositivos al repositorio de descubrimiento.
  * Implementar `start_scan()` para iniciar un escaneo.
  * Implementar `get_scan_progress()` para obtener progreso.
  * Implementar `shutdown()` para detener el motor.
* [ ] **12.4.3 — Crear Servicio de OUI Lookup en `crates/infrastructure/src/discovery/oui_lookup.rs`:**
  * Crear el archivo `crates/infrastructure/src/discovery/oui_lookup.rs`.
  * Implementar lookup de fabricante usando OUI database.
  * Cargar OUI database desde archivo o API.
  * Implementar `get_manufacturer(mac: &str) -> Option<String>`.
* [ ] **12.4.4 — Crear Módulo en `crates/infrastructure/src/discovery/mod.rs`:**
  * Agregar `pub mod scan_engine;`
  * Agregar `pub mod oui_lookup;`
  * Exportar tipos.
* [ ] **12.4.5 — Agregar Módulo a `crates/infrastructure/src/lib.rs`:**
  * Agregar `pub mod discovery;`
* [ ] **12.4.6 — Verificar Compilación:**
  * Ejecutar `cargo check -p infrastructure` y corregir errores.

---

## Slice 12.5: Endpoints HTTP/2 Seguros para Escaneo 🌐

> **Objetivo:** Implementar los endpoints HTTP optimizados en Axum 0.8 para iniciar escaneos y gestionar dispositivos descubiertos con validación de token.

* [ ] **12.5.1 — Crear Handler en `crates/infrastructure/src/handlers/discovery_handler.rs`:**
  * Crear el archivo `crates/infrastructure/src/handlers/discovery_handler.rs`.
  * Definir DTOs para request/response:
    * `StartScanRequest` - para iniciar escaneo
    * `StartScanResponse` - respuesta de inicio
    * `DiscoveredDeviceResponse` - respuesta de dispositivo
    * `DevicesPaginatedResponse` - lista paginada de dispositivos
    * `NetworkScanResponse` - respuesta de escaneo
    * `DiscoveryFilters` - filtros para query
  * Implementar `start_network_scan()`:
    * Extraer y validar token de autorización.
    - Validar permisos de usuario.
    - Convertir request a `ScanConfig`.
    - Validar rango de IPs.
    - Llamar a `scan_engine.start_scan()`.
    - Retornar respuesta de inicio con scan_id.
  * Implementar `get_scan_progress()`:
    - Extraer y validar token.
    - Llamar a `scan_engine.get_scan_progress()`.
    - Retornar progreso del escaneo.
  * Implementar `get_discovered_devices()`:
    * Extraer y validar token.
    - Aplicar filtros de query.
    - Llamar a `discovery_repo.get_devices()`.
    - Retornar lista paginada de dispositivos.
  * Implementar `get_device_by_ip()`:
    - Extraer y validar token.
    - Llamar a `discovery_repo.get_device_by_ip()`.
    - Retornar dispositivo o 404.
  * Implementar `mark_device_authorized()`:
    - Extraer y validar token.
    - Validar permisos de usuario.
    - Llamar a `discovery_repo.mark_device_authorized()`.
    - Retornar respuesta de éxito.
  * Implementar `mark_device_unauthorized()`:
    - Extraer y validar token.
    - Validar permisos de usuario.
    - Llamar a `discovery_repo.mark_device_unauthorized()`.
    - Retornar respuesta de éxito.
  * Implementar `get_network_scans()`:
    - Extraer y validar token.
    - Llamar a `discovery_repo.get_scans()`.
    - Retornar lista de escaneos.
* [ ] **12.5.2 — Registrar Handler en `crates/infrastructure/src/handlers/mod.rs`:**
  * Agregar `pub mod discovery_handler;`
  - Exportar funciones.
* [ ] **12.5.3 — Agregar Rutas en `crates/infrastructure/src/lib.rs`:**
  * Agregar rutas:
    - `POST /api/v1/discovery/scan` - start_network_scan
    - `GET /api/v1/discovery/scan/{id}/progress` - get_scan_progress
    - `GET /api/v1/discovery/devices` - get_discovered_devices
    - `GET /api/v1/discovery/devices/ip/{ip}` - get_device_by_ip
    - `PUT /api/v1/discovery/devices/{id}/authorize` - mark_device_authorized
    - `PUT /api/v1/discovery/devices/{id}/unauthorize` - mark_device_unauthorized
    - `GET /api/v1/discovery/scans` - get_network_scans
* [ ] **12.5.4 — Verificar Compilación:**
  * Ejecutar `cargo check -p infrastructure` y corregir errores.

---

## Slice 12.6: Dashboard de Descubrimiento (Svelte 5 + UI) 🎨

> **Objetivo:** Crear el dashboard de visualización de descubrimiento de red con Svelte 5 y TanStack Query para mostrar dispositivos descubiertos y resultados de escaneos.

* [ ] **12.6.1 — Crear Página en `apps/web/src/routes/dashboard/discovery/+page.svelte`:**
  * Crear el archivo `apps/web/src/routes/dashboard/discovery/+page.svelte`.
  * Implementar query con `createQuery` para obtener dispositivos descubiertos.
  * Configurar `refetchInterval` para actualización en tiempo real (cada 30 segundos).
  * Implementar filtros por tipo de dispositivo, estado, autorización, fabricante.
  - Mostrar estadísticas:
    - Total de dispositivos
    - Dispositivos online
    - Dispositivos autorizados
    - Dispositivos no autorizados
  - Mostrar tabla de dispositivos con:
    - Dirección IP
    - Dirección MAC
    - Hostname
    - Tipo de dispositivo
    - Fabricante
    - Estado (con colores)
    - Autorización (con badges)
    - Última vez visto
  - Implementar acciones:
    - Marcar como autorizado
    - Marcar como no autorizado
    - Ver detalles
  - Usar componentes UI: Card, Badge, Button, Table.
  - Implementar paginación.
* [ ] **12.6.2 — Crear Página de Escaneo en `apps/web/src/routes/dashboard/discovery/scan/+page.svelte`:**
  * Crear el archivo `apps/web/src/routes/dashboard/discovery/scan/+page.svelte`.
  - Implementar formulario para iniciar escaneo:
    - Rango de IPs
    - Tipo de escaneo
    - Puertos a escanear
    - Timeout
  - Implementar visualización de progreso del escaneo.
  - Mostrar dispositivos encontrados en tiempo real.
  - Implementar cancelación de escaneo.
* [ ] **12.6.3 — Crear Página de Detalles en `apps/web/src/routes/dashboard/discovery/[id]/+page.svelte`:**
  * Crear el archivo `apps/web/src/routes/dashboard/discovery/[id]/+page.svelte`.
  - Implementar query para obtener dispositivo por ID.
  - Mostrar detalles completos del dispositivo:
    - Información de red (IP, MAC, hostname)
    - Información de hardware (fabricante, tipo)
    - Puertos abiertos
    - Servicios detectados
    - Historial de escaneos
  - Mostrar metadata JSON formateado.
  - Implementar botones de acción.
* [ ] **12.6.4 — Agregar Ruta al Sidebar en `apps/web/src/lib/components/layout/Sidebar.svelte`:**
  - Agregar `{ href: '/dashboard/discovery', label: 'Descubrimiento', icon: Search, description: 'Escaneo de red y descubrimiento de dispositivos' }`
  - Agregar icono `Search` a los imports.
* [ ] **12.6.5 — Verificar Compilación:**
  - Ejecutar `npm run check` en `apps/web` y corregir errores.

---

## Slice 12.7: Clasificación de Dispositivos y OUI Lookup 🏷️

> **Objetivo:** Implementar la clasificación automática de dispositivos basada en puertos, servicios y OUI lookup para identificar fabricantes.

* [ ] **12.7.1 — Crear Servicio de Clasificación en `crates/infrastructure/src/discovery/classification_service.rs`:**
  * Crear el archivo `crates/infrastructure/src/discovery/classification_service.rs`.
  * Definir reglas de clasificación:
    - Router: puertos 22, 23, 80, 443, 161 (SNMP)
    - Switch: puertos 22, 23, 161 (SNMP)
    - Server: puertos 22, 80, 443, 3306, 5432
    - PC: puertos 135, 139, 445 (SMB)
    - IoT: puertos 80, 443, 1883 (MQTT)
    - Printer: puertos 9100, 515, 631
  - Implementar `classify_device(ports: Vec<u16>, services: Vec<String>) -> DeviceType`.
  - Implementar clasificación basada en heurísticas.
  - Implementar aprendizaje de patrones (opcional).
* [ ] **12.7.2 — Integrar OUI Lookup con Clasificación:**
  - Usar OUI lookup para obtener fabricante.
  - Usar fabricante para mejorar clasificación.
  - Ejemplo: Cisco -> Router/Switch, HP -> Printer/Server.
* [ ] **12.7.3 — Integrar con Motor de Escaneo:**
  - Agregar `ClassificationService` al `ScanEngine`.
  - Clasificar dispositivos automáticamente al descubrirlos.
  - Actualizar dispositivo con clasificación.
* [ ] **12.7.4 — Verificar Compilación:**
  - Ejecutar `cargo check -p infrastructure` y corregir errores.

---

## Slice 12.8: Pruebas de Escaneo y Validación de Datos 🧪

> **Objetivo:** Implementar pruebas de escaneo de red y validación de datos para asegurar que el sistema de descubrimiento funciona correctamente.

* [ ] **12.8.1 — Crear Tests de Integración en `crates/infrastructure/src/tests/discovery_tests.rs`:**
  * Crear el archivo `crates/infrastructure/src/tests/discovery_tests.rs`.
  - Implementar test `test_log_discovered_device()`:
    - Crear dispositivo descubierto.
    - Llamar a endpoint POST /api/v1/discovery/devices.
    - Verificar que se guarde en la base de datos.
  - Implementar test `test_get_discovered_devices()`:
    - Crear múltiples dispositivos.
    - Llamar a endpoint GET /api/v1/discovery/devices.
    - Verificar que se retornen los dispositivos.
  - Implementar test `test_start_network_scan()`:
    - Iniciar escaneo de red.
    - Verificar que se cree el registro de escaneo.
    - Verificar que se detecten dispositivos.
  - Implementar test `test_mark_device_authorized()`:
    - Crear dispositivo no autorizado.
    - Llamar a endpoint PUT /api/v1/discovery/devices/{id}/authorize.
    - Verificar que cambie a autorizado.
* [ ] **12.8.2 — Crear Tests de Escaneo:**
  - Implementar test `test_ip_scan()`:
    - Escanear rango de IPs pequeño.
    - Verificar que se detecten hosts activos.
    - Verificar que se obtengan direcciones MAC.
  - Implementar test `test_port_scan()`:
    - Escanear puertos específicos.
    - Verificar que se detecten puertos abiertos.
    - Verificar que se identifiquen servicios.
* [ ] **12.8.3 — Crear Tests de Clasificación:**
  - Implementar test `test_device_classification()`:
    - Crear dispositivo con puertos específicos.
    - Llamar a servicio de clasificación.
    - Verificar que se clasifique correctamente.
* [ ] **12.8.4 — Ejecutar Tests:**
  - Ejecutar `cargo test --package infrastructure discovery_tests` y corregir errores.

---

## 🎯 Criterios de Aceptación del Módulo 12

- [ ] Las tablas `discovered_devices` y `network_scans` existen en MySQL con los índices correctos.
- [ ] Los modelos de dominio `DiscoveredDevice`, `NetworkScan`, `DeviceType`, `ScanStatus` están definidos.
- [ ] El trait `DiscoveryPort` está implementado con todos los métodos requeridos.
- [ ] El repositorio `DiscoveryRepository` implementa `DiscoveryPort` correctamente.
- [ ] El motor de escaneo `ScanEngine` escanea rangos de IPs de forma paralela.
- [ ] Los endpoints HTTP `/api/v1/discovery/*` funcionan correctamente.
- [ ] El dashboard de descubrimiento muestra dispositivos en tiempo real.
- [ ] La clasificación de dispositivos funciona correctamente.
- [ ] El OUI lookup identifica fabricantes correctamente.
- [ ] Las pruebas de escaneo pasan exitosamente.

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
- Módulo 5: Topología de Red (para integración con mapa de red)
- Módulo 11: Detección de Intrusiones (para detectar dispositivos no autorizados)
- Módulo 6: Auditoría Inmutable (para log de acciones de descubrimiento)

---

## 📚 Referencias

- **pnet:** https://docs.rs/pnet/latest/pnet/ (manipulación de paquetes de red)
- **rayon:** https://docs.rs/rayon/latest/rayon/ (procesamiento paralelo)
- **trust-dns-resolver:** https://docs.rs/trust-dns-resolver/latest/trust_dns_resolver/ (resolución DNS)
- **mac_address:** https://docs.rs/mac_address/latest/mac_address/ (manipulación de MAC)
- **OUI Database:** https://standards-oui.ieee.org/ (base de datos de fabricantes)

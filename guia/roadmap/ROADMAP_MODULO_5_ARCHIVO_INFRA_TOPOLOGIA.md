# 🗺️ Roadmap — Módulo 5: Archivos de Infraestructura y Topologías

> **Propósito:** Construir el repositorio centralizado de activos de red, planos en SVG, imágenes de racks de servidores de las sedes y respaldos de configuración, abstrayendo el almacenamiento físico para garantizar la resiliencia offline de los agentes remotos.
> **Entregable:** Gestor documental reactivo en Svelte 5 con visor interactivo de diagramas de red, backend de streaming multipart en Axum 0.8 que valida formatos permitidos (`.svg`, `.png`, `.jpg`, `.cfg`, `.txt`), y persistencia relacional en Sea-ORM.
> **Regla de Pureza:** El dominio no entiende de carpetas compartidas o buckets S3. Solo procesa el negocio de infraestructura: asociación a una Sede, hash de integridad para auditoría de cambios de configuración y validación estricta de extensiones de red.
> **Stack:** Rust 2024 · Axum 0.8 · Sea-ORM 1.1 · SHA-256 · SvelteKit 2 · Svelte 5 (Runes) · Tailwind v4 · Docker
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
| 5.1 | Esquema Relacional de Activos por Sede | [ ] |
| 5.2 | Modelos de Infraestructura y Filtros de Dominio (`crates/domain`) | [ ] |
| 5.3 | Entidades de Activos y Consultas en Sea-ORM (`crates/database`) | [ ] |
| 5.4 | Adaptador de Sistema de Archivos y Generador de Rutas Regionales | [ ] |
| 5.5 | Endpoints de Carga Streaming y Descarga de Backups en Axum | [ ] |
| 5.6 | UI Drag-and-Drop de Planos e Imágenes (Svelte 5 Runes) | [ ] |
| 5.7 | Visor Interactivo de SVG y Galería de Racks por Sede | [ ] |
| 5.8 | Pruebas de Integridad de Backups y Sanitización de SVG | [ ] |
| **Módulo 5 Total** |  | [ ] |

---

## Slice 5.1: Esquema Relacional de Activos por Sede 🔥

> **Objetivo:** Diseñar la base de datos para almacenar los metadatos de los archivos, vinculándolos directamente a las sedes de la Gobernación del Beni.

```
[ ] Crear archivo de migración plano en `data/migrations/0005_infrastructure_files.sql`
    [ ] Definir tabla `network_files` (id, filename, file_type [TOPOLOGY_SVG/RACK_IMG/CONFIG_BACKUP], file_size_bytes, storage_key, sha256_checksum, sede_id, user_id, created_at, deleted_at)
    [ ] Crear índices dedicados para acelerar las búsquedas regionales: `idx_files_sede_type` compuesto por `(sede_id, file_type)`
    [ ] Configurar llaves foráneas: `user_id` hacia `users(id) ON DELETE SET NULL` y `sede_id` amarrado a la tabla de sedes/configuración global.

[ ] Ejecutar la migración en tu contenedor de desarrollo:
    [ ] Comando: docker exec -i redes-db-dev mysql -u redes -predes redes_dev < data/migrations/0005_infrastructure_files.sql

[ ] Verificar consistencia en las tablas físicas:
    [ ] Comando: docker exec -it redes-db-dev mysql -u redes -predes redes_dev -e "DESCRIBE network_files;"

```

---

## Slice 5.2: Modelos de Infraestructura y Filtros de Dominio (`crates/domain`) 🔥

> **Objetivo:** Blindar el dominio con reglas de negocio específicas para los archivos admitidos en la administración de redes.

```
[ ] Actualizar `crates/domain/src/errors.rs`
    [ ] Añadir variantes: `UnsupportedNetworkFormat`, `CorruptedBackup`, `SedeNotFound`, `FileStorageError(String)`

[ ] Crear `crates/domain/src/models/infrastructure_file.rs`
    [ ] Definir enum fuertemente tipado `NetworkFileType` con las opciones: `TopologySvg`, `RackImage`, `ConfigBackup`.
    [ ] Implementar función de validación estricta de firma de archivos (Magic Numbers / Mime Types):
        - `TopologySvg`: Solo acepta texto plano con estructura XML válida `<svg>...</svg>`.
        - `ConfigBackup`: Solo acepta extensiones `.cfg`, `.txt`, o flujos de texto plano menores a 2MB.
        - `RackImage`: Solo formatos comprimidos visuales `.png` o `.jpg` para el inventario técnico.
    [ ] Definir el trait/puerto asíncrono `NetworkStoragePort` para desvincular el guardado en disco del dominio.

[ ] Validar compilación de lógica pura:
    [ ] Ejecutar: cargo check -p domain

```

---

## Slice 5.3: Entidades de Activos y Consultas en Sea-ORM (`crates/database`) 🔥

> **Objetivo:** Implementar los repositorios para la persistencia y auditoría de versiones de archivos de red.

```
[ ] Mapear entidades con Sea-ORM en `crates/database/src/entities/`
    [ ] Generar `network_file_entity.rs` agregando las macros de mapeo automático de enums y Soft Delete.

[ ] Crear `crates/database/src/repositories/network_file_repository.rs`
    [ ] Implementar `insert_file(...)` para registrar nuevos planos o respaldos.
    [ ] Implementar `get_latest_backup_by_device(device_id)` para extraer de forma instantánea la última configuración válida de un switch o router específico.
    [ ] Implementar consulta estructurada para extraer todos los archivos activos agrupados por `sede_id`.

```

---

## Slice 5.4: Adaptador de Sistema de Archivos y Generador de Rutas Regionales 🔥

> **Objetivo:** Codificar el software de infraestructura que escribe los binarios en las unidades de disco organizadas de forma lógica por regiones.

```
[ ] Configurar dependencias en `crates/infrastructure/Cargo.toml`
    [ ] Validar soporte para: `tokio-util` (Streams rápidos de I/O) y `sha2` (Cálculo del hash del firmware/respaldo).

[ ] Crear el adaptador físico en `crates/infrastructure/src/storage/regional_storage.rs`
    [ ] Implementar el trait `NetworkStoragePort` apuntando al almacenamiento local aislado.
    [ ] Programar el generador de rutas jerárquicas en disco para estructurar los respaldos de forma limpia:
        - Estructura: `/var/lib/redes/storage/sedes/{sede_id}/{file_type}/`
        - Ejemplo de archivo físico: `/var/lib/redes/storage/sedes/trinidad/backups/router-core.cfg`

```

---

## Slice 5.5: Endpoints de Carga Streaming y Descarga de Backups en Axum 🔥

> **Objetivo:** Exponer la API HTTP para que la UI o los agentes remotos suban mapas y configuraciones por flujos de datos ultra eficientes.

```
[ ] Crear `crates/infrastructure/src/handlers/network_file_handler.rs`
    [ ] Implementar el endpoint `POST /api/v1/infrastructure/upload` (Protegido por RBAC, requiere rol `ADMIN` u `OPERATOR`):
        - Leer mediante el extractor `Multipart` el flujo sin cargarlo por completo en memoria.
        - Calcular la firma SHA-256 en caliente. Si el hash coincide con el último respaldo guardado del mismo dispositivo, abortar la escritura en disco para evitar redundancia y optimizar almacenamiento.
    [ ] Implementar el endpoint `GET /api/v1/infrastructure/download/:id` que despache planos SVG con la cabecera `Content-Type: image/svg+xml` de forma directa para su correcta renderización en el navegador.

[ ] Conectar las rutas en `crates/infrastructure/src/router.rs`.

```

---

## Slice 5.6: UI Drag-and-Drop de Planos e Imágenes (Svelte 5 Runes) 🔥

> **Objetivo:** Diseñar el panel visual interactivo de carga optimizado para el flujo de trabajo de los administradores de redes.

```
[ ] Crear componente reactivo en `apps/web/src/lib/components/NetworkUploader.svelte`
    [ ] Maquetar la zona de arrastre con Tailwind v4, mostrando iconos descriptivos de lo que se espera recibir (un cable de red para `.cfg`, un mapa para `.svg`).
    [ ] Configurar la reactividad nativa con la rune `$state` para las variables de control del cliente: `uploading`, `progress`, `sedeSelected`, `fileCategory`.
    [ ] Validar en el cliente las extensiones antes de disparar el `fetch` para evitar peticiones infructuosas al backend de Axum.

```

---

## Slice 5.7: Visor Interactivo de SVG y Galería de Racks por Sede 🔥

> **Objetivo:** Construir la interfaz de monitoreo visual donde el operador pueda auditar la topología de red y el estado físico de los racks.

```
[ ] Diseñar la vista del centro de control en `apps/web/src/routes/dashboard/infrastructure/+page.svelte`
    [ ] Crear un selector dinámico de Sedes (Trinidad / Riberalta / Guayaramerín) alimentado por tus estados reactivos.
    [ ] **Visor de Topología:** Si la sede cuenta con un archivo `TOPOLOGY_SVG`, renderizar el contenido SVG directamente embebido en el HTML para que sea escalable, interactivo y responda a eventos del mouse.
    [ ] **Galería de Infraestructura:** Implementar una rejilla visual para mostrar las fotos de los racks de servidores locales, facilitando el reconocimiento físico de componentes en caso de fallos de hardware.

```

---

## Slice 5.8: Pruebas de Integridad de Backups y Sanitización de SVG 🔥

> **Objetivo:** Romper el sistema intencionalmente en pruebas de laboratorio para asegurar que no se comprometa la seguridad de la red regional.

```
[ ] Prueba 1 (Sanitización SVG): Intentar cargar un mapa topográfico modificado con scripts maliciosos de JavaScript ocultos (`<script>alert('hack')</script>`). El validador del backend debe rechazar o limpiar la entrada antes de guardarla.
[ ] Prueba 2 (Deduplicación de configuraciones): Forzar al sistema a subir el respaldo del router de Riberalta dos veces seguidas sin haber hecho modificaciones. Confirmar que la API retorna HTTP 200 pero reutiliza el registro anterior gracias a la coincidencia de hashes SHA-256.
[ ] Prueba 3 (Flujo Feliz Regional E2E): Seleccionar la sede de Trinidad, cargar una foto del rack principal y el mapa SVG, verificar que se ordenan perfectamente en sus respectivas carpetas del contenedor docker y comprobar que la UI de Svelte 5 renderiza la topología de forma nativa e interactiva.

```

---

## Entregable del Módulo 5

Al terminar estos checks, tu sistema tendrá un repositorio documental blindado, rápido y adaptado 100% a las necesidades operativas de la Gobernación. Tendrás los planos SVG cargando directo en pantalla y las configuraciones históricas de tus dispositivos bajo estricto control de auditoría de firmas SHA-256.

¿Cómo lo ves, Milton? ¿Quedó alineado con la visión? Si estás listo, dime y pasamos a estructurar el **Módulo 6: Auditoría**.
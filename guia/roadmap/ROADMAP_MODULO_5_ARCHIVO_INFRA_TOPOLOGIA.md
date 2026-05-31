# 🗺️ Roadmap Actualizado — Módulo 5: Infraestructura y Topologías

### 📂 Gestión Documental y Activos de Red para la Gobernación del Beni

```text
Propósito: Construir el repositorio centralizado de activos de red, planos SVG, imágenes de racks y respaldos de configuración, garantizando la resiliencia operativa y auditoría de cambios.
Entregable: Gestor documental reactivo en Svelte 5 con visor de diagramas, backend de streaming en Axum que valida integridad vía SHA-256 y persistencia en MySQL Workbench.
Regla de Pureza: El dominio no conoce de carpetas físicas; solo procesa reglas de infraestructura: asociación a Sede, hash de integridad y validación estricta de extensiones técnicas.
Estados: [ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

### 📊 Matriz de Progreso General

| Slice | Nombre | Referencia ADR | Progreso |
| --- | --- | --- | --- |
| **5.1** | Esquema Relacional de Activos (MySQL Workbench) | `ADR-0004`, `ADR-0005` | [ ] |
| **5.2** | Modelos de Infraestructura y Filtros de Dominio | `ADR-0001`, `ADR-0008` | [ ] |
| **5.3** | Entidades de Activos y Auditoría (Sea-ORM) | `ADR-0004` | [ ] |
| **5.4** | Adaptador de Almacenamiento Regional Local | `ADR-0012` | [ ] |
| **5.5** | Endpoints de Carga Streaming y Descarga en Axum | `ADR-0003`, `ADR-0013` | [ ] |
| **5.6** | UI Drag-and-Drop (Svelte 5 + TanStack Query) | `ADR-0017` [UPLOAD] | [ ] |
| **5.7** | Visor de Topología SVG y Galería de Racks | `ADR-0017` [VIEWER] | [ ] |
| **5.8** | Pruebas de Integridad y Sanitización de SVG | `ADR-0010` | [ ] |
| **M5** | **Módulo 5 Total** |  | **[ ]** |

---

## Slice 5.1: Esquema Relacional de Activos (MySQL Workbench) 🗄️

> **Objetivo:** Preparar la base de datos local para indexar metadatos de archivos técnicos vinculados a las sedes regionales del Beni.

* [ ] **5.1.1 — Diseño del Archivo SQL:**
* Crear `data/migrations/0005_infrastructure_files.sql`.
* Definir la tabla `network_files` con campos críticos: tipo de archivo (SVG/IMG/CFG), tamaño en bytes, hash SHA-256 para evitar duplicados y la relación con la sede (Trinidad/Riberalta/etc.).


* [ ] **5.1.2 — Ejecución en Workbench:**
* Abrir el script en **MySQL Workbench** y ejecutarlo para impactar tu instancia local.


* [ ] **5.1.3 — Verificación de Índices:**
* Confirmar que los índices compuestos por sede y tipo de archivo estén activos para asegurar que la búsqueda de planos sea instantánea.



---

## Slice 5.2: Modelos de Infraestructura y Filtros de Dominio 🧠

> **Objetivo:** Definir las reglas de negocio en Rust que impidan la subida de archivos basura o peligrosos al servidor de la Gobernación.

* [ ] **5.2.1 — Validación de Formatos de Red:**
* Implementar en el dominio el tipado estricto para `TopologySvg`, `RackImage` y `ConfigBackup`.
* Crear validadores que analicen el contenido: los archivos `.cfg` deben ser texto plano y los `.svg` deben ser XML válido, rechazando cualquier archivo que exceda los límites de memoria definidos.



---

## Slice 5.3: Entidades de Activos y Auditoría (Sea-ORM) 🔌

> **Objetivo:** Implementar los repositorios para guardar la "huella digital" de cada plano y configuración.

* [ ] **5.3.1 — Mapeo de Entidades:**
* Generar las entidades de Sea-ORM en Rust que soporten el borrado lógico (*Soft Delete*), permitiendo recuperar planos borrados accidentalmente.


* [ ] **5.3.2 — Lógica de Repositorio:**
* Desarrollar la función para obtener el "Último Respaldo Válido" de un ruteador específico usando el hash SHA-256 para detectar si hubo cambios reales en la configuración.



---

## Slice 5.4: Adaptador de Almacenamiento Regional Local ⚙️

> **Objetivo:** Programar el componente de infraestructura que organiza físicamente los archivos en tu disco local bajo una jerarquía regional lógica.

* [ ] **5.4.1 — Estructura Jerárquica de Carpetas:**
* Implementar el adaptador que guarda archivos en rutas organizadas: `/storage/sedes/{nombre_sede}/{tipo_archivo}/`.
* Asegurar que el sistema use `tokio::fs` para operaciones de escritura no bloqueantes, manteniendo el servidor de Axum rápido durante las subidas.



---

## Slice 5.5: Endpoints de Carga Streaming y Descarga en Axum 🛣️

> **Objetivo:** Crear las rutas de API optimizadas para manejar flujos de datos grandes sin saturar la memoria RAM del servidor.

* [ ] **5.5.1 — Carga vía Multipart Streaming:**
* Implementar el endpoint `POST /api/v1/infrastructure/upload` usando flujos de datos (*streaming*).
* Integrar el cálculo del hash SHA-256 "al vuelo": si el archivo que se intenta subir es idéntico al último respaldo existente, la API debe informar que ya existe para ahorrar espacio en disco.


* [ ] **5.5.2 — Despacho de Planos SVG:**
* Crear el endpoint de descarga que sirva los diagramas con las cabeceras de visualización correctas (`image/svg+xml`) para que el navegador los renderice como gráficos interactivos.



---

## Slice 5.6: UI Drag-and-Drop (Svelte 5 + TanStack Query + Zod) 🎨

> **Objetivo:** Diseñar una interfaz de carga moderna y segura en la ruta `/dashboard/infrastructure/upload`.

* [ ] **5.6.1 — Validador de Cliente con Zod:**
* Usar **Zod** en el frontend para validar que el archivo seleccionado cumpla con el tamaño y la extensión antes de que salga del navegador del técnico.


* [ ] **5.6.2 — Zona de Arrastre Reactiva:**
* Construir el componente con **Svelte 5 Runes** gestionando los estados de `uploading` y `progress` de forma visual con Tailwind v4.
* Usar `createMutation` de **TanStack Query** para manejar la subida y refrescar la lista de archivos automáticamente tras el éxito.



---

## Slice 5.7: Visor de Topología SVG y Galería de Racks 🖼️

> **Objetivo:** Construir el centro de monitoreo visual donde el operador audita físicamente las sedes.

* [ ] **5.7.1 — Visor Interactivo de SVG:**
* Programar el visor que embebe el código XML del SVG directamente en la página, permitiendo que el operador haga zoom y vea los nombres de los dispositivos en el plano de red.


* [ ] **5.7.2 — Galería de Sedes:**
* Implementar la cuadrícula visual de fotos de los gabinetes de servidores, facilitando que un técnico nuevo identifique cuál cable debe tocar en el rack físico de Trinidad.



---

## Slice 5.8: Pruebas de Integridad y Sanitización 🏁

> **Objetivo:** Validar que el sistema sea resistente a ataques y errores humanos.

* [ ] **5.8.1 — Prueba de Seguridad SVG:**
* Intentar subir un mapa SVG con código JavaScript malicioso. El backend debe rechazarlo o sanitizarlo exitosamente.


* [ ] **5.8.2 — Prueba de Deduplicación:**
* Subir dos veces el mismo archivo de configuración de un switch. Confirmar en tu **MySQL Workbench** que solo existe un registro físico y que el sistema ahorró espacio.


* [ ] **5.8.3 — Ciclo Completo Regional:**
* Subir una topología para Riberalta, verificar su guardado en la carpeta correcta en tu disco local y confirmar su renderizado perfecto en la UI de Svelte 5.



---

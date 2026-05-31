# 🗺️ Roadmap Actualizado — Módulo 6: Auditoría Inmutable

### 🛡️ Caja Negra Digital y Trazabilidad Forense de la Red del Beni

```text
Propósito: Capturar de forma quirúrgica y en tiempo real cada acción crítica realizada por los operadores sobre la red, garantizando un registro inalterable para auditorías técnicas e investigaciones de incidentes.
Entregable: Sistema de telemetría de eventos con almacenamiento indexado en Sea-ORM, extractor de contexto de red en Axum 0.8 (IPs, User-Agent) y visor cronológico de auditoría (Timeline) interactivo en Svelte 5.
Regla de Pureza: Registro de solo escritura (Append-Only). El dominio prohíbe explícitamente operaciones de UPDATE o DELETE. Las estructuras almacenan capturas formateadas independientes del estado volátil futuro de la base de datos.
Estados: [ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

### 📊 Matriz de Progreso General

| Slice | Nombre | Referencia ADR | Progreso |
| --- | --- | --- | --- |
| **6.1** | Esquema Estricto Append-Only (MySQL Workbench) | `ADR-0004`, `ADR-0009` | [ ] |
| **6.2** | Modelos de Seguridad y Puertos del Dominio | `ADR-0001` | [ ] |
| **6.3** | Entidad Inmutable y Persistencia (Sea-ORM) | `ADR-0004` | [ ] |
| **6.4** | Interceptores y Extractores de Red en Axum 0.8 | `ADR-0003`, `ADR-0012` | [ ] |
| **6.5** | API de Consulta y Filtros Avanzados de Auditoría | `ADR-0003`, `ADR-0006` | [ ] |
| **6.6** | UI de Línea de Tiempo Quirúrgica (Svelte 5 + UI) | `ADR-0017` [TIMELINE] | [ ] |
| **6.7** | Alertas Visuales para Eventos Regionales Críticos | `ADR-0011`, `ADR-0017` | [ ] |
| **6.8** | Pruebas de Estrés e Intentos de Alteración de Logs | `ADR-0010` | [ ] |
| **M6** | **Módulo 6 Total** |  | **[ ]** |

---

## Slice 6.1: Esquema Estricto Append-Only (MySQL Workbench) 🗄️

> **Objetivo:** Crear la estructura física de almacenamiento en tu base de datos local diseñada para búsquedas forenses rápidas y bloqueo absoluto de modificaciones.

* [ ] **6.1.1 — Diseño del Archivo SQL:**
* Crear el archivo plano en `data/migrations/0006_audit_trail.sql`.
* Definir la tabla `audit_logs` incluyendo campos como `username_snapshot` (para mantener la autoría intacta aunque el usuario sea eliminado en el futuro) y la columna estructurada `structured_payload` para metadatos en JSON.


* [ ] **6.1.2 — Restricciones en Workbench:**
* Configurar los índices compuestos necesarios para que las búsquedas por tipo de acción y fecha vuelen.
* Configurar a nivel de base de datos en tu **MySQL Workbench** un rol de conexión restringido para la aplicación que revoque explícitamente los privilegios de ejecución de comandos `UPDATE` y `DELETE` sobre esta tabla.



---

## Slice 6.2: Modelos de Eventos de Seguridad y Puertos 🧠

> **Objetivo:** Tipar de forma estricta las acciones del sistema en la capa central del negocio, aislando la lógica de los controladores web.

* [ ] **6.2.1 — Catálogo de Acciones del Dominio:**
* Definir el enum fuertemente tipado `AuditAction` para operaciones críticas: reconocimiento de caídas de red, descargas de respaldos de ruteadores, mutaciones en planos SVG de sedes y cambios de credenciales de seguridad.


* [ ] **6.2.2 — Puertos de Auditoría:**
* Crear la estructura de datos `AuditEvent` con metadatos fijos (IP, estampa de tiempo, recurso afectado) y declarar el trait/puerto asíncrono para el enrutamiento de logs en el dominio.



---

## Slice 6.3: Entidad Inmutable y Repositorio de Auditoría (Sea-ORM) 🔌

> **Objetivo:** Implementar la persistencia con Sea-ORM asegurando que el repositorio cumpla el contrato inmutable.

* [ ] **6.3.1 — Mapeo de Campos JSON:**
* Configurar la entidad en Rust conectando el campo `structured_payload` a un tipo JSON nativo compatible con MySQL.


* [ ] **6.3.2 — Repositorio Inmirable:**
* Programar el método de inserción y la consulta de lectura pagipada filtrada por rangos de fecha y sede regional, omitiendo deliberadamente cualquier función de modificación de registros en el código.



---

## Slice 6.4: Interceptores y Extractores de Contexto de Red en Axum 0.8 ⚙️

> **Objetivo:** Capturar automáticamente los datos de red del operador en cada petición HTTP sin duplicar código en tus controladores.

* [ ] **6.4.1 — Extractor de Red Asíncrono:**
* Desarrollar el middleware extractor en Axum para capturar la dirección IP real del cliente (analizando cabeceras `X-Real-IP` o la información del socket nativo) junto con el `User-Agent` del navegador.


* [ ] **6.4.2 — Helpers de Inserción Limpia:**
* Programar macros ligeras para que registrar un evento de seguridad dentro de tus handlers del dashboard sea una sola línea limpia de código.



---

## Slice 6.5: API de Consulta y Filtros Avanzados de Auditoría 🛣️

> **Objetivo:** Exponer los endpoints seguros para que el personal de alta jerarquía técnica inspeccione el historial del sistema.

* [ ] **6.5.1 — Construcción del Controlador:**
* Implementar el endpoint seguro `GET /api/v1/audit/logs` blindado por tu sistema de roles (RBAC) con acceso restrictivo únicamente para roles de control como `SUPER_ADMIN` o `AUDITOR`.
* Forzar que la ordenación de los registros sea estrictamente descendente (`ORDER BY created_at DESC`).



---

## Slice 6.6: UI de Línea de Tiempo Quirúrgica (Svelte 5 + TanStack Query + shadcn) 🎨

> **Objetivo:** Diseñar la bitácora visual interactiva en la ruta `/dashboard/audit` simulando una línea de tiempo táctica de alta legibilidad, clonando el estilo Zinc Ultra Dark de tu captura de pantalla principal.

* [ ] **6.6.1 — Consumo de Flujo Forense con TanStack Query:**
* Configurar `createQuery` para devorar los bloques de logs de Axum en tiempo real, controlando filtros de búsqueda por IP o acción sin provocar saltos o pestañeos en la interfaz.


* [ ] **6.6.2 — Línea de Tiempo Estilizada con shadcn-svelte:**
* Diseñar la estructura de eventos en disposición vertical usando contenedores limpios de **shadcn-svelte**. El sistema renderizará las cadenas exactas en el navegador de la siguiente forma:
* **[Reconocimiento de Alerta Crítica]:** El usuario **Milton Daniel** reconoció la alerta de caída del *switch de Riberalta* desde la dirección IP `192.168.10.45` usando Firefox.
* **[Descarga de Respaldo]:** El usuario **Operador_Beni** descargó la configuración `router-core-trinidad.cfg`. IP de origen: `192.168.1.100`.
* **[Mutación de Esquema]:** El usuario **Admin_Redes** actualizó la topología SVG de la sede *Guayaramerín*.





---

## Slice 6.7: Alertas Visuales para Eventos Regionales Críticos 🚨

> **Objetivo:** Resaltar visualmente las acciones sospechosas o de alta prioridad para facilitar el escaneo rápido del operador.

* [ ] **6.7.1 — Esquemas de Contrato con Zod:**
* Implementar esquemas de **Zod** en el cliente para validar que los payloads JSON variables que llegan del historial de auditoría tengan la estructura correcta antes de inyectarlos en la interfaz.


* [ ] **6.7.2 — Estados Derivados Reactivos:**
* Crear subcomponentes de fila usando la rune `$derived` de Svelte 5. Si el código de acción corresponde a un evento de peligro (como un acceso denegado o una caída de equipo), teñir el borde de la tarjeta con alertas de color rojo intenso de Tailwind v4; si es rutinario, aplicar tonos oscuros neutrales.



---

## Slice 6.8: Pruebas de Estrés e Intentos de Alteración de Logs 🏁

> **Objetivo:** Forzar fallos provocados en el entorno local para certificar la inmutabilidad de la bitácora.

* [ ] **6.8.1 — Intento de Hackeo en Workbench:**
* **Prueba de Fuego:** Intentar ejecutar una consulta de edición manual (`UPDATE audit_logs SET user_id = ...`) directamente en las pestañas de comandos de tu **MySQL Workbench** usando el usuario de la aplicación. El motor local debe abortar la operación y retornar un error de denegación de permisos de inmediato.


* [ ] **6.8.2 — Trazabilidad E2E en Caliente:**
* Iniciar sesión, ejecutar un evento técnico real en tu UI del Dashboard (Módulo 3), y verificar mediante Workbench que la línea se guardó intacta con tu IP local, estampa de tiempo exacta de Bolivia y el payload JSON estructurado perfecto.

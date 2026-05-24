# ADR 0006: RBAC, Sesiones y Auditoría
**Autores:** Milton Hipamo / Laboratorio 3030

**Relacionado con:** ADR 0001 (Hexagonal), ADR 0002 (Configuración), ADR 0003 (Backend), ADR 0004 (MySQL), ADR 0005 (Migraciones)

---

## 📋 Contexto y Decisión

**Problema:** El sistema procesa y almacena información crítica de infraestructura de red regional, inventarios físicos de telecomunicaciones, alertas de intrusión y configuraciones operativas de agentes remotos. Se requiere un control de acceso granular de alta seguridad, trazabilidad forense absoluta ante incidentes y cumplimiento de estándares institucionales sin penalizar el rendimiento del servidor.

**Decisión:** Implementar un modelo de Control de Acceso Basado en Roles (**RBAC**) con permisos atómicos, gestión de **sesiones seguras mediante PASETO v4 Local** (rechazando JWT tradicional), borrado lógico universal (**Soft Delete**) y un subsistema automatizado de **auditoría forense obligatoria** para cada mutación de estado.

---

## 🗄️ Modelo de Datos y Estructura de Persistencia (MySQL)

El esquema de seguridad se consolida sobre 8 tablas relacionales optimizadas, abstrayendo el acceso físico mediante entidades fuertemente tipeadas de Sea-ORM:

| Tabla | Propósito Operacional | Estrategia de Seguridad / Restricción |
| --- | --- | --- |
| `users` | Registro maestro de operadores del sistema. | `is_active = FALSE` por defecto; `email` actúa como identificador único y llave primaria lógica. |
| `roles` | Roles base del ecosistema (`admin`, `operator`, `viewer`, `agent`). | Control de estado mediante Soft Delete. |
| `permissions` | Catálogo de permisos granulares con el formato estricto `recurso:acción` (ej: `devices:write`). | Control de estado mediante Soft Delete. |
| `user_roles` | Tabla intermedia de asignación de roles a usuarios. | Llaves foráneas con eliminación en cascada lógica. |
| `role_permissions` | Tabla intermedia de mapeo de permisos por cada rol. | Llaves foráneas con eliminación en cascada lógica. |
| `tokens` | Almacenamiento de tokens de un solo uso para verificación de correo y restablecimiento de claves. | Almacenamiento exclusivo del hash criptográfico **SHA-256** (`token_hash`), jamás el token en texto plano. |
| `sessions` | Control y revocación de sesiones activas en el sistema. | Almacenamiento exclusivo del hash **SHA-256** derivado del refresh token de larga duración. |
| `audit_logs` | Historial inmutable y permanente de mutaciones de estado. | Columna `JSON` para payloads. **Optimización MySQL:** Uso de columnas generadas virtuales indexadas por usuario, recurso y acción. |

### Reglas Inmutables del Modelo (Código 3026)

* **Minimalismo en Identidad:** Se elimina el uso de campos redundantes como `username` o `avatar_url`. La autenticación y comunicación se rigen estrictamente por `email` + `name`.
* **Prohibición de DELETE Físico:** Ningún registro operativo se elimina físicamente del disco. Las operaciones de borrado actualizan el campo `deleted_at` (Soft Delete), aislándose de las consultas ordinarias mediante filtros globales automatizados en los repositorios de Rust.
* **Opacidad de Secretos de Tránsito:** Los tokens generados solo existen en texto plano en la memoria volátil de la aplicación o dentro del cuerpo del correo electrónico enviado al usuario. En la base de datos MySQL solo residen sus representaciones en hashes SHA-256 para mitigar ataques de inyección u obtención ilícita de volcados de datos.

---

## 🛡️ Reglas Obligatorias de Control y Auditoría

| Regla | Nombre | Descripción |
| --- | --- | --- |
| **R1** | Sesiones Autocontenidas | Se prohíbe el uso de estados de sesión opacos en el servidor; el acceso inmediato se valida mediante tokens de acceso PASETO v4 inmutables y firmados localmente. |
| **R2** | Inmutabilidad de Auditoría | La tabla `audit_logs` solo acepta operaciones de inserción (`INSERT`). Queda terminantemente bloqueada cualquier consulta de actualización (`UPDATE`) o eliminación (`DELETE`) mediante restricciones a nivel de base de datos. |
| **R3** | Cache Dinámica de RBAC | Para evitar impactos de latencia por consultas repetitivas de unión (`JOIN`), la matriz de permisos de un usuario activo se almacena en una caché asíncrona de alta velocidad (`moka`) con un tiempo de vida (TTL) estrictamente controlado. |
| **R4** | Contexto Obligatorio | Todo Handler de Axum que ejecute una mutación debe capturar e inyectar al caso de uso el `user_id` y la dirección IP del remitente para poblar el log de auditoría. |
| **R5** | Validación de Contraseñas | El sistema rechaza claves simples. Se impone una política por software: longitud mínima de 12 caracteres, inclusión obligatoria de mayúsculas, minúsculas, números y caracteres especiales. |

---

## 🛠️ Herramientas de Seguridad Aprobadas

| Herramienta | Versión | Propósito en el Ecosistema |
| --- | --- | --- |
| `pasetors` | `0.7.x` | Implementación nativa y criptográficamente segura de tokens PASETO v4 (Protocolo V4.Local). |
| `argon2` | `0.5.x` | Algoritmo de hashing de contraseñas de última generación configurado en su variante `argon2id` (resistente a ataques de GPU y Side-Channel). |
| `secrecy` | `0.10.x` | Contenedores opacos en memoria para asegurar que las contraseñas crudas o llaves simétricas se limpien de forma segura (`Zeroize`). |
| `moka` | `0.12.x` | Caché en memoria asíncrona y concurrente para optimizar la resolución instantánea de permisos RBAC. |
| `sea-orm` | `1.1.x` | ORM asíncrono y mapeador seguro para la abstracción y filtrado automático de Soft Deletes en MySQL. |
| `cargo-nextest` | `0.9.x` | Entorno industrial de ejecución de pruebas concurrentes para validar la hermeticidad de los middlewares de autenticación. |

> **Nota de Diseño:** Se descarta explícitamente el uso de crates de sesión automatizados como `tower-sessions`. El ciclo de vida de las sesiones y la rotación de los tokens de refresco se gestionan de forma manual y controlada mediante lógica de negocio en la capa de aplicación, garantizando un control absoluto sobre el comportamiento del sistema ante revocaciones inmediatas.
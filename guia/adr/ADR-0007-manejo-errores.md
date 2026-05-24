# Resumen — ADR 0007: Manejo de Errores (DomainError + AppError + HTTP)
**Autores:** Milton Hipamo / Laboratorio 3030

**Relacionado con:** ADR 0001 (Hexagonal), ADR 0003 (Backend), ADR 0004 (MySQL), ADR 0006 (RBAC)

---

## 📋 Contexto y Decisión

**Problema:** Una gestión deficiente de excepciones en producción puede filtrar detalles críticos del sistema como la estructura interna de las tablas de la base de datos, trazas de ejecución en memoria, nombres de archivos del host, rutas de endpoints privados o secretos del sistema. Se requiere un mecanismo de control de errores que sea descriptivo para el operador, transparente y contextual para el desarrollador, y completamente seguro y opaco para el cliente externo.

**Decisión:** Establecer una jerarquía estricta de **tres niveles independientes** para la canalización e interpretación de errores dentro del ecosistema del monorepo:

```
┌─────────────────────────────────────────────────────────────┐
│ 1. DomainError (crates/domain)                              │ -> Lenguaje puro de negocio (No sabe qué es la web o HTTP)
└──────────────┬──────────────────────────────────────────────┘
               │ (Mapeado / Envuelto por)
               ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. AppError (apps/api)                                      │ -> Traductor técnico de infraestructura y flujos de Axum
└──────────────┬──────────────────────────────────────────────┘
               │ (Transformado atómicamente en)
               ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. HTTP Response (JSON Payload)                             │ -> Payload sanitizado, seguro y estandarizado para el cliente
└─────────────────────────────────────────────────────────────┘

```

---

## 🏗️ Los Tres Niveles de Control

### Nivel 1 — DomainError (`crates/domain`)

Define los fallos en el lenguaje agnóstico del negocio (Ubiquitous Language). **Tiene prohibido importar dependencias web, macros de Axum o anotaciones de serialización HTTP.**

| Variante del Enum | Significado Operacional dentro del Dominio |
| --- | --- |
| `Validation` | Una regla de negocio o invariante del dominio fue violada. |
| `NotFound` | El recurso solicitado no existe en los registros lógicos. |
| `AlreadyExists` | Intento de duplicación de una entidad única (ej: MAC, Email). |
| `InvalidCredentials` | Combinación de identidad y secreto incorrecta. |
| `InvalidToken` | Estructura o firma criptográfica inválida o alterada. |
| `SessionExpired` | El tiempo de vida útil del token ha concluido. |
| `Unauthorized` | El usuario carece de una identidad válida en el sistema. |
| `MissingPermission` | El rol del usuario no mapea con el permiso requerido. |
| `Conflict` | La operación colisiona con el estado actual de la entidad. |
| `Database` | Fallo de persistencia subyacente de bajo nivel. |
| `Internal` | Comportamiento inesperado o fallo crítico del sistema. |

### Nivel 2 — AppError (`apps/api`)

Actúa como un wrapper y adaptador de entrada web. Captura los fallos del dominio o de los componentes de infraestructura y los asocia a códigos de estado HTTP semánticos con mensajes de salida controlados.

| Status Code | Variante de AppError | Origen Típico del Fallo |
| --- | --- | --- |
| **400** | `BadRequest` | JSON malformado, tipos de datos corruptos en query params. |
| **401** | `InvalidCredentials` | Intento de autenticación fallido con Argon2id. |
| **401** | `InvalidToken` | Token PASETO v4 manipulado o con firma corrupta. |
| **401** | `SessionExpired` | Token de refresco expirado en la base de datos. |
| **403** | `Forbidden` | El usuario está autenticado pero falló la evaluación RBAC. |
| **403** | `MissingPermission` | Operación denegada por ausencia de un privilegio `recurso:acción`. |
| **404** | `NotFound` | Ruta HTTP inexistente o recurso del dominio no encontrado. |
| **409** | `AlreadyExists` | Colisión de registros a nivel de restricciones únicas en MySQL. |
| **422** | `Validation` | Datos sintácticamente correctos pero que violan invariantes del negocio. |
| **429** | `RateLimited` | Activación de las políticas de protección de ráfagas en el middleware. |
| **500** | `Internal` | Error genérico. **Cualquier traza interna es interceptada y borrada.** |
| **503** | `ServiceUnavailable` | El pool de conexiones hacia el motor de **MySQL** está saturado o caído. |
| **504** | `GatewayTimeout` | Tiempo de espera agotado al conectar con agentes o pasarelas externas. |

### Nivel 3 — HTTP Response (Salida Estándar JSON)

Axum transforma el `AppError` implementando de manera nativa el trait `IntoResponse`. La salida devuelta al cliente externo se unifica bajo una estructura JSON inmutable:

```json
{
  "error": "Mapeo abreviado de la variante (ej: NOT_FOUND)",
  "message": "Mensaje legible y seguro orientado al usuario de la interfaz",
  "details": null, 
  "request_id": "uuid-generado-por-el-middleware-de-trazabilidad"
}

```

> **Nota sobre `details`:** El campo `details` se poblará con una lista estructurada de campos fallidos únicamente cuando ocurra un error `422 Validation`. Para errores internos del servidor (`500`), este campo permanecerá como `null` para evitar fugas de información.

---

## 🛡️ Principios Obligatorios de Hermeticidad (Código 3026)

* **Opacidad Total en Producción:** Queda terminantemente prohibido que viajen hacia el exterior strings que contengan palabras clave como `SELECT`, `MySQL`, `Connection Refused`, o dumps de variables. El cliente solo recibe códigos de error estandarizados.
* **Segmentación del Árbol de Errores:** Las librerías de conversión de errores genéricos con esteroides (`anyhow`) pertenecen única y exclusivamente a los ejecutables finales de la capa de aplicación (`apps/`). Los crates lógicos (`crates/`) deben utilizar tipos fuertemente definidos.
* **Estandarización de Validaciones (422 vs 400):** Un payload mal estructurado sintácticamente (como un corchete faltante en el JSON) devuelve un error `400 Bad Request`. Un payload estructurado correctamente pero con datos lógicos inválidos (como un correo electrónico sin el símbolo `@`) devuelve un error `422 Unprocessable Entity`.

---

## 🛠️ Herramientas de Gestión de Errores Aprobadas

| Herramienta | Versión | Ubicación Permitida | Propósito Arquitectónico |
| --- | --- | --- | --- |
| `thiserror` | `2.0.x` | Exclusivo en `crates/*` | Generación automática y eficiente de los traits `Display` y `Error` de Rust en enums cerrados del dominio sin sobrecostos de rendimiento. |
| `anyhow` | `1.0.x` | Exclusivo en `apps/*` | Captura y enriquecimiento de errores con contextos dinámicos en los puntos de entrada, ideal para la integración de flujos asíncronos en Axum. |
| `serde_json` | `1.0.x` | Exclusivo en `apps/api` | Serialización final controlada de la estructura JSON de respuesta HTTP sanitizada. |
| `tracing` | `0.1.x` | Todo el Workspace | Sistema de telemetría estructurado encargado de registrar el error real completo con su stack trace en los archivos de log internos, asociándolo al `request_id`. |
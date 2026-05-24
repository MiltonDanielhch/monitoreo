# Resumen — ADR 0002: Configuración Tipeada (Fail-Fast y Secretos)
**Autores:** Milton Hipamo / Laboratorio 3030
---

## 📋 Contexto y Decisión

**Problema:** Los errores de configuración (variables de entorno faltantes, secretos expuestos en logs, tipos de datos incorrectos o formatos inválidos) causan comportamientos impredecibles y fallos silenciosos en entornos de producción.

**Decisión:** Implementar una estrategia de configuración **fuertemente tipeada, centralizada y validada en el segundo cero** del ciclo de vida de la aplicación mediante un enfoque **Fail-Fast**: el binario abortará inmediatamente la ejecución en el arranque si la configuración es inválida, incompleta o insegura.

---

## 🏗️ Jerarquía y Mecanismo de Carga

El sistema consolida la configuración en un único struct de Rust (`AppConfig`) subdividido en módulos lógicos (Servidor, BD, Seguridad, Mailer, Storage). La resolución de variables sigue un orden de prioridad estricto de arriba hacia abajo:

1. **Variables de Entorno del Sistema:** Inyectadas directamente por el host en producción (Docker / Coolify).
2. **Archivo `.env.local`:** Archivo opcional exclusivo para anulaciones de desarrollo local (**Prohibido en Git**).
3. **Archivo `.env`:** Variables base de desarrollo y configuración por defecto para testing (**Prohibido en Git**).
4. **Plantilla `.env.example`:** Documento de referencia pública con datos ficticios (**Siempre en Git**).

---

## ⚡ Validaciones Fail-Fast en el Arranque

El inicializador del sistema realizará comprobaciones semánticas y sintácticas duras antes de levantar el pool de conexiones o el servidor HTTP. El proceso fallará inmediatamente si:

* **Tipado e Integridad:** Falta cualquier variable obligatoria o su valor no coincide con el tipo estricto esperado (ej. un puerto que no sea un entero `u16`).
* **Esquema de Base de Datos:** `DATABASE_URL` no inicia estrictamente con el prefijo **`mysql://`** (requerido para Sea-ORM/MySQL).
* **Esquema del Agente:** `SQLITE_URL` no representa una ruta de archivo válida o almacenable para el agente remoto.
* **Criptografía Crítica:** `PASETO_SECRET` cuenta con una longitud menor a 32 bytes o se detecta que posee baja entropía.
* **Formatos Fuertes:** Correos electrónicos, cadenas de conexión S3 o URLs de Healthchecks no superan las validaciones de estructura (regex/parsers).
* **Restricciones de Producción (`APP_ENV = "production"`):**
* Exigencia de HTTPS obligatorio en todas las URLs públicas y endpoints de servicios.
* Presencia obligatoria de credenciales reales para servicios externos (Resend S3).
* Verificación de que las claves secretas no utilicen los valores por defecto de desarrollo.



---

## 🛡️ Reglas Obligatorias de Seguridad

| Regla | Nombre | Descripción |
| --- | --- | --- |
| **R1** | Aislamiento en Git | Los archivos `.env`, `.env.local` y cualquier volcado de variables reales deben estar explícitamente declarados en el `.gitignore`. |
| **R2** | Sincronización del Ejemplo | Cada nueva variable introducida al struct `AppConfig` debe reflejarse inmediatamente en `.env.example` con valores ficticios e instructivos. |
| **R3** | Inyección Hermética | Los secretos de producción jamás se escriben en archivos planos dentro del servidor; se manejan únicamente en la memoria efímera del contenedor. |
| **R4** | Opacidad en Logs | Queda estrictamente prohibido imprimir secretos o configuraciones sensibles en la consola o sistemas de telemetría. |
| **R5** | Prohibición de Hardcode | Ninguna clave, password, salt o secreto puede existir como un literal de texto en el código fuente (`src/`). |
| **R6** | Secretos en Memoria | Todos los campos sensibles dentro del struct de configuración deben estar envueltos en tipos opacos como `SecretString` o `SecretBox`. |
| **R7** | Entropía Asegurada | Las claves criptográficas del sistema deben ser validadas en arranque asegurando que provengan de fuentes con suficiente aleatoriedad térmica (`OsRng`). |
| **R8** | Cifrado en Tránsito | Todo endpoint de API externa configurado para producción debe validar el protocolo seguro TLS/HTTPS. |

---

## 🛠️ Herramientas de Configuración Aprobadas

| Herramienta | Versión | Propósito en el Ecosistema |
| --- | --- | --- |
| `config` | `0.15.x` | Deserialización y unificación de múltiples fuentes de datos en estructuras tipadas de Rust. |
| `dotenvy` | `0.15.x` | Fork reactivado y seguro para la carga dinámica de archivos de entorno en fases de desarrollo. |
| `secrecy` | `0.10.x` | Envoltura de seguridad en memoria que implementa la limpieza automática de datos (`Zeroize`) ante caídas. |
| `shadow-rs` | `0.35.x` | Inyección de metadatos de compilación (commit hash, branch, versión de rustc) para el arranque del sistema. |
| `taplo` | `0.10.x` | Analizador, linter y formateador de consistencia para estructuras TOML del Workspace. |

---

> **Nota de Implementación (Código 3026):** Al envolver secretos con `secrecy::SecretString`, los intentos accidentales de registrar la configuración mediante macros de rastreo (`println!("{:?}", config)` o `info!(?config)`) imprimirán de manera segura e inmutable la cadena `[REDACTED]`, bloqueando cualquier vector de fuga de credenciales en los archivos de log.
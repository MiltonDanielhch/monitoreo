# Resumen — ADR 0008: Seguridad: Argon2id + PASETO v4 Local + Refresh Tokens
**Autores:** Milton Hipamo / Laboratorio 3030
**Relacionado con:** ADR 0001 (Hexagonal), ADR 0003 (Backend), ADR 0004 (MySQL), ADR 0006 (RBAC), ADR 0007 (Errores)

---

## 📋 Contexto y Decisión

**Problema:** El sistema de monitoreo de red y control regional requiere un mecanismo de autenticación y autorización moderno, soberano y de alto rendimiento. Los tokens JWT tradicionales introducen vulnerabilidades críticas por diseño (manipulación del header `alg`, claves débiles por falta de estandarización, payload expuesto de forma insegura).

**Decisión:** Adoptar el algoritmo **Argon2id** para el hashing de contraseñas de operadores, implementar **PASETO v4 Local** como estándar único e inmutable para tokens de acceso de corta duración, y persistir **Refresh Tokens opacos con rotación obligatoria (RTR - Refresh Token Rotation)** dentro de la base de datos MySQL.

> 🚫 **JWT queda terminantemente prohibido** dentro de todo el espacio de trabajo (workspace).

---

## 🔐 Stack Criptográfico y Tecnológico Aprobado

| Herramienta / Crate | Propósito Arquitectónico | Versión | Restricción / Configuración |
| --- | --- | --- | --- |
| `argon2` | Hashing adaptativo de credenciales. | `0.5.x` | Configuración variante `Argon2id` según directrices OWASP. |
| `pasetors` | Criptografía simétrica autenticada (AEAD). | `0.7.x` | Features: `["v4", "std"]`. Implementa XChaCha20-Poly1305. |
| `secrecy` | Envoltura segura de secretos en memoria. | `0.10.x` | Garantiza borrado automático (`Zeroize`) en dumps de memoria. |
| `rand_core` | Generador de números aleatorios seguro (CSPRNG). | `0.6.x` | Generación de sales criptográficas y entropía de tokens. |
| `uuid` | Identificadores únicos universales versión 7. | `1.x` | Llaves primarias ordenables por tiempo para indexación óptima en MySQL. |
| `tokio` | Entorno de ejecución asíncrono. | `1.x` | Gestión del bucle de limpieza de tokens caducados en segundo plano. |

**Nota de Compatibilidad:** La suite criptográfica corre sobre **Rust 1.95.0** (Stable, liberado en abril de 2026), garantizando el soporte nativo de las últimas optimizaciones del compilador.

---

## 📜 Las 8 Reglas Inmutables de Seguridad (Código 3026)

| # | Regla de Seguridad | Descripción Técnica / Mecanismo de Control |
| --- | --- | --- |
| **1** | **Bloqueo Estricto de JWT** | La dependencia `jsonwebtoken` o similares están prohibidas. `cargo-deny` fallará inmediatamente en el pipeline de CI/CD si detecta su introducción. |
| **2** | **Fail-Fast en Clave Simétrica** | La variable de entorno `PASETO_SECRET` se valida atómicamente durante el arranque del servidor. Debe poseer exactamente 32 bytes (256 bits). Si falta o es incorrecta, el sistema provoca un `panic!` inmediato. |
| **3** | **Middlewares Excluyentes** | Todo extractor o middleware de autenticación en Axum que intercepte un token con estructura o prefijos heredados de JWT (`eyJ...`) abortará la petición con un rechazo explícito sin evaluar la base de datos. |
| **4** | **Opacidad de Refresh Tokens** | Los tokens de refresco son cadenas aleatorias de alta entropía. En las tablas de **MySQL**, se almacenan exclusivamente procesados mediante el hash criptográfico **SHA-256**. |
| **5** | **Limpieza Eficiente sin Sobrecosto** | El purgado de sesiones y tokens expirados se ejecuta mediante una tarea nativa en segundo plano controlada por **`tokio::spawn`** y **`tokio::time::interval`**, evitando sobrecargar el backend con motores de colas externos pesados. |
| **6** | **Trazabilidad de Identidad** | Tras una validación PASETO exitosa, el middleware inyecta el `user_id` directamente en los *tracing spans* activos para asegurar la correlación forense de logs descrita en el ADR 0007. |
| **7** | **Toolchain de Vanguardia** | El entorno mínimo de compilación e infraestructura está anclado en **Rust 1.95.0**, asegurando la inmunidad contra regresiones de rendimiento en primitivas de red. |
| **8** | **Criptografía Segura NAtiva** | `pasetors` se compila con las características `["v4", "std"]` activas, forzando el uso del protocolo V4 de cifrado simétrico robusto sobre librerías estándar. |

---

## 🔄 Flujo de Rotación de Tokens (RTR)

Cuando un operador solicita la renovación de su acceso mediante un Refresh Token:

1. El sistema hashes el token entrante y lo busca en la tabla `sessions` de MySQL.
2. Si el token es válido y no ha expirado, se genera un **nuevo par** de Access Token (PASETO) y Refresh Token.
3. El Refresh Token viejo se invalida inmediatamente y el nuevo toma su lugar.
4. **Protección contra Brechas:** Si el sistema detecta que un Refresh Token ya invalidado intenta utilizarse de nuevo, se asume un ataque por reutilización. La API revoca inmediatamente **todas** las sesiones activas asociadas a ese `user_id` de manera fulminante.
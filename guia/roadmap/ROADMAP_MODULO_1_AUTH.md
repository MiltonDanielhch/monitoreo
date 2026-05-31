Aquí tienes el **Roadmap Completo e Integrado** para el **Módulo 1**. He consolidado el documento inyectando de forma explícita cada uno de los **ADRs reales** obtenidos en la auditoría de tu script `audit.py`. De esta forma, cada tarea técnica queda perfectamente vinculada a los estándares del **Lab 3030** y el **Código 3026**.

---

# 🗺️ Roadmap Completo — Módulo 1: Autenticación, Usuarios y RBAC

### 🛠️ Gobernado por la Matriz de Arquitectura del Lab 3030

> **Propósito:** Construir el núcleo de identidad, control de sesiones persistentes y protección de rutas mediante permisos por roles (RBAC) sin acoplar el dominio a la infraestructura.
> **Regla de Pureza (`ADR-0001`):** El dominio es agnóstico a los tokens. No sabe qué es un PASETO, qué es un JSON, qué es Sea-ORM o qué es Docker. Solo procesa identidades, contratos de servicios y errores tipificados.
> **Stack Tecnológico:** Rust 2024 (v1.95.0 · `ADR-0003`) · Axum 0.8 (`ADR-0003`) · Sea-ORM 1.1 (`ADR-0004`) · pasetors 0.7 (`ADR-0008`) · Argon2id (`ADR-0008`) · SvelteKit 2 / Svelte 5 Runes (`ADR-0017`) · Tailwind v4 · Docker Compose / MySQL (`ADR-0013`).

---

## Estados

```text
[ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

---

## Progreso General y Trazabilidad de ADRs

| Slice | Nombre del Bloque Técnico | ADRs Vinculados | Estado |
| --- | --- | --- | --- |
| **1.0** | Inicialización del Entorno de Redes y ADRs | `ADR-0008`, `ADR-0011`, `ADR-0018` | [x] |
| **1.1** | Esquema de Base de Datos y Sesiones en Docker | `ADR-0004`, `ADR-0005`, `ADR-0013` | [x] |
| **1.2** | Capa del Dominio Pura: Modelos y Errores (`crates/domain`) | `ADR-0001`, `ADR-0007` | [x] |
| **1.3** | Tipos de Contrato y DTOs Unificados (`crates/shared_types`) | `ADR-0011`, `ADR-0016` | [x] |
| **1.4** | Infraestructura de Persistencia Sea-ORM (`crates/database`) | `ADR-0001`, `ADR-0004` | [x] |
| **1.5** | Suite Criptográfica: Argon2id, PASETO v4 y SHA-256 (`crates/infrastructure`) | `ADR-0008` | [x] |
| **1.6** | Controladores HTTP, Rotación (RTR) y Extractores de Axum | `ADR-0003`, `ADR-0006`, `ADR-0009` | [x] |
| **1.7** | Interfaz de Acceso y Gestión de Estado Reactivo (Svelte 5 Runes) | `ADR-0017` | [x] |
| **1.8** | Filtros de Navegación del Cliente SvelteKit (`+layout.ts`) | `ADR-0017` | [x] |
| **1.9** | Pruebas de Ataque por Reutilización y Resistencia E2E | `ADR-0010` | [x] |
| **M1-B** | Tarea en Segundo Plano: Purga Asíncrona de Sesiones Muertas | `ADR-0015` | [x] |

---

## Detalle de Ejecución por Slices

### Slice 1.0: Inicialización del Entorno de Redes y ADRs 📑

> **Objetivo:** Establecer la gobernanza técnica del monorepo y asegurar que el entorno local arranque de forma predecible.

```text
[x] Validar que el compilador local esté anclado en `rustc 1.95.0` o superior (Rust 2024).
[x] Almacenar formalmente el `ADR-0008: Seguridad: Argon2id + PASETO v4 Local + Refresh Tokens` en la ruta `guia/adr/ADR-0008-seguridad-auth-paseto.md`.
[x] Configurar variables de entorno iniciales y cargarlas de forma segura mediante secretos tipeados (`ADR-0002`) en `.env.local`:
    [x] `DATABASE_URL=mysql://root:Milton123@127.0.0.1:3306/redes_dev`
    [x] `PASETO_SECRET=[Clave simétrica criptográfica de exactamente 32 bytes / 256 bits]`
```

### Slice 1.1: Esquema de Base de Datos y Sesiones en Docker 🐳

> **Objetivo:** Modelar el esquema físico para dar soporte al almacenamiento de credenciales y al mecanismo de control de sesiones concurrentes con UUIDv7 de alta performance.

```text
[x] Crear archivo de migración SQL nativo gestionado por el sistema de versionado (`ADR-0005`) en `data/migrations/0001_init_auth.sql`:
    [x] Definir tabla `roles` con clave primaria (VARCHAR(36) o BINARY(16) para indexación eficiente de UUIDv7).
    [x] Definir tabla `users` incluyendo `password_hash` (VARCHAR(255) para soportar Argon2id) y relación hacia `roles`.
    [x] Definir tabla `user_sessions` (id, user_id, refresh_token_hash VARCHAR(64), expires_at, created_at, updated_at).
    [x] NOTA: El `refresh_token_hash` guardará el hash SHA-256 del token para garantizar la opacidad en la base de datos de persistencia (`ADR-0004`).
[x] Desplegar la base de datos e inyectar la migración en el contenedor orquestado por Docker Compose (`ADR-0013`):
    [x] Comando: docker exec -i redes-db-dev mysql -u redes -predes redes_dev < data/migrations/0001_init_auth.sql
[x] Insertar datos semilla (Seeding `ADR-0005`) iniciales con los roles operativos requeridos para la consolidación electoral regional en Beni:
    [x] `INSERT INTO roles VALUES ('ADMIN'), ('OPERATOR'), ('MONITOR');`

```

### Slice 1.2: Capa del Dominio Pura: Modelos y Errores (`crates/domain`) 🦀

> **Objetivo:** Encapsular las reglas del negocio de control de accesos aislando por completo el código de dependencias web o de persistencia (`ADR-0001`).

```text
[x] Implementar el árbol de fallos del dominio con errores tipificados (`ADR-0007`) en `crates/domain/src/errors.rs` derivando `thiserror::Error`.
    [x] Declarar variantes atómicas: `InvalidEmail`, `WeakPassword`, `InvalidCredentials`, `UserSuspended`, `SessionExpired`, `ReusedTokenAttack`, `RateLimitExceeded`.
[x] Crear los modelos de negocio puros en `crates/domain/src/models/`:
    [x] `user.rs`: Struct `User` con validaciones de formato de correo e invariantes de fortaleza intrínseca de contraseña.
    [x] `session.rs`: Entidad lógica que mapea los tiempos de expiración y vigencia de las sesiones.
[x] Asegurar compilación limpia sin acoplamiento a crates de infraestructura externa:
    [x] Comando: cargo check -p domain

```

### Slice 1.3: Tipos de Contrato y DTOs Unificados (`crates/shared_types`) 📦

> **Objetivo:** Definir las estructuras de datos de transporte compartidas de forma simétrica entre el backend en Rust y la interfaz de usuario en el esquema de monorepo (`ADR-0011`).

```text
[x] Configurar el crate interno compartible `crates/shared_types` en el archivo de configuración del workspace.
[x] Definir los Data Transfer Objects (DTOs) decorados con `serde::Deserialize` y `Serialize`:
    [x] `LoginRequest` (email, pasword envuelto en `secrecy`).
    [x] `TokenRefreshRequest` (refresh_token opaco).
    [x] `AuthResponse` (access_token, refresh_token, user_info).
[x] Decorar las estructuras con macros de `utoipa::ToSchema` para habilitar la autogeneración de la documentación interactiva OpenAPI (`ADR-0016`).

```

### Slice 1.4: Infraestructura de Persistencia Sea-ORM (`crates/database`) 🗄️

> **Objetivo:** Mapear el modelo relacional a objetos de Rust a través de Sea-ORM y proveer repositorios desacoplados del dominio mediante abstracción de puertos (`ADR-0001`, `ADR-0004`).

```text
[x] Generar o escribir las entidades de mapeo relacional de Sea-ORM en `crates/database/src/entities/`:
    [x] Configurar `role_entity.rs`, `user_entity.rs` y `user_session_entity.rs`.
[x] Crear el repositorio concreto en `crates/database/src/repositories/auth_repository.rs`:
    [x] Implementar el método `find_session_by_hash(hash: &str)` utilizando cargas conjuntas (`find_also_related`) para traer al usuario y su rol en una sola operación.
    [x] Desarrollar funciones atómicas y transaccionales para crear, rotar o purgar (eliminar) registros de sesión.
    [x] Garantizar que los métodos de salida traduzcan los tipos de Sea-ORM a entidades puras del dominio (`crates/domain`).

```

### Slice 1.5: Suite Criptográfica: Argon2id, PASETO v4 y SHA-256 (`crates/infrastructure`) 🔒

> **Objetivo:** Dar cumplimiento estricto al estándar criptográfico avanzado definido en tu guía de seguridad corporativa (`ADR-0008`).

```text
[x] Configurar dependencias robustas en `crates/infrastructure/Cargo.toml`: `argon2`, `pasetors` (features = ["v4", "std"]), `secrecy`, `sha2` y `rand_core`.
[x] Crear módulo de contraseñas en `crypto/password.rs`:
    [x] Implementar `hash_password(password: &SecretString) -> Result<String, DomainError>` usando la variante robusta Argon2id.
    [x] Implementar `verify_password(hash: &str, password: &SecretString) -> bool`.
[x] Crear módulo de tokens de acceso simétricos en `crypto/paseto.rs`:
    [x] Validar en el inicio de la aplicación que la clave cargada en memoria tenga exactamente 32 bytes reales mediante envolturas protectoras.
    [x] Implementar `generate_access_token` emitiendo un token V4 Local con firmas criptográficas válidas.
    [x] Implementar `verify_access_token` retornando los claims validados o `DomainError::SessionExpired`.
[x] Crear módulo de tokens opacos en `crypto/opaque.rs`:
    [x] Implementar generador de Refresh Tokens utilizando entropía criptográfica de alta calidad (`rand_core`).
    [x] Implementar la función SHA-256 para hashear los tokens antes de impactar búsquedas en MySQL.

```

### Slice 1.6: Controladores HTTP, Rotación (RTR) y Extractores de Axum 📡

> **Objetivo:** Exponer los endpoints del API REST en Axum 0.8 (`ADR-0003`), implementar la rotación obligatoria de tokens e inyectar el control por roles RBAC (`ADR-0006`).

```text
[x] Implementar los controladores HTTP en `crates/infrastructure/src/handlers/auth_handler.rs`:
    [x] Endpoint `/api/auth/login`: Valida credenciales, genera el par PASETO + Refresh Token opaco, calcula el hash SHA-256 y persiste la sesión en MySQL.
    [x] Endpoint `/api/auth/refresh`: Aplica la lógica **RTR**. Si el token de refresco ya fue usado (hash duplicado o ya marcado en la base de datos), invoca la **Regla de Protección de Brechas** eliminando de inmediato todas las sesiones activas del `user_id` de forma fulminante.
[x] Implementar el extractor de seguridad y autorización en `crates/infrastructure/src/middleware/rbac.rs`:
    [x] Implementar el trait `FromRequestParts` de Axum para el validador de tipos `RequireRole`.
    [x] Interceptar la cabecera `Authorization`. Si el token inicia con prefijos antiguos basados en JWT (`eyJ...`), abortar la petición inmediatamente aplicando la política estricta anti-JWT.
    [x] Decodificar el PASETO v4 Local, verificar claims de rol e inyectar el identificador del operador en el contexto de logs de auditoría inmutable (`ADR-0006`).
[x] Incorporar políticas de **Rate Limiting** (`ADR-0009`) en las rutas críticas `/api/auth/login` y `/api/auth/refresh` para neutralizar vectores de fuerza bruta usando `axum-governor`.

```

### Slice M1-B: Tarea en Segundo Plano: Purga Asíncrona de Sesiones Muertas ⏳

> **Objetivo:** Mantener la higiene, los índices limpios y el tamaño controlado de la base de datos MySQL en producción utilizando hilos de ejecución asíncronos controlados.

```text
[x] Configurar un componente en segundo plano (Background Worker) utilizando las capacidades asíncronas de Tokio (`ADR-0015`).
[x] Implementar un bucle infinito controlado por un temporizador asíncrono (`tokio::time::interval`) configurado para ejecutarse de manera periódica (ej. cada hora).
[x] Programar la ejecución de una consulta SQL optimizada a través del repositorio para eliminar físicamente de la tabla `user_sessions` todas aquellas sesiones expiradas cuya fecha actual sea mayor a `expires_at`.

```

### Slice 1.7: Interfaz de Acceso y Gestión de Estado Reactivo (Svelte 5 Runes) 🎨

> **Objetivo:** Construir la UI del login y administrar de forma fluida el ciclo de vida de las sesiones web mediante tecnologías frontend modernas (`ADR-0017`).

```text
[x] Crear el archivo centralizado de sesión en tu aplicación cliente: `apps/web/src/lib/auth.svelte.ts`.
    [x] Usar la primitiva `$state` para mantener en memoria volátil de la pestaña el `accessToken` (JWT) y el rol decodificado.
    [x] Usar la primitiva `$derived` para computar de forma reactiva el estado booleano de `isAuthenticated`.
    [x] Implementar el mecanismo automático en segundo plano para solicitar un nuevo par de tokens mediante el endpoint de refresco antes de que el JWT expire.
[x] Maquetar la vista de acceso en `apps/web/src/routes/login/+page.svelte`:
    [x] Diseñar un formulario limpio utilizando clases utilitarias de Tailwind v4.
    [x] Vincular los inputs mediante enlaces reactivos bidireccionales (`bind:value`) nativos de Svelte 5.

```

### Slice 1.8: Filtros de Navegación del Cliente SvelteKit (`+layout.ts`) 🛡️

> **Objetivo:** Interceptor el flujo del enrutador en el navegador para restringir visualmente accesos a nivel de cliente antes de renderizar componentes (`ADR-0017`).

```text
[x] Crear o modificar el archivo de carga e interceptación global en `apps/web/src/routes/+layout.ts`.
[x] Evaluar de forma atómica el path de navegación a través de `url.pathname`.
[x] Si la ruta destino pertenece al espacio protegido (ej. `/dashboard/*`, `/infraestructura/*`) y el estado reactivo computado de `isAuthenticated` es falso, cortar la ejecución del enrutador inmediatamente y redirigir al usuario lanzando `throw redirect(307, '/login')`.

```

### Slice 1.9: Pruebas de Ataque por Reutilización y Resistencia E2E 🧪

> **Objetivo:** Forzar fallos de seguridad controlados utilizando la suite de testing para verificar la robustez de las defensas implantadas (`ADR-0010`).

```text
[x] **Prueba de Bloqueo JWT:** Intentar inyectar una cabecera `Authorization: Bearer eyJ...` en un endpoint protegido. Axum debe rechazar la petición en la frontera de red con un código HTTP 401 sin consultar la base de datos.
[x] **Prueba de Ataque RTR (Reutilización):** Simular la interceptación de un Refresh Token viejo. Al enviarlo dos veces al endpoint de refresco, comprobar que el backend dispare la alerta en los logs forenses y limpie a cero todas las sesiones de ese operador en MySQL de forma instantánea.
[x] **Prueba de Ciclo Limpio:** Iniciar sesión con éxito, verificar el correcto descifrado del token JWT, realizar operaciones simuladas y ejecutar un logout que destruya efectivamente el hash del token en el contenedor Docker.

```

---

## Entregable del Módulo 1

Al completar este roadmap, habrás construido una infraestructura de seguridad soberana para el **Lab 3030**. Ningún atacante podrá manipular matemáticamente tus tokens de acceso en el cliente (gracias al cifrado simétrico autenticado de **JWT HS256**) y mantendrás el control absoluto y en tiempo real de la invalidación de accesos directamente en tu base de datos montada en Docker.

---

> 💻 **Código 3026 Sintonizado.** Ya tenemos el dominio pura sangrado y los tipos compartidos listos. ¿Por qué frente específico de este mapa de ruta consolidado quieres que empecemos a escribir líneas de código ahora mismo?
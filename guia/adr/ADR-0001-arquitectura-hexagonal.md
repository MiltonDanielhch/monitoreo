# Resumen — ADR 0001: Arquitectura Hexagonal y Monolito Modular
**Autores:** Milton Hipamo / Laboratorio 3030
---

## 📋 Contexto y Decisión

**Problema:** Elegir cómo organizar el código sin caer en la complejidad operacional y los costos de red de los microservicios, considerando un equipo pequeño que opera sobre servidores VPS de bajo costo con alta exigencia de rendimiento y estabilidad local.

**Decisión:** Usar un **monolito modular** como unidad de despliegue y empaquetamiento, gobernado internamente por una **arquitectura hexagonal (puertos y adaptadores)** como disciplina estricta de organización y desacoplamiento del código.

> *Un monorepo bien estructurado vence a los microservicios mal diseñados para equipos pequeños. La eficiencia del sistema radica en mantener baja latencia y bajo costo operativo sin comprometer la separación de responsabilidades.*

---

## 📁 Estructura del Monorepo

```
redes/
├── apps/
│   ├── api/           # Backend Axum (Ensamblador central y DI)
│   ├── web/           # Dashboard SvelteKit 2 + Svelte 5 (Runes, SSR deshabilitado para Auth)
│   └── agent/         # Agente de monitoreo ligero en sedes remotas (SQLite local)
│
├── crates/
│   ├── domain/        # Núcleo puro — Entidades, Value Objects y Puertos Async
│   ├── application/   # Orquestación — Casos de uso e interacción de negocio
│   ├── database/      # Adaptador de persistencia con Sea-ORM + MySQL
│   ├── auth/          # Seguridad criptográfica: PASETO v4 + argon2id
│   ├── inventory/     # Inventario de dispositivos y activos de red
│   ├── jobs/          # Tareas en segundo plano con tokio
│   ├── sync/          # Algoritmos de sincronización offline / remota
│   ├── snmp/          # Recolección de métricas vía SNMP, ICMP ping y descubrimiento
│   ├── topology/      # Estructura de grafos de red y análisis de puntos de fallo (SPOF)
│   ├── storage/       # Adaptador de almacenamiento de archivos y assets (S3/Local)
│   └── infrastructure/# Transversal: Middleware compartido, config global, logger y mailer
│
├── data/
│   ├── migrations/    # Migraciones SQL nativas para MySQL 8.0+
│   └── seeds/         # Semillas de inicialización del sistema (Admin raíz)
│
├── infra/
│   ├── docker/        # Entornos de contenedores y servicios locales (docker Ready)
│   └── coolify/       # Configuraciones automatizadas de despliegue continuo
│
└── justfile           # Orquestador de comandos de desarrollo rápidos

```

---

## 📦 Matriz de Crates y Dependencias

| Crate | Responsabilidad | Dependencias Directas |
| --- | --- | --- |
| `domain` | Entidades puras, value objects, contratos de puertos (`async traits`) y errores del dominio. | `thiserror`, `uuid`, `time`, `serde` |
| `application` | Implementación directa de los Casos de Uso (Flujos lineales de negocio). | `domain` |
| `database` | Implementación de repositorios de datos usando **Sea-ORM 1.1** y conectores **MySQL**. | `domain`, `sea-orm`, `moka` |
| `auth` | Criptografía institucional, firmado/verificación de tokens PASETO v4 Local y hashing adaptativo. | `domain`, `pasetors`, `argon2`, `secrecy` |
| `inventory` | Modelos tácticos y lógica de negocio para el inventario físico de infraestructura. | `domain` |
| `jobs` | Planificación, colas de ejecución y control de workers en segundo plano. | `domain`, `apalis` |
| `sync` | Mecanismos de empaquetamiento y transporte de datos en redes con conectividad intermitente. | `domain`, `tokio`, `serde` |
| `snmp` | Capa física de descubrimiento de red, ráfagas ICMP y escaneo asíncrono de dispositivos. | `domain`, `snmp`, `surge-ping`, `tokio` |
| `topology` | Procesamiento analítico de mapas de red mediante estructuras matriciales o de grafos. | `domain` |
| `storage` | Abstracción de persistencia de archivos multimedia, reportes PDF y logs raw. | `domain`, `aws-config`, `aws-sdk-s3` |
| `infrastructure` | Proveedor genérico y transversal: cargador de `.env`, setup del logger, middleware base. | `axum`, `utoipa`, `tower`, `tower-http` |

> **Regla de Inyección:** Los crates de negocio (`inventory`, `snmp`, `auth`, etc.) no dependen de `infrastructure` ni de `database`. Es la aplicación ejecutable `apps/api` la encargada de enlazar y proveer las instancias de infraestructura a los casos de uso mediante la firma de sus puertos correspondientes.

---

## 📱 Apps Implementadas

| App | Responsabilidad | Dependencias de Crates |
| --- | --- | --- |
| `api` | Servidor HTTP Axum de producción. Inicializa los pools de conexión y realiza la inyección de dependencias (DI). | `infrastructure`, `database`, `auth`, `application`, todos los módulos funcionales. |
| `web` | Interfaz administrativa reactiva e intuitiva con Svelte 5. Comunicación limpia vía API Rest. | Frontend independiente (Cero acoplamiento a crates de Rust). |
| `agent` | Binario nativo ultraligero compilado para hardware embebido o sedes remotas. Persistencia local. | `domain` (Tipos compartidos), `snmp`, `sync`, `tokio`, `rusqlite`. |

---

## 🛡️ Reglas de Frontera Arquitectónica (Estándar Código 3026)

| Regla | Descripción | Verificación Mecánica |
| --- | --- | --- |
| **R1** | **Pureza del Núcleo:** `crates/domain` no puede importar tecnologías web ni de bases de datos. | `cargo tree -p domain --depth 1` (Solo debe listar dependencias base). |
| **R2** | **Aislamiento SQL:** El dialecto SQL o macros del ORM sólo existen dentro de adaptadores específicos. | `grep -r "sea_orm|sqlx" crates/domain/` → Cero resultados. |
| **R3** | **Handlers Delgados:** Las funciones de enrutamiento en Axum no contienen lógica de decisión, solo deserializan y delegan. | Inspección de firmas de controladores en `apps/api`. |
| **R4** | **Asincronía Nativa (Rust 2024):** Se prohíbe el uso de la macro `#[async_trait]`. Los puertos utilizan la sintaxis nativa `async fn` sin sobrecostos en el heap. | Revisión de código en `crates/domain/src/ports/`. |
| **R5** | **Flujo Jerárquico de Dependencias:** El núcleo no conoce los extremos. Ningún crate del dominio o aplicación puede compilar apuntando hacia dependencias de adaptadores externos. | `cargo check --workspace` garantiza la limpieza del árbol. |

---

## 🔄 Fases del Ciclo de Desarrollo

### 1. Diseño Táctico (Core)

* **Domain-Driven Design (DDD):** El software se modela reflejando fielmente la infraestructura real mediante entidades bien delimitadas y Value Objects inmutables que previenen estados inválidos por tipado fuerte (Newtype Pattern).
* **Puertos Asíncronos Nativos:** Los contratos (`traits`) del dominio exponen funciones asíncronas limpias, preparadas para interactuar directamente con la naturaleza asíncrona de los servicios de entrada/salida modernos.

### 2. Adaptación e Infraestructura (Construcción)

* **Adaptadores Concretos:** Se implementan los accesos a datos, protocolos de red (SNMP) y criptografía sin alterar una sola línea de código del dominio.
* **Robustez Tecnológica:** Aprovechamiento de las capacidades nativas de Rust 2024 para garantizar seguridad de memoria en concurrencia y despliegues estables de bajo consumo de CPU.

### 3. Evolución del Sistema sin Fricción

* **Cambiar de Motor de Datos:** Modificar el motor físico (ej. migrar de MySQL a otra solución) altera únicamente la implementación del crate `database/`, dejando intacta la lógica operativa del negocio.
* **Cambiar el Framework Web:** Reemplazar o actualizar Axum a versiones mayores afecta únicamente a los adaptadores de entrada en `apps/api/`, el core permanece inmune.

---

## 🛠️ Herramientas de Calidad Aprobadas

| Herramienta | Propósito en el Pipeline |
| --- | --- |
| `cargo-deny` | Auditoría estricta de licencias y sanidad del árbol de dependencias externas. |
| `cargo-audit` | Escaneo en tiempo real de vulnerabilidades reportadas en el registro de crates. |
| `cargo-nextest` | Motor de ejecución industrial de tests automatizados en paralelo de alta velocidad. |
| `bacon` | Herramienta de productividad para recompilación incremental automática en desarrollo. |
| `just` | Automatizador de tareas locales estandarizadas (Migraciones, Seeds, Linters). |
| `lefthook` | Gestor de Git Hooks ultra rápido para bloqueos tempranos en `pre-commit` y `pre-push`. |
| `taplo` | Formateador estricto y validador de consistencia sintáctica para archivos `Cargo.toml`. |
| `insta` | Framework para pruebas basadas en instantáneas (*Snapshot Testing*) en payloads complejos. |
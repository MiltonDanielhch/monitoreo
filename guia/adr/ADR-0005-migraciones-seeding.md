# Resumen — ADR 0005: Migraciones Sea-ORM y Seeding Idempotente con MySQL
**Autores:** Milton Hipamo / Laboratorio 3030

---

## 📋 Contexto y Decisión

**Problema:** Modificar de forma manual o desordenada la estructura física de la base de datos MySQL en producción introduce riesgos de inconsistencia de datos, caídas del servicio y pérdida de trazabilidad entre entornos (desarrollo, testing y producción).

**Decisión:** Adoptar el sistema de **Migraciones Nativas de Sea-ORM 1.1** (gestionadas como código o SQL controlado, versionadas y reversibles) en combinación con **archivos de Seeds (semillas) estrictamente idempotentes** bajo la sintaxis de MySQL. La ejecución se automatizará mediante `just`, estableciendo un flujo automático en desarrollo y estrictamente manual en entornos de producción.

**Objetivo:** El entorno de datos completo del ecosistema debe poder levantarse, estructurarse y poblarse desde cero mediante un único comando automatizado.

---

## 📁 Estructura del Monorepo (Capa de Datos)

```
data/
├── migrations/               # Crate de migraciones gestionado por Sea-ORM
│   ├── src/
│   │   ├── m20260101_000001_create_users.rs  # Lógica estructural Up/Down
│   │   ├── m20260101_000002_create_rbac.rs
│   │   └── main.rs          # Binario ejecutable de migraciones
│   └── Cargo.toml
│
└── seeds/
    ├── system/              # Seeds institucionales de producción (Idempotentes)
    │   ├── 001_roles.sql
    │   ├── 002_permissions.sql
    │   └── 003_admin_user.sql
    │
    └── development/         # Seeds de simulación exclusiva para entornos Dev
        ├── 001_demo_sedes.sql
        ├── 002_demo_devices.sql
        └── 003_demo_metrics.sql

```

---

## 📜 Reglas de Oro para Migraciones (8 Reglas)

| Regla | Nombre | Descripción |
| --- | --- | --- |
| **R1** | Estructura Dual | Toda migración debe implementar de forma obligatoria el método `up` (aplicar cambios) y el método `down` (revertir cambios). |
| **R2** | Inmutabilidad Histórica | Una vez que una migración ha sido mezclada en la rama principal y ejecutada, se vuelve completamente inmutable. |
| **R3** | Corrección Hacia Adelante | Está prohibido alterar el archivo de una migración vieja. Cualquier error detectado se soluciona generando una nueva migración correctiva. |
| **R4** | Restricción del Destructivo | Los métodos `down` de reversión se ejecutan únicamente en desarrollo local. **Queda terminantemente prohibido su uso en producción.** |
| **R5** | Separación de Responsabilidades | Las migraciones alteran únicamente el esquema estructural físico (tablas, índices, llaves). Jamás deben contener datos de negocio o registros. |
| **R6** | Generación de Entidades | Tras aplicar una migración con éxito en desarrollo, se debe ejecutar el generador de Sea-ORM para sincronizar los structs de Rust del dominio (`sea-orm-cli generate entity`). |
| **R7** | Verificación en CI | El pipeline de integración continua valida la validez sintáctica de las migraciones compilando el crate `data/migrations` antes de dar luz verde. |
| **R8** | Control de Despliegue | Las ejecuciones en producción son manuales e inyectadas por el administrador, impidiendo que el backend altere la base de datos en caliente de forma automática. |

---

## 🌱 Seeds Idempotentes (Sintaxis MySQL)

### 1. Seeds de Sistema (Ejecutables en Producción)

Se ejecutan una única vez durante el despliegue inicial de la plataforma. Para garantizar que puedan volver a correrse sin generar errores de duplicación o pánicos, utilizan la instrucción **`INSERT IGNORE INTO`** o **`ON DUPLICATE KEY UPDATE`** de MySQL 8.0.

* `001_roles.sql`: Roles raíz del ecosistema (`admin`, `operator`, `viewer`, `agent`).
* `002_permissions.sql`: Permisos granulares de infraestructura (`users:read`, `devices:write`, `alerts:read`).
* `003_role_permissions.sql`: Mapeo atómico de la matriz de control de acceso (RBAC).
* `004_admin_user.sql`: Credenciales del administrador primario del sistema (forzando cambio en el primer inicio de sesión).

### 2. Seeds de Desarrollo (Exclusivos para Dev/Test)

**Baneados por completo de producción.** Proveen un entorno realista con datos de telemetría de red simulados para pruebas de rendimiento local.

* `001_demo_sedes.sql`: Sedes geográficas de prueba regional (Sede Central Trinidad, Riberalta, Guayaramerín).
* `002_demo_devices.sql`: Switches, routers, servidores y access points simulados.
* `003_demo_metrics.sql`: Historial masivo de ráfagas SNMP, pings ICMP y logs crudos para rellenar los gráficos del dashboard.

---

## ⚙️ Automatización con Justfile (`Código 3026`)

| Comando | Entorno | Acción Ejecutada |
| --- | --- | --- |
| `just db-migrate` | Desarrollo | Corre las migraciones de Sea-ORM, inyecta los seeds de sistema y añade la data de simulación dev. |
| `just db-migrate-prod` | Producción | Ejecuta únicamente las migraciones y las semillas de sistema obligatorias (pide confirmación en consola). |
| `just db-status` | Todos | Muestra el listado de migraciones aplicadas y pendientes en el motor vivo. |
| `just db-revert` | Desarrollo | Revierte la última migración aplicada en el entorno local (ejecuta el método `down`). |
| `just db-reset` | Desarrollo | Destruye por completo el esquema de MySQL local y lo vuelve a levantar limpio con toda su estructura. |
| `just db-generate` | Desarrollo | Utiliza `sea-orm-cli` para leer la base de datos local y regenerar los modelos fuertemente tipeados de Rust. |

---

## 🛠️ Herramientas de Calidad Aprobadas

| Herramienta | Versión | Propósito Operacional |
| --- | --- | --- |
| `sea-orm-cli` | `1.1.x` | Herramienta de comandos para la gestión de migraciones y mapeo inverso de entidades de Rust. |
| `just` | `v1.x` | Orquestador y automatizador unificado de tareas del monorepo. |
| `cargo-nextest` | `0.9.x` | Motor paralelo de pruebas para validar transacciones aisladas sobre la base de datos de test. |
| `mysqldump` | MySQL 8.0 | Utilidad nativa de resguardo para la exportación de estructuras y datos lógicos en producción. |
| `docker compose` | `v2.x+` | Infraestructura de aislamiento para levantar el servidor de MySQL local de manera inmediata. |
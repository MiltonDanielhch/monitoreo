# ADR 0018: Sintonía CLI: Generador Arquitectónico + RBAC + Guardián de Arquitectura
**Autores:** Milton Hipamo / Laboratorio 3030

**Relacionado con:** ADR 0001 (Arquitectura Hexagonal), ADR 0004 (MySQL), ADR 0006 (Sea-ORM), ADR 0008 (PASETO Auth), ADR 0010 (Testing), ADR 0012 (Tooling), ADR 0013 (Docker Compose), ADR 0016 (OpenAPI), ADR 0020 (Monitoreo Regional)

---

## 📋 Contexto y Decisión

**Problema:** La arquitectura hexagonal, si bien blinda el núcleo de negocio, exige la creación de múltiples archivos repetitivos por cada nuevo módulo funcional (entidades, puertos, casos de uso, repositorios, controladores, DTOs, migraciones, semillas RBAC y especificaciones OpenAPI). Con el crecimiento del monorepo, aumenta exponencialmente el riesgo de violar el desacoplamiento mediante importaciones cruzadas prohibidas, evasión de borrado lógico (*Soft Delete*), uso accidental de JWT primitivos o el olvido de registrar nuevos contratos en OpenAPI.

**Decisión:** Diseñar y desarrollar **Sintonía CLI**, una utilidad interna de línea de comandos escrita en Rust puro utilizando `clap`, `tera` y manipulación del árbol de sintaxis abstracta (AST) vía `syn`/`quote`. Su propósito es doble: actuar como un andamiaje (*scaffolding*) acelerador de código y como el guardián automatizado de las restricciones de diseño.

> **Regla de Oro Inmutable:** Sintonía CLI solo comenzará a escribir archivos de manera automatizada una vez que hayamos estructurado, refinado y validado manualmente 3 módulos reales en producción. La automatización nace del entendimiento absoluto, nunca de la suposición.

---

## 🎯 Regla de los 3 Módulos de Control

| Módulo de Pruebas | Método de Construcción | Objetivo de Sintonía |
| --- | --- | --- |
| `users` | 100% Manual | Definición y congelamiento del patrón base, inyección de dependencias y flujo PASETO. |
| `sedes` | 100% Manual | Validación de la consistencia CRUD bajo los adaptadores de Sea-ORM. |
| `devices` | 100% Manual | Verificación de relaciones complejas, tablas intermedias y control de acceso RBAC. |
| **Módulo 4 en adelante** | **Sintonía CLI** | Automatización masiva sobre moldes de código ya probados en el servidor real. |

---

## 🏗️ Arquitectura de Sintonía CLI

```
apps/cli/
├── src/
│   ├── main.rs                 ← Punto de entrada de la aplicación CLI
│   ├── commands/
│   │   ├── new.rs              ← Inicializador de sub-proyectos
│   │   ├── doctor.rs           ← Auditor del entorno de desarrollo (Docker, Rust, Just)
│   │   ├── check.rs            ← Motor del Guardián Arquitectónico
│   │   ├── db.rs               ← Envoltura de abstracción sobre Sea-ORM CLI
│   │   └── generate/           ← Controladores de comandos de andamiaje (g module)
│   ├── generators/             ← Motores encargados de renderizar las plantillas (.tera)
│   ├── templates/              ← Moldes estáticos de código estricto (.tera)
│   └── utils/                  ← Analizadores sintácticos y validadores de nomenclatura

```

---

## 🛠️ Matriz de Comandos Operativos

| Comando Ejecutable | Acción Técnica en el Espacio de Trabajo |
| --- | --- |
| `sintonia doctor` | Diagnóstico completo del estado del entorno de desarrollo local. |
| `sintonia g module <nombre>` | Generación integral de un nuevo módulo hexagonal (todas las capas). |
| `sintonia g module <nombre> --dry-run` | Simulación exacta en terminal que muestra los cambios sin escribir en disco. |
| `sintonia g module <nombre> --no-rbac` | Omite la inyección de semillas de permisos en la estructura. |
| `sintonia g entity <nombre>` | Genera exclusivamente el archivo de entidad de dominio. |
| `sintonia db migrate` | Invoca de forma transparente las migraciones pendientes de Sea-ORM hacia MySQL. |
| `sintonia db reset` | Purgado estructural completo y reinicialización limpia del motor MySQL 8.0. |
| `sintonia check arch` | Ejecución del motor de auditoría estricta de dependencias y reglas de diseño. |

---

## 📦 Impacto de Estructura: `sintonia g module device`

Al ejecutar el comando para el dominio de dispositivos, el CLI creará y modificará la infraestructura del monorepo de la siguiente manera:

| Componente Arquitectónico | Ruta Destino Inmutable | Propósito |
| --- | --- | --- |
| **Entidad de Dominio** | `crates/domain/src/entities/device.rs` | Estructura pura con tipos nativos y lógica agnóstica. |
| **Puerto de Persistencia** | `crates/domain/src/ports/device_repository.rs` | Trait de abstracción de datos para el núcleo del sistema. |
| **Casos de Uso (5)** | `crates/application/src/use_cases/devices/` | Operaciones de negocio aisladas (create, update, list, etc.). |
| **Repositorio Adaptador** | `crates/database/src/repositories/device_repository.rs` | Implementación del puerto utilizando macros de Sea-ORM. |
| **Handler HTTP + DTO** | `crates/infrastructure/src/http/devices/` | Puntos de entrada de Axum 0.8 y esquemas de validación. |
| **Suite de Pruebas** | `tests/integration/devices_test.rs` | Cobertura automatizada unitaria e integrada con Nextest. |
| **Migración de Datos** | `data/migrations/YYYYMMDD_create_devices.sql` | Esquema de definición relacional nativo para MySQL. |
| **Semillas RBAC** | Inyección en el flujo de inicialización. | Permisos específicos (`devices:read`, `devices:write`). |
| **Registro OpenAPI** | Edición sintáctica en `apps/api/src/docs.rs` | Inyección de la entidad en las macros globales de Utoipa. |

---

## 🛡️ Guardián Arquitectónico (`sintonia check arch`)

El comando de verificación opera analizando el código fuente. Evalúa las reglas inmutables del laboratorio arrojando códigos de salida estándar (`0` para éxito, `1` para violación técnica):

| Regla de Diseño | Método de Validación del Guardián | Estado |
| --- | --- | --- |
| **Aislamiento del Dominio** | Bloquea el uso de `sea_orm`, `sqlx` o `axum` dentro de `crates/domain`. | ✔ Estricto |
| **Prohibición de JWT** | Escanea el árbol de dependencias rechazando `jsonwebtoken` en todo el workspace. | ✔ Estricto |
| **Criptografía Soberana** | Exige la presencia obligatoria de `pasetors` para el manejo de sesiones. | ✔ Estricto |
| **Integridad Temporal** | Bloquea el uso del crate obsoleto `chrono`. El uso de `time` es mandatorio. | ✔ Estricto |
| **Borrado Lógico Obligatorio** | Detecta sentencias SQL que contengan `DELETE FROM`. Obliga el uso de `deleted_at`. | ✔ Estricto |
| **Sincronización del Contrato** | Escanea los handlers de Axum; si falta la macro `#[utoipa::path]`, detiene el pipeline. | ✔ Estricto |

---

## 🛠️ Toolchain de Dependencias Fijadas

| Crate Autorizado | Versión Exacta | Rol en el CLI de Sintonía |
| --- | --- | --- |
| `clap` | **4.6.x** | Parseo tipado y robusto de los argumentos pasados por consola. |
| `tera` | **1.20.x** | Motor de plantillas veloz que inyecta variables sobre esqueletos de Rust. |
| `syn` | **2.0.x** | Análisis sintáctico profundo del código para ediciones de código seguras. |
| `quote` | **1.0.x** | Generación limpia de fragmentos de código Rust respetando la higiene sintáctica. |
| `comfy-table` | **7.2.x** | Formateo estético en cuadros de texto para las salidas del Guardián. |
| `cargo_metadata` | **0.23.x** | Inspección en tiempo real de la topología del espacio de trabajo de Cargo. |
| `cargo-nextest` | **0.9.x** | Orquestador de alta velocidad encargado de ejecutar la suite de tests generada. |

---

## ❌ Decisiones y Enfoques Descartados

* **Edición mediante Expresiones Regulares (Regex Replacements):** Descartada de raíz por su fragilidad extrema. Modificar archivos de rutas o configuraciones OpenAPI usando strings manipulados con expresiones regulares corrompe el formato con facilidad ante cualquier salto de línea imprevisto. Toda alteración estructural se ejecuta mediante el análisis sintáctico de tokens reales con `syn`.
* **Soporte Multi-Motor de Persistencia:** Se rechaza de forma unánime escribir lógica compatible con PostgreSQL o SQLite. El CLI está optimizado única y exclusivamente para exprimir el rendimiento de **MySQL 8.0**, reduciendo la complejidad de las plantillas de migración a sentencias relacionales nativas del motor.

---

## 📜 Directrices Derivadas del CLI

* **Principio de Idempotencia:** Sintonía CLI jamás sobrescribirá un archivo de lógica de negocio existente de manera silenciosa. Si un archivo ya reside en el disco, el CLI abortará la operación de inmediato, exigiendo el uso explícito de la bandera `--force` para confirmar el reemplazo.
* **Preservación de Marcadores de Inyección:** Los archivos centrales modificados dinámicamente por el CLI contendrán comentarios de anclaje estricto inviolables (e.g., `// sintonia:openapi:models`). Si un desarrollador altera o elimina manualmente estos comentarios, el comando `sintonia check arch` marcará el estado del repositorio como corrupto y detendrá los despliegues.
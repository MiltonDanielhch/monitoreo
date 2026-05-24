# Resumen — ADR 0012: Herramientas: mise + just + pnpm + lefthook
**Autores:** Milton Hipamo / Laboratorio 3030

**Relacionado con:** ADR 0004 (MySQL), ADR 0006 (Sea-ORM), ADR 0010 (Testing), ADR 0011 (Estándares), ADR 0013 (Deploy), ADR 0016 (OpenAPI)

---

## 📋 Contexto y Decisión

**Problema:** La falta de un estándar de herramientas (*tooling*) unificado en el espacio de trabajo provoca que se ejecuten comandos inconsistentes entre entornos, se omitan pasos críticos de validación antes de los commits, varíen las versiones de los entornos de ejecución (runtimes) y se ralentice el ciclo de desarrollo debido a flujos manuales y fragmentados.

**Decisión:** Adoptar de forma oficial y obligatoria 4 herramientas pilares para gobernar el ciclo de vida del monorepo, garantizando la reproducibilidad absoluta bajo la filosofía de eficiencia del Código 3026:

| Herramienta | Rol Arquitectónico | Versión Mínima | Estado |
| --- | --- | --- | --- |
| `mise` | Gestor unificado de entornos de ejecución y toolchains | **2026.5.x** | ✅ Activa |
| `just` | Orquestador de tareas universal (Task Runner) | **1.51.0** | ✅ Activa |
| `pnpm` | Gestor de paquetes y dependencias JavaScript/TypeScript | **10.27.x** | ✅ Activa |
| `lefthook` | Ejecutor ultra-rápido de Git-Hooks para control local | **2.1.x** | ✅ Activa |

---

## 🔧 Las 4 Herramientas Pilares

### 1 — mise: Gestor de Toolchains y Entornos

Reemplaza de forma definitiva soluciones fragmentadas como `asdf`, `nvm` o `rustup` manuales. Centraliza la declaración de versiones de compiladores y variables globales en un único archivo declarativo `mise.toml` en la raíz del monorepo.

```toml
[tools]
rust = "1.95"
node = "26"
pnpm = "10.27"
just = "1.51"

[env]
RUST_LOG = "info"
# Configuración determinista de persistencia del Laboratorio 3030
DATABASE_URL = "mysql://root:secret@127.0.0.1:3306/laboratorio_3030"

```

### 2 — just: Task Runner Universal

Sustituye a los antiguos `Makefiles` y scripts en Bash. Ofrece una sintaxis limpia, ejecución multiplataforma nativa y autodocumentación automática accesible mediante el comando `just --list`.

**Comandos Principales del Ecosistema 3026:**

| Comando | Propósito y Acción Operativa |
| --- | --- |
| `just setup` | Inicializa el entorno local: instala herramientas de `mise`, aprovisiona contenedores en docker, ejecuta migraciones y activa los Git-Hooks de `lefthook`. |
| `just dev` | Levanta el entorno de desarrollo concurrente: ejecuta el backend en Axum y la interfaz en Svelte 5 en paralelo. |
| `just test` | Ejecuta la suite de pruebas unitarias y de integración utilizando `cargo-nextest`. |
| `just quality` | Ejecuta el pipeline estricto de calidad local: `fmt`, `clippy`, `audit`, `deny` y `typos`. |
| `just migrate` | Sincroniza el esquema de la base de datos MySQL local utilizando el CLI oficial de **Sea-ORM**. |
| `just sync-types` | Extrae el esquema OpenAPI generado por el backend en Rust e inyecta los tipos fuertemente tipados directamente en la aplicación web en Svelte 5. |
| `just deploy` | Compila los artefactos de producción, ejecuta el pipeline del Puente y empuja los cambios al servidor de despliegue inmutable (Coolify). |

### 3 — pnpm: Package Manager JavaScript Profesional

Gestor oficial para la interfaz de usuario basada en **Svelte 5 (Runes)**. Seleccionado por su arquitectura de almacenamiento eficiente basada en enlaces duros (*hardlinks*) que optimiza el espacio en disco físico, su gestión estricta de monorepos (*workspaces*) y su protección nativa contra ataques en el supply-chain de paquetes de Node.

**Configuración del Espacio de Trabajo (`pnpm-workspace.yaml`):**

```yaml
packages:
  - "apps/web"       # Aplicación de interfaz de usuario Svelte 5
  - "packages/*"     # Paquetes de tipos compartidos y utilidades TypeScript

```

### 4 — lefthook: Control de Calidad en el Commit

Reemplaza herramientas pesadas basadas en Node como `husky`. Escrito en Go, ejecuta validaciones concurrentes directamente sobre los archivos modificados en el ciclo de Git, asegurando que ningún código rompa los estándares del **ADR 0011** antes de salir de la máquina local.

**Flujos de Trabajo Automatizados:**

* **pre-commit:** Ejecuta en paralelo de forma ultra-rápida: `cargo fmt --check`, `pnpm lint`, `typos .` y verificaciones sintácticas rápidas.
* **pre-push:** Bloquea el envío al repositorio si fallan validaciones pesadas: `cargo clippy --all-targets -- -D warnings`, `cargo nextest run` y `cargo deny check`.

---

## 🚀 Ciclo de Onboarding Inmediato

El aprovisionamiento de una nueva estación de trabajo o la restauración del entorno local se reduce a tres comandos deterministas controlados por el toolchain:

```bash
git clone ...
cd laboratorio_3030
mise install      # Instala de forma aislada Rust, Node.js, pnpm y just
just setup        # Levanta MySQL docker, ejecuta migraciones Sea-ORM e instala Git-Hooks
just dev          # Inicia el entorno reactivo local (bacon + Svelte 5 dev server)

```

---

## 🎯 Principios Operativos del Toolchain

### Un comando = Una intención clara y explícita

Se elimina la necesidad de memorizar banderas complejas o configuraciones ocultas de los compiladores.

* `just dev` en lugar de memorizar encadenamientos manuales de sub-scripts de carga.
* `just test` aislando los procesos mediante configuraciones predefinidas en el runner.

### Simetría Absoluta: Local = Puente = Producción

Los comandos ejecutados de forma manual por el desarrollador en la fase de Laboratorio son **exactamente los mismos** que ejecuta el servidor de Integración Continua en el Puente. Se prohíbe la existencia de scripts "exclusivos de CI" que oculten errores de compilación o de tipos.

### Calidad Automatizada e Independiente de la Memoria Humana

La suite de Git-Hooks garantiza el formato, la compilación limpia, la ausencia de vulnerabilidades y la consistencia del tipado de forma automatizada. Si el código no cumple con la sintonía del estándar, el monorepo bloquea la operación en el origen.

---

## 🛠️ Herramientas Completas del Ecosistema 3026

| Herramienta / Crate | Propósito Arquitectónico en el Monorepo | Versión Oficial | Estado |
| --- | --- | --- | --- |
| `mise` | Gestión unificada de entornos de ejecución y toolchains. | `2026.5.x` | ✅ Activa |
| `just` | Orquestador y entrypoint operativo del proyecto. | `1.51.x` | ✅ Activa |
| `pnpm` | Package manager oficial para el ecosistema TypeScript. | `10.27.x` | ✅ Activa |
| `lefthook` | Enforcement local de políticas de calidad sobre Git. | `2.1.x` | ✅ Activa |
| `bacon` | Feedback loop interactivo en segundo plano para compilación. | `3.22.x` | ✅ Activa |
| `cargo-nextest` | Runner oficial de testing concurrente y aislado. | `0.9.x` | ✅ Activa |
| `cargo-deny` | Auditoría estricta de licencias y dependencias duplicadas. | `0.19.x` | ✅ Activa |
| `cargo-audit` | Escaneo de CVEs y fallos de seguridad en crates externos. | `0.22.x` | ✅ Activa |
| `sea-orm-cli` | Gestión de migraciones y generación de modelos MySQL. | `1.1.x` | ✅ Activa |
| `typos` | Corrector ortográfico estático de código fuente y documentos. | `1.46.x` | ✅ Activa |

---
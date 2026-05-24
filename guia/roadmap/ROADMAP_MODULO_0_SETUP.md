# 🛠️ Módulo 0: Setup / Spike (Universal) — Edición Local Estricta (`mise` + Workbench)

```text
Propósito: Validar que todo el stack tecnológico habla de punta a punta antes de construir cualquier lógica de negocio.
Entregable: Navegador muestra "Backend: Conectado | DB: Conectada" llamando al API local.
Regla del Fallo Rápido: Si esto no funciona en 2-4 horas, tu toolchain está rota. No sigas hasta arreglarlo.
Stack: mise · Rust 2024 (1.95) · Axum 0.8 · Sea-ORM 1.1 · MySQL 8.0+ Nativo · SvelteKit 2 · Svelte 5 · Tailwind v4 · Bacon · Just
Estados: [ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

## 📊 Progreso General

| Slice | Nombre | ADR Asociado | Progreso |
| --- | --- | --- | --- |
| **0.1** | Inicialización con `mise` y DB Nativa | `ADR-0004`, `ADR-0012` | [x] |
| **0.2** | Estructura de directorios Hexagonal | `ADR-0001` | [x] |
| **0.3** | Workspace Cargo (Backend) | `ADR-0003`, `ADR-0007` | [ ] |
| **0.4** | Variables de Entorno y Conexión MySQL | `ADR-0002`, `ADR-0004` | [ ] |
| **0.5** | API Axum — Healthcheck + CORS | `ADR-0003`, `ADR-0011` | [ ] |
| **0.6** | SvelteKit + Tailwind v4 (Frontend) | `ADR-0017` | [ ] |
| **0.7** | Fetch End-to-End Reactivo | `ADR-0017`, `ADR-0020` | [ ] |
| **0.8** | Automatización con Justfile y Test E2E | `ADR-0011`, `ADR-0012` | [ ] |
| **M0** | **Módulo 0 Total** | **Génesis del Lab 3030** | **[ ]** |

---

## ⚡ Slice 0.1: Inicialización con `mise` y DB Nativa

> **📌 Vinculación Arquitectónica:**
> * `ADR-0012-herramientas-desarrollo.md` -> Control estricto y reproducible de la toolchain global del monorepo a través de `mise` para congelar las versiones del compilador de Rust, Node runtime y el manejador de paquetes pnpm.
> * `ADR-0004-persistencia-mysql-seaorm-nativo.md` -> Uso del motor relacional MySQL mediante la instancia local nativa tradicional compartida y administrada por interfaz visual.
> 
> 

* [x] **0.1.1 — Configurar el entorno unificado con `mise**`
* Crear el archivo `.mise.toml` en la raíz de tu carpeta de trabajo para asegurar que todos los compiladores queden congelados en la versión correcta:


```toml
# ./.mise.toml
[tools]
rust = "1.95"
node = "24"
pnpm = "10"

```



```
  * Instalar todas las herramientas en aislamiento ejecutando en tu terminal:
  ```bash
  mise install

```

* Validar que tu consola responda con las versiones idénticas gestionadas de forma determinista por `mise`:

```bash
cargo --version
node --version
pnpm --version

```

* [x] **0.1.2 — Preparar el esquema en MySQL Workbench**
* Abre MySQL Workbench y conéctate a tu instancia local tradicional de desarrollo (`127.0.0.1:3306`).
* Ejecuta una nueva pestaña de consulta SQL para instanciar la base de datos del laboratorio aislada con soporte nativo completo para emojis y caracteres internacionales:


```sql
CREATE DATABASE IF NOT EXISTS redes_dev CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

```



```

* [x] **0.1.3 — Instalar el monitor inteligente del compilador**
  * Instala `bacon` de forma global para evitar el uso excesivo de memoria RAM e hilos de CPU en segundo plano durante el desarrollo activo:
  ```bash
  cargo install bacon

```

---

## 📂 Slice 0.2: Estructura de directorios Hexagonal

> **📌 Vinculación Arquitectónica:**
> * `ADR-0001-arquitectura-hexagonal.md` -> Desacoplamiento físico total del núcleo de negocio (dominio puro) respecto a los adaptadores externos de persistencia de datos (Sea-ORM) y la capa de transporte HTTP (Axum).
> 
> 

* [x] **0.2.1 — Generar el layout físico del monorepo**
* Ejecuta el siguiente comando en la raíz para estructurar de golpe todas las capas independientes de la arquitectura limpia:


```bash
mkdir -p data/migrations crates/domain crates/database crates/infrastructure apps/api

```



```

* [x] **0.2.2 — Validar el árbol resultante**
  ```text
  redes/
  ├── Cargo.toml              # Raíz del Workspace de Rust
  ├── .env.local              # Configuración local de secretos (Estilo Laravel)
  ├── .gitignore              # Archivos ignorados por Git
  ├── justfile                # Automatización del flujo de desarrollo local
  ├── .mise.toml              # Toolchain bloqueada de desarrollo
  ├── data/
  │   └── migrations/         # Archivos SQL planos de control inicial
  ├── crates/
  │   ├── domain/             # Núcleo Puro: Entidades y Errores (0 dependencias externas)
  │   ├── database/           # Adaptador de Salida: Conexión y Repositorios con Sea-ORM
  │   └── infrastructure/     # Adaptador de Entrada: Rutas, Controladores y Enrutador de Axum
  └── apps/
      ├── api/                # Binario ejecutable / Orquestador y arranque del backend
      └── web/                # Frontend SPA de alto rendimiento: SvelteKit 2 + Svelte 5 (Runes)

```

---

## 📦 Slice 0.3: Workspace Cargo (Backend)

> **📌 Vinculación Arquitectónica:**
> * `ADR-0003-stack-backend-rust-axum.md` -> Definición explícita de las versiones estables y dependencias autorizadas dentro de la infraestructura de Rust.
> * `ADR-0007-manejo-errores.md` -> Inclusión estricta de `thiserror` para el modelado fuertemente tipado de fallas de dominio y `anyhow` para capturar errores inesperados en los adaptadores de infraestructura.
> 
> 

* [ ] **0.3.1 — Configurar `Cargo.toml` en la Raíz**
```toml
[workspace]
members = [
    "crates/domain",
    "crates/database",
    "crates/infrastructure",
    "apps/api"
]
resolver = "2"

[workspace.dependencies]
tokio = { version = "1.52.3", features = ["macros", "rt", "net", "time", "signal"] }
axum = "0.8"
tower = "0.5"
tower-http = { version = "0.6", features = ["cors"] }
sea-orm = { version = "1.1", features = ["sqlx-mysql", "runtime-tokio-native-tls", "macros"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.11", features = ["v7", "serde"] }
time = { version = "0.3", features = ["serde", "formatting", "parsing"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

```



```

* [ ] **0.3.2 — Configurar `crates/domain/Cargo.toml`**
  ```toml
  [package]
  name = "domain"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  serde = { workspace = true }
  thiserror = { workspace = true }
  uuid = { workspace = true }

```

* [ ] **0.3.3 — Configurar `crates/database/Cargo.toml**`
```toml
[package]
name = "database"
version = "0.1.0"
edition = "2024"

[dependencies]
domain = { path = "../domain" }
sea-orm = { workspace = true }
tokio = { workspace = true }
tracing = { workspace = true }

```



```

* [ ] **0.3.4 — Configurar `crates/infrastructure/Cargo.toml`**
  ```toml
  [package]
  name = "infrastructure"
  version = "0.1.0"
  edition = "2024"

  [dependencies]
  domain = { path = "../domain" }
  database = { path = "../database" }
  axum = { workspace = true }
  tower = { workspace = true }
  tower-http = { workspace = true }
  serde = { workspace = true }
  serde_json = { workspace = true }
  tokio = { workspace = true }
  tracing = { workspace = true }

```

* [ ] **0.3.5 — Configurar `apps/api/Cargo.toml**`
```toml
[package]
name = "api"
version = "0.1.0"
edition = "2024"

[dependencies]
domain = { path = "../../crates/domain" }
database = { path = "../../crates/database" }
infrastructure = { path = "../../crates/infrastructure" }
tokio = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
anyhow = { workspace = true }

```



```

* [ ] **0.3.6 — Inicializar el entorno Bacon**
  * Ejecuta `bacon init` en la raíz del monorepo. Abre una pestaña secundaria en tu terminal y arranca `bacon`. El sistema debe compilar los crates vacíos y reportar el estado inicial en verde (`Ok`).

---

## 🔌 Slice 0.4: Variables de Entorno y Conexión MySQL

> **📌 Vinculación Arquitectónica:**
> * `ADR-0002-configuracion-tipeada-secretos.md` -> Despliegue seguro de credenciales locales aisladas de la historia del control de versiones.
> * `ADR-0004-persistencia-mysql-seaorm-nativo.md` -> Ajuste fino del pool de conexiones asíncronas para resguardar recursos en hardware limitado de desarrollo local.

* [ ] **0.4.1 — Estructurar `.env.local` al estilo Laravel**
  * Crea el archivo `.env.local` en la raíz de tu monorepo apuntando a los accesos directos configurados en tu Workbench:
  ```env
  SERVER_PORT=3000

  # Conexión directa a la instancia nativa local en el puerto 3306
  DATABASE_URL=mysql://root:Milton123@127.0.0.1:3306/redes_dev

```

* [ ] **0.4.2 — Escribir el adaptador de base de datos en `crates/database/src/lib.rs**`
```rust
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub async fn establish_connection(db_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut opt = sea_orm::ConnectOptions::new(db_url.to_owned());

    // Sintonía fina Código 3026: Reutilización de canales sin saturar el motor nativo
    opt.max_connections(10)
       .min_connections(2)
       .connect_timeout(Duration::from_secs(8))
       .idle_timeout(Duration::from_secs(8));

    let db = Database::connect(opt).await?;
    tracing::info!("Conexión exitosa a la instancia local de MySQL vía Workbench.");
    Ok(db)
}

```



```

---

## 📡 Slice 0.5: API Axum — Healthcheck + CORS

> **📌 Vinculación Arquitectónica:**
> * `ADR-0003-stack-backend-rust-axum.md` -> Diseño desacoplado del enrutador HTTP e implementación de políticas seguras de orígenes cruzados (CORS) para el desarrollo local.
> * `ADR-0011-estandares-desarrollo.md` -> Estandarización de contratos de datos con respuestas JSON estructuradas e inyección de dependencias por estados seguros compartidos.

* [ ] **0.5.1 — Desarrollar el enrutador en `crates/infrastructure/src/lib.rs`**
  ```rust
  use axum::{routing::get, Json, Router, extract::State};
  use sea_orm::DatabaseConnection;
  use tower_http::cors::{CorsLayer, Any};
  use serde::Serialize;

  #[derive(Clone)]
  pub struct AppState {
      pub db: DatabaseConnection,
  }

  #[derive(Serialize)]
  pub struct HealthResponse {
      pub status: String,
      pub database: String,
  }

  pub fn create_router(state: AppState) -> Router {
      // CORS liberado exclusivamente para comunicación en localhost con el puerto de SvelteKit
      let cors = CorsLayer::new()
          .allow_origin(Any)
          .allow_methods(Any)
          .allow_headers(Any);

      Router::new()
          .route("/api/health", get(health_check))
          .with_state(state)
          .layer(cors)
  }

  async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
      let db_alive = state.db.ping().await.is_ok();
      Json(HealthResponse {
          status: "OK".to_string(),
          database: if db_alive { "Conectada" } else { "Desconectada" }.to_string(),
      })
  }

```

* [ ] **0.5.2 — Implementar el punto de entrada principal en `apps/api/src/main.rs**`
```rust
use database::establish_connection;
use infrastructure::{create_router, AppState};
use tokio::net::TcpListener;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Fallback seguro en caso de requerir valores por defecto nativos
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:Milton123@127.0.0.1:3306/redes_dev".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());

    let db_connection = establish_connection(&database_url).await
        .map_err(|e| anyhow::anyhow!("Fallo crítico al enlazar MySQL local: {}", e))?;

    let state = AppState { db: db_connection };
    let app = create_router(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Servidor Axum escuchando activamente en http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}

```



```

---

## 🎨 Slice 0.6: SvelteKit + Tailwind v4 (Frontend)

> **📌 Vinculación Arquitectónica:**
> * `ADR-0017-frontend-sveltekit-svelte5.md` -> Arquitectura desacoplada basada en un cliente SPA puro (Single Page Application) que consume contratos de datos asíncronos y utiliza Tailwind v4 para optimización de CSS atómico.

* [ ] **0.6.1 — Instanciar la aplicación frontend**
  * Sitúate dentro de la carpeta `apps/` de tu terminal gestionada por `mise` y ejecuta el generador interactivo oficial:
  ```bash
  cd apps
  pnpm create svelte@latest web
  # Selecciones mandatorias en el menú interactivo:
  # 1. Skeleton project
  # 2. Yes, using TypeScript syntax
  # 3. Svelte 5 (Runes)
  cd web
  pnpm install

```

* [ ] **0.6.2 — Instalar la última especificación de estilos de Tailwind v4**
```bash
pnpm add tailwindcss @tailwindcss/vite

```



```

* [ ] **0.6.3 — Acoplar el compilador nativo rápido en `apps/web/vite.config.ts`**
  ```typescript
  import { sveltekit } from '@sveltejs/kit/vite';
  import { defineConfig } from 'vite';
  import tailwindcss from '@tailwindcss/vite';

  export default defineConfig({
      plugins: [sveltekit(), tailwindcss()]
  });

```

* [ ] **0.6.4 — Declarar la directiva de procesamiento atómico en `apps/web/src/app.css**`
```css
@import "tailwindcss";

```



```

* [ ] **0.6.5 — Configurar el cascarón raíz de renderizado en `apps/web/src/routes/+layout.svelte`**
  ```html
  <script lang="ts">
      import '../app.css';
      let { children } = $props();
  </script>

  <main class="min-h-screen bg-slate-900 text-slate-100 font-sans antialiased">
      {@render children()}
  </main>

```

---

## 🌐 Slice 0.7: Fetch End-to-End Reactivo

> **📌 Vinculación Arquitectónica:**
> * `ADR-0017-frontend-sveltekit-svelte5.md` -> Manejo fino del estado de la interfaz de usuario de forma asíncrona mediante el uso nativo de la runa `$state`.
> * `ADR-0020-monitoreo-infraestructura.md` -> Prototipo inicial de panel de telemetría reactiva para el control visual de la salud de los servicios.
> 
> 

* [ ] **0.7.1 — Escribir la interfaz de validación en `apps/web/src/routes/+page.svelte**`
```html
<script lang="ts">
    import { onMount } from 'svelte';

    interface HealthData {
        status: string;
        database: string;
    }

    // Estados reactivos puros de Svelte 5 (Runes)
    let backendStatus = $state("Cargando...");
    let dbStatus = $state("Cargando...");
    let errorMsg = $state<string | null>(null);

    onMount(async () => {
        try {
            const res = await fetch('http://localhost:3000/api/health');
            if (!res.ok) throw new Error("Fallo de red al intentar conectar con el API local de Axum");
            const data: HealthData = await res.json();
            backendStatus = data.status === "OK" ? "Conectado" : "Error";
            dbStatus = data.database;
        } catch (err: any) {
            backendStatus = "Desconectado";
            dbStatus = "Desconectada";
            errorMsg = err.message;
        }
    });
</script>

<div class="flex flex-col items-center justify-center min-h-screen p-6">
    <div class="bg-slate-800 p-8 rounded-2xl shadow-xl border border-slate-700 max-w-md w-full">
        <h1 class="text-2xl font-bold mb-6 text-center text-indigo-400 tracking-tight">
            Laboratorio 3026 — Spike E2E Nativo
        </h1>

        <div class="space-y-4">
            <div class="flex justify-between items-center bg-slate-900/50 p-4 rounded-xl">
                <span class="font-medium text-slate-300">Estado del Backend:</span>
                <span class="px-3 py-1 rounded-full text-xs font-bold transition-colors"
                      class:bg-green-500={backendStatus === "Conectado"}
                      class:bg-red-500={backendStatus === "Desconectado"}>
                    {backendStatus}
                </span>
            </div>

            <div class="flex justify-between items-center bg-slate-900/50 p-4 rounded-xl">
                <span class="font-medium text-slate-300">MySQL (Workbench Engine):</span>
                <span class="px-3 py-1 rounded-full text-xs font-bold transition-colors"
                      class:bg-green-500={dbStatus === "Conectada"}
                      class:bg-red-500={dbStatus === "Desconectada"}>
                    {dbStatus}
                </span>
            </div>
        </div>

        {#if errorMsg}
            <div class="mt-6 p-3 bg-red-950/40 border border-red-800 text-red-300 text-xs rounded-lg transition-all">
                <p class="font-bold">Detalle de error en infraestructura:</p>
                <p class="font-mono mt-1 break-all">{errorMsg}</p>
            </div>
        {/if}
    </div>
</div>

```



```

---

## 🛠️ Slice 0.8: Automatización con Justfile y Test E2E

> **📌 Vinculación Arquitectónica:**
> * `ADR-0012-herramientas-desarrollo.md` -> Centralización operativa de comandos locales mediante el uso simplificado de `just` para mitigar la fricción cognitiva en terminales paralelas.
> * `ADR-0011-estandares-desarrollo.md` -> Verificación sintáctica estricta e integridad estática del sistema de tipos contractuales compartidos.

* [ ] **0.8.1 — Crear el archivo `justfile` en la raíz del proyecto**
  ```makefile
  # Comandos de automatización rápida — Laboratorio 3030 / Código 3026

  # Monitorear continuamente errores sintácticos del backend en tiempo real usando Bacon
  watch-backend:
      bacon

  # Levantar el servidor de la API leyendo las variables locales de entorno sobre el metal
  run-api:
      env $$(cat .env.local | xargs) cargo run --bin api

  # Lanzar el entorno de desarrollo local rápido para el frontend en Svelte 5 (Vite)
  run-web:
      cd apps/web && pnpm dev

  # Comprobación de tipos integral en todo el monorepo de forma simultánea
  check-all:
      cargo check --workspace
      cd apps/web && pnpm check

```

* [ ] **0.8.2 — Test de ejecución final (Doble luz verde)**
1. Asegúrate en **MySQL Workbench** de que tu servidor nativo local esté encendido (`Running`).
2. Abre una terminal en la raíz del proyecto y arranca la API de Rust: `just run-api`.
3. Abre otra terminal paralela en la raíz y lanza el cliente web: `just run-web`.
4. Ingresa en tu navegador preferido a `http://localhost:5173`.
5. **Verificación Exitosa:** Si ambas tarjetas se iluminan en color verde, tu Spike de validación nativo está completo. El canal de comunicación queda inaugurado.
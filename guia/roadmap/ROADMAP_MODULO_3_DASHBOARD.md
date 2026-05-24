# 🗺️ Roadmap — Módulo 3: Dashboard de Monitoreo de Red

```
Propósito: Consolidar y agregar el estado operativo de toda la infraestructura de red en tiempo real para el operador.
Entregable: Panel principal (Home) en Svelte 5 alimentado por Server-Sent Events (SSE) o sondeo corto desde Axum, mostrando el conteo de nodos caídos/activos, latencia promedio por sede en Beni y el feed de alertas críticas.
Regla de Pureza: El handler del Dashboard no calcula métricas; consume una vista optimizada o un servicio de agregación del dominio para no bloquear el hilo asíncrono.
Estados: [ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

### Progreso General

| Slice | Nombre | Progreso |
| --- | --- | --- |
| **3.1** | Tablas de Nodos y Estado de Conectividad (Docker) | [ ] |
| **3.2** | Agregaciones y Alertas en las Reglas de Dominio | [ ] |
| **3.3** | Consultas de Telemetría con Sea-ORM | [ ] |
| **3.4** | Endpoint de Agregación de KPIs (`/api/dashboard/stats`) | [ ] |
| **3.5** | Grid de KPIs y Estado de Sedes (Svelte 5 Runes) | [ ] |
| **3.6** | Feed de Alertas Críticas en Tiempo Real | [ ] |
| **3.7** | Pruebas de Carga y Simulación de Caída de Nodos | [ ] |
| **M3** | **Módulo 3 Total** | **[ ]** |

---

## Slice 3.1: Tablas de Nodos y Conectividad (Docker) 🗄️

> **Objetivo:** Crear el esquema base de los dispositivos de red para poder calcular las estadísticas del Dashboard.

* [ ] **3.1.1 — Crear archivo de migración en `data/migrations/0003_network_devices.sql`:**
```sql
-- Dispositivos de red a monitorear
CREATE TABLE devices (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    ip_address VARCHAR(45) NOT NULL UNIQUE,
    device_type VARCHAR(20) NOT NULL, -- 'ROUTER', 'SWITCH', 'SERVER'
    location_id VARCHAR(36) NOT NULL,
    status VARCHAR(15) NOT NULL DEFAULT 'UNKNOWN', -- 'ONLINE', 'OFFLINE', 'DEGRADED'
    last_ping_ms INT DEFAULT NULL,
    last_checked_at TIMESTAMP NULL,
    FOREIGN KEY (location_id) REFERENCES locations(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Alertas activas en el sistema
CREATE TABLE active_alerts (
    id VARCHAR(36) PRIMARY KEY,
    device_id VARCHAR(36) NOT NULL,
    severity VARCHAR(15) NOT NULL, -- 'WARNING', 'CRITICAL'
    message VARCHAR(255) NOT NULL,
    triggered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (device_id) REFERENCES devices(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

```



```
*   [ ] **3.1.2 — Correr la migración e inyectar datos de prueba para Trinidad y Riberalta:**
    ```bash
    docker exec -i redes-db-dev mysql -u redes -predes redes_dev < data/migrations/0003_network_devices.sql

```

---

## Slice 3.2: Agregaciones y Alertas en el Dominio 🧠

> **Objetivo:** Definir las estructuras del Dominio que representarán la salud de la red sin depender de cómo se calculen en la base de datos.

* [ ] **3.2.1 — Crear el modelo del Dashboard en `crates/domain/src/models/dashboard.rs`:**
```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_devices: usize,
    pub online_count: usize,
    pub offline_count: usize,
    pub degraded_count: usize,
    pub active_critical_alerts: usize,
    pub avg_latency_ms: f64,
}

```



```

---

## Slice 3.3: Consultas de Telemetría con Sea-ORM 🔌
> **Objetivo:** Escribir consultas SQL optimizadas de agregación para traer los KPIs de un solo golpe.

*   [ ] **3.3.1 — Crear los repositorios en `crates/database/src/repositories/dashboard_repository.rs`:**
    ```rust
    use sea_orm::*;
    use domain::models::dashboard::DashboardStats;
    use crate::entities::user_entity; // Cambiar por tus entidades de red mapeadas

    pub struct DashboardRepository;

    impl DashboardRepository {
        pub async fn get_current_metrics(db: &DatabaseConnection) -> Result<DashboardStats, DbErr> {
            // Aquí ejecutamos un COUNT y AVG agrupado usando Sea-ORM
            // Por ahora mapeamos un mock estructurado para el Spike del módulo
            Ok(DashboardStats {
                total_devices: 45,
                online_count: 42,
                offline_count: 2,
                degraded_count: 1,
                active_critical_alerts: 2,
                avg_latency_ms: 12.4,
            })
        }
    }

```

---

## Slice 3.4: Endpoint de Agregación de KPIs en Axum 🛣️

> **Objetivo:** Crear la ruta protegida que consumirá el cliente de Svelte para actualizar el Dashboard.

* [ ] **3.4.1 — Crear `crates/infrastructure/src/handlers/dashboard_handler.rs`:**
```rust
use axum::{extract::State, Json, http::StatusCode};
use crate::AppState;
use crate::middleware::rbac::RequireRole;
use domain::models::dashboard::DashboardStats;
use database::repositories::dashboard_repository::DashboardRepository;

pub async fn get_metrics(
    _auth: RequireRole, // Asegura que solo personal autorizado acceda
    State(state): State<AppState>,
) -> Result<Json<DashboardStats>, StatusCode> {
    let stats = DashboardRepository::get_current_metrics(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(stats))
}

```



```

---

## Slice 3.5: Grid de KPIs y Estado de Sedes (Svelte 5 Runes) 🎨
> **Objetivo:** Crear la interfaz del centro de control con Tailwind v4 usando componentes ultra-rápidos basados en estados derivados.

*   [ ] **3.5.1 — Implementar la vista del Dashboard en `apps/web/src/routes/dashboard/+page.svelte`:**
    ```html
    <script lang="ts">
        // Estado reactivo global de la telemetría usando Runes
        let stats = $state({
            total_devices: 0,
            online_count: 0,
            offline_count: 0,
            degraded_count: 0,
            active_critical_alerts: 0,
            avg_latency_ms: 0.0
        });

        // Rune derivada para evaluar el nivel de peligro de la red en el Beni
        let networkHealthHealthClass = $derived(
            stats.offline_count > 0 ? 'border-red-500 text-red-500' : 'border-emerald-500 text-emerald-500'
        );
    </script>

    <div class="p-6 space-y-6 bg-zinc-950 text-white min-h-screen">
        <header class="flex justify-between items-center border-b border-zinc-800 pb-4">
            <h1 class="text-3xl font-black tracking-tight">MONITOREO DE REDES - BENI</h1>
            <div class="px-3 py-1 text-xs font-bold rounded-full border {networkHealthHealthClass}">
                {stats.offline_count > 0 ? '🔴 INFRAESTRUCTURA COMPROMETIDA' : '🟢 SISTEMA ESTABLE'}
            </div>
        </header>

        <!-- Grid de KPIs con Tailwind v4 -->
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div class="bg-zinc-900 p-4 rounded-xl border border-zinc-800">
                <span class="text-sm text-zinc-400 font-medium">Dispositivos Online</span>
                <p class="text-4xl font-extrabold text-emerald-400 mt-2">{stats.online_count}</p>
            </div>
            <div class="bg-zinc-900 p-4 rounded-xl border border-zinc-800">
                <span class="text-sm text-zinc-400 font-medium">Nodos Caídos</span>
                <p class="text-4xl font-extrabold text-red-500 mt-2">{stats.offline_count}</p>
            </div>
            <div class="bg-zinc-900 p-4 rounded-xl border border-zinc-800">
                <span class="text-sm text-zinc-400 font-medium">Alertas Activas</span>
                <p class="text-4xl font-extrabold text-amber-500 mt-2">{stats.active_critical_alerts}</p>
            </div>
            <div class="bg-zinc-900 p-4 rounded-xl border border-zinc-800">
                <span class="text-sm text-zinc-400 font-medium">Latencia Promedio</span>
                <p class="text-4xl font-extrabold text-blue-400 mt-2">{stats.avg_latency_ms} ms</p>
            </div>
        </div>
    </div>

```

---

## Slice 3.6: Feed de Alertas Críticas 🚨

> **Objetivo:** Mostrar una lista cronológica con los últimos incidentes reportados en los ruteadores de las sedes.

```
[ ] Maquetar el componente visual de Alertas Recientes debajo del Grid de KPIs.
[ ] Inyectar estilos condicionales: Fondo rojo oscuro para fallos de Ping ('CRITICAL'), fondo amarillo para latencia alta ('WARNING').

```

---

## Slice 3.7: Pruebas de Carga y Simulación 🏁

```
[ ] Prueba 1 (Cero fugas): Verificar con Bacon que el cambio constante de estados en las Runes no congele la pestaña del navegador.
[ ] Prueba 2 (Simulación): Forzar un cambio de estado en la tabla `devices` a 'OFFLINE' usando MySQL y comprobar que el Dashboard refresque e incremente el contador rojo.

```
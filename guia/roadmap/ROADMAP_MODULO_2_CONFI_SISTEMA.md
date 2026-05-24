# 🗺️ Roadmap — Módulo 2: Configuración y Umbrales del Sistema

```
Propósito: Gestionar las variables operativas del sistema, definición de sedes regionales y los umbrales de alerta globales (latencia, ping, SNMP) sin alterar el código fuente.
Entregable: Panel de configuración en Svelte 5 (Runes) que permita modificar los parámetros del sistema en tiempo real, persistidos en MySQL y cacheados eficientemente en memoria en el backend de Axum.
Regla de Pureza: El Dominio expone Value Objects estrictos para validar que los umbrales y IPs sean lógicamente correctos antes de guardarse.
Estados: [ ] Pendiente   [~] En progreso   [x] Completado   [!] Bloqueado

```

### Progreso General

| Slice | Nombre | Progreso |
| --- | --- | --- |
| **2.1** | Esquema Clave-Valor y Sedes SQL (Docker) | [ ] |
| **2.2** | Reglas de Negocio y Umbrales (`crates/domain`) | [ ] |
| **2.3** | Repositorio de Configuración (`crates/database`) | [ ] |
| **2.4** | Servicio de Caché In-Memory (`crates/infrastructure`) | [ ] |
| **2.5** | Endpoints de Configuración y Estado en Axum | [ ] |
| **2.6** | Formulario de Configuración Dinámico (Svelte 5 Runes) | [ ] |
| **2.7** | Pruebas de Integración y Cambio de Umbrales | [ ] |
| **M2** | **Módulo 2 Total** | **[ ]** |

---

## Slice 2.1: Esquema Clave-Valor y Sedes SQL (Docker) 🗄️

> **Objetivo:** Crear una estructura de datos flexible para configuraciones dinámicas y el catálogo base de las sedes de monitoreo en Beni.

* [ ] **2.1.1 — Crear archivo de migración en `data/migrations/0002_system_settings.sql`:**
```sql
-- Tabla para sedes regionales (Trinidad, Riberalta, Guayaramerín, etc.)
CREATE TABLE locations (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    code VARCHAR(10) NOT NULL UNIQUE, -- Ej: 'TRI', 'RIB'
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Tabla de configuración Clave-Valor tipada
CREATE TABLE system_settings (
    `key` VARCHAR(100) PRIMARY KEY,
    `value` TEXT NOT NULL,
    value_type VARCHAR(20) NOT NULL, -- 'string', 'number', 'boolean', 'json'
    description VARCHAR(255),
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Insertar configuraciones por defecto del sistema (Umbrales de red)
INSERT INTO system_settings (`key`, `value`, value_type, description) VALUES 
('network.ping_interval_seconds', '30', 'number', 'Intervalo de tiempo para ejecutar pings a los dispositivos'),
('threshold.latency_warning_ms', '150', 'number', 'Umbral de latencia para marcar alerta amarilla'),
('threshold.latency_critical_ms', '300', 'number', 'Umbral de latencia para levantar incidente crítico'),
('threshold.packet_loss_max_percent', '5', 'number', 'Porcentaje máximo permitido de pérdida de paquetes');

```



```
*   [ ] **2.1.2 — Aplicar la migración en tu contenedor activo de Docker:**
    ```bash
    docker exec -i redes-db-dev mysql -u redes -predes redes_dev < data/migrations/0002_system_settings.sql

```

* [ ] **2.1.3 — Verificar la consistencia de las tablas desde la terminal:**
```bash
docker exec -it redes-db-dev mysql -u redes -predes redes_dev -e "SELECT * FROM system_settings;"

```



```

---

## Slice 2.2: Reglas de Negocio y Umbrales (`crates/domain`) 🧠
> **Objetivo:** Crear los tipos e invariantes que impidan que se guarden configuraciones absurdas o peligrosas en el sistema.

*   [ ] **2.2.1 — Crear los modelos de configuración en `crates/domain/src/models/settings.rs`:**
    ```rust
    use serde::{Serialize, Deserialize};
    use crate::errors::DomainError;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NetworkThresholds {
        pub ping_interval: u32,
        pub latency_warning: u32,
        pub latency_critical: u32,
        pub packet_loss_max: u32,
    }

    impl NetworkThresholds {
        /// Regla de Dominio: El umbral crítico jamás puede ser menor o igual al de advertencia
        pub fn validate(&self) -> Result<(), DomainError> {
            if self.latency_critical <= self.latency_warning {
                return Err(DomainError::InvalidSettingValue("El umbral crítico de latencia debe ser mayor al de advertencia.".to_string()));
            }
            if self.packet_loss_max > 100 {
                return Err(DomainError::InvalidSettingValue("La pérdida de paquetes no puede superar el 100%.".to_string()));
            }
            Ok(())
        }
    }

```

* [ ] **2.2.2 — Extender `DomainError` en `crates/domain/src/errors.rs`:**
```rust
// Agregar la variante para configuraciones inválidas
#[error("Configuración inválida: {0}")]
InvalidSettingValue(String),

```



```

---

## Slice 2.3: Repositorio de Configuración (`crates/database`) 🔌
> **Objetivo:** Implementar la lectura y escritura de parámetros utilizando las entidades de Sea-ORM.

*   [ ] **2.3.1 — Escribir la entidad Sea-ORM en `crates/database/src/entities/setting_entity.rs`:**
    ```rust
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
    #[sea_orm(table_name = "system_settings")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub key: String,
        pub value: String,
        pub value_type: String,
        pub description: Option<String>,
        pub updated_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}

```

* [ ] **2.3.2 — Crear el repositorio operativo en `crates/database/src/repositories/setting_repository.rs`:**
```rust
use sea_orm::*;
use crate::entities::setting_entity::{Entity as SettingEntity, Model as SettingModel, ActiveModel as SettingActiveModel};

pub struct SettingRepository;

impl SettingRepository {
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<SettingModel>, DbErr> {
        SettingEntity::find().all(db).await
    }

    pub async fn update_key(db: &DatabaseConnection, key: &str, value: &str) -> Result<SettingModel, DbErr> {
        let mut setting: SettingActiveModel = SettingEntity::find_by_id(key.to_string())
            .one(db)
            .await?
            .ok_or(DbErr::Custom(format!("Configuración {} no encontrada", key)))?
            .into();

        setting.value = Set(value.to_string());
        setting.update(db).await
    }
}

```



```

---

## Slice 2.4: Servicio de Caché In-Memory (`crates/infrastructure`) ⚡
> **Objetivo:** Evitar pegarle a la base de datos en cada ping de red leyendo los umbrales directamente desde un estado seguro en memoria (`Arc<RwLock>`).

*   [ ] **2.4.1 — Diseñar el gestor de configuración en tiempo de ejecución en `crates/infrastructure/src/services/config_cache.rs`:**
    ```rust
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use domain::models::settings::NetworkThresholds;

    #[derive(Clone)]
    pub struct RuntimeConfig {
        pub thresholds: Arc<RwLock<NetworkThresholds>>,
    }

    impl RuntimeConfig {
        pub fn new(initial: NetworkThresholds) -> Self {
            Self {
                thresholds: Arc::new(RwLock::new(initial)),
            }
        }

        pub async fn update_thresholds(&self, new_thresholds: NetworkThresholds) {
            let mut lock = self.thresholds.write().await;
            *lock = new_thresholds;
        }
    }

```

---

## Slice 2.5: Endpoints de Configuración y Estado en Axum 🛣️

> **Objetivo:** Exponer la API para leer y actualizar las configuraciones globales, protegida por el middleware RBAC del Módulo 1.

* [ ] **2.5.1 — Crear el handler en `crates/infrastructure/src/handlers/setting_handler.rs`:**
```rust
use axum::{extract::State, Json, http::StatusCode};
use crate::AppState;
use crate::middleware::rbac::RequireRole;
use domain::models::settings::NetworkThresholds;
use database::repositories::setting_repository::SettingRepository;

pub async fn get_thresholds(
    _auth: RequireRole, // Obligatorio estar logueado
    State(state): State<AppState>,
) -> Json<NetworkThresholds> {
    let current = state.runtime_config.thresholds.read().await;
    Json(current.clone())
}

pub async fn update_thresholds(
    _auth: RequireRole,
    State(state): State<AppState>,
    Json(payload): Json<NetworkThresholds>,
) -> Result<StatusCode, StatusCode> {
    // 1. Validar reglas de negocio en el Dominio
    payload.validate().map_err(|_| StatusCode::BAD_REQUEST)?;

    // 2. Persistir en la Base de Datos (Docker)
    SettingRepository::update_key(&state.db, "threshold.latency_warning_ms", &payload.latency_warning.to_string()).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    SettingRepository::update_key(&state.db, "threshold.latency_critical_ms", &payload.latency_critical.to_string()).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    SettingRepository::update_key(&state.db, "threshold.packet_loss_max_percent", &payload.packet_loss_max.to_string()).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 3. Sincronizar la caché en caliente de memoria inmediatamente
    state.runtime_config.update_thresholds(payload).await;

    Ok(StatusCode::OK)
}

```



```
*   [ ] **2.5.2 — Mapear las rutas en `crates/infrastructure/src/router.rs`:**
    ```rust
    // Dentro de las rutas protegidas:
    let settings_routes = Router::new()
        .route("/settings/thresholds", get(get_thresholds).put(update_thresholds));

```

---

## Slice 2.6: Formulario de Configuración Dinámico (Svelte 5 Runes) 🎨

> **Objetivo:** Construir la interfaz de control con validaciones reactivas e indicadores de cambio de estado visual.

* [ ] **2.6.1 — Crear la estructura de datos del formulario usando Runes en `apps/web/src/routes/dashboard/settings/+page.svelte`:**
```html
<script lang="ts">
    import { onMount } from 'svelte';

    // Estados reactivos puros de Svelte 5
    let latencyWarning = $state(150);
    let latencyCritical = $state(300);
    let packetLossMax = $state(5);
    let isSaving = $state(false);
    let errorMessage = $state('');

    // Propiedad derivada para validación en tiempo real en la UI
    let isValid = $derived(latencyCritical > latencyWarning && packetLossMax <= 100);

    async function saveSettings() {
        if (!isValid) return;
        isSaving = true;
        // Petición PUT hacia el backend de Axum usando el token guardado del Módulo 1
        isSaving = false;
    }
</script>

<div class="p-6 bg-zinc-900 text-white rounded-lg border border-zinc-800">
    <h2 class="text-2xl font-bold mb-4">Umbrales de Alerta de Red</h2>

    <div class="space-y-4">
        <div>
            <label class="block text-sm text-zinc-400">Latencia de Advertencia (ms)</label>
            <input type="number" bind:value={latencyWarning} class="w-full bg-zinc-800 p-2 rounded mt-1" />
        </div>
        <div>
            <label class="block text-sm text-zinc-400">Latencia Crítica (ms)</label>
            <input type="number" bind:value={latencyCritical} class="w-full bg-zinc-800 p-2 rounded mt-1" />
        </div>
    </div>

    {#if !isValid}
        <p class="text-red-500 text-sm mt-3">⚠️ El umbral crítico debe ser estrictamente mayor al de advertencia.</p>
    {/if}

    <button onclick={saveSettings} disabled={!isValid || isSaving} class="mt-6 bg-blue-600 px-4 py-2 rounded disabled:opacity-50">
        {isSaving ? 'Guardando...' : 'Aplicar Cambios del Sistema'}
    </button>
</div>

```



```

---

## Slice 2.7: Pruebas de Integración y Cambio de Umbrales 🏁
> **Objetivo:** Verificar que los cambios aplicados desde la UI impacten la caché de memoria del Backend sin reinicios.


```

[ ] Prueba 1 (Validación): Intentar guardar un umbral crítico de 100ms y uno de advertencia de 200ms. El backend debe rebotar la petición con HTTP 400.
[ ] Prueba 2 (Persistencia): Guardar valores correctos (Advertencia: 120ms, Crítico: 250ms). Verificar mediante consola Docker que los valores cambiaron en la tabla `system_settings`.
[ ] Prueba 3 (Caché en caliente): Realizar un GET inmediato a /api/settings/thresholds y confirmar que retorna los nuevos datos sin haber reiniciado el binario de Rust.

```

---

¿Dejamos asentado este roadmap para el **Módulo 2** y empezamos con el diseño de la base de datos de configuraciones en Docker? Tu terminal de Bacon se encargará de avisarnos si rompemos algo.

```
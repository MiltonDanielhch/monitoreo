// crates/infrastructure/src/handlers/worker_config_handler.rs
// Handlers para configuración de workers en segundo plano
// Vinculado con ADR-0015-tokio-jobs.md

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use crate::AppState;

/// Configuración de workers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    /// Días de retención para pruning
    pub retention_days: i32,
    /// Intervalo de ping en segundos
    pub ping_interval_seconds: u64,
    /// Intervalo de SNMP en segundos
    pub snmp_interval_seconds: u64,
    /// Máximo de reintentos para ping
    pub ping_max_retries: u32,
    /// Hora de pruning (0-23)
    pub pruning_hour: u32,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            retention_days: 90,
            ping_interval_seconds: 30,
            snmp_interval_seconds: 21600, // 6 horas
            ping_max_retries: 3,
            pruning_hour: 2,
        }
    }
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Handler para obtener configuración de workers
pub async fn get_worker_config(
    State(_state): State<AppState>,
) -> Result<Json<WorkerConfig>, (StatusCode, Json<ErrorResponse>)> {
    // Por ahora, retornamos configuración por defecto
    // TODO: Implementar persistencia en base de datos
    Ok(Json(WorkerConfig::default()))
}

/// Handler para actualizar configuración de workers
pub async fn update_worker_config(
    State(_state): State<AppState>,
    Json(config): Json<WorkerConfig>,
) -> Result<Json<WorkerConfig>, (StatusCode, Json<ErrorResponse>)> {
    // Validar que los valores estén en rangos aceptables
    if config.retention_days < 1 || config.retention_days > 365 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "retention_days debe estar entre 1 y 365".to_string(),
            }),
        ));
    }

    if config.ping_interval_seconds < 10 || config.ping_interval_seconds > 300 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "ping_interval_seconds debe estar entre 10 y 300".to_string(),
            }),
        ));
    }

    if config.snmp_interval_seconds < 3600 || config.snmp_interval_seconds > 86400 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "snmp_interval_seconds debe estar entre 3600 y 86400".to_string(),
            }),
        ));
    }

    if config.ping_max_retries < 1 || config.ping_max_retries > 10 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "ping_max_retries debe estar entre 1 y 10".to_string(),
            }),
        ));
    }

    if config.pruning_hour > 23 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "pruning_hour debe estar entre 0 y 23".to_string(),
            }),
        ));
    }

    // TODO: Implementar persistencia en base de datos
    // Por ahora, solo validamos y retornamos la configuración actualizada
    tracing::info!("Configuración de workers actualizada: {:?}", config);

    Ok(Json(config))
}

// crates/infrastructure/src/handlers/devices_handler.rs
// Controlador HTTP para gestión de dispositivos
// Vinculado con ADR-0003-stack-backend-rust-axum.md

use axum::{Json, extract::State};
use crate::AppState;
use database::repositories::dashboard_repository::DeviceInfo;
use tracing::{info, error};

#[derive(Debug, serde::Serialize)]
pub struct DevicesResponse {
    pub devices: Vec<DeviceInfo>,
}

pub async fn get_devices(
    State(state): State<AppState>,
) -> Result<Json<DevicesResponse>, (axum::http::StatusCode, String)> {
    info!("Obteniendo lista de dispositivos");

    let devices = state.dashboard_repo.find_all_devices()
        .await
        .map_err(|e| {
            error!("Error obteniendo devices: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(DevicesResponse { devices }))
}
// crates/infrastructure/src/handlers/locations_handler.rs
// Controlador HTTP para gestión de locations/sedes
// Vinculado con ADR-0003-stack-backend-rust-axum.md

use axum::{Json, extract::State};
use crate::AppState;
use tracing::{info, error};

#[derive(Debug, serde::Serialize)]
pub struct LocationResponse {
    pub id: String,
    pub name: String,
    pub code: String,
    pub region: String,
    pub parent_id: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_active: bool,
}

pub async fn get_locations(
    State(state): State<AppState>,
) -> Result<Json<Vec<LocationResponse>>, (axum::http::StatusCode, String)> {
    info!("Obteniendo lista de sedes");

    let locations = state.settings_repo.find_all_locations()
        .await
        .map_err(|e| {
            error!("Error obteniendo locations: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    let response: Vec<LocationResponse> = locations.into_iter().map(|loc| {
        LocationResponse {
            id: loc.id.to_string(),
            name: loc.name,
            code: loc.code,
            region: loc.region,
            parent_id: loc.parent_id.map(|id| id.to_string()),
            latitude: loc.latitude,
            longitude: loc.longitude,
            is_active: loc.is_active,
        }
    }).collect();

    Ok(Json(response))
}
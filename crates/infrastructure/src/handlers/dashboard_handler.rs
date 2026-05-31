// crates/infrastructure/src/handlers/dashboard_handler.rs
// Controladores HTTP para dashboard
// Vinculado con ADR-0003-stack-backend-rust-axum.md

use axum::{Json, extract::State};
use crate::AppState;
use database::repositories::dashboard_repository::AlertInfo;
use tracing::{info, error};

#[derive(Debug, serde::Serialize)]
pub struct DashboardStatsResponse {
    pub active_locations: i32,
    pub online_devices: i32,
    pub total_devices: i32,
    pub pending_alerts: i32,
    pub critical_alerts: i32,
    pub total_bandwidth_gbps: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct RecentAlertsResponse {
    pub alerts: Vec<AlertInfo>,
}

pub async fn get_dashboard_stats(
    State(state): State<AppState>,
) -> Result<Json<DashboardStatsResponse>, (axum::http::StatusCode, String)> {
    info!("Obteniendo estadísticas del dashboard");

    let stats = state.dashboard_repo.get_stats()
        .await
        .map_err(|e| {
            error!("Error obteniendo stats: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(DashboardStatsResponse {
        active_locations: stats.active_locations,
        online_devices: stats.online_devices,
        total_devices: stats.total_devices,
        pending_alerts: stats.pending_alerts,
        critical_alerts: stats.critical_alerts,
        total_bandwidth_gbps: stats.total_bandwidth_gbps,
    }))
}

pub async fn get_recent_alerts(
    State(state): State<AppState>,
) -> Result<Json<RecentAlertsResponse>, (axum::http::StatusCode, String)> {
    info!("Obteniendo alertas recientes");

    let alerts = state.dashboard_repo.get_recent_alerts(10)
        .await
        .map_err(|e| {
            error!("Error obteniendo alertas: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    Ok(Json(RecentAlertsResponse { alerts }))
}
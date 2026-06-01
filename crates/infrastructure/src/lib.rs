// crates/infrastructure/src/lib.rs
// Adaptador de entrada - Rutas, Controladores y Enrutador de Axum
// Vinculado con ADR-0003-stack-backend-rust-axum.md

// Este crate contiene la capa de transporte HTTP y middlewares
// Los handlers deben ser delgados y delegar la lógica a la capa de aplicación

pub mod config;
pub mod crypto;
pub mod handlers;
pub mod middleware;
pub mod notifications;
pub mod storage;
pub mod telemetry;
pub mod workers;

use axum::{routing::{get, post, put}, Json, Router, extract::State};
use database::{DatabaseConnection, AuthRepository, SettingsRepository, DashboardRepository};
use crate::config::RuntimeConfig;
use crate::handlers::WorkerStats;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use serde::Serialize;
use std::sync::Arc;
use secrecy::SecretString;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub auth_repo: Arc<AuthRepository>,
    pub settings_repo: Arc<SettingsRepository>,
    pub dashboard_repo: Arc<DashboardRepository>,
    pub runtime_config: RuntimeConfig,
    pub paseto_secret: SecretString,
    pub worker_stats: Arc<WorkerStats>,
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
        .route("/api/auth/login", post(handlers::auth_handler::login))
        .route("/api/auth/refresh", post(handlers::auth_handler::refresh))
        .route("/api/auth/logout", post(handlers::auth_handler::logout))
        .route("/api/settings/thresholds", get(handlers::settings_handler::get_thresholds))
        .route("/api/settings/thresholds", put(handlers::settings_handler::update_thresholds))
        .route("/api/dashboard/stats", get(handlers::dashboard_handler::get_dashboard_stats))
        .route("/api/dashboard/alerts", get(handlers::dashboard_handler::get_recent_alerts))
        .route("/api/locations", get(handlers::locations_handler::get_locations))
        .route("/api/devices", get(handlers::get_devices))
        .route("/api/workers/stats", get(handlers::get_worker_stats))
        .route("/api/v1/notifications/logs", post(handlers::notification_handler::get_notification_logs))
        .route("/api/v1/notifications/test-smtp", post(handlers::notification_handler::test_smtp_connection))
        .route("/api/v1/infrastructure/upload", post(handlers::infrastructure_file_handler::upload_file))
        .route("/api/v1/infrastructure/download/{id}", get(handlers::infrastructure_file_handler::download_file))
        .route("/api/v1/infrastructure/files", get(handlers::infrastructure_file_handler::list_files))
        .route("/api/v1/audit/logs", get(handlers::audit_handler::get_audit_logs))
        .route("/api/v1/audit/entity-history", get(handlers::audit_handler::get_entity_history))
        .route("/api/v1/telemetry/ingest", post(handlers::telemetry_handler::ingest_telemetry))
        .route("/api/v1/telemetry/register", post(handlers::telemetry_handler::register_agent))
        .layer(TraceLayer::new_for_http())
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

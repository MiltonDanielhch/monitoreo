// crates/infrastructure/src/lib.rs
// Adaptador de entrada - Rutas, Controladores y Enrutador de Axum
// Vinculado con ADR-0003-stack-backend-rust-axum.md

// Este crate contiene la capa de transporte HTTP y middlewares
// Los handlers deben ser delgados y delegar la lógica a la capa de aplicación

use axum::{routing::get, Json, Router, extract::State};
use database::DatabaseConnection;
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

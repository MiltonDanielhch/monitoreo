// crates/infrastructure/src/lib.rs
// Adaptador de entrada - Rutas, Controladores y Enrutador de Axum
// Vinculado con ADR-0003-stack-backend-rust-axum.md

// Este crate contiene la capa de transporte HTTP y middlewares
// Los handlers deben ser delgados y delegar la lógica a la capa de aplicación

pub mod crypto;
pub mod handlers;
pub mod middleware;
pub mod workers;

use axum::{routing::{get, post}, Json, Router, extract::State};
use database::{DatabaseConnection, AuthRepository};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use serde::Serialize;
use std::sync::Arc;
use secrecy::SecretString;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub auth_repo: Arc<AuthRepository>,
    pub paseto_secret: SecretString,
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

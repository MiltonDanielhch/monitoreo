// crates/infrastructure/src/handlers/telemetry_handler.rs
// Handlers HTTP para telemetría de agentes
// Vinculado con ADR-0003-stack-backend-rust-axum.md

use crate::AppState;
use axum::{
    extract::State,
    http::HeaderMap,
    Json,
};
use database::TelemetryRepository;
use domain::models::telemetry::{TelemetryMetrics, TelemetryBatch, TelemetryPort};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

/// DTO para respuesta de error
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// DTO para solicitud de telemetría
#[derive(Deserialize)]
pub struct TelemetryRequest {
    pub cpu_usage_percent: Option<f64>,
    pub memory_usage_percent: Option<f64>,
    pub latency_ms: Option<i32>,
    pub packet_loss_percent: Option<f64>,
    pub bandwidth_mbps: Option<f64>,
    pub disk_usage_percent: Option<f64>,
    pub temperature_celsius: Option<f64>,
    pub uptime_seconds: Option<i64>,
}

/// DTO para respuesta de éxito
#[derive(Serialize)]
pub struct TelemetryResponse {
    pub success: bool,
    pub message: String,
}

/// Extrae el token de autorización del header
fn extract_token(headers: &HeaderMap) -> Result<String, String> {
    headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
        .ok_or_else(|| "Missing or invalid authorization header".to_string())
}

/// Valida el token hash contra el repositorio
async fn validate_token(
    token: String,
    repo: &TelemetryRepository,
) -> Result<(), String> {
    // En un sistema real, aquí se validaría el token JWT/PASEO
    // Por ahora, validamos que el token no esté vacío
    if token.is_empty() {
        return Err("Token cannot be empty".to_string());
    }

    // Validar que el agente existe con este token hash
    let agent = repo
        .get_agent_by_token(token)
        .await
        .map_err(|e| format!("Error validating token: {}", e))?;

    if agent.is_none() {
        return Err("Invalid token".to_string());
    }

    Ok(())
}

/// Endpoint para recibir telemetría de un agente
pub async fn ingest_telemetry(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<TelemetryRequest>,
) -> Result<Json<TelemetryResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let telemetry_repo = TelemetryRepository::new(state.db.clone());

    // Extraer y validar token
    let token = extract_token(&headers).map_err(|e| {
        (
            axum::http::StatusCode::UNAUTHORIZED,
            Json(ErrorResponse { error: e }),
        )
    })?;

    validate_token(token, &telemetry_repo)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::UNAUTHORIZED,
                Json(ErrorResponse { error: e }),
            )
        })?;

    // Crear métricas
    let metrics = TelemetryMetrics {
        cpu_usage_percent: payload.cpu_usage_percent,
        memory_usage_percent: payload.memory_usage_percent,
        latency_ms: payload.latency_ms,
        packet_loss_percent: payload.packet_loss_percent,
        bandwidth_mbps: payload.bandwidth_mbps,
        disk_usage_percent: payload.disk_usage_percent,
        temperature_celsius: payload.temperature_celsius,
        uptime_seconds: payload.uptime_seconds,
    };

    // Validar métricas
    metrics.validate().map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Invalid metrics: {}", e),
            }),
        )
    })?;

    // Obtener el agente del token para obtener el agent_id
    let token_hash = extract_token(&headers).unwrap();
    let agent = telemetry_repo
        .get_agent_by_token(token_hash)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error fetching agent: {}", e),
                }),
            )
        })?;

    let agent = agent.ok_or_else(|| {
        (
            axum::http::StatusCode::UNAUTHORIZED,
            Json(ErrorResponse { error: "Agent not found".to_string() }),
        )
    })?;

    // Crear lote de telemetría
    let batch = TelemetryBatch::new(agent.id, metrics);

    // Ingestar métricas
    telemetry_repo
        .ingest_metrics(batch)
        .await
        .map_err(|e| {
            error!("Error ingesting telemetry: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error ingesting telemetry: {}", e),
                }),
            )
        })?;

    info!("Telemetry ingested successfully for agent: {}", agent.name);

    Ok(Json(TelemetryResponse {
        success: true,
        message: "Telemetry ingested successfully".to_string(),
    }))
}

/// DTO para solicitud de registro de agente
#[derive(Deserialize)]
pub struct RegisterAgentRequest {
    pub name: String,
    pub sede_id: String,
    pub agent_type: String,
    pub ip_address: String,
    pub api_token: String,
}

/// DTO para respuesta de registro de agente
#[derive(Serialize)]
pub struct RegisterAgentResponse {
    pub success: bool,
    pub agent_id: String,
    pub message: String,
}

/// Endpoint para registrar un nuevo agente
pub async fn register_agent(
    State(state): State<AppState>,
    Json(payload): Json<RegisterAgentRequest>,
) -> Result<Json<RegisterAgentResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let telemetry_repo = TelemetryRepository::new(state.db.clone());

    // Generar hash del token (en un sistema real, usar bcrypt/argon2)
    let token_hash = format!("hash_{}", payload.api_token);

    // Crear agente
    let agent = domain::models::telemetry::RemoteAgent::new(
        format!("{}-{}", chrono::Utc::now().timestamp(), payload.name),
        payload.name,
        payload.sede_id,
        domain::models::telemetry::AgentType::from_str(&payload.agent_type),
        payload.ip_address,
        token_hash,
    );

    // Validar agente
    agent.validate_ip().map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Invalid IP address: {}", e),
            }),
        )
    })?;

    agent.validate_token_hash().map_err(|e| {
        (
            axum::http::StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Invalid token: {}", e),
            }),
        )
    })?;

    // Registrar agente
    telemetry_repo
        .register_agent(agent.clone())
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error registering agent: {}", e),
                }),
            )
        })?;

    info!("Agent registered successfully: {}", agent.name);

    Ok(Json(RegisterAgentResponse {
        success: true,
        agent_id: agent.id,
        message: "Agent registered successfully".to_string(),
    }))
}

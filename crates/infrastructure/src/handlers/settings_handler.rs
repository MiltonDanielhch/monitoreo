// crates/infrastructure/src/handlers/settings_handler.rs
// Controladores HTTP para gestión de configuración y umbrales
// Vinculado con ADR-0003-stack-backend-rust-axum.md y ADR-0016-openapi.md

use axum::{Json, extract::State, http::StatusCode};
use shared_types::{ThresholdUpdateRequest, ThresholdResponse};
use crate::AppState;
use domain::{ThresholdValue, ThresholdSettings, DomainError};
use tracing::{info, error};

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    message: String,
}

pub async fn get_thresholds(
    State(state): State<AppState>,
) -> Result<Json<ThresholdResponse>, (StatusCode, Json<ErrorResponse>)> {
    let config = state.runtime_config.read().await;

    Ok(Json(ThresholdResponse {
        ping_ms: shared_types::ThresholdValueDto {
            warning: config.thresholds.ping_ms.warning,
            critical: config.thresholds.ping_ms.critical,
        },
        latency_ms: shared_types::ThresholdValueDto {
            warning: config.thresholds.latency_ms.warning,
            critical: config.thresholds.latency_ms.critical,
        },
        packet_loss_percent: shared_types::ThresholdValueDto {
            warning: config.thresholds.packet_loss_percent.warning,
            critical: config.thresholds.packet_loss_percent.critical,
        },
    }))
}

pub async fn update_thresholds(
    State(state): State<AppState>,
    Json(req): Json<ThresholdUpdateRequest>,
) -> Result<Json<ThresholdResponse>, (StatusCode, Json<ErrorResponse>)> {
    let validation_result = validate_thresholds(&req);
    if let Err(e) = validation_result {
        error!("Error validando thresholds: {}", e);
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { message: e.to_string() })));
    }

    let validated = validation_result.unwrap();

    if let Err(e) = update_thresholds_in_db(&state, &req).await {
        error!("Error actualizando thresholds en DB: {}", e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { message: e.to_string() })));
    }

    state.runtime_config.update_thresholds(validated.clone()).await;

    info!("Thresholds actualizados exitosamente");

    Ok(Json(ThresholdResponse {
        ping_ms: shared_types::ThresholdValueDto {
            warning: validated.ping_ms.warning,
            critical: validated.ping_ms.critical,
        },
        latency_ms: shared_types::ThresholdValueDto {
            warning: validated.latency_ms.warning,
            critical: validated.latency_ms.critical,
        },
        packet_loss_percent: shared_types::ThresholdValueDto {
            warning: validated.packet_loss_percent.warning,
            critical: validated.packet_loss_percent.critical,
        },
    }))
}

fn validate_thresholds(req: &ThresholdUpdateRequest) -> Result<ThresholdSettings, DomainError> {
    let ping_ms = ThresholdValue::new(req.ping_ms.warning, req.ping_ms.critical)?;
    let latency_ms = ThresholdValue::new(req.latency_ms.warning, req.latency_ms.critical)?;
    let packet_loss_percent = ThresholdValue::new(req.packet_loss_percent.warning, req.packet_loss_percent.critical)?;

    Ok(ThresholdSettings {
        ping_ms,
        latency_ms,
        packet_loss_percent,
    })
}

async fn update_thresholds_in_db(
    state: &AppState,
    req: &ThresholdUpdateRequest,
) -> Result<(), DomainError> {
    let repo = &state.settings_repo;

    repo.update_setting("threshold_ping_warning_ms", &req.ping_ms.warning.to_string()).await?;
    repo.update_setting("threshold_ping_critical_ms", &req.ping_ms.critical.to_string()).await?;
    repo.update_setting("threshold_latency_warning_ms", &req.latency_ms.warning.to_string()).await?;
    repo.update_setting("threshold_latency_critical_ms", &req.latency_ms.critical.to_string()).await?;
    repo.update_setting("threshold_packet_loss_warning_percent", &req.packet_loss_percent.warning.to_string()).await?;
    repo.update_setting("threshold_packet_loss_critical_percent", &req.packet_loss_percent.critical.to_string()).await?;

    Ok(())
}
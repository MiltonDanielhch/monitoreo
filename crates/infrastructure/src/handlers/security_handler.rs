// crates/infrastructure/src/handlers/security_handler.rs
// Handlers HTTP para eventos de seguridad
// Vinculado con ADR-0003 (Stack Backend Rust Axum) y ADR-0006 (Seguridad JWT)

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::AppState;
use domain::models::security::{
    SecurityEvent, SecurityFilters, IntrusionType, Severity, SecurityStatus, SecurityPort
};

/// DTO para crear evento de seguridad
#[derive(Debug, Deserialize)]
pub struct SecurityEventRequest {
    pub event_type: String,
    pub severity: String,
    pub source_ip: String,
    pub source_mac: Option<String>,
    pub target_device_id: Option<String>,
    pub target_sede_id: Option<String>,
    pub description: String,
    pub metadata: Option<serde_json::Value>,
}

/// DTO para respuesta de evento de seguridad
#[derive(Debug, Serialize)]
pub struct SecurityEventResponse {
    pub id: String,
    pub event_type: String,
    pub severity: String,
    pub status: String,
    pub source_ip: String,
    pub source_mac: Option<String>,
    pub target_device_id: Option<String>,
    pub target_sede_id: Option<String>,
    pub description: String,
    pub metadata: Option<serde_json::Value>,
    pub detected_at: String,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<String>,
}

/// DTO para filtros de query
#[derive(Debug, Deserialize)]
pub struct SecurityEventQuery {
    pub severity: Option<String>,
    pub status: Option<String>,
    pub source_ip: Option<String>,
    pub target_device_id: Option<String>,
    pub target_sede_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

/// DTO para respuesta paginada
#[derive(Debug, Serialize)]
pub struct SecurityEventsPaginatedResponse {
    pub events: Vec<SecurityEventResponse>,
    pub total: usize,
}

/// DTO para respuesta de error
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// DTO para resolver evento
#[derive(Debug, Deserialize)]
pub struct ResolveEventRequest {
    pub resolved_by: String,
}

/// Handler para registrar evento de seguridad
pub async fn log_security_event(
    State(state): State<AppState>,
    Json(request): Json<SecurityEventRequest>,
) -> Result<Json<SecurityEventResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validar IP
    if let Err(e) = SecurityEvent::validate_ip(&request.source_ip) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: e }),
        ));
    }

    // Validar MAC si está presente
    if let Some(ref mac) = request.source_mac {
        if let Err(e) = SecurityEvent::validate_mac(mac) {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { error: e }),
            ));
        }
    }

    let event_type = IntrusionType::from(request.event_type.clone());
    let severity = Severity::from(request.severity.clone());

    let mut event = SecurityEvent::new(
        event_type,
        severity,
        request.source_ip.clone(),
        request.description.clone(),
    );

    event.source_mac = request.source_mac;
    event.target_device_id = request.target_device_id;
    event.target_sede_id = request.target_sede_id;
    event.metadata = request.metadata;

    state.security_repo.log_event(event.clone())
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    Ok(Json(SecurityEventResponse {
        id: event.id,
        event_type: event.event_type.to_string(),
        severity: event.severity.to_string(),
        status: event.status.to_string(),
        source_ip: event.source_ip,
        source_mac: event.source_mac,
        target_device_id: event.target_device_id,
        target_sede_id: event.target_sede_id,
        description: event.description,
        metadata: event.metadata,
        detected_at: event.detected_at,
        resolved_at: event.resolved_at,
        resolved_by: event.resolved_by,
    }))
}

/// Handler para obtener eventos de seguridad
pub async fn get_security_events(
    State(state): State<AppState>,
    Query(query): Query<SecurityEventQuery>,
) -> Result<Json<SecurityEventsPaginatedResponse>, (StatusCode, Json<ErrorResponse>)> {
    let filters = SecurityFilters {
        severity: query.severity.map(Severity::from),
        status: query.status.map(SecurityStatus::from),
        source_ip: query.source_ip,
        target_device_id: query.target_device_id,
        target_sede_id: query.target_sede_id,
        date_from: query.date_from,
        date_to: query.date_to,
    };

    let events = state.security_repo.get_events(filters)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    let total = events.len();
    let response_events = events.iter().map(|e| SecurityEventResponse {
        id: e.id.clone(),
        event_type: e.event_type.to_string(),
        severity: e.severity.to_string(),
        status: e.status.to_string(),
        source_ip: e.source_ip.clone(),
        source_mac: e.source_mac.clone(),
        target_device_id: e.target_device_id.clone(),
        target_sede_id: e.target_sede_id.clone(),
        description: e.description.clone(),
        metadata: e.metadata.clone(),
        detected_at: e.detected_at.clone(),
        resolved_at: e.resolved_at.clone(),
        resolved_by: e.resolved_by.clone(),
    }).collect();

    Ok(Json(SecurityEventsPaginatedResponse {
        events: response_events,
        total,
    }))
}

/// Handler para obtener evento por ID
pub async fn get_security_event_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<SecurityEventResponse>, (StatusCode, Json<ErrorResponse>)> {
    let event = state.security_repo.get_event_by_id(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    match event {
        Some(e) => Ok(Json(SecurityEventResponse {
            id: e.id,
            event_type: e.event_type.to_string(),
            severity: e.severity.to_string(),
            status: e.status.to_string(),
            source_ip: e.source_ip,
            source_mac: e.source_mac,
            target_device_id: e.target_device_id,
            target_sede_id: e.target_sede_id,
            description: e.description,
            metadata: e.metadata,
            detected_at: e.detected_at,
            resolved_at: e.resolved_at,
            resolved_by: e.resolved_by,
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse { error: "Evento no encontrado".to_string() }),
        )),
    }
}

/// Handler para resolver evento
pub async fn resolve_security_event(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<ResolveEventRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    state.security_repo.resolve_event(id, request.resolved_by)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    Ok(Json(serde_json::json!({ "message": "Evento resuelto exitosamente" })))
}

/// Handler para marcar como falso positivo
pub async fn mark_false_positive(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    state.security_repo.mark_false_positive(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    Ok(Json(serde_json::json!({ "message": "Evento marcado como falso positivo" })))
}

/// Handler para obtener eventos por severidad
pub async fn get_events_by_severity(
    State(state): State<AppState>,
    Path(severity): Path<String>,
) -> Result<Json<SecurityEventsPaginatedResponse>, (StatusCode, Json<ErrorResponse>)> {
    let severity = Severity::from(severity);
    let events = state.security_repo.get_events_by_severity(severity)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() })))?;

    let total = events.len();
    let response_events = events.iter().map(|e| SecurityEventResponse {
        id: e.id.clone(),
        event_type: e.event_type.to_string(),
        severity: e.severity.to_string(),
        status: e.status.to_string(),
        source_ip: e.source_ip.clone(),
        source_mac: e.source_mac.clone(),
        target_device_id: e.target_device_id.clone(),
        target_sede_id: e.target_sede_id.clone(),
        description: e.description.clone(),
        metadata: e.metadata.clone(),
        detected_at: e.detected_at.clone(),
        resolved_at: e.resolved_at.clone(),
        resolved_by: e.resolved_by.clone(),
    }).collect();

    Ok(Json(SecurityEventsPaginatedResponse {
        events: response_events,
        total,
    }))
}

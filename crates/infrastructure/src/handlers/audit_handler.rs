// crates/infrastructure/src/handlers/audit_handler.rs
// Handlers HTTP para auditoría inmutable
// Vinculado con ADR-0003-stack-backend-rust-axum.md

use crate::AppState;
use axum::{
    extract::{Query, State},
    Json,
};
use database::AuditRepository;
use domain::models::audit::{AuditAction, AuditFilters, AuditPort};
use serde::{Deserialize, Serialize};

/// DTO para respuesta de auditoría
#[derive(Serialize)]
pub struct AuditLogsResponse {
    pub logs: Vec<AuditLogDto>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

/// DTO para registro de auditoría
#[derive(Serialize)]
pub struct AuditLogDto {
    pub id: String,
    pub timestamp: String,
    pub user_id: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// DTO para consulta de auditoría
#[derive(Deserialize)]
pub struct AuditLogsQuery {
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub ip_address: Option<String>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

/// DTO para respuesta de error
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Endpoint para consultar registros de auditoría
pub async fn get_audit_logs(
    State(state): State<AppState>,
    Query(params): Query<AuditLogsQuery>,
) -> Result<Json<AuditLogsResponse>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let audit_repo = AuditRepository::new(state.db.clone());

    // Construir filtros
    let mut filters = AuditFilters::new();

    if let Some(user_id) = params.user_id {
        filters = filters.with_user_id(user_id);
    }
    if let Some(action_str) = params.action {
        let action = AuditAction::from_str(&action_str);
        filters = filters.with_action(action);
    }
    if let Some(entity_type) = params.entity_type {
        filters = filters.with_entity_type(entity_type);
    }
    if let Some(entity_id) = params.entity_id {
        filters = filters.with_entity_id(entity_id);
    }
    if let Some(start) = params.start_date {
        if let Some(end) = params.end_date {
            filters = filters.with_date_range(start, end);
        }
    }
    if let Some(ip_address) = params.ip_address {
        filters = filters.with_ip_address(ip_address);
    }

    // Paginación
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(50);
    let offset = (page - 1) * per_page;

    // Consultar logs
    let logs = audit_repo
        .query_logs(filters, per_page, offset)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error fetching audit logs: {}", e),
                }),
            )
        })?;

    let total = logs.len();
    let log_dtos: Vec<AuditLogDto> = logs
        .into_iter()
        .map(|log| AuditLogDto {
            id: log.id,
            timestamp: log.timestamp,
            user_id: log.user_id,
            action: log.action.to_string(),
            entity_type: log.entity_type,
            entity_id: log.entity_id,
            old_value: log.old_value,
            new_value: log.new_value,
            ip_address: log.ip_address,
            user_agent: log.user_agent,
            metadata: log.metadata.and_then(|m| serde_json::from_str(&serde_json::to_string(&m).unwrap_or_default()).ok()),
        })
        .collect();

    Ok(Json(AuditLogsResponse {
        logs: log_dtos,
        total,
        page,
        per_page,
    }))
}

/// Endpoint para obtener historial de una entidad
pub async fn get_entity_history(
    State(state): State<AppState>,
    Query(params): Query<EntityHistoryQuery>,
) -> Result<Json<Vec<AuditLogDto>>, (axum::http::StatusCode, Json<ErrorResponse>)> {
    let audit_repo = AuditRepository::new(state.db.clone());

    let logs = audit_repo
        .get_entity_history(params.entity_type, params.entity_id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Error fetching entity history: {}", e),
                }),
            )
        })?;

    let log_dtos: Vec<AuditLogDto> = logs
        .into_iter()
        .map(|log| AuditLogDto {
            id: log.id,
            timestamp: log.timestamp,
            user_id: log.user_id,
            action: log.action.to_string(),
            entity_type: log.entity_type,
            entity_id: log.entity_id,
            old_value: log.old_value,
            new_value: log.new_value,
            ip_address: log.ip_address,
            user_agent: log.user_agent,
            metadata: log.metadata.and_then(|m| serde_json::from_str(&serde_json::to_string(&m).unwrap_or_default()).ok()),
        })
        .collect();

    Ok(Json(log_dtos))
}

#[derive(Deserialize)]
pub struct EntityHistoryQuery {
    pub entity_type: String,
    pub entity_id: String,
}

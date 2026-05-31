// crates/infrastructure/src/handlers/notification_handler.rs
// Controladores HTTP para gestión de notificaciones
// Vinculado con ADR-0003-stack-backend-rust-axum.md y ADR-0016-openapi.md
// Módulo 4: Motor de Notificaciones

use axum::{Json, extract::State, http::StatusCode};
use crate::AppState;
use database::NotificationRepository;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct NotificationLogResponse {
    pub id: String,
    pub channel_id: String,
    pub template_id: String,
    pub recipient: String,
    pub status: String,
    pub attempt_count: i32,
    pub max_attempts: i32,
    pub error_message: Option<String>,
    pub sent_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct NotificationLogsPaginatedResponse {
    pub logs: Vec<NotificationLogResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

#[derive(Debug, Deserialize)]
pub struct NotificationLogsQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct TestSmtpRequest {
    pub channel_id: String,
    pub recipient: String,
}

#[derive(Debug, Serialize)]
pub struct TestSmtpResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

/// GET /api/v1/notifications/logs - Obtener logs de notificaciones paginados
/// Protegido por RBAC (solo roles autorizados)
pub async fn get_notification_logs(
    State(state): State<AppState>,
    Json(query): Json<NotificationLogsQuery>,
) -> Result<Json<NotificationLogsPaginatedResponse>, (StatusCode, Json<ErrorResponse>)> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    let notification_repo = NotificationRepository::new(state.db.clone());

    // Obtener logs paginados
    let logs = notification_repo
        .get_logs_paginated(page, per_page)
        .await
        .map_err(|e| {
            tracing::error!("Error al obtener logs de notificaciones: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: format!("Error al obtener logs: {}", e),
                }),
            )
        })?;

    // Contar total de logs
    let total = notification_repo
        .count_logs()
        .await
        .map_err(|e| {
            tracing::error!("Error al contar logs de notificaciones: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: format!("Error al contar logs: {}", e),
                }),
            )
        })?;

    // Convertir entidades a DTOs
    let logs_dto: Vec<NotificationLogResponse> = logs
        .into_iter()
        .map(|log| NotificationLogResponse {
            id: log.id,
            channel_id: log.channel_id,
            template_id: log.template_id,
            recipient: log.recipient,
            status: log.status,
            attempt_count: log.attempt_count,
            max_attempts: log.max_attempts,
            error_message: log.error_message,
            sent_at: log.sent_at.map(|dt| dt.to_string()),
            created_at: log.created_at.to_string(),
        })
        .collect();

    Ok(Json(NotificationLogsPaginatedResponse {
        logs: logs_dto,
        total,
        page,
        per_page,
    }))
}

/// POST /api/v1/notifications/test-smtp - Probar conexión SMTP
/// Protegido por RBAC (solo roles autorizados)
pub async fn test_smtp_connection(
    State(state): State<AppState>,
    Json(req): Json<TestSmtpRequest>,
) -> Result<Json<TestSmtpResponse>, (StatusCode, Json<ErrorResponse>)> {
    let notification_repo = NotificationRepository::new(state.db.clone());

    // Obtener el canal de notificación
    let channel = notification_repo
        .get_channel(&req.channel_id)
        .await
        .map_err(|e| {
            tracing::error!("Error al obtener canal {}: {}", req.channel_id, e);
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: format!("Canal no encontrado: {}", e),
                }),
            )
        })?;

    // Parsear configuración SMTP
    let smtp_config = crate::notifications::SmtpConfig::from_json(&channel.config).map_err(
        |e| {
            tracing::error!("Error al parsear config SMTP: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    message: format!("Error en configuración SMTP: {}", e),
                }),
            )
        },
    )?;

    // Crear adaptador SMTP y probar conexión
    let smtp_adapter = crate::notifications::SmtpAdapter::new(smtp_config);

    match smtp_adapter.test_connection() {
        Ok(_) => {
            tracing::info!("Test SMTP exitoso para canal {}", req.channel_id);
            Ok(Json(TestSmtpResponse {
                success: true,
                message: "Conexión SMTP probada exitosamente".to_string(),
            }))
        }
        Err(e) => {
            tracing::error!("Test SMTP fallido para canal {}: {}", req.channel_id, e);
            Ok(Json(TestSmtpResponse {
                success: false,
                message: format!("Error de conexión: {}", e),
            }))
        }
    }
}

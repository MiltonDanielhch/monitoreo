// crates/infrastructure/src/middleware/audit_middleware.rs
// Middleware de auditoría para capturar información de red
// Vinculado con ADR-0003-stack-backend-rust-axum.md y ADR-0009-auditoria-inmutable.md

use axum::{
    extract::{ConnectInfo, Request},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use database::AuditRepository;
use domain::models::audit::{AuditAction, AuditFilters, AuditLog, AuditPort};
use std::net::SocketAddr;
use uuid::uuid;

/// Extractor para obtener información de la solicitud
#[derive(Debug, Clone)]
pub struct RequestInfo {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub user_id: Option<String>,
}

/// Middleware de auditoría que registra automáticamente las acciones
pub async fn audit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Response {
    // Extraer información de la solicitud
    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    // Extraer user_id del header de autorización si existe
    let user_id = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|_token| {
            // En un sistema real, aquí se decodificaría el token JWT/PASEO
            // Por ahora, retornamos None
            None
        });

    // Guardar la información en las extensiones de la solicitud
    request.extensions_mut().insert(RequestInfo {
        ip_address,
        user_agent,
        user_id,
    });

    // Continuar con la siguiente capa
    next.run(request).await
}

/// Extractor para obtener RequestInfo de las extensiones
pub fn extract_request_info(request: &Request) -> RequestInfo {
    request
        .extensions()
        .get::<RequestInfo>()
        .cloned()
        .unwrap_or_else(|| RequestInfo {
            ip_address: None,
            user_agent: None,
            user_id: None,
        })
}

/// Servicio de auditoría para registrar eventos
pub struct AuditService {
    repo: AuditRepository,
}

impl AuditService {
    pub fn new(repo: AuditRepository) -> Self {
        Self { repo }
    }

    /// Registra un evento de auditoría
    pub async fn log_event(&self, log: AuditLog) -> Result<(), String> {
        self.repo.log_event(log).await.map_err(|e| e.to_string())
    }

    /// Registra un evento de auditoría con información de la solicitud
    pub async fn log_from_request(
        &self,
        request: &Request,
        action: AuditAction,
        entity_type: String,
        entity_id: Option<String>,
        old_value: Option<serde_json::Value>,
        new_value: Option<serde_json::Value>,
    ) -> Result<(), String> {
        let info = extract_request_info(request);

        let log = AuditLog::new(
            uuid!("550e8400-e29b-41d4-a716-446655440000").to_string(),
            info.user_id,
            action,
            entity_type,
            entity_id,
            old_value,
            new_value,
            info.ip_address,
            info.user_agent,
        );

        self.log_event(log).await
    }

    /// Consulta registros de auditoría
    pub async fn query_logs(
        &self,
        filters: AuditFilters,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<AuditLog>, String> {
        self.repo
            .query_logs(filters, limit, offset)
            .await
            .map_err(|e| e.to_string())
    }

    /// Obtiene el historial de una entidad
    pub async fn get_entity_history(
        &self,
        entity_type: String,
        entity_id: String,
    ) -> Result<Vec<AuditLog>, String> {
        self.repo
            .get_entity_history(entity_type, entity_id)
            .await
            .map_err(|e| e.to_string())
    }
}

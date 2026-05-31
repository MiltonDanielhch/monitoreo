// crates/domain/src/models/audit.rs
// Modelos de dominio para auditoría inmutable
// Vinculado con ADR-0009-auditoria-inmutable.md

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tipo de acción de auditoría
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AuditAction {
    Create,
    Update,
    Delete,
    Login,
    Logout,
    Upload,
    Download,
    View,
    Export,
    Import,
    Other(String),
}

impl AuditAction {
    pub fn to_string(&self) -> String {
        match self {
            AuditAction::Create => "CREATE".to_string(),
            AuditAction::Update => "UPDATE".to_string(),
            AuditAction::Delete => "DELETE".to_string(),
            AuditAction::Login => "LOGIN".to_string(),
            AuditAction::Logout => "LOGOUT".to_string(),
            AuditAction::Upload => "UPLOAD".to_string(),
            AuditAction::Download => "DOWNLOAD".to_string(),
            AuditAction::View => "VIEW".to_string(),
            AuditAction::Export => "EXPORT".to_string(),
            AuditAction::Import => "IMPORT".to_string(),
            AuditAction::Other(s) => s.clone().to_uppercase(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "CREATE" => AuditAction::Create,
            "UPDATE" => AuditAction::Update,
            "DELETE" => AuditAction::Delete,
            "LOGIN" => AuditAction::Login,
            "LOGOUT" => AuditAction::Logout,
            "UPLOAD" => AuditAction::Upload,
            "DOWNLOAD" => AuditAction::Download,
            "VIEW" => AuditAction::View,
            "EXPORT" => AuditAction::Export,
            "IMPORT" => AuditAction::Import,
            _ => AuditAction::Other(s.to_string()),
        }
    }
}

/// Registro de auditoría inmutable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub timestamp: String,
    pub user_id: Option<String>,
    pub action: AuditAction,
    pub entity_type: String,
    pub entity_id: Option<String>,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl AuditLog {
    /// Crea un nuevo registro de auditoría
    pub fn new(
        id: String,
        user_id: Option<String>,
        action: AuditAction,
        entity_type: String,
        entity_id: Option<String>,
        old_value: Option<serde_json::Value>,
        new_value: Option<serde_json::Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        Self {
            id,
            timestamp: chrono::Utc::now().to_rfc3339(),
            user_id,
            action,
            entity_type,
            entity_id,
            old_value,
            new_value,
            ip_address,
            user_agent,
            metadata: None,
        }
    }

    /// Agrega metadata al registro
    pub fn with_metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Valida que el registro sea inmutable (no permite modificaciones)
    pub fn validate_immutability(&self) -> Result<(), crate::errors::DomainError> {
        // En un sistema append-only, no se permite modificar registros existentes
        // Esta validación es conceptual y se usa en la capa de dominio
        Ok(())
    }

    /// Valida que los valores JSON sean válidos
    pub fn validate_json_values(&self) -> Result<(), crate::errors::DomainError> {
        if let Some(ref old) = self.old_value {
            if !old.is_object() && !old.is_array() && !old.is_null() {
                return Err(crate::errors::DomainError::Infrastructure(
                    "old_value must be a JSON object, array or null".to_string(),
                ));
            }
        }
        if let Some(ref new) = self.new_value {
            if !new.is_object() && !new.is_array() && !new.is_null() {
                return Err(crate::errors::DomainError::Infrastructure(
                    "new_value must be a JSON object, array or null".to_string(),
                ));
            }
        }
        Ok(())
    }
}

/// Filtros de consulta de auditoría
#[derive(Debug, Clone, Default)]
pub struct AuditFilters {
    pub user_id: Option<String>,
    pub action: Option<AuditAction>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub ip_address: Option<String>,
}

impl AuditFilters {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn with_action(mut self, action: AuditAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn with_entity_type(mut self, entity_type: String) -> Self {
        self.entity_type = Some(entity_type);
        self
    }

    pub fn with_entity_id(mut self, entity_id: String) -> Self {
        self.entity_id = Some(entity_id);
        self
    }

    pub fn with_date_range(mut self, start: String, end: String) -> Self {
        self.start_date = Some(start);
        self.end_date = Some(end);
        self
    }

    pub fn with_ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    }
}

/// Puerto del dominio para el servicio de auditoría
#[async_trait::async_trait]
pub trait AuditPort: Send + Sync {
    /// Registra un evento de auditoría (append-only)
    async fn log_event(&self, log: AuditLog) -> Result<(), crate::errors::DomainError>;

    /// Consulta registros de auditoría con filtros
    async fn query_logs(&self, filters: AuditFilters, limit: usize, offset: usize) -> Result<Vec<AuditLog>, crate::errors::DomainError>;

    /// Obtiene un registro específico por ID (solo lectura)
    async fn get_log_by_id(&self, id: String) -> Result<Option<AuditLog>, crate::errors::DomainError>;

    /// Obtiene el historial de cambios de una entidad específica
    async fn get_entity_history(&self, entity_type: String, entity_id: String) -> Result<Vec<AuditLog>, crate::errors::DomainError>;
}

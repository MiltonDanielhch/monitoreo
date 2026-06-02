// crates/domain/src/models/security.rs
// Modelos de dominio para eventos de seguridad y detección de intrusiones
// Vinculado con ADR-0001 (Dominio Puro) y ADR-0015 (Tokio Jobs)

use serde::{Deserialize, Serialize};
use std::fmt;

/// Tipos de intrusión detectados
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntrusionType {
    PortScan,
    DDoS,
    UnauthorizedAccess,
    MalwareDetection,
    Phishing,
    DataExfiltration,
    Other(String),
}

impl fmt::Display for IntrusionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntrusionType::PortScan => write!(f, "port_scan"),
            IntrusionType::DDoS => write!(f, "ddos"),
            IntrusionType::UnauthorizedAccess => write!(f, "unauthorized_access"),
            IntrusionType::MalwareDetection => write!(f, "malware_detection"),
            IntrusionType::Phishing => write!(f, "phishing"),
            IntrusionType::DataExfiltration => write!(f, "data_exfiltration"),
            IntrusionType::Other(s) => write!(f, "{}", s),
        }
    }
}

impl From<String> for IntrusionType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "port_scan" => IntrusionType::PortScan,
            "ddos" => IntrusionType::DDoS,
            "unauthorized_access" => IntrusionType::UnauthorizedAccess,
            "malware_detection" => IntrusionType::MalwareDetection,
            "phishing" => IntrusionType::Phishing,
            "data_exfiltration" => IntrusionType::DataExfiltration,
            other => IntrusionType::Other(other.to_string()),
        }
    }
}

/// Niveles de severidad de eventos de seguridad
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Critical => write!(f, "critical"),
            Severity::High => write!(f, "high"),
            Severity::Medium => write!(f, "medium"),
            Severity::Low => write!(f, "low"),
        }
    }
}

impl From<String> for Severity {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => Severity::Critical,
            "high" => Severity::High,
            "medium" => Severity::Medium,
            "low" => Severity::Low,
            _ => Severity::Low, // Default to low for unknown values
        }
    }
}

/// Estados de eventos de seguridad
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityStatus {
    Detected,
    Investigating,
    Resolved,
    FalsePositive,
}

impl fmt::Display for SecurityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityStatus::Detected => write!(f, "detected"),
            SecurityStatus::Investigating => write!(f, "investigating"),
            SecurityStatus::Resolved => write!(f, "resolved"),
            SecurityStatus::FalsePositive => write!(f, "false_positive"),
        }
    }
}

impl From<String> for SecurityStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "detected" => SecurityStatus::Detected,
            "investigating" => SecurityStatus::Investigating,
            "resolved" => SecurityStatus::Resolved,
            "false_positive" => SecurityStatus::FalsePositive,
            _ => SecurityStatus::Detected, // Default to detected for unknown values
        }
    }
}

/// Evento de seguridad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: String,
    pub event_type: IntrusionType,
    pub severity: Severity,
    pub status: SecurityStatus,
    pub source_ip: String,
    pub source_mac: Option<String>,
    pub target_device_id: Option<String>,
    pub target_sede_id: Option<String>,
    pub description: String,
    pub metadata: Option<serde_json::Value>,
    pub detected_at: String, // RFC3339
    pub resolved_at: Option<String>, // RFC3339
    pub resolved_by: Option<String>,
}

impl SecurityEvent {
    /// Constructor que genera ID único
    pub fn new(
        event_type: IntrusionType,
        severity: Severity,
        source_ip: String,
        description: String,
    ) -> Self {
        Self {
            id: format!("sec_{}", chrono::Utc::now().timestamp_millis()),
            event_type,
            severity,
            status: SecurityStatus::Detected,
            source_ip,
            source_mac: None,
            target_device_id: None,
            target_sede_id: None,
            description,
            metadata: None,
            detected_at: chrono::Utc::now().to_rfc3339(),
            resolved_at: None,
            resolved_by: None,
        }
    }

    /// Valida formato de IP
    pub fn validate_ip(ip: &str) -> Result<(), String> {
        // Validación básica de IPv4 e IPv6
        if ip.contains('.') {
            // IPv4
            let parts: Vec<&str> = ip.split('.').collect();
            if parts.len() != 4 {
                return Err("Formato IPv4 inválido".to_string());
            }
            for part in parts {
                let num: u8 = part.parse().map_err(|_| "Formato IPv4 inválido".to_string())?;
                if num > 255 {
                    return Err("Formato IPv4 inválido".to_string());
                }
            }
        } else if ip.contains(':') {
            // IPv6 - validación básica
            if ip.split(':').count() != 8 {
                return Err("Formato IPv6 inválido".to_string());
            }
        } else {
            return Err("Formato de IP inválido".to_string());
        }
        Ok(())
    }

    /// Valida formato de MAC address
    pub fn validate_mac(mac: &str) -> Result<(), String> {
        let parts: Vec<&str> = mac.split(':').collect();
        if parts.len() != 6 {
            return Err("Formato MAC inválido".to_string());
        }
        for part in parts {
            if part.len() != 2 {
                return Err("Formato MAC inválido".to_string());
            }
            u8::from_str_radix(part, 16).map_err(|_| "Formato MAC inválido".to_string())?;
        }
        Ok(())
    }

    /// Retorna true si severity es Critical
    pub fn is_critical(&self) -> bool {
        self.severity == Severity::Critical
    }

    /// Marca el evento como resuelto
    pub fn mark_as_resolved(&mut self, resolved_by: String) {
        self.status = SecurityStatus::Resolved;
        self.resolved_at = Some(chrono::Utc::now().to_rfc3339());
        self.resolved_by = Some(resolved_by);
    }

    /// Marca el evento como falso positivo
    pub fn mark_as_false_positive(&mut self) {
        self.status = SecurityStatus::FalsePositive;
    }
}

/// Filtros para consultas de eventos de seguridad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFilters {
    pub severity: Option<Severity>,
    pub status: Option<SecurityStatus>,
    pub source_ip: Option<String>,
    pub target_device_id: Option<String>,
    pub target_sede_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

impl Default for SecurityFilters {
    fn default() -> Self {
        Self {
            severity: None,
            status: None,
            source_ip: None,
            target_device_id: None,
            target_sede_id: None,
            date_from: None,
            date_to: None,
        }
    }
}

/// Trait para puerto de seguridad
#[async_trait::async_trait]
pub trait SecurityPort: Send + Sync {
    /// Registrar un evento de seguridad
    async fn log_event(&self, event: SecurityEvent) -> Result<(), DomainError>;

    /// Obtener eventos con filtros
    async fn get_events(&self, filters: SecurityFilters) -> Result<Vec<SecurityEvent>, DomainError>;

    /// Obtener evento por ID
    async fn get_event_by_id(&self, id: String) -> Result<Option<SecurityEvent>, DomainError>;

    /// Resolver un evento
    async fn resolve_event(&self, id: String, resolved_by: String) -> Result<(), DomainError>;

    /// Marcar evento como falso positivo
    async fn mark_false_positive(&self, id: String) -> Result<(), DomainError>;

    /// Obtener eventos por severidad
    async fn get_events_by_severity(&self, severity: Severity) -> Result<Vec<SecurityEvent>, DomainError>;

    /// Obtener eventos por estado
    async fn get_events_by_status(&self, status: SecurityStatus) -> Result<Vec<SecurityEvent>, DomainError>;

    /// Obtener eventos por dispositivo
    async fn get_events_by_device(&self, device_id: String) -> Result<Vec<SecurityEvent>, DomainError>;
}

/// Error de dominio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainError {
    pub message: String,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DomainError {}

impl From<String> for DomainError {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl From<&str> for DomainError {
    fn from(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

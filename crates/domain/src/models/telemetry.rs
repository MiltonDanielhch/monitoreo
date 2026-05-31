// crates/domain/src/models/telemetry.rs
// Modelos de dominio para telemetría de agentes
// Vinculado con ADR-0001-arquitectura-hexagonal.md

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tipo de agente remoto
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AgentType {
    Router,
    Switch,
    Server,
    Firewall,
    AccessPoint,
    Other(String),
}

impl AgentType {
    pub fn to_string(&self) -> String {
        match self {
            AgentType::Router => "ROUTER".to_string(),
            AgentType::Switch => "SWITCH".to_string(),
            AgentType::Server => "SERVER".to_string(),
            AgentType::Firewall => "FIREWALL".to_string(),
            AgentType::AccessPoint => "ACCESS_POINT".to_string(),
            AgentType::Other(s) => s.clone().to_uppercase(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "ROUTER" => AgentType::Router,
            "SWITCH" => AgentType::Switch,
            "SERVER" => AgentType::Server,
            "FIREWALL" => AgentType::Firewall,
            "ACCESS_POINT" => AgentType::AccessPoint,
            _ => AgentType::Other(s.to_string()),
        }
    }
}

/// Estado del agente
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AgentStatus {
    Active,
    Inactive,
    Error,
    Maintenance,
}

impl AgentStatus {
    pub fn to_string(&self) -> String {
        match self {
            AgentStatus::Active => "ACTIVE".to_string(),
            AgentStatus::Inactive => "INACTIVE".to_string(),
            AgentStatus::Error => "ERROR".to_string(),
            AgentStatus::Maintenance => "MAINTENANCE".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "ACTIVE" => AgentStatus::Active,
            "INACTIVE" => AgentStatus::Inactive,
            "ERROR" => AgentStatus::Error,
            "MAINTENANCE" => AgentStatus::Maintenance,
            _ => AgentStatus::Inactive,
        }
    }
}

/// Agente remoto
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteAgent {
    pub id: String,
    pub name: String,
    pub sede_id: String,
    pub agent_type: AgentType,
    pub ip_address: String,
    pub api_token_hash: String,
    pub last_seen: Option<String>,
    pub status: AgentStatus,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl RemoteAgent {
    /// Crea un nuevo agente remoto
    pub fn new(
        id: String,
        name: String,
        sede_id: String,
        agent_type: AgentType,
        ip_address: String,
        api_token_hash: String,
    ) -> Self {
        Self {
            id,
            name,
            sede_id,
            agent_type,
            ip_address,
            api_token_hash,
            last_seen: None,
            status: AgentStatus::Active,
            metadata: None,
        }
    }

    /// Valida que la dirección IP sea válida
    pub fn validate_ip(&self) -> Result<(), crate::errors::DomainError> {
        // Validación básica de dirección IP
        if self.ip_address.is_empty() {
            return Err(crate::errors::DomainError::Infrastructure(
                "IP address cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    /// Valida que el token hash no esté vacío
    pub fn validate_token_hash(&self) -> Result<(), crate::errors::DomainError> {
        if self.api_token_hash.is_empty() {
            return Err(crate::errors::DomainError::Infrastructure(
                "Token hash cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}

/// Métricas de telemetría
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryMetrics {
    pub cpu_usage_percent: Option<f64>,
    pub memory_usage_percent: Option<f64>,
    pub latency_ms: Option<i32>,
    pub packet_loss_percent: Option<f64>,
    pub bandwidth_mbps: Option<f64>,
    pub disk_usage_percent: Option<f64>,
    pub temperature_celsius: Option<f64>,
    pub uptime_seconds: Option<i64>,
}

impl TelemetryMetrics {
    /// Crea nuevas métricas
    pub fn new() -> Self {
        Self {
            cpu_usage_percent: None,
            memory_usage_percent: None,
            latency_ms: None,
            packet_loss_percent: None,
            bandwidth_mbps: None,
            disk_usage_percent: None,
            temperature_celsius: None,
            uptime_seconds: None,
        }
    }

    /// Valida que los valores estén en rangos aceptables
    pub fn validate(&self) -> Result<(), crate::errors::DomainError> {
        if let Some(cpu) = self.cpu_usage_percent {
            if cpu < 0.0 || cpu > 100.0 {
                return Err(crate::errors::DomainError::Infrastructure(
                    "CPU usage must be between 0 and 100".to_string(),
                ));
            }
        }
        if let Some(memory) = self.memory_usage_percent {
            if memory < 0.0 || memory > 100.0 {
                return Err(crate::errors::DomainError::Infrastructure(
                    "Memory usage must be between 0 and 100".to_string(),
                ));
            }
        }
        if let Some(latency) = self.latency_ms {
            if latency < 0 {
                return Err(crate::errors::DomainError::Infrastructure(
                    "Latency cannot be negative".to_string(),
                ));
            }
        }
        if let Some(packet_loss) = self.packet_loss_percent {
            if packet_loss < 0.0 || packet_loss > 100.0 {
                return Err(crate::errors::DomainError::Infrastructure(
                    "Packet loss must be between 0 and 100".to_string(),
                ));
            }
        }
        Ok(())
    }

    /// Verifica si alguna métrica excede los umbrales críticos
    pub fn is_critical(&self, cpu_threshold: f64, memory_threshold: f64, latency_threshold: i32) -> bool {
        if let Some(cpu) = self.cpu_usage_percent {
            if cpu > cpu_threshold {
                return true;
            }
        }
        if let Some(memory) = self.memory_usage_percent {
            if memory > memory_threshold {
                return true;
            }
        }
        if let Some(latency) = self.latency_ms {
            if latency > latency_threshold {
                return true;
            }
        }
        false
    }
}

/// Lote de métricas de telemetría
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryBatch {
    pub id: String,
    pub agent_id: String,
    pub metrics: TelemetryMetrics,
    pub created_at: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl TelemetryBatch {
    /// Crea un nuevo lote de métricas
    pub fn new(agent_id: String, metrics: TelemetryMetrics) -> Self {
        // Generar un UUID simple usando el timestamp y un contador
        let id = format!("{}-{}", chrono::Utc::now().timestamp(), agent_id);
        Self {
            id,
            agent_id,
            metrics,
            created_at: chrono::Utc::now().to_rfc3339(),
            metadata: None,
        }
    }

    /// Valida el lote de métricas
    pub fn validate(&self) -> Result<(), crate::errors::DomainError> {
        if self.agent_id.is_empty() {
            return Err(crate::errors::DomainError::Infrastructure(
                "Agent ID cannot be empty".to_string(),
            ));
        }
        self.metrics.validate()
    }
}

/// Puerto del dominio para el servicio de telemetría
#[async_trait::async_trait]
pub trait TelemetryPort: Send + Sync {
    /// Registra un agente remoto
    async fn register_agent(&self, agent: RemoteAgent) -> Result<(), crate::errors::DomainError>;

    /// Obtiene un agente por ID
    async fn get_agent(&self, id: String) -> Result<Option<RemoteAgent>, crate::errors::DomainError>;

    /// Obtiene un agente por token hash
    async fn get_agent_by_token(&self, token_hash: String) -> Result<Option<RemoteAgent>, crate::errors::DomainError>;

    /// Actualiza el estado de un agente
    async fn update_agent_status(&self, id: String, status: AgentStatus) -> Result<(), crate::errors::DomainError>;

    /// Registra un lote de métricas
    async fn ingest_metrics(&self, batch: TelemetryBatch) -> Result<(), crate::errors::DomainError>;

    /// Registra múltiples lotes de métricas (inserción masiva)
    async fn ingest_metrics_batch(&self, batches: Vec<TelemetryBatch>) -> Result<(), crate::errors::DomainError>;

    /// Obtiene métricas de un agente en un rango de tiempo
    async fn get_agent_metrics(
        &self,
        agent_id: String,
        start: String,
        end: String,
    ) -> Result<Vec<TelemetryBatch>, crate::errors::DomainError>;

    /// Obtiene métricas recientes de todos los agentes
    async fn get_recent_metrics(&self, limit: usize) -> Result<Vec<TelemetryBatch>, crate::errors::DomainError>;
}

// crates/domain/src/models/settings.rs
// Modelos de configuración del sistema y umbrales
// Vinculado con ADR-0001 (Arquitectura Hexagonal)

use crate::DomainError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdValue {
    pub warning: f64,
    pub critical: f64,
}

impl ThresholdValue {
    pub fn new(warning: f64, critical: f64) -> Result<Self, DomainError> {
        if critical <= warning {
            return Err(DomainError::InvalidSettingValue(
                format!("El umbral crítico ({}) debe ser mayor al de advertencia ({})", critical, warning)
            ));
        }
        Ok(Self { warning, critical })
    }

    pub fn is_critical(&self, value: f64) -> bool {
        value >= self.critical
    }

    pub fn is_warning(&self, value: f64) -> bool {
        value >= self.warning && value < self.critical
    }

    pub fn is_healthy(&self, value: f64) -> bool {
        value < self.warning
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSettings {
    pub thresholds: ThresholdSettings,
    pub check_interval_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdSettings {
    pub ping_ms: ThresholdValue,
    pub latency_ms: ThresholdValue,
    pub packet_loss_percent: ThresholdValue,
}

impl ThresholdSettings {
    pub fn is_healthy(&self, ping_ms: f64, latency_ms: f64, packet_loss_percent: f64) -> bool {
        self.ping_ms.is_healthy(ping_ms)
            && self.latency_ms.is_healthy(latency_ms)
            && self.packet_loss_percent.is_healthy(packet_loss_percent)
    }

    pub fn get_status(&self, ping_ms: f64, latency_ms: f64, packet_loss_percent: f64) -> &'static str {
        if self.ping_ms.is_critical(ping_ms)
            || self.latency_ms.is_critical(latency_ms)
            || self.packet_loss_percent.is_critical(packet_loss_percent)
        {
            "critical"
        } else if self.ping_ms.is_warning(ping_ms)
            || self.latency_ms.is_warning(latency_ms)
            || self.packet_loss_percent.is_warning(packet_loss_percent)
        {
            "warning"
        } else {
            "healthy"
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: uuid::Uuid,
    pub name: String,
    pub code: String,
    pub region: String,
    pub parent_id: Option<uuid::Uuid>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Location {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        code: String,
        region: String,
        parent_id: Option<uuid::Uuid>,
        latitude: Option<f64>,
        longitude: Option<f64>,
    ) -> Self {
        Self {
            id,
            name,
            code,
            region,
            parent_id,
            latitude,
            longitude,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSetting {
    pub id: uuid::Uuid,
    pub key_name: String,
    pub value: String,
    pub value_type: SettingValueType,
    pub category: String,
    pub description: Option<String>,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SettingValueType {
    Integer,
    Float,
    String,
    Boolean,
}

impl SettingValueType {
    pub fn parse(&self, value: &str) -> Result<f64, DomainError> {
        match self {
            SettingValueType::Integer | SettingValueType::Float => {
                value.parse::<f64>()
                    .map_err(|_| DomainError::InvalidSettingValue(
                        format!("No se pudo parsear '{}' como número", value)
                    ))
            }
            SettingValueType::String => Err(DomainError::InvalidSettingValue(
                "No se puede parsear String como número".to_string()
            )),
            SettingValueType::Boolean => Err(DomainError::InvalidSettingValue(
                "No se puede parsear Boolean como número".to_string()
            )),
        }
    }
}

impl std::fmt::Display for SettingValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingValueType::Integer => write!(f, "integer"),
            SettingValueType::Float => write!(f, "float"),
            SettingValueType::String => write!(f, "string"),
            SettingValueType::Boolean => write!(f, "boolean"),
        }
    }
}
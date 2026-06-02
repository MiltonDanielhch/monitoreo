// crates/domain/src/models/report.rs
// Modelos de dominio para reportes y SLA
// Vinculado con ADR-0001 (Arquitectura Hexagonal)

use serde::{Deserialize, Serialize};

/// Estado contractual del SLA
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlaStatus {
    /// SLA cumplido
    Compliant,
    /// SLA incumplido (breached)
    Breached,
    /// SLA en riesgo (cerca del umbral)
    AtRisk,
}

/// Tipo de reporte
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportType {
    /// Reporte mensual de SLA
    MonthlySla,
    /// Reporte de incidentes
    IncidentReport,
    /// Reporte de tendencias
    TrendReport,
}

/// Reporte de SLA mensual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaReport {
    pub id: String,
    pub sede_id: String,
    pub sede_name: String,
    pub month: String,
    pub year: i32,
    pub availability_target: f64, // Objetivo de disponibilidad (ej. 99.5%)
    pub availability_achieved: f64, // Disponibilidad alcanzada
    pub status: SlaStatus,
    pub uptime_minutes: i64,
    pub downtime_minutes: i64,
    pub total_events: i64,
    pub avg_latency_ms: f64,
    pub avg_bandwidth_mbps: f64,
    pub incident_count: i64,
    pub generated_at: String, // RFC3339
    pub generated_by: String,
}

/// Métricas de SLA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaMetrics {
    pub uptime_percentage: f64,
    pub uptime_minutes: i64,
    pub downtime_minutes: i64,
    pub total_minutes: i64,
    pub avg_latency_ms: f64,
    pub avg_bandwidth_mbps: f64,
}

/// Filtros para reportes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportFilters {
    pub sede_id: Option<String>,
    pub month: Option<String>,
    pub year: Option<i32>,
    pub status: Option<SlaStatus>,
}

/// Error de dominio para reportes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainError(pub String);

impl From<String> for DomainError {
    fn from(s: String) -> Self {
        DomainError(s)
    }
}

impl SlaReport {
    /// Constructor que genera ID único
    pub fn new(
        sede_id: String,
        sede_name: String,
        month: String,
        year: i32,
        availability_target: f64,
        availability_achieved: f64,
        uptime_minutes: i64,
        downtime_minutes: i64,
        total_events: i64,
        avg_latency_ms: f64,
        avg_bandwidth_mbps: f64,
        incident_count: i64,
        generated_by: String,
    ) -> Self {
        let status = Self::calculate_status(availability_achieved, availability_target);

        Self {
            id: format!("sla_{}_{}_{}", sede_id, month, year),
            sede_id,
            sede_name,
            month,
            year,
            availability_target,
            availability_achieved,
            status,
            uptime_minutes,
            downtime_minutes,
            total_events,
            avg_latency_ms,
            avg_bandwidth_mbps,
            incident_count,
            generated_at: chrono::Utc::now().to_rfc3339(),
            generated_by,
        }
    }

    /// Calcular estado del SLA basado en el umbral
    fn calculate_status(achieved: f64, target: f64) -> SlaStatus {
        let threshold = 0.5; // Margen de 0.5% para estado "en riesgo"

        if achieved >= target {
            SlaStatus::Compliant
        } else if achieved >= (target - threshold) {
            SlaStatus::AtRisk
        } else {
            SlaStatus::Breached
        }
    }

    /// Verificar si el SLA está cumplido
    pub fn is_compliant(&self) -> bool {
        self.status == SlaStatus::Compliant
    }

    /// Verificar si el SLA está incumplido
    pub fn is_breached(&self) -> bool {
        self.status == SlaStatus::Breached
    }

    /// Obtener porcentaje de cumplimiento
    pub fn compliance_percentage(&self) -> f64 {
        (self.availability_achieved / self.availability_target) * 100.0
    }

    /// Obtener métricas de SLA
    pub fn metrics(&self) -> SlaMetrics {
        let total_minutes = self.uptime_minutes + self.downtime_minutes;
        let uptime_percentage = if total_minutes > 0 {
            (self.uptime_minutes as f64 / total_minutes as f64) * 100.0
        } else {
            0.0
        };

        SlaMetrics {
            uptime_percentage,
            uptime_minutes: self.uptime_minutes,
            downtime_minutes: self.downtime_minutes,
            total_minutes,
            avg_latency_ms: self.avg_latency_ms,
            avg_bandwidth_mbps: self.avg_bandwidth_mbps,
        }
    }
}

impl SlaMetrics {
    /// Calcular SLA desde uptime y downtime
    pub fn from_uptime_downtime(uptime_minutes: i64, downtime_minutes: i64) -> Self {
        let total_minutes = uptime_minutes + downtime_minutes;
        let uptime_percentage = if total_minutes > 0 {
            (uptime_minutes as f64 / total_minutes as f64) * 100.0
        } else {
            0.0
        };

        Self {
            uptime_percentage,
            uptime_minutes,
            downtime_minutes,
            total_minutes,
            avg_latency_ms: 0.0,
            avg_bandwidth_mbps: 0.0,
        }
    }
}

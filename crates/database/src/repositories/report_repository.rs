// crates/database/src/repositories/report_repository.rs
// Repositorio para consultas de agregación y cálculo de SLA
// Vinculado con ADR-0004 (Persistencia con Sea-ORM)

use sea_orm::{DatabaseConnection, DbErr, FromQueryResult, Statement};
use sea_orm::sqlx::types::chrono::NaiveDateTime;
use std::sync::Arc;

/// Resultado de agregación de SLA mensual
#[derive(Debug, Clone, FromQueryResult)]
pub struct SlaMonthlyAggregation {
    pub sede_id: String,
    pub month: String,
    pub total_events: i64,
    pub uptime_events: i64,
    pub downtime_events: i64,
    pub estimated_downtime_minutes: i64,
    pub estimated_uptime_minutes: i64,
    pub sla_percentage: f64,
    pub avg_latency_ms: Option<f64>,
    pub avg_bandwidth_mbps: Option<f64>,
    pub period_start: NaiveDateTime,
    pub period_end: NaiveDateTime,
}

/// Filtros para consultas de SLA
#[derive(Debug, Clone)]
pub struct SlaFilters {
    pub sede_id: Option<String>,
    pub month: Option<String>,
    pub year: Option<i32>,
    pub min_sla_threshold: Option<f64>,
}

pub struct ReportRepository {
    db: Arc<DatabaseConnection>,
}

impl ReportRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// Obtener agregación de SLA mensual con filtros
    pub async fn get_sla_monthly(
        &self,
        filters: SlaFilters,
    ) -> Result<Vec<SlaMonthlyAggregation>, DbErr> {
        let mut query = String::from(
            r#"
            SELECT 
                sede_id,
                DATE_FORMAT(detected_at, '%Y-%m') AS month,
                COUNT(*) AS total_events,
                SUM(CASE WHEN status = 'up' THEN 1 ELSE 0 END) AS uptime_events,
                SUM(CASE WHEN status = 'down' THEN 1 ELSE 0 END) AS downtime_events,
                SUM(CASE WHEN status = 'down' THEN 1 ELSE 0 END) * 5 AS estimated_downtime_minutes,
                SUM(CASE WHEN status = 'up' THEN 1 ELSE 0 END) * 5 AS estimated_uptime_minutes,
                ROUND(
                    (SUM(CASE WHEN status = 'up' THEN 1 ELSE 0 END) * 100.0 / COUNT(*)),
                    2
                ) AS sla_percentage,
                AVG(latency_ms) AS avg_latency_ms,
                AVG(bandwidth_mbps) AS avg_bandwidth_mbps,
                MIN(detected_at) AS period_start,
                MAX(detected_at) AS period_end
            FROM telemetry_metrics
            WHERE detected_at >= DATE_SUB(NOW(), INTERVAL 12 MONTH)
            "#
        );

        let mut where_clauses = Vec::new();

        if let Some(ref sede_id) = filters.sede_id {
            where_clauses.push(format!("sede_id = '{}'", sede_id));
        }

        if let Some(ref month) = filters.month {
            where_clauses.push(format!("DATE_FORMAT(detected_at, '%Y-%m') = '{}'", month));
        }

        if let Some(year) = filters.year {
            where_clauses.push(format!("YEAR(detected_at) = {}", year));
        }

        if let Some(threshold) = filters.min_sla_threshold {
            where_clauses.push(format!("sla_percentage < {}", threshold));
        }

        if !where_clauses.is_empty() {
            query.push_str(" AND ");
            query.push_str(&where_clauses.join(" AND "));
        }

        query.push_str(
            r#"
            GROUP BY sede_id, DATE_FORMAT(detected_at, '%Y-%m')
            ORDER BY sede_id, month DESC
            "#
        );

        SlaMonthlyAggregation::find_by_statement(Statement::from_string(
            sea_orm::DbBackend::MySql,
            query,
        ))
        .all(&*self.db)
        .await
    }

    /// Obtener SLA de una sede específica en un periodo
    pub async fn get_sla_by_sede_month(
        &self,
        sede_id: &str,
        month: &str,
    ) -> Result<Option<SlaMonthlyAggregation>, DbErr> {
        let results = self.get_sla_monthly(SlaFilters {
            sede_id: Some(sede_id.to_string()),
            month: Some(month.to_string()),
            year: None,
            min_sla_threshold: None,
        }).await?;

        Ok(results.into_iter().next())
    }

    /// Obtener resumen de SLA de todas las sedes en un mes
    pub async fn get_sla_all_sedes_month(
        &self,
        month: &str,
    ) -> Result<Vec<SlaMonthlyAggregation>, DbErr> {
        self.get_sla_monthly(SlaFilters {
            sede_id: None,
            month: Some(month.to_string()),
            year: None,
            min_sla_threshold: None,
        }).await
    }

    /// Detectar sedes que incumplen el SLA
    pub async fn get_sla_breached(
        &self,
        threshold: f64,
        month: Option<String>,
    ) -> Result<Vec<SlaMonthlyAggregation>, DbErr> {
        self.get_sla_monthly(SlaFilters {
            sede_id: None,
            month,
            year: None,
            min_sla_threshold: Some(threshold),
        }).await
    }

    /// Obtener tendencias de SLA de una sede en los últimos N meses
    pub async fn get_sla_trends(
        &self,
        sede_id: &str,
        months: i32,
    ) -> Result<Vec<SlaMonthlyAggregation>, DbErr> {
        let query = format!(
            r#"
            SELECT 
                sede_id,
                DATE_FORMAT(detected_at, '%Y-%m') AS month,
                COUNT(*) AS total_events,
                SUM(CASE WHEN status = 'up' THEN 1 ELSE 0 END) AS uptime_events,
                SUM(CASE WHEN status = 'down' THEN 1 ELSE 0 END) AS downtime_events,
                SUM(CASE WHEN status = 'down' THEN 1 ELSE 0 END) * 5 AS estimated_downtime_minutes,
                SUM(CASE WHEN status = 'up' THEN 1 ELSE 0 END) * 5 AS estimated_uptime_minutes,
                ROUND(
                    (SUM(CASE WHEN status = 'up' THEN 1 ELSE 0 END) * 100.0 / COUNT(*)),
                    2
                ) AS sla_percentage,
                AVG(latency_ms) AS avg_latency_ms,
                AVG(bandwidth_mbps) AS avg_bandwidth_mbps,
                MIN(detected_at) AS period_start,
                MAX(detected_at) AS period_end
            FROM telemetry_metrics
            WHERE sede_id = '{}'
            AND detected_at >= DATE_SUB(NOW(), INTERVAL {} MONTH)
            GROUP BY sede_id, DATE_FORMAT(detected_at, '%Y-%m')
            ORDER BY month DESC
            LIMIT {}
            "#,
            sede_id, months, months
        );

        SlaMonthlyAggregation::find_by_statement(Statement::from_string(
            sea_orm::DbBackend::MySql,
            query,
        ))
        .all(&*self.db)
        .await
    }

    /// Obtener estadísticas globales de SLA
    pub async fn get_global_sla_stats(
        &self,
        month: Option<String>,
    ) -> Result<GlobalSlaStats, DbErr> {
        let mut query = String::from(
            r#"
            SELECT 
                COUNT(DISTINCT sede_id) AS total_sedes,
                AVG(sla_percentage) AS avg_sla_percentage,
                MIN(sla_percentage) AS min_sla_percentage,
                MAX(sla_percentage) AS max_sla_percentage,
                SUM(estimated_downtime_minutes) AS total_downtime_minutes,
                SUM(estimated_uptime_minutes) AS total_uptime_minutes
            FROM (
                SELECT 
                    sede_id,
                    DATE_FORMAT(detected_at, '%Y-%m') AS month,
                    ROUND(
                        (SUM(CASE WHEN status = 'up' THEN 1 ELSE 0 END) * 100.0 / COUNT(*)),
                        2
                    ) AS sla_percentage,
                    SUM(CASE WHEN status = 'down' THEN 1 ELSE 0 END) * 5 AS estimated_downtime_minutes,
                    SUM(CASE WHEN status = 'up' THEN 1 ELSE 0 END) * 5 AS estimated_uptime_minutes
                FROM telemetry_metrics
                WHERE detected_at >= DATE_SUB(NOW(), INTERVAL 12 MONTH)
            "#
        );

        if let Some(ref month) = month {
            query.push_str(&format!(" AND DATE_FORMAT(detected_at, '%Y-%m') = '{}'", month));
        }

        query.push_str(
            r#"
            GROUP BY sede_id, month
            ) AS monthly_sla
            "#
        );

        GlobalSlaStats::find_by_statement(Statement::from_string(
            sea_orm::DbBackend::MySql,
            query,
        ))
        .one(&*self.db)
        .await?
        .ok_or_else(|| DbErr::Custom("No se encontraron estadísticas globales".to_string()))
    }
}

/// Estadísticas globales de SLA
#[derive(Debug, Clone, FromQueryResult)]
pub struct GlobalSlaStats {
    pub total_sedes: i64,
    pub avg_sla_percentage: Option<f64>,
    pub min_sla_percentage: Option<f64>,
    pub max_sla_percentage: Option<f64>,
    pub total_downtime_minutes: Option<i64>,
    pub total_uptime_minutes: Option<i64>,
}

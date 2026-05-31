// crates/database/src/repositories/dashboard_repository.rs
// Repositorio de dashboard con consultas de agregación
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

use crate::entities::{Device, ActiveAlert, Location};
use crate::entities::location_entity::Column as LocationColumn;
use crate::entities::device_entity::Column as DeviceColumn;
use crate::entities::active_alert_entity::Column as AlertColumn;
use domain::{DomainError, Result};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

pub struct DashboardRepository {
    db: DatabaseConnection,
}

impl DashboardRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_stats(&self) -> Result<DashboardStats> {
        let active_locations = Location::find()
            .filter(LocationColumn::IsActive.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?
            .len() as i32;

        let online_devices = Device::find()
            .filter(DeviceColumn::Status.eq("online"))
            .filter(DeviceColumn::IsActive.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?
            .len() as i32;

        let total_devices = Device::find()
            .filter(DeviceColumn::IsActive.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?
            .len() as i32;

        let pending_alerts = ActiveAlert::find()
            .filter(AlertColumn::IsAcknowledged.eq(false))
            .filter(AlertColumn::ResolvedAt.is_null())
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?
            .len() as i32;

        let critical_alerts = ActiveAlert::find()
            .filter(AlertColumn::Severity.eq("critical"))
            .filter(AlertColumn::IsAcknowledged.eq(false))
            .filter(AlertColumn::ResolvedAt.is_null())
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?
            .len() as i32;

        let total_bandwidth: f64 = Device::find()
            .filter(DeviceColumn::Status.eq("online"))
            .filter(DeviceColumn::IsActive.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?
            .into_iter()
            .map(|d| d.bandwidth_gbps.unwrap_or_default().to_string().parse::<f64>().unwrap_or(0.0))
            .sum();

        Ok(DashboardStats {
            active_locations,
            online_devices,
            total_devices,
            pending_alerts,
            critical_alerts,
            total_bandwidth_gbps: total_bandwidth,
        })
    }

    pub async fn get_recent_alerts(&self, limit: i32) -> Result<Vec<AlertInfo>> {
        let alerts = ActiveAlert::find()
            .filter(AlertColumn::ResolvedAt.is_null())
            .order_by(AlertColumn::CreatedAt, sea_orm::Order::Desc)
            .limit(limit as u64)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        alerts.into_iter()
            .map(|a| {
                Ok(AlertInfo {
                    id: a.id,
                    severity: a.severity,
                    title: a.title,
                    description: a.description,
                    device_id: a.device_id,
                    location_id: a.location_id,
                    metric_name: a.metric_name,
                    metric_value: a.metric_value.map(|d| d.to_string().parse::<f64>().unwrap_or(0.0)),
                    created_at: a.created_at.to_string(),
                })
            })
            .collect()
    }

    pub async fn find_all_devices(&self) -> Result<Vec<DeviceInfo>> {
        let devices = Device::find()
            .filter(DeviceColumn::IsActive.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        devices.into_iter()
            .map(|d| {
                Ok(DeviceInfo {
                    id: d.id,
                    name: d.name,
                    device_type: d.device_type,
                    location_id: d.location_id,
                    ip_address: d.ip_address,
                    bandwidth_gbps: d.bandwidth_gbps.unwrap_or_default().to_string().parse::<f64>().unwrap_or(0.0),
                    status: d.status,
                    is_active: d.is_active,
                    last_seen: d.last_seen.map(|dt| dt.to_string()),
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DashboardStats {
    pub active_locations: i32,
    pub online_devices: i32,
    pub total_devices: i32,
    pub pending_alerts: i32,
    pub critical_alerts: i32,
    pub total_bandwidth_gbps: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AlertInfo {
    pub id: String,
    pub severity: String,
    pub title: String,
    pub description: Option<String>,
    pub device_id: Option<String>,
    pub location_id: Option<String>,
    pub metric_name: Option<String>,
    pub metric_value: Option<f64>,
    pub created_at: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub location_id: Option<String>,
    pub ip_address: Option<String>,
    pub bandwidth_gbps: f64,
    pub status: String,
    pub is_active: bool,
    pub last_seen: Option<String>,
}
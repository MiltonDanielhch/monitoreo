// crates/database/src/repositories/settings_repository.rs
// Repositorio de configuración del sistema con Sea-ORM
// Vinculado con ADR-0004-persistencia-mysql-seaorm-docker.md

use crate::entities::{location_entity, system_setting_entity};
use domain::{DomainError, Result};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, IntoActiveModel, ActiveModelTrait};
use uuid::Uuid;

pub struct SettingsRepository {
    db: DatabaseConnection,
}

impl SettingsRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_all_locations(&self) -> Result<Vec<domain::Location>> {
        let locations = location_entity::Entity::find()
            .filter(location_entity::Column::IsActive.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        locations.into_iter()
            .map(|loc| {
                Ok(domain::Location::new(
                    Uuid::parse_str(&loc.id).map_err(|_| DomainError::Infrastructure("Invalid UUID".to_string()))?,
                    loc.name,
                    loc.code,
                    loc.region,
                    loc.latitude.map(|d| d.to_string().parse::<f64>().ok()).flatten(),
                    loc.longitude.map(|d| d.to_string().parse::<f64>().ok()).flatten(),
                ))
            })
            .collect()
    }

    pub async fn find_all_settings(&self) -> Result<Vec<domain::SystemSetting>> {
        let settings = system_setting_entity::Entity::find()
            .filter(system_setting_entity::Column::IsActive.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        settings.into_iter()
            .map(|s| {
                let value_type = match s.value_type.as_str() {
                    "integer" => domain::SettingValueType::Integer,
                    "float" => domain::SettingValueType::Float,
                    "string" => domain::SettingValueType::String,
                    "boolean" => domain::SettingValueType::Boolean,
                    _ => domain::SettingValueType::String,
                };

                Ok(domain::SystemSetting {
                    id: Uuid::parse_str(&s.id).map_err(|_| DomainError::Infrastructure("Invalid UUID".to_string()))?,
                    key_name: s.key_name,
                    value: s.value,
                    value_type,
                    category: s.category,
                    description: s.description,
                    min_value: s.min_value,
                    max_value: s.max_value,
                    is_active: s.is_active,
                })
            })
            .collect()
    }

    pub async fn update_setting(&self, key_name: &str, new_value: &str) -> Result<()> {
        let setting = system_setting_entity::Entity::find()
            .filter(system_setting_entity::Column::KeyName.eq(key_name))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?
            .ok_or_else(|| DomainError::Infrastructure(format!("Setting '{}' not found", key_name)))?;

        let mut active_model = setting.into_active_model();
        active_model.value = sea_orm::Set(new_value.to_string());

        active_model.update(&self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        Ok(())
    }

    pub async fn get_thresholds(&self) -> Result<domain::ThresholdSettings> {
        let settings = self.find_all_settings().await?;
        let threshold_settings: Vec<&domain::SystemSetting> = settings.iter()
            .filter(|s| s.category == "thresholds")
            .collect();

        let mut ping_warning = 100.0;
        let mut ping_critical = 500.0;
        let mut latency_warning = 150.0;
        let mut latency_critical = 800.0;
        let mut packet_loss_warning = 5.0;
        let mut packet_loss_critical = 15.0;

        for setting in threshold_settings {
            match setting.key_name.as_str() {
                "threshold_ping_warning_ms" => {
                    ping_warning = setting.value_type.parse(&setting.value).unwrap_or(ping_warning);
                }
                "threshold_ping_critical_ms" => {
                    ping_critical = setting.value_type.parse(&setting.value).unwrap_or(ping_critical);
                }
                "threshold_latency_warning_ms" => {
                    latency_warning = setting.value_type.parse(&setting.value).unwrap_or(latency_warning);
                }
                "threshold_latency_critical_ms" => {
                    latency_critical = setting.value_type.parse(&setting.value).unwrap_or(latency_critical);
                }
                "threshold_packet_loss_warning_percent" => {
                    packet_loss_warning = setting.value_type.parse(&setting.value).unwrap_or(packet_loss_warning);
                }
                "threshold_packet_loss_critical_percent" => {
                    packet_loss_critical = setting.value_type.parse(&setting.value).unwrap_or(packet_loss_critical);
                }
                _ => {}
            }
        }

        Ok(domain::ThresholdSettings {
            ping_ms: domain::ThresholdValue::new(ping_warning, ping_critical)
                .map_err(|e| DomainError::InvalidSettingValue(e.to_string()))?,
            latency_ms: domain::ThresholdValue::new(latency_warning, latency_critical)
                .map_err(|e| DomainError::InvalidSettingValue(e.to_string()))?,
            packet_loss_percent: domain::ThresholdValue::new(packet_loss_warning, packet_loss_critical)
                .map_err(|e| DomainError::InvalidSettingValue(e.to_string()))?,
        })
    }
}
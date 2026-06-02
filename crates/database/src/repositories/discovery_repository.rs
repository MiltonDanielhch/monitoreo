// crates/database/src/repositories/discovery_repository.rs
// Repositorio de dispositivos descubiertos y escaneos de red con Sea-ORM
// Vinculado con ADR-0004 (Persistencia con Sea-ORM)

use crate::entities::discovered_device_entity;
use crate::entities::discovered_device_entity::Entity as DiscoveredDeviceEntity;
use crate::entities::discovered_device_entity::Column as DiscoveredDeviceColumn;
use crate::entities::network_scan_entity;
use crate::entities::network_scan_entity::Entity as NetworkScanEntity;
use crate::entities::network_scan_entity::Column as NetworkScanColumn;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Order, QueryFilter,
    QueryOrder, Set,
};
use domain::models::discovery::{
    DeviceStatus, DeviceType, DiscoveredDevice, DiscoveryFilters, DiscoveryPort, NetworkScan,
    ScanFilters, ScanStatus,
};
use domain::errors::DomainError;
use std::sync::Arc;

pub struct DiscoveryRepository {
    db: Arc<DatabaseConnection>,
}

impl DiscoveryRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    /// Convertir modelo de dominio DiscoveredDevice a entidad de base de datos
    fn device_domain_to_entity(device: &DiscoveredDevice) -> discovered_device_entity::ActiveModel {
        let open_ports_json =
            serde_json::to_string(&device.open_ports).unwrap_or_else(|_| "[]".to_string());
        let services_json =
            serde_json::to_string(&device.services).unwrap_or_else(|_| "[]".to_string());
        let metadata_json = device
            .metadata
            .as_ref()
            .map(|m| serde_json::to_string(m).unwrap_or_default());

        discovered_device_entity::ActiveModel {
            id: Set(device.id.clone()),
            ip_address: Set(device.ip_address.clone()),
            mac_address: Set(device.mac_address.clone()),
            hostname: Set(device.hostname.clone()),
            device_type: Set(device.device_type.to_string()),
            os_fingerprint: Set(device.os_fingerprint.clone()),
            manufacturer: Set(device.manufacturer.clone()),
            open_ports: Set(Some(open_ports_json)),
            services: Set(Some(services_json)),
            status: Set(device.status.to_string()),
            is_authorized: Set(device.is_authorized),
            last_seen: Set(
                chrono::DateTime::parse_from_rfc3339(&device.last_seen)
                    .unwrap_or_else(|_| chrono::Utc::now().into())
                    .naive_utc(),
            ),
            first_seen: Set(
                chrono::DateTime::parse_from_rfc3339(&device.first_seen)
                    .unwrap_or_else(|_| chrono::Utc::now().into())
                    .naive_utc(),
            ),
            scan_id: Set(Some(device.scan_id.clone())),
            sede_id: Set(device.sede_id.clone()),
            metadata: Set(metadata_json),
            created_at: Set(chrono::Utc::now().naive_utc()),
            updated_at: Set(chrono::Utc::now().naive_utc()),
        }
    }

    /// Convertir entidad de base de datos a modelo de dominio DiscoveredDevice
    fn device_entity_to_domain(
        entity: &discovered_device_entity::Model,
    ) -> Result<DiscoveredDevice, DomainError> {
        let open_ports: Vec<u16> = entity
            .open_ports
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();

        let services: Vec<String> = entity
            .services
            .as_ref()
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();

        let metadata = entity
            .metadata
            .as_ref()
            .and_then(|m| serde_json::from_str(m).ok());

        let last_seen_dt: chrono::DateTime<chrono::Utc> =
            chrono::DateTime::from_naive_utc_and_offset(entity.last_seen, chrono::Utc);
        let first_seen_dt: chrono::DateTime<chrono::Utc> =
            chrono::DateTime::from_naive_utc_and_offset(entity.first_seen, chrono::Utc);

        Ok(DiscoveredDevice {
            id: entity.id.clone(),
            ip_address: entity.ip_address.clone(),
            mac_address: entity.mac_address.clone(),
            hostname: entity.hostname.clone(),
            device_type: DeviceType::from(entity.device_type.clone()),
            os_fingerprint: entity.os_fingerprint.clone(),
            manufacturer: entity.manufacturer.clone(),
            open_ports,
            services,
            status: DeviceStatus::from(entity.status.clone()),
            is_authorized: entity.is_authorized,
            last_seen: last_seen_dt.to_rfc3339(),
            first_seen: first_seen_dt.to_rfc3339(),
            scan_id: entity.scan_id.clone().unwrap_or_default(),
            sede_id: entity.sede_id.clone(),
            metadata,
        })
    }

    /// Convertir modelo de dominio NetworkScan a entidad de base de datos
    fn scan_domain_to_entity(scan: &NetworkScan) -> network_scan_entity::ActiveModel {
        network_scan_entity::ActiveModel {
            id: Set(scan.id.clone()),
            scan_type: Set(scan.scan_type.clone()),
            ip_range: Set(scan.ip_range.clone()),
            status: Set(scan.status.to_string()),
            devices_found: Set(scan.devices_found),
            started_at: Set(
                chrono::DateTime::parse_from_rfc3339(&scan.started_at)
                    .unwrap_or_else(|_| chrono::Utc::now().into())
                    .naive_utc(),
            ),
            completed_at: Set(scan.completed_at.as_ref().map(|c| {
                chrono::DateTime::parse_from_rfc3339(c)
                    .unwrap_or_else(|_| chrono::Utc::now().into())
                    .naive_utc()
            })),
            duration_seconds: Set(scan.duration_seconds),
            sede_id: Set(scan.sede_id.clone()),
            created_by: Set(scan.created_by.clone()),
            created_at: Set(chrono::Utc::now().naive_utc()),
        }
    }

    /// Convertir entidad de base de datos a modelo de dominio NetworkScan
    fn scan_entity_to_domain(
        entity: &network_scan_entity::Model,
    ) -> Result<NetworkScan, DomainError> {
        let started_at_dt: chrono::DateTime<chrono::Utc> =
            chrono::DateTime::from_naive_utc_and_offset(entity.started_at, chrono::Utc);

        let completed_at = entity.completed_at.as_ref().map(|c| {
            let dt: chrono::DateTime<chrono::Utc> =
                chrono::DateTime::from_naive_utc_and_offset(*c, chrono::Utc);
            dt.to_rfc3339()
        });

        Ok(NetworkScan {
            id: entity.id.clone(),
            scan_type: entity.scan_type.clone(),
            ip_range: entity.ip_range.clone(),
            status: ScanStatus::from(entity.status.clone()),
            devices_found: entity.devices_found,
            started_at: started_at_dt.to_rfc3339(),
            completed_at,
            duration_seconds: entity.duration_seconds,
            sede_id: entity.sede_id.clone(),
            created_by: entity.created_by.clone(),
        })
    }
}

#[async_trait::async_trait]
impl DiscoveryPort for DiscoveryRepository {
    async fn log_device(&self, device: DiscoveredDevice) -> Result<(), DomainError> {
        let active_model = Self::device_domain_to_entity(&device);
        DiscoveredDeviceEntity::insert(active_model)
            .exec(&*self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error al insertar dispositivo: {}", e)))?;
        Ok(())
    }

    async fn get_devices(
        &self,
        filters: DiscoveryFilters,
    ) -> Result<Vec<DiscoveredDevice>, DomainError> {
        let mut query = DiscoveredDeviceEntity::find();

        if let Some(device_type) = &filters.device_type {
            query = query.filter(DiscoveredDeviceColumn::DeviceType.eq(device_type.to_string()));
        }
        if let Some(status) = &filters.status {
            query = query.filter(DiscoveredDeviceColumn::Status.eq(status.to_string()));
        }
        if let Some(is_authorized) = filters.is_authorized {
            query = query.filter(DiscoveredDeviceColumn::IsAuthorized.eq(is_authorized));
        }
        if let Some(sede_id) = &filters.sede_id {
            query = query.filter(DiscoveredDeviceColumn::SedeId.eq(sede_id.clone()));
        }
        if let Some(manufacturer) = &filters.manufacturer {
            query =
                query.filter(DiscoveredDeviceColumn::Manufacturer.eq(manufacturer.clone()));
        }

        query = query.order_by(DiscoveredDeviceColumn::LastSeen, Order::Desc);

        let entities = query
            .all(&*self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error al consultar dispositivos: {}", e)))?;

        entities
            .iter()
            .map(Self::device_entity_to_domain)
            .collect()
    }

    async fn get_device_by_ip(
        &self,
        ip: String,
    ) -> Result<Option<DiscoveredDevice>, DomainError> {
        let entity = DiscoveredDeviceEntity::find()
            .filter(DiscoveredDeviceColumn::IpAddress.eq(ip))
            .one(&*self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error al buscar dispositivo por IP: {}", e)))?;

        match entity {
            Some(e) => Ok(Some(Self::device_entity_to_domain(&e)?)),
            None => Ok(None),
        }
    }

    async fn get_device_by_mac(
        &self,
        mac: String,
    ) -> Result<Option<DiscoveredDevice>, DomainError> {
        let entity = DiscoveredDeviceEntity::find()
            .filter(DiscoveredDeviceColumn::MacAddress.eq(mac))
            .one(&*self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error al buscar dispositivo por MAC: {}", e)))?;

        match entity {
            Some(e) => Ok(Some(Self::device_entity_to_domain(&e)?)),
            None => Ok(None),
        }
    }

    async fn update_device_status(&self, id: String, status: DeviceStatus) -> Result<(), DomainError> {
        let device = DiscoveredDeviceEntity::find()
            .filter(DiscoveredDeviceColumn::Id.eq(id.clone()))
            .one(&*self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error al buscar dispositivo: {}", e)))?;

        match device {
            Some(d) => {
                let mut active_model: discovered_device_entity::ActiveModel = d.into();
                active_model.status = Set(status.to_string());
                active_model.last_seen = Set(chrono::Utc::now().naive_utc());
                active_model.updated_at = Set(chrono::Utc::now().naive_utc());
                active_model
                    .update(&*self.db)
                    .await
                    .map_err(|e| DomainError::Infrastructure(format!("Error al actualizar dispositivo: {}", e)))?;
                Ok(())
            }
            None => Err(DomainError::DeviceNotFound(id)),
        }
    }

    async fn mark_device_authorized(&self, id: String) -> Result<(), DomainError> {
        let device = DiscoveredDeviceEntity::find()
            .filter(DiscoveredDeviceColumn::Id.eq(id.clone()))
            .one(&*self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error al buscar dispositivo: {}", e)))?;

        match device {
            Some(d) => {
                let mut active_model: discovered_device_entity::ActiveModel = d.into();
                active_model.is_authorized = Set(true);
                active_model.updated_at = Set(chrono::Utc::now().naive_utc());
                active_model
                    .update(&*self.db)
                    .await
                    .map_err(|e| DomainError::Infrastructure(format!("Error al autorizar dispositivo: {}", e)))?;
                Ok(())
            }
            None => Err(DomainError::DeviceNotFound(id)),
        }
    }

    async fn mark_device_unauthorized(&self, id: String) -> Result<(), DomainError> {
        let device = DiscoveredDeviceEntity::find()
            .filter(DiscoveredDeviceColumn::Id.eq(id.clone()))
            .one(&*self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error al buscar dispositivo: {}", e)))?;

        match device {
            Some(d) => {
                let mut active_model: discovered_device_entity::ActiveModel = d.into();
                active_model.is_authorized = Set(false);
                active_model.updated_at = Set(chrono::Utc::now().naive_utc());
                active_model
                    .update(&*self.db)
                    .await
                    .map_err(|e| DomainError::Infrastructure(format!("Error al desautorizar dispositivo: {}", e)))?;
                Ok(())
            }
            None => Err(DomainError::DeviceNotFound(id)),
        }
    }

    async fn create_scan(&self, scan: NetworkScan) -> Result<(), DomainError> {
        let active_model = Self::scan_domain_to_entity(&scan);
        NetworkScanEntity::insert(active_model)
            .exec(&*self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error al crear escaneo: {}", e)))?;
        Ok(())
    }

    async fn get_scan(&self, id: String) -> Result<Option<NetworkScan>, DomainError> {
        let entity = NetworkScanEntity::find()
            .filter(NetworkScanColumn::Id.eq(id))
            .one(&*self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error al buscar escaneo: {}", e)))?;

        match entity {
            Some(e) => Ok(Some(Self::scan_entity_to_domain(&e)?)),
            None => Ok(None),
        }
    }

    async fn get_scans(&self, filters: ScanFilters) -> Result<Vec<NetworkScan>, DomainError> {
        let mut query = NetworkScanEntity::find();

        if let Some(status) = &filters.status {
            query = query.filter(NetworkScanColumn::Status.eq(status.to_string()));
        }
        if let Some(sede_id) = &filters.sede_id {
            query = query.filter(NetworkScanColumn::SedeId.eq(sede_id.clone()));
        }
        if let Some(scan_type) = &filters.scan_type {
            query = query.filter(NetworkScanColumn::ScanType.eq(scan_type.clone()));
        }

        query = query.order_by(NetworkScanColumn::StartedAt, Order::Desc);

        let entities = query
            .all(&*self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error al consultar escaneos: {}", e)))?;

        entities.iter().map(Self::scan_entity_to_domain).collect()
    }

    async fn update_scan_status(&self, id: String, status: ScanStatus) -> Result<(), DomainError> {
        let scan = NetworkScanEntity::find()
            .filter(NetworkScanColumn::Id.eq(id.clone()))
            .one(&*self.db)
            .await
            .map_err(|e| DomainError::Infrastructure(format!("Error al buscar escaneo: {}", e)))?;

        match scan {
            Some(s) => {
                let mut active_model: network_scan_entity::ActiveModel = s.into();
                active_model.status = Set(status.to_string());
                active_model.completed_at = Set(Some(chrono::Utc::now().naive_utc()));
                active_model
                    .update(&*self.db)
                    .await
                    .map_err(|e| DomainError::Infrastructure(format!("Error al actualizar escaneo: {}", e)))?;
                Ok(())
            }
            None => Err(DomainError::ScanNotFound(id)),
        }
    }
}

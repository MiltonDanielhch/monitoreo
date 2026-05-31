// crates/database/src/repositories/network_file_repository.rs
// Repositorio para archivos de infraestructura de red
// Vinculado con ADR-0004-persistencia-sea-orm.md

use crate::entities::network_file_entity;
use domain::models::infrastructure_file::{InfrastructureFile, NetworkFileType};
use domain::errors::Result as DomainResult;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

pub struct NetworkFileRepository {
    db: DatabaseConnection,
}

impl NetworkFileRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Inserta un nuevo archivo de infraestructura
    pub async fn insert_file(&self, file: &InfrastructureFile) -> DomainResult<String> {
        let now = chrono::Utc::now().naive_utc();
        let new_file = network_file_entity::ActiveModel {
            id: Set(file.id.clone()),
            filename: Set(file.filename.clone()),
            file_type: Set(file.file_type.to_string()),
            file_size_bytes: Set(file.file_size_bytes as i64),
            storage_key: Set(file.storage_key.clone()),
            sha256_checksum: Set(file.sha256_checksum.clone()),
            sede_id: Set(file.sede_id.clone()),
            user_id: Set(file.user_id.clone()),
            created_at: Set(now),
            deleted_at: Set(None),
        };

        let result = new_file.insert(&self.db).await.map_err(|e| {
            domain::errors::DomainError::Infrastructure(format!("Error inserting file: {}", e))
        })?;

        Ok(result.id)
    }

    /// Obtiene el último respaldo de configuración por dispositivo
    pub async fn get_latest_backup_by_device(
        &self,
        device_id: &str,
    ) -> DomainResult<Option<InfrastructureFile>> {
        let result = network_file_entity::Entity::find()
            .filter(network_file_entity::Column::FileType.eq("CONFIG_BACKUP"))
            .filter(network_file_entity::Column::DeletedAt.is_null())
            .filter(network_file_entity::Column::StorageKey.contains(device_id))
            .order_by_desc(network_file_entity::Column::CreatedAt)
            .one(&self.db)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!(
                    "Error fetching backup: {}",
                    e
                ))
            })?;

        Ok(result.map(|model| self.model_to_domain(model)))
    }

    /// Obtiene todos los archivos activos agrupados por sede
    pub async fn get_files_by_sede(&self, sede_id: &str) -> DomainResult<Vec<InfrastructureFile>> {
        let results = network_file_entity::Entity::find()
            .filter(network_file_entity::Column::SedeId.eq(sede_id))
            .filter(network_file_entity::Column::DeletedAt.is_null())
            .order_by_desc(network_file_entity::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!(
                    "Error fetching files by sede: {}",
                    e
                ))
            })?;

        Ok(results.into_iter().map(|m| self.model_to_domain(m)).collect())
    }

    /// Obtiene archivos por tipo
    pub async fn get_files_by_type(
        &self,
        file_type: NetworkFileType,
    ) -> DomainResult<Vec<InfrastructureFile>> {
        let results = network_file_entity::Entity::find()
            .filter(network_file_entity::Column::FileType.eq(file_type.to_string()))
            .filter(network_file_entity::Column::DeletedAt.is_null())
            .order_by_desc(network_file_entity::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!(
                    "Error fetching files by type: {}",
                    e
                ))
            })?;

        Ok(results.into_iter().map(|m| self.model_to_domain(m)).collect())
    }

    /// Verifica si existe un archivo con el mismo checksum (deduplicación)
    pub async fn file_exists_by_checksum(&self, checksum: &str) -> DomainResult<bool> {
        let count = network_file_entity::Entity::find()
            .filter(network_file_entity::Column::Sha256Checksum.eq(checksum))
            .filter(network_file_entity::Column::DeletedAt.is_null())
            .count(&self.db)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!(
                    "Error checking checksum: {}",
                    e
                ))
            })?;

        Ok(count > 0)
    }

    /// Soft delete de un archivo
    pub async fn soft_delete_file(&self, file_id: &str) -> DomainResult<()> {
        let file = network_file_entity::Entity::find_by_id(file_id.to_string())
            .one(&self.db)
            .await
            .map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!(
                    "Error finding file: {}",
                    e
                ))
            })?;

        if let Some(model) = file {
            let mut active_model: network_file_entity::ActiveModel = model.into();
            active_model.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
            active_model.update(&self.db).await.map_err(|e| {
                domain::errors::DomainError::Infrastructure(format!(
                    "Error soft deleting file: {}",
                    e
                ))
            })?;
        }

        Ok(())
    }

    /// Convierte un modelo de Sea-ORM a entidad de dominio
    fn model_to_domain(&self, model: network_file_entity::Model) -> InfrastructureFile {
        let file_type = match model.file_type.as_str() {
            "TOPOLOGY_SVG" => NetworkFileType::TopologySvg,
            "RACK_IMAGE" => NetworkFileType::RackImage,
            "CONFIG_BACKUP" => NetworkFileType::ConfigBackup,
            _ => NetworkFileType::ConfigBackup, // fallback
        };

        InfrastructureFile {
            id: model.id,
            filename: model.filename,
            file_type,
            file_size_bytes: model.file_size_bytes as u64,
            storage_key: model.storage_key,
            sha256_checksum: model.sha256_checksum,
            sede_id: model.sede_id,
            user_id: model.user_id,
        }
    }
}

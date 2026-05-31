// crates/infrastructure/src/storage/regional_storage.rs
// Adaptador de almacenamiento regional local para archivos de infraestructura
// Vinculado con ADR-0012-adaptador-almacenamiento.md

use async_trait::async_trait;
use domain::errors::Result as DomainResult;
use domain::models::infrastructure_file::{InfrastructureFile, NetworkStoragePort};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Adaptador de almacenamiento regional local
/// Organiza archivos por sede y tipo en una estructura jerárquica
pub struct RegionalStorageAdapter {
    base_path: PathBuf,
}

impl RegionalStorageAdapter {
    /// Crea un nuevo adaptador de almacenamiento regional
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    /// Genera la ruta de almacenamiento para un archivo
    fn generate_storage_path(&self, file: &InfrastructureFile) -> PathBuf {
        let file_type_dir = match file.file_type {
            domain::models::infrastructure_file::NetworkFileType::TopologySvg => "topology",
            domain::models::infrastructure_file::NetworkFileType::RackImage => "racks",
            domain::models::infrastructure_file::NetworkFileType::ConfigBackup => "backups",
        };

        self.base_path
            .join("sedes")
            .join(&file.sede_id)
            .join(file_type_dir)
            .join(&file.filename)
    }

    /// Genera la ruta del directorio para una sede y tipo de archivo
    fn generate_directory_path(&self, sede_id: &str, file_type: &str) -> PathBuf {
        let file_type_dir = match file_type {
            "TOPOLOGY_SVG" => "topology",
            "RACK_IMAGE" => "racks",
            "CONFIG_BACKUP" => "backups",
            _ => "other",
        };

        self.base_path
            .join("sedes")
            .join(sede_id)
            .join(file_type_dir)
    }

    /// Asegura que el directorio existe
    async fn ensure_directory_exists(&self, path: &PathBuf) -> DomainResult<()> {
        if !path.exists() {
            fs::create_dir_all(path).await.map_err(|e| {
                domain::errors::DomainError::FileStorageError(format!(
                    "Error creating directory: {}",
                    e
                ))
            })?;
        }
        Ok(())
    }

    /// Calcula el hash SHA-256 de un contenido
    pub fn calculate_checksum(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        hex::encode(hasher.finalize())
    }
}

#[async_trait]
impl NetworkStoragePort for RegionalStorageAdapter {
    /// Guarda un archivo en el almacenamiento
    async fn save_file(&self, file: &InfrastructureFile, content: &[u8]) -> DomainResult<String> {
        let storage_path = self.generate_storage_path(file);
        let directory_path = storage_path.parent().ok_or_else(|| {
            domain::errors::DomainError::FileStorageError(
                "Invalid storage path".to_string(),
            )
        })?;

        // Asegurar que el directorio existe
        self.ensure_directory_exists(&directory_path.to_path_buf()).await?;

        // Escribir el archivo
        let mut file_handle = fs::File::create(&storage_path).await.map_err(|e| {
            domain::errors::DomainError::FileStorageError(format!(
                "Error creating file: {}",
                e
            ))
        })?;

        file_handle.write_all(content).await.map_err(|e| {
            domain::errors::DomainError::FileStorageError(format!(
                "Error writing file: {}",
                e
            ))
        })?;

        Ok(storage_path.to_string_lossy().to_string())
    }

    /// Recupera un archivo del almacenamiento
    async fn get_file(&self, storage_key: &str) -> DomainResult<Vec<u8>> {
        let path = PathBuf::from(storage_key);

        if !path.exists() {
            return Err(domain::errors::DomainError::FileStorageError(
                "File not found".to_string(),
            ));
        }

        let content = fs::read(&path).await.map_err(|e| {
            domain::errors::DomainError::FileStorageError(format!(
                "Error reading file: {}",
                e
            ))
        })?;

        Ok(content)
    }

    /// Elimina un archivo del almacenamiento
    async fn delete_file(&self, storage_key: &str) -> DomainResult<()> {
        let path = PathBuf::from(storage_key);

        if path.exists() {
            fs::remove_file(&path).await.map_err(|e| {
                domain::errors::DomainError::FileStorageError(format!(
                    "Error deleting file: {}",
                    e
                ))
            })?;
        }

        Ok(())
    }

    /// Verifica si un archivo existe
    async fn file_exists(&self, storage_key: &str) -> DomainResult<bool> {
        let path = PathBuf::from(storage_key);
        Ok(path.exists())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_checksum() {
        let content = b"test content";
        let checksum = RegionalStorageAdapter::calculate_checksum(content);
        assert_eq!(checksum.len(), 64); // SHA-256 produces 64 hex characters
    }

    #[test]
    fn test_generate_storage_path() {
        let adapter = RegionalStorageAdapter::new(PathBuf::from("/var/lib/redes/storage"));
        let file = InfrastructureFile::new(
            "test-id".to_string(),
            "test.svg".to_string(),
            domain::models::infrastructure_file::NetworkFileType::TopologySvg,
            1024,
            "".to_string(),
            "".to_string(),
            "trinidad".to_string(),
            None,
        );

        let path = adapter.generate_storage_path(&file);
        assert!(path.to_string_lossy().contains("trinidad"));
        assert!(path.to_string_lossy().contains("topology"));
    }
}

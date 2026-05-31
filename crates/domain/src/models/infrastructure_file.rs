// crates/domain/src/models/infrastructure_file.rs
// Modelos de dominio para archivos de infraestructura de red
// Vinculado con ADR-0001-arquitectura-limpia.md y ADR-0008-validacion-dominio.md

use crate::errors::{DomainError, Result};
use std::fmt;

/// Tipos de archivos de infraestructura de red permitidos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NetworkFileType {
    /// Diagrama SVG de topología de red
    TopologySvg,
    /// Imagen de rack de servidores (PNG/JPG)
    RackImage,
    /// Respaldo de configuración de dispositivo (CFG/TXT)
    ConfigBackup,
}

impl fmt::Display for NetworkFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkFileType::TopologySvg => write!(f, "TOPOLOGY_SVG"),
            NetworkFileType::RackImage => write!(f, "RACK_IMAGE"),
            NetworkFileType::ConfigBackup => write!(f, "CONFIG_BACKUP"),
        }
    }
}

impl TryFrom<&str> for NetworkFileType {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self> {
        match value.to_uppercase().as_str() {
            "TOPOLOGY_SVG" => Ok(NetworkFileType::TopologySvg),
            "RACK_IMAGE" => Ok(NetworkFileType::RackImage),
            "CONFIG_BACKUP" => Ok(NetworkFileType::ConfigBackup),
            _ => Err(DomainError::UnsupportedNetworkFormat),
        }
    }
}

/// Validador de contenido de archivos de infraestructura
pub struct FileValidator;

impl FileValidator {
    /// Valida que un archivo SVG tenga estructura XML válida
    pub fn validate_svg(content: &[u8]) -> Result<()> {
        let content_str = std::str::from_utf8(content)
            .map_err(|_| DomainError::CorruptedBackup)?;

        // Verificar que sea un SVG válido (contiene <svg> y </svg>)
        if !content_str.contains("<svg") || !content_str.contains("</svg>") {
            return Err(DomainError::CorruptedBackup);
        }

        // Verificar que no contenga scripts maliciosos
        if content_str.contains("<script") {
            return Err(DomainError::CorruptedBackup);
        }

        Ok(())
    }

    /// Valida que un archivo de configuración sea texto plano y no exceda el tamaño
    pub fn validate_config_backup(content: &[u8], max_size_bytes: usize) -> Result<()> {
        // Verificar tamaño máximo
        if content.len() > max_size_bytes {
            return Err(DomainError::CorruptedBackup);
        }

        // Verificar que sea texto plano (sin caracteres nulos)
        if content.contains(&0) {
            return Err(DomainError::CorruptedBackup);
        }

        Ok(())
    }

    /// Valida que una imagen tenga extensión permitida
    pub fn validate_image_extension(filename: &str) -> Result<()> {
        let lower = filename.to_lowercase();
        if !lower.ends_with(".png") && !lower.ends_with(".jpg") && !lower.ends_with(".jpeg") {
            return Err(DomainError::UnsupportedNetworkFormat);
        }
        Ok(())
    }

    /// Valida que un archivo de configuración tenga extensión permitida
    pub fn validate_config_extension(filename: &str) -> Result<()> {
        let lower = filename.to_lowercase();
        if !lower.ends_with(".cfg") && !lower.ends_with(".txt") {
            return Err(DomainError::UnsupportedNetworkFormat);
        }
        Ok(())
    }
}

/// Entidad de dominio para archivo de infraestructura
#[derive(Debug, Clone)]
pub struct InfrastructureFile {
    pub id: String,
    pub filename: String,
    pub file_type: NetworkFileType,
    pub file_size_bytes: u64,
    pub storage_key: String,
    pub sha256_checksum: String,
    pub sede_id: String,
    pub user_id: Option<String>,
}

impl InfrastructureFile {
    /// Crea un nuevo archivo de infraestructura
    pub fn new(
        id: String,
        filename: String,
        file_type: NetworkFileType,
        file_size_bytes: u64,
        storage_key: String,
        sha256_checksum: String,
        sede_id: String,
        user_id: Option<String>,
    ) -> Self {
        Self {
            id,
            filename,
            file_type,
            file_size_bytes,
            storage_key,
            sha256_checksum,
            sede_id,
            user_id,
        }
    }

    /// Valida el contenido del archivo según su tipo
    pub fn validate_content(&self, content: &[u8]) -> Result<()> {
        match self.file_type {
            NetworkFileType::TopologySvg => FileValidator::validate_svg(content),
            NetworkFileType::ConfigBackup => {
                FileValidator::validate_config_backup(content, 2 * 1024 * 1024) // 2MB max
            }
            NetworkFileType::RackImage => {
                // Las imágenes se validan por extensión en el upload
                FileValidator::validate_image_extension(&self.filename)
            }
        }
    }
}

/// Trait para puerto de almacenamiento de archivos de red
#[async_trait::async_trait]
pub trait NetworkStoragePort: Send + Sync {
    /// Guarda un archivo en el almacenamiento
    async fn save_file(
        &self,
        file: &InfrastructureFile,
        content: &[u8],
    ) -> Result<String>;

    /// Recupera un archivo del almacenamiento
    async fn get_file(&self, storage_key: &str) -> Result<Vec<u8>>;

    /// Elimina un archivo del almacenamiento
    async fn delete_file(&self, storage_key: &str) -> Result<()>;

    /// Verifica si un archivo existe
    async fn file_exists(&self, storage_key: &str) -> Result<bool>;
}

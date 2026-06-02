// crates/domain/src/errors.rs
// Errores tipificados del dominio - Árbol de fallos con thiserror
// Vinculado con ADR-0007-manejo-errores.md

use thiserror::Error;

/// Errores del dominio de autenticación
/// Vinculado con ADR-0007-manejo-errores.md
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Formato de correo electrónico inválido")]
    InvalidEmail,

    #[error("La contraseña no cumple con los requisitos de fortaleza mínima")]
    WeakPassword,

    #[error("Credenciales inválidas")]
    InvalidCredentials,

    #[error("Usuario suspendido")]
    UserSuspended,

    #[error("Sesión expirada")]
    SessionExpired,

    #[error("Detectado ataque por reutilización de token")]
    ReusedTokenAttack,

    #[error("Límite de velocidad excedido")]
    RateLimitExceeded,

    #[error("Valor de configuración inválido: {0}")]
    InvalidSettingValue(String),

    #[error("Error de infraestructura: {0}")]
    Infrastructure(String),

    // Errores del dominio de notificaciones - Módulo 4
    #[error("Plantilla de notificación no encontrada")]
    TemplateNotFound,

    #[error("Destinatario inválido: {0}")]
    InvalidRecipient(String),

    #[error("Excedido el límite de reintentos para notificación")]
    MaxRetriesExceeded,

    #[error("Canal de notificación no disponible")]
    ChannelUnavailable,

    #[error("Error al renderizar plantilla: {0}")]
    TemplateRenderError(String),

    // Errores del dominio de infraestructura - Módulo 5
    #[error("Formato de archivo no soportado para infraestructura de red")]
    UnsupportedNetworkFormat,

    #[error("Archivo de respaldo corrupto o inválido")]
    CorruptedBackup,

    #[error("Sede no encontrada: {0}")]
    SedeNotFound(String),

    #[error("Error de almacenamiento de archivos: {0}")]
    FileStorageError(String),

    // Errores del dominio de descubrimiento de red - Módulo 12
    #[error("Dispositivo no encontrado: {0}")]
    DeviceNotFound(String),

    #[error("Escaneo no encontrado: {0}")]
    ScanNotFound(String),

    #[error("Rango de IP inválido: {0}")]
    InvalidIpRange(String),

    #[error("Fabricante OUI no encontrado para MAC: {0}")]
    OuiNotFound(String),

    #[error("Clasificación de dispositivo fallida: {0}")]
    ClassificationError(String),
}

pub type Result<T> = std::result::Result<T, DomainError>;

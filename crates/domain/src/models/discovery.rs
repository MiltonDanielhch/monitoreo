// crates/domain/src/models/discovery.rs
// Modelos de dominio para descubrimiento de red y asset discovery
// Vinculado con ADR-0001 (Dominio Puro)
// El dominio no sabe de protocolos de red o sockets - recibe estructuras puras

use serde::{Deserialize, Serialize};
use std::fmt;

use crate::errors::DomainError;

/// Tipos de dispositivos descubiertos en la red
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    Router,
    Switch,
    Server,
    PC,
    Mobile,
    IoT,
    Printer,
    Unknown,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::Router => write!(f, "router"),
            DeviceType::Switch => write!(f, "switch"),
            DeviceType::Server => write!(f, "server"),
            DeviceType::PC => write!(f, "pc"),
            DeviceType::Mobile => write!(f, "mobile"),
            DeviceType::IoT => write!(f, "iot"),
            DeviceType::Printer => write!(f, "printer"),
            DeviceType::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<String> for DeviceType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "router" => DeviceType::Router,
            "switch" => DeviceType::Switch,
            "server" => DeviceType::Server,
            "pc" => DeviceType::PC,
            "mobile" => DeviceType::Mobile,
            "iot" => DeviceType::IoT,
            "printer" => DeviceType::Printer,
            _ => DeviceType::Unknown,
        }
    }
}

/// Estados de un escaneo de red
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScanStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl fmt::Display for ScanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanStatus::Pending => write!(f, "pending"),
            ScanStatus::Running => write!(f, "running"),
            ScanStatus::Completed => write!(f, "completed"),
            ScanStatus::Failed => write!(f, "failed"),
        }
    }
}

impl From<String> for ScanStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "pending" => ScanStatus::Pending,
            "running" => ScanStatus::Running,
            "completed" => ScanStatus::Completed,
            "failed" => ScanStatus::Failed,
            _ => ScanStatus::Pending,
        }
    }
}

/// Estados de un dispositivo descubierto
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceStatus {
    Online,
    Offline,
    Unknown,
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceStatus::Online => write!(f, "online"),
            DeviceStatus::Offline => write!(f, "offline"),
            DeviceStatus::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<String> for DeviceStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "online" => DeviceStatus::Online,
            "offline" => DeviceStatus::Offline,
            _ => DeviceStatus::Unknown,
        }
    }
}

/// Dispositivo descubierto en la red
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredDevice {
    pub id: String,
    pub ip_address: String,
    pub mac_address: Option<String>,
    pub hostname: Option<String>,
    pub device_type: DeviceType,
    pub os_fingerprint: Option<String>,
    pub manufacturer: Option<String>,
    pub open_ports: Vec<u16>,
    pub services: Vec<String>,
    pub status: DeviceStatus,
    pub is_authorized: bool,
    pub last_seen: String, // RFC3339
    pub first_seen: String, // RFC3339
    pub scan_id: String,
    pub sede_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl DiscoveredDevice {
    /// Constructor que genera ID único
    pub fn new(ip_address: String, scan_id: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: format!("dev_{}", chrono::Utc::now().timestamp_millis()),
            ip_address,
            mac_address: None,
            hostname: None,
            device_type: DeviceType::Unknown,
            os_fingerprint: None,
            manufacturer: None,
            open_ports: Vec::new(),
            services: Vec::new(),
            status: DeviceStatus::Unknown,
            is_authorized: false,
            last_seen: now.clone(),
            first_seen: now,
            scan_id,
            sede_id: None,
            metadata: None,
        }
    }

    /// Valida formato de IP (IPv4 o IPv6)
    pub fn validate_ip(ip: &str) -> Result<(), String> {
        if ip.contains('.') {
            // IPv4
            let parts: Vec<&str> = ip.split('.').collect();
            if parts.len() != 4 {
                return Err("Formato IPv4 inválido".to_string());
            }
            for part in parts {
                let num: u8 = part.parse().map_err(|_| "Formato IPv4 inválido".to_string())?;
                if num > 255 {
                    return Err("Valor de octeto IPv4 fuera de rango".to_string());
                }
            }
        } else if ip.contains(':') {
            // IPv6 - validación básica
            let parts: Vec<&str> = ip.split(':').collect();
            if parts.len() != 8 {
                return Err("Formato IPv6 inválido".to_string());
            }
            for part in parts {
                if part.len() != 4 {
                    if part.is_empty() {
                        continue; // :: compression
                    }
                    return Err("Formato IPv6 inválido".to_string());
                }
                u16::from_str_radix(part, 16).map_err(|_| "Formato IPv6 inválido".to_string())?;
            }
        } else {
            return Err("Formato de IP inválido".to_string());
        }
        Ok(())
    }

    /// Valida formato de MAC address (formato XX:XX:XX:XX:XX:XX)
    pub fn validate_mac(mac: &str) -> Result<(), String> {
        let parts: Vec<&str> = mac.split(':').collect();
        if parts.len() != 6 {
            return Err("Formato MAC inválido: debe tener 6 octetos".to_string());
        }
        for part in parts {
            if part.len() != 2 {
                return Err("Formato MAC inválido: cada octeto debe tener 2 caracteres".to_string());
            }
            u8::from_str_radix(part, 16).map_err(|_| "Formato MAC inválido: octeto no es hex válido".to_string())?;
        }
        Ok(())
    }

    /// Retorna true si el dispositivo está en línea
    pub fn is_online(&self) -> bool {
        self.status == DeviceStatus::Online
    }

    /// Marca el dispositivo como fuera de línea
    pub fn mark_as_offline(&mut self) {
        self.status = DeviceStatus::Offline;
        self.last_seen = chrono::Utc::now().to_rfc3339();
    }

    /// Marca el dispositivo como autorizado
    pub fn mark_as_authorized(&mut self) {
        self.is_authorized = true;
    }

    /// Marca el dispositivo como no autorizado
    pub fn mark_as_unauthorized(&mut self) {
        self.is_authorized = false;
    }
}

/// Registro de un escaneo de red
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkScan {
    pub id: String,
    pub scan_type: String,
    pub ip_range: String,
    pub status: ScanStatus,
    pub devices_found: i32,
    pub started_at: String, // RFC3339
    pub completed_at: Option<String>, // RFC3339
    pub duration_seconds: Option<i32>,
    pub sede_id: Option<String>,
    pub created_by: String,
}

impl NetworkScan {
    /// Constructor que genera ID único
    pub fn new(scan_type: String, ip_range: String, created_by: String) -> Self {
        Self {
            id: format!("scan_{}", chrono::Utc::now().timestamp_millis()),
            scan_type,
            ip_range,
            status: ScanStatus::Pending,
            devices_found: 0,
            started_at: chrono::Utc::now().to_rfc3339(),
            completed_at: None,
            duration_seconds: None,
            sede_id: None,
            created_by,
        }
    }

    /// Inicia el escaneo (cambia estado a Running)
    pub fn start(&mut self) {
        self.status = ScanStatus::Running;
        self.started_at = chrono::Utc::now().to_rfc3339();
    }

    /// Completa el escaneo (cambia estado a Completed)
    pub fn complete(&mut self, devices_found: i32) {
        self.status = ScanStatus::Completed;
        self.completed_at = Some(chrono::Utc::now().to_rfc3339());
        self.devices_found = devices_found;
        // Calcular duración en segundos
        if let Ok(start) = chrono::DateTime::parse_from_rfc3339(&self.started_at) {
            let duration = chrono::Utc::now().signed_duration_since(start.with_timezone(&chrono::Utc));
            self.duration_seconds = Some(duration.num_seconds() as i32);
        }
    }

    /// Marca el escaneo como fallido
    pub fn fail(&mut self) {
        self.status = ScanStatus::Failed;
        self.completed_at = Some(chrono::Utc::now().to_rfc3339());
    }
}

/// Filtros para consultas de dispositivos descubiertos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiscoveryFilters {
    pub device_type: Option<DeviceType>,
    pub status: Option<DeviceStatus>,
    pub is_authorized: Option<bool>,
    pub sede_id: Option<String>,
    pub manufacturer: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

/// Filtros para consultas de escaneos de red
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScanFilters {
    pub status: Option<ScanStatus>,
    pub sede_id: Option<String>,
    pub scan_type: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

/// Trait para puerto de descubrimiento de red
#[async_trait::async_trait]
pub trait DiscoveryPort: Send + Sync {
    /// Registrar un dispositivo descubierto
    async fn log_device(&self, device: DiscoveredDevice) -> Result<(), DomainError>;

    /// Obtener dispositivos con filtros
    async fn get_devices(&self, filters: DiscoveryFilters) -> Result<Vec<DiscoveredDevice>, DomainError>;

    /// Obtener dispositivo por IP
    async fn get_device_by_ip(&self, ip: String) -> Result<Option<DiscoveredDevice>, DomainError>;

    /// Obtener dispositivo por MAC
    async fn get_device_by_mac(&self, mac: String) -> Result<Option<DiscoveredDevice>, DomainError>;

    /// Actualizar estado de un dispositivo
    async fn update_device_status(&self, id: String, status: DeviceStatus) -> Result<(), DomainError>;

    /// Marcar dispositivo como autorizado
    async fn mark_device_authorized(&self, id: String) -> Result<(), DomainError>;

    /// Marcar dispositivo como no autorizado
    async fn mark_device_unauthorized(&self, id: String) -> Result<(), DomainError>;

    /// Crear un nuevo escaneo
    async fn create_scan(&self, scan: NetworkScan) -> Result<(), DomainError>;

    /// Obtener escaneo por ID
    async fn get_scan(&self, id: String) -> Result<Option<NetworkScan>, DomainError>;

    /// Obtener escaneos con filtros
    async fn get_scans(&self, filters: ScanFilters) -> Result<Vec<NetworkScan>, DomainError>;

    /// Actualizar estado de un escaneo
    async fn update_scan_status(&self, id: String, status: ScanStatus) -> Result<(), DomainError>;
}

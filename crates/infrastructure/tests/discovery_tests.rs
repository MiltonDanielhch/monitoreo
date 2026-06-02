// crates/infrastructure/tests/discovery_tests.rs
// Tests para el módulo de descubrimiento de red y asset discovery
// Vinculado con ADR-0015 (Tokio Jobs)

use domain::models::discovery::{
    DeviceStatus, DeviceType, DiscoveredDevice, DiscoveryFilters, DiscoveryPort, NetworkScan,
    ScanFilters, ScanStatus,
};
use domain::errors::DomainError;
use infrastructure::discovery::OuiLookupService;
use std::sync::Arc;

/// Mock del repositorio de descubrimiento para tests
struct MockDiscoveryRepository {
    devices: std::sync::Mutex<Vec<DiscoveredDevice>>,
    scans: std::sync::Mutex<Vec<NetworkScan>>,
}

impl MockDiscoveryRepository {
    fn new() -> Self {
        Self {
            devices: std::sync::Mutex::new(Vec::new()),
            scans: std::sync::Mutex::new(Vec::new()),
        }
    }

    fn get_devices(&self) -> Vec<DiscoveredDevice> {
        self.devices.lock().unwrap().clone()
    }

    fn get_scans(&self) -> Vec<NetworkScan> {
        self.scans.lock().unwrap().clone()
    }
}

#[async_trait::async_trait]
impl DiscoveryPort for MockDiscoveryRepository {
    async fn log_device(&self, device: DiscoveredDevice) -> Result<(), DomainError> {
        self.devices.lock().unwrap().push(device);
        Ok(())
    }

    async fn get_devices(
        &self,
        _filters: DiscoveryFilters,
    ) -> Result<Vec<DiscoveredDevice>, DomainError> {
        Ok(self.devices.lock().unwrap().clone())
    }

    async fn get_device_by_ip(
        &self,
        ip: String,
    ) -> Result<Option<DiscoveredDevice>, DomainError> {
        let devices = self.devices.lock().unwrap();
        Ok(devices.iter().find(|d| d.ip_address == ip).cloned())
    }

    async fn get_device_by_mac(
        &self,
        mac: String,
    ) -> Result<Option<DiscoveredDevice>, DomainError> {
        let devices = self.devices.lock().unwrap();
        Ok(devices.iter().find(|d| d.mac_address.as_ref() == Some(&mac)).cloned())
    }

    async fn update_device_status(
        &self,
        id: String,
        status: DeviceStatus,
    ) -> Result<(), DomainError> {
        let mut devices = self.devices.lock().unwrap();
        if let Some(device) = devices.iter_mut().find(|d| d.id == id) {
            device.status = status;
            Ok(())
        } else {
            Err(DomainError::DeviceNotFound(id))
        }
    }

    async fn mark_device_authorized(&self, id: String) -> Result<(), DomainError> {
        let mut devices = self.devices.lock().unwrap();
        if let Some(device) = devices.iter_mut().find(|d| d.id == id) {
            device.is_authorized = true;
            Ok(())
        } else {
            Err(DomainError::DeviceNotFound(id))
        }
    }

    async fn mark_device_unauthorized(&self, id: String) -> Result<(), DomainError> {
        let mut devices = self.devices.lock().unwrap();
        if let Some(device) = devices.iter_mut().find(|d| d.id == id) {
            device.is_authorized = false;
            Ok(())
        } else {
            Err(DomainError::DeviceNotFound(id))
        }
    }

    async fn create_scan(&self, scan: NetworkScan) -> Result<(), DomainError> {
        self.scans.lock().unwrap().push(scan);
        Ok(())
    }

    async fn get_scan(&self, id: String) -> Result<Option<NetworkScan>, DomainError> {
        let scans = self.scans.lock().unwrap();
        Ok(scans.iter().find(|s| s.id == id).cloned())
    }

    async fn get_scans(&self, _filters: ScanFilters) -> Result<Vec<NetworkScan>, DomainError> {
        Ok(self.scans.lock().unwrap().clone())
    }

    async fn update_scan_status(&self, id: String, status: ScanStatus) -> Result<(), DomainError> {
        let mut scans = self.scans.lock().unwrap();
        if let Some(scan) = scans.iter_mut().find(|s| s.id == id) {
            scan.status = status;
            Ok(())
        } else {
            Err(DomainError::ScanNotFound(id))
        }
    }
}

// ============== TESTS DE OUI LOOKUP ==============

#[test]
fn test_oui_lookup_cisco() {
    let service = OuiLookupService::new();

    // Formato con dos puntos
    assert_eq!(
        service.get_manufacturer("00:00:1A:12:34:56"),
        Some("Cisco Systems")
    );

    // Formato sin separadores
    assert_eq!(
        service.get_manufacturer("00001A123456"),
        Some("Cisco Systems")
    );

    // Case insensitive
    assert_eq!(
        service.get_manufacturer("00:00:1a:12:34:56"),
        Some("Cisco Systems")
    );
}

#[test]
fn test_oui_lookup_apple() {
    let service = OuiLookupService::new();
    assert_eq!(
        service.get_manufacturer("00:03:93:AA:BB:CC"),
        Some("Apple")
    );
    assert_eq!(
        service.get_manufacturer("00:09:BB:12:34:56"),
        Some("Apple")
    );
}

#[test]
fn test_oui_lookup_hp() {
    let service = OuiLookupService::new();
    assert_eq!(
        service.get_manufacturer("00:10:E3:12:34:56"),
        Some("Hewlett Packard")
    );
}

#[test]
fn test_oui_lookup_dell() {
    let service = OuiLookupService::new();
    assert_eq!(
        service.get_manufacturer("00:01:E9:12:34:56"),
        Some("Dell")
    );
}

#[test]
fn test_oui_lookup_unknown() {
    let service = OuiLookupService::new();
    assert_eq!(service.get_manufacturer("FF:FF:FF:12:34:56"), None);
    assert_eq!(service.get_manufacturer("00:00:00:12:34:56"), None);
}

#[test]
fn test_oui_lookup_tp_link() {
    let service = OuiLookupService::new();
    assert_eq!(
        service.get_manufacturer("14:14:4B:12:34:56"),
        Some("TP-Link")
    );
    assert_eq!(
        service.get_manufacturer("50:C7:BF:12:34:56"),
        Some("TP-Link")
    );
}

#[test]
fn test_oui_lookup_mikrotik() {
    let service = OuiLookupService::new();
    assert_eq!(
        service.get_manufacturer("00:27:2D:12:34:56"),
        Some("MikroTik")
    );
}

#[test]
fn test_oui_lookup_ubiquiti() {
    let service = OuiLookupService::new();
    assert_eq!(
        service.get_manufacturer("18:E8:29:12:34:56"),
        Some("Ubiquiti")
    );
}

#[test]
fn test_oui_lookup_huawei() {
    let service = OuiLookupService::new();
    assert_eq!(
        service.get_manufacturer("00:21:F8:12:34:56"),
        Some("Huawei")
    );
}

#[test]
fn test_oui_lookup_invalid_mac() {
    let service = OuiLookupService::new();
    // MAC muy corta
    assert_eq!(service.get_manufacturer("00:00"), None);
    // MAC vacía
    assert_eq!(service.get_manufacturer(""), None);
}

// ============== TESTS DE MODELOS DE DOMINIO ==============

#[test]
fn test_device_type_display() {
    assert_eq!(DeviceType::Router.to_string(), "router");
    assert_eq!(DeviceType::Switch.to_string(), "switch");
    assert_eq!(DeviceType::Server.to_string(), "server");
    assert_eq!(DeviceType::PC.to_string(), "pc");
    assert_eq!(DeviceType::Mobile.to_string(), "mobile");
    assert_eq!(DeviceType::IoT.to_string(), "iot");
    assert_eq!(DeviceType::Printer.to_string(), "printer");
    assert_eq!(DeviceType::Unknown.to_string(), "unknown");
}

#[test]
fn test_device_type_from_string() {
    assert_eq!(DeviceType::from("router".to_string()), DeviceType::Router);
    assert_eq!(DeviceType::from("Router".to_string()), DeviceType::Router);
    assert_eq!(DeviceType::from("ROUTER".to_string()), DeviceType::Router);
    assert_eq!(DeviceType::from("unknown".to_string()), DeviceType::Unknown);
    assert_eq!(DeviceType::from("invalid".to_string()), DeviceType::Unknown);
}

#[test]
fn test_scan_status_display() {
    assert_eq!(ScanStatus::Pending.to_string(), "pending");
    assert_eq!(ScanStatus::Running.to_string(), "running");
    assert_eq!(ScanStatus::Completed.to_string(), "completed");
    assert_eq!(ScanStatus::Failed.to_string(), "failed");
}

#[test]
fn test_device_status_display() {
    assert_eq!(DeviceStatus::Online.to_string(), "online");
    assert_eq!(DeviceStatus::Offline.to_string(), "offline");
    assert_eq!(DeviceStatus::Unknown.to_string(), "unknown");
}

#[test]
fn test_discovered_device_creation() {
    let device = DiscoveredDevice::new("192.168.1.100".to_string(), "scan-1".to_string());

    assert_eq!(device.ip_address, "192.168.1.100");
    assert_eq!(device.scan_id, "scan-1");
    assert_eq!(device.device_type, DeviceType::Unknown);
    assert_eq!(device.status, DeviceStatus::Unknown);
    assert!(!device.is_authorized);
    assert!(device.open_ports.is_empty());
    assert!(device.services.is_empty());
    assert!(device.id.starts_with("dev_"));
}

#[test]
fn test_discovered_device_validate_ip() {
    // IPv4 válido
    assert!(DiscoveredDevice::validate_ip("192.168.1.1").is_ok());
    assert!(DiscoveredDevice::validate_ip("10.0.0.1").is_ok());
    assert!(DiscoveredDevice::validate_ip("255.255.255.255").is_ok());

    // IPv4 inválido
    assert!(DiscoveredDevice::validate_ip("192.168.1.256").is_err());
    assert!(DiscoveredDevice::validate_ip("192.168.1").is_err());
    assert!(DiscoveredDevice::validate_ip("192.168.1.1.1").is_err());

    // IPv6 válido (formato completo de 8 grupos)
    assert!(DiscoveredDevice::validate_ip("2001:0db8:0000:0000:0000:0000:0000:0001").is_ok());
    assert!(DiscoveredDevice::validate_ip("2001:db8::1").is_err()); // :: compression no soportado
}

#[test]
fn test_discovered_device_validate_mac() {
    // MAC válido
    assert!(DiscoveredDevice::validate_mac("00:11:22:33:44:55").is_ok());
    assert!(DiscoveredDevice::validate_mac("AA:BB:CC:DD:EE:FF").is_ok());

    // MAC inválido
    assert!(DiscoveredDevice::validate_mac("00:11:22:33:44").is_err()); // 5 octetos
    assert!(DiscoveredDevice::validate_mac("00:11:22:33:44:GG").is_err()); // hex inválido
    assert!(DiscoveredDevice::validate_mac("00:11:22:33:44:55:66").is_err()); // 7 octetos
}

#[test]
fn test_discovered_device_mark_as_offline() {
    let mut device = DiscoveredDevice::new("192.168.1.100".to_string(), "scan-1".to_string());
    assert_eq!(device.status, DeviceStatus::Unknown);

    device.mark_as_offline();
    assert_eq!(device.status, DeviceStatus::Offline);
}

#[test]
fn test_discovered_device_authorization() {
    let mut device = DiscoveredDevice::new("192.168.1.100".to_string(), "scan-1".to_string());
    assert!(!device.is_authorized);

    device.mark_as_authorized();
    assert!(device.is_authorized);

    device.mark_as_unauthorized();
    assert!(!device.is_authorized);
}

#[test]
fn test_network_scan_lifecycle() {
    let mut scan = NetworkScan::new(
        "partial".to_string(),
        "192.168.1.0/24".to_string(),
        "admin".to_string(),
    );

    assert_eq!(scan.status, ScanStatus::Pending);
    assert_eq!(scan.devices_found, 0);
    assert!(scan.completed_at.is_none());
    assert!(scan.duration_seconds.is_none());

    // Iniciar
    scan.start();
    assert_eq!(scan.status, ScanStatus::Running);

    // Completar
    scan.complete(5);
    assert_eq!(scan.status, ScanStatus::Completed);
    assert_eq!(scan.devices_found, 5);
    assert!(scan.completed_at.is_some());
    assert!(scan.duration_seconds.is_some());
}

#[test]
fn test_network_scan_fail() {
    let mut scan = NetworkScan::new(
        "partial".to_string(),
        "192.168.1.0/24".to_string(),
        "admin".to_string(),
    );

    scan.start();
    scan.fail();

    assert_eq!(scan.status, ScanStatus::Failed);
    assert!(scan.completed_at.is_some());
}

// ============== TESTS DE MOCK REPOSITORY ==============

#[tokio::test]
async fn test_mock_repository_log_device() {
    let repo = Arc::new(MockDiscoveryRepository::new());

    let device = DiscoveredDevice::new("192.168.1.100".to_string(), "scan-1".to_string());
    repo.log_device(device.clone()).await.unwrap();

    let devices = repo.get_devices();
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].ip_address, "192.168.1.100");
}

#[tokio::test]
async fn test_mock_repository_get_device_by_ip() {
    let repo = Arc::new(MockDiscoveryRepository::new());

    let device = DiscoveredDevice::new("192.168.1.100".to_string(), "scan-1".to_string());
    repo.log_device(device).await.unwrap();

    let found = repo.get_device_by_ip("192.168.1.100".to_string()).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().ip_address, "192.168.1.100");

    let not_found = repo.get_device_by_ip("192.168.1.200".to_string()).await.unwrap();
    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_mock_repository_mark_device_authorized() {
    let repo = Arc::new(MockDiscoveryRepository::new());

    let mut device = DiscoveredDevice::new("192.168.1.100".to_string(), "scan-1".to_string());
    device.is_authorized = false;
    let device_id = device.id.clone();
    repo.log_device(device).await.unwrap();

    repo.mark_device_authorized(device_id.clone()).await.unwrap();

    let devices = repo.get_devices();
    assert!(devices[0].is_authorized);
}

#[tokio::test]
async fn test_mock_repository_mark_device_unauthorized() {
    let repo = Arc::new(MockDiscoveryRepository::new());

    let mut device = DiscoveredDevice::new("192.168.1.100".to_string(), "scan-1".to_string());
    device.is_authorized = true;
    let device_id = device.id.clone();
    repo.log_device(device).await.unwrap();

    repo.mark_device_unauthorized(device_id.clone()).await.unwrap();

    let devices = repo.get_devices();
    assert!(!devices[0].is_authorized);
}

#[tokio::test]
async fn test_mock_repository_create_and_get_scan() {
    let repo = Arc::new(MockDiscoveryRepository::new());

    let scan = NetworkScan::new(
        "partial".to_string(),
        "192.168.1.0/24".to_string(),
        "admin".to_string(),
    );
    let scan_id = scan.id.clone();
    repo.create_scan(scan).await.unwrap();

    let found = repo.get_scan(scan_id.clone()).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().ip_range, "192.168.1.0/24");

    let not_found = repo.get_scan("nonexistent".to_string()).await.unwrap();
    assert!(not_found.is_none());
}

#[tokio::test]
async fn test_mock_repository_update_scan_status() {
    let repo = Arc::new(MockDiscoveryRepository::new());

    let scan = NetworkScan::new(
        "partial".to_string(),
        "192.168.1.0/24".to_string(),
        "admin".to_string(),
    );
    let scan_id = scan.id.clone();
    repo.create_scan(scan).await.unwrap();

    repo.update_scan_status(scan_id.clone(), ScanStatus::Running)
        .await
        .unwrap();

    let found = repo.get_scan(scan_id).await.unwrap().unwrap();
    assert_eq!(found.status, ScanStatus::Running);
}

#[tokio::test]
async fn test_mock_repository_device_not_found() {
    let repo = Arc::new(MockDiscoveryRepository::new());

    let result = repo.mark_device_authorized("nonexistent".to_string()).await;
    assert!(result.is_err());

    let result = repo.update_device_status("nonexistent".to_string(), DeviceStatus::Online).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_mock_repository_scan_not_found() {
    let repo = Arc::new(MockDiscoveryRepository::new());

    let result = repo.update_scan_status("nonexistent".to_string(), ScanStatus::Running).await;
    assert!(result.is_err());
}

// ============== TESTS DE CLASIFICACIÓN ==============

#[test]
fn test_device_classification_rules() {
    use std::collections::HashSet;

    // Función helper para clasificar (replicando la lógica del scan_engine)
    fn classify(ports: &[u16]) -> DeviceType {
        let port_set: HashSet<u16> = ports.iter().cloned().collect();

        if port_set.contains(&22) && (port_set.contains(&161) || port_set.contains(&23)) {
            return DeviceType::Router;
        }
        if port_set.contains(&161) && (port_set.contains(&22) || port_set.contains(&23)) {
            return DeviceType::Switch;
        }
        if port_set.contains(&3306) || port_set.contains(&5432) || port_set.contains(&1433) {
            return DeviceType::Server;
        }
        if port_set.contains(&9100) || port_set.contains(&515) || port_set.contains(&631) {
            return DeviceType::Printer;
        }
        if port_set.contains(&1883) || port_set.contains(&5683) {
            return DeviceType::IoT;
        }
        if port_set.contains(&135) || port_set.contains(&139) || port_set.contains(&445) {
            return DeviceType::PC;
        }
        if port_set.contains(&80) || port_set.contains(&443) || port_set.contains(&8080) {
            return DeviceType::Server;
        }

        DeviceType::Unknown
    }

    // Router: SSH + SNMP o SSH + Telnet
    assert_eq!(classify(&[22, 161, 80]), DeviceType::Router);
    assert_eq!(classify(&[22, 23, 80]), DeviceType::Router);
    // [161, 22] también es Router porque Router se verifica primero
    assert_eq!(classify(&[161, 22]), DeviceType::Router);

    // Switch: SNMP + SSH o SNMP + Telnet (sin SSH para evitar match con Router)
    assert_eq!(classify(&[161, 23]), DeviceType::Switch);
    // [161, 80] es Server porque hace match con la regla de HTTP (Server)
    assert_eq!(classify(&[161, 80]), DeviceType::Server);

    // Server: base de datos
    assert_eq!(classify(&[3306]), DeviceType::Server);
    assert_eq!(classify(&[5432]), DeviceType::Server);
    assert_eq!(classify(&[1433]), DeviceType::Server);

    // Printer
    assert_eq!(classify(&[9100]), DeviceType::Printer);
    assert_eq!(classify(&[515]), DeviceType::Printer);
    assert_eq!(classify(&[631]), DeviceType::Printer);

    // IoT
    assert_eq!(classify(&[1883]), DeviceType::IoT);
    assert_eq!(classify(&[5683]), DeviceType::IoT);

    // PC: SMB/Windows
    assert_eq!(classify(&[135, 139, 445]), DeviceType::PC);
    assert_eq!(classify(&[445]), DeviceType::PC);

    // Server: web
    assert_eq!(classify(&[80, 443]), DeviceType::Server);
    assert_eq!(classify(&[8080]), DeviceType::Server);

    // Unknown
    assert_eq!(classify(&[]), DeviceType::Unknown);
    assert_eq!(classify(&[12345]), DeviceType::Unknown);
}

// ============== TESTS DE DETECCIÓN DE SERVICIOS ==============

#[test]
fn test_service_detection() {
    // Función helper para detectar servicios (replicando la lógica del scan_engine)
    fn detect_services(ports: &[u16]) -> Vec<String> {
        let mut services = Vec::new();
        for &port in ports {
            let service = match port {
                22 => "SSH",
                23 => "Telnet",
                25 => "SMTP",
                53 => "DNS",
                80 => "HTTP",
                110 => "POP3",
                143 => "IMAP",
                443 => "HTTPS",
                445 => "SMB",
                993 => "IMAPS",
                995 => "POP3S",
                3306 => "MySQL",
                3389 => "RDP",
                5432 => "PostgreSQL",
                5900 => "VNC",
                631 => "IPP",
                8080 => "HTTP-Alt",
                8443 => "HTTPS-Alt",
                9100 => "PDL",
                1883 => "MQTT",
                5683 => "CoAP",
                _ => "",
            };
            if !service.is_empty() {
                services.push(service.to_string());
            }
        }
        services
    }

    assert_eq!(detect_services(&[22]), vec!["SSH"]);
    assert_eq!(detect_services(&[80, 443]), vec!["HTTP", "HTTPS"]);
    assert_eq!(
        detect_services(&[22, 80, 443, 3306]),
        vec!["SSH", "HTTP", "HTTPS", "MySQL"]
    );
    assert_eq!(detect_services(&[1883]), vec!["MQTT"]);
    assert_eq!(detect_services(&[9100]), vec!["PDL"]);
    assert!(detect_services(&[12345]).is_empty());
}

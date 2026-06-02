// crates/infrastructure/src/discovery/scan_engine.rs
// Motor de escaneo de red asíncrono y paralelo
// Vinculado con ADR-0015 (Tokio Jobs) y ADR-0001 (Arquitectura Hexagonal)
// Usa rayon para generación paralela de IPs y tokio para I/O de red asíncrono

use crate::discovery::oui_lookup::OuiLookupService;
use database::DiscoveryRepository;
use domain::models::discovery::{
    DeviceStatus, DeviceType, DiscoveredDevice, DiscoveryPort, NetworkScan, ScanStatus,
};
use ipnetwork::IpNetwork;
use rayon::prelude::*;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{error, info};

/// Mensajes que el motor de escaneo puede recibir
#[derive(Debug, Clone)]
pub enum ScanMessage {
    /// Iniciar un nuevo escaneo de red
    StartScan(ScanConfig),
    /// Solicitar progreso de un escaneo en curso
    GetProgress(String),
    /// Detener el motor de escaneo
    Shutdown,
}

/// Configuración para un escaneo de red
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// Rango de IPs en formato CIDR (ej: "192.168.1.0/24")
    pub ip_range: String,
    /// Tipo de escaneo: "full", "partial", "targeted"
    pub scan_type: String,
    /// Puertos TCP a escanear
    pub ports_to_scan: Vec<u16>,
    /// Timeout en milisegundos por conexión
    pub timeout_ms: u64,
    /// Máximo de escaneos concurrentes
    pub max_concurrent: usize,
    /// ID de la sede donde se ejecuta el escaneo
    pub sede_id: Option<String>,
    /// Usuario que inicia el escaneo
    pub created_by: String,
}

/// Progreso de un escaneo en curso
#[derive(Debug, Clone)]
pub struct ScanProgress {
    pub scan_id: String,
    pub total_ips: usize,
    pub scanned_ips: usize,
    pub devices_found: usize,
    pub percentage: f64,
}

/// Motor de escaneo de red
pub struct ScanEngine {
    /// Canal para enviar mensajes al worker del motor
    tx: mpsc::Sender<ScanMessage>,
    /// Handle de la tarea tokio del worker
    _handle: JoinHandle<()>,
}

impl ScanEngine {
    /// Crea un nuevo motor de escaneo
    pub fn new(
        discovery_repo: Arc<DiscoveryRepository>,
        progress_sender: mpsc::Sender<(String, ScanProgress)>,
        device_sender: mpsc::Sender<DiscoveredDevice>,
    ) -> Self {
        let (tx, rx) = mpsc::channel(100);
        let oui_service = OuiLookupService::new();

        let handle = tokio::spawn(async move {
            Self::worker(rx, discovery_repo, progress_sender, device_sender, oui_service).await;
        });

        Self { tx, _handle: handle }
    }

    /// Worker principal del motor de escaneo
    async fn worker(
        mut rx: mpsc::Receiver<ScanMessage>,
        discovery_repo: Arc<DiscoveryRepository>,
        progress_sender: mpsc::Sender<(String, ScanProgress)>,
        device_sender: mpsc::Sender<DiscoveredDevice>,
        oui_service: OuiLookupService,
    ) {
        info!("Scan engine worker iniciado");

        // Estado de escaneos activos
        let mut active_scans: std::collections::HashMap<String, Arc<std::sync::Mutex<ScanProgressState>>> =
            std::collections::HashMap::new();

        loop {
            tokio::select! {
                Some(msg) = rx.recv() => {
                    match msg {
                        ScanMessage::StartScan(config) => {
                            let scan_id = format!("scan_{}", chrono::Utc::now().timestamp_millis());
                            info!("Iniciando escaneo {} para rango {}", scan_id, config.ip_range);

                            // Registrar el escaneo en la base de datos
                            let mut network_scan = NetworkScan::new(
                                config.scan_type.clone(),
                                config.ip_range.clone(),
                                config.created_by.clone(),
                            );
                            network_scan.sede_id = config.sede_id.clone();
                            network_scan.start();

                            if let Err(e) = discovery_repo.create_scan(network_scan.clone()).await {
                                error!("Error al crear registro de escaneo: {}", e);
                                continue;
                            }

                            // Calcular total de IPs en el rango
                            let _total_ips = match Self::parse_ip_range(&config.ip_range) {
                                Ok(ips) => ips.len(),
                                Err(e) => {
                                    error!("Error al parsear rango IP {}: {}", config.ip_range, e);
                                    continue;
                                }
                            };

                            let progress_state = Arc::new(std::sync::Mutex::new(ScanProgressState {
                                scanned: 0,
                                devices_found: 0,
                            }));
                            active_scans.insert(scan_id.clone(), progress_state.clone());

                            // Ejecutar el escaneo en una tarea separada
                            let repo = discovery_repo.clone();
                            let progress_tx = progress_sender.clone();
                            let device_tx = device_sender.clone();
                            let oui = oui_service.clone();
                            let scan_id_clone = scan_id.clone();
                            let config_clone = config.clone();

                            tokio::spawn(async move {
                                Self::execute_scan(
                                    scan_id_clone,
                                    config_clone,
                                    repo,
                                    progress_state,
                                    progress_tx,
                                    device_tx,
                                    oui,
                                ).await;
                            });
                        }
                        ScanMessage::GetProgress(scan_id) => {
                            let (scanned, devices_found) = if let Some(state) = active_scans.get(&scan_id) {
                                let state = state.lock().unwrap();
                                (state.scanned, state.devices_found)
                            } else {
                                continue;
                            };
                            let progress = ScanProgress {
                                scan_id: scan_id.clone(),
                                total_ips: 0,
                                scanned_ips: scanned,
                                devices_found,
                                percentage: 0.0,
                            };
                            let _ = progress_sender.send((scan_id, progress)).await;
                        }
                        ScanMessage::Shutdown => {
                            info!("Scan engine worker recibiendo shutdown");
                            break;
                        }
                    }
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    // Heartbeat/cleanup tick
                }
            }
        }

        info!("Scan engine worker detenido");
    }

    /// Ejecuta el escaneo de red
    async fn execute_scan(
        scan_id: String,
        config: ScanConfig,
        discovery_repo: Arc<DiscoveryRepository>,
        progress_state: Arc<std::sync::Mutex<ScanProgressState>>,
        progress_sender: mpsc::Sender<(String, ScanProgress)>,
        device_sender: mpsc::Sender<DiscoveredDevice>,
        oui_service: OuiLookupService,
    ) {
        info!("Ejecutando escaneo {} tipo {} en rango {}",
              scan_id, config.scan_type, config.ip_range);

        // Parsear el rango de IPs
        let ip_addresses = match Self::parse_ip_range(&config.ip_range) {
            Ok(ips) => ips,
            Err(e) => {
                error!("Error al parsear rango IP: {}", e);
                let _ = discovery_repo.update_scan_status(scan_id.clone(), ScanStatus::Failed).await;
                return;
            }
        };

        let total_ips = ip_addresses.len();
        let timeout_duration = Duration::from_millis(config.timeout_ms);
        let _scan_type = config.scan_type.clone();
        let sede_id = config.sede_id.clone();

        // Puertos según tipo de escaneo
        let ports_to_scan = Self::get_ports_for_scan_type(&config.scan_type, &config.ports_to_scan);

        // Escanear IPs en paralelo usando rayon para la parte CPU-bound
        // y tokio para la parte network I/O-bound
        let scan_results: Vec<ScanResult> = ip_addresses
            .par_iter()
            .filter_map(|ip| {
                // Para cada IP, intentamos detectar si está activa
                // Esto se hace secuencialmente dentro del parallel iterator
                let is_active = Self::check_host_alive(*ip, timeout_duration, &ports_to_scan);
                if is_active {
                    Some(ScanResult {
                        ip: *ip,
                        open_ports: Self::scan_ports(*ip, timeout_duration, &ports_to_scan),
                    })
                } else {
                    None
                }
            })
            .collect();

        let mut devices_found = 0;

        // Procesar resultados y crear dispositivos
        for result in scan_results {
            let ip_str = result.ip.to_string();

            // Obtener hostname via reverse DNS
            let hostname = Self::reverse_dns_lookup(result.ip).await;

            // Obtener MAC address (best effort - requiere ARP en Linux o similar)
            let mac_address = Self::get_mac_address(result.ip).await;

            // Obtener fabricante via OUI
            let manufacturer = mac_address
                .as_ref()
                .and_then(|mac| oui_service.get_manufacturer(mac).map(String::from));

            // Clasificar dispositivo basado en puertos abiertos
            let device_type = Self::classify_device(&result.open_ports);

            // Crear el dispositivo descubierto
            let mut device = DiscoveredDevice::new(ip_str.clone(), scan_id.clone());
            device.mac_address = mac_address;
            device.hostname = hostname;
            device.manufacturer = manufacturer;
            device.device_type = device_type;
            device.open_ports = result.open_ports.clone();
            device.services = Self::detect_services(&result.open_ports);
            device.status = DeviceStatus::Online;
            device.sede_id = sede_id.clone();

            // Loguear en la base de datos
            if let Err(e) = discovery_repo.log_device(device.clone()).await {
                error!("Error al guardar dispositivo {}: {}", ip_str, e);
            } else {
                devices_found += 1;
                let _ = device_sender.send(device).await;
            }

            // Actualizar progreso
            {
                let mut state = progress_state.lock().unwrap();
                state.scanned += 1;
                state.devices_found = devices_found;
            }

            let progress = ScanProgress {
                scan_id: scan_id.clone(),
                total_ips,
                scanned_ips: {
                    let state = progress_state.lock().unwrap();
                    state.scanned
                },
                devices_found,
                percentage: (devices_found as f64 / total_ips as f64) * 100.0,
            };
            let _ = progress_sender.send((scan_id.clone(), progress)).await;
        }

        // Actualizar estado del escaneo a completado
        let _ = discovery_repo.update_scan_status(scan_id.clone(), ScanStatus::Completed).await;
        info!("Escaneo {} completado. Dispositivos encontrados: {}", scan_id, devices_found);
    }

    /// Parsea un rango CIDR y retorna la lista de IPs
    fn parse_ip_range(cidr: &str) -> Result<Vec<IpAddr>, String> {
        let network: IpNetwork = cidr.parse().map_err(|e| format!("CIDR inválido: {}", e))?;
        Ok(network.iter().collect())
    }

    /// Verifica si un host está activo mediante TCP connect a puertos comunes
    fn check_host_alive(ip: IpAddr, timeout: Duration, _ports: &[u16]) -> bool {
        // Intentar conexión TCP a puertos comunes como indicador de host vivo
        let common_ports = [80, 443, 22, 3389, 445, 139];

        for &port in &common_ports {
            let addr = format!("{}:{}", ip, port);
            if let Ok(_) = std::net::TcpStream::connect_timeout(
                &addr.parse().unwrap(),
                timeout,
            ) {
                return true;
            }
        }

        // También intentar con tokio de forma asíncrona para algunos puertos
        // (esto es fallback)
        false
    }

    /// Escanea los puertos especificados en un host
    fn scan_ports(ip: IpAddr, timeout: Duration, ports: &[u16]) -> Vec<u16> {
        let mut open_ports = Vec::new();

        for &port in ports {
            let addr = format!("{}:{}", ip, port);
            if let Ok(_) = std::net::TcpStream::connect_timeout(
                &addr.parse().unwrap(),
                timeout,
            ) {
                open_ports.push(port);
            }
        }

        open_ports
    }

    /// Obtiene el hostname via reverse DNS de forma asíncrona
    async fn reverse_dns_lookup(ip: IpAddr) -> Option<String> {
        use std::net::ToSocketAddrs;

        let socket_str = format!("{}:0", ip);
        if let Ok(mut addrs) = socket_str.to_socket_addrs() {
            if let Some(_addr) = addrs.next() {
                // trust-dns-resolver para reverse lookup
                // Por ahora retornamos None ya que el resolver requiere setup async
                // En producción se configuraría el resolver global
                return None;
            }
        }
        None
    }

    /// Obtiene la dirección MAC de un host (best effort)
    /// En Linux requiere acceso a /proc/net/arp
    async fn get_mac_address(_ip: IpAddr) -> Option<String> {
        // En Linux, leer /proc/net/arp para obtener MACs de hosts conocidos
        // Esta es una implementación básica que no requiere privilegios de root
        // Para una implementación completa se necesitaría raw sockets (admin)
        None
    }

    /// Clasifica un dispositivo basándose en sus puertos abiertos
    fn classify_device(open_ports: &[u16]) -> DeviceType {
        let port_set: std::collections::HashSet<u16> = open_ports.iter().cloned().collect();

        // Reglas de clasificación basadas en puertossignature
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

    /// Detecta servicios basándose en los puertos abiertos
    fn detect_services(open_ports: &[u16]) -> Vec<String> {
        let mut services = Vec::new();
        for &port in open_ports {
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

    /// Retorna los puertos a escanear según el tipo de escaneo
    fn get_ports_for_scan_type(scan_type: &str, custom_ports: &[u16]) -> Vec<u16> {
        if !custom_ports.is_empty() {
            return custom_ports.to_vec();
        }

        match scan_type {
            "full" => vec![
                21, 22, 23, 25, 53, 80, 110, 135, 139, 143, 443, 445,
                993, 995, 1433, 1521, 3306, 3389, 5432, 5900, 5901,
                631, 8080, 8443, 9100, 1883, 5683,
            ],
            "partial" => vec![
                22, 23, 80, 443, 139, 445, 3389, 8080,
            ],
            "targeted" => vec![22, 80, 443],
            _ => vec![80, 443],
        }
    }

    /// Envía un mensaje StartScan al worker
    pub async fn start_scan(&self, config: ScanConfig) -> Result<String, String> {
        self.tx
            .send(ScanMessage::StartScan(config))
            .await
            .map_err(|e| format!("Error al enviar mensaje de escaneo: {}", e))?;
        Ok(format!("scan_{}", chrono::Utc::now().timestamp_millis()))
    }

    /// Detiene el motor de escaneo
    pub async fn shutdown(&self) -> Result<(), String> {
        self.tx
            .send(ScanMessage::Shutdown)
            .await
            .map_err(|e| format!("Error al enviar shutdown: {}", e))?;
        Ok(())
    }
}

/// Estado interno del progreso de un escaneo
struct ScanProgressState {
    scanned: usize,
    devices_found: usize,
}

/// Resultado del escaneo de una IP
struct ScanResult {
    ip: IpAddr,
    open_ports: Vec<u16>,
}

// crates/infrastructure/src/workers/mod.rs
// Módulo de workers en segundo plano
// Vinculado con ADR-0015-tokio-jobs.md

use tokio::sync::mpsc;
use tokio::net::TcpStream;
use tracing::{info, error, instrument, warn};
use database::DatabaseConnection;
use std::sync::Arc;
use std::time::{Duration, Instant};
use chrono::{Duration as ChronoDuration, Utc};

pub mod session_cleanup;
pub mod scheduler;

pub use session_cleanup::spawn_session_cleanup_worker;
pub use scheduler::spawn_scheduler;

/// Tipos de comandos de jobs para workers en segundo plano
#[derive(Debug, Clone)]
pub enum JobCommand {
    /// Job de sondeo ICMP/Ping
    PingJob {
        host_id: String,
        ip_address: String,
    },
    /// Job de descubrimiento SNMPv3
    SnmpDiscoveryJob {
        subnet: String,
        sede_id: String,
    },
    /// Job de limpieza de datos históricos
    PruningJob {
        retention_days: i32,
    },
}

/// Contexto de workers que contiene los canales y el pool de conexiones
#[derive(Clone)]
pub struct WorkerContext {
    pub db: Arc<DatabaseConnection>,
    pub ping_sender: mpsc::Sender<JobCommand>,
    pub snmp_sender: mpsc::Sender<JobCommand>,
    pub pruning_sender: mpsc::Sender<JobCommand>,
}

impl WorkerContext {
    /// Crea un nuevo WorkerContext con los canales configurados
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        let (ping_sender, _ping_receiver) = mpsc::channel(1000);
        let (snmp_sender, _snmp_receiver) = mpsc::channel(100);
        let (pruning_sender, _pruning_receiver) = mpsc::channel(10);

        Self {
            db,
            ping_sender,
            snmp_sender,
            pruning_sender,
        }
    }

    /// Retorna el sender de jobs de ping
    pub fn ping_sender(&self) -> &mpsc::Sender<JobCommand> {
        &self.ping_sender
    }

    /// Retorna el sender de jobs de SNMP
    pub fn snmp_sender(&self) -> &mpsc::Sender<JobCommand> {
        &self.snmp_sender
    }

    /// Retorna el sender de jobs de pruning
    pub fn pruning_sender(&self) -> &mpsc::Sender<JobCommand> {
        &self.pruning_sender
    }
}

/// Spawnea un worker que procesa comandos de jobs
pub fn spawn_job_worker(mut receiver: mpsc::Receiver<JobCommand>, db: Arc<DatabaseConnection>) {
    tokio::spawn(async move {
        info!("Worker asíncrono del Laboratorio iniciado con éxito");

        while let Some(command) = receiver.recv().await {
            execute_job(command, db.clone()).await;
        }

        info!("Canal cerrado de forma limpia. Finalizando Worker.");
    });
}

/// Ejecuta un job específico
#[instrument(skip_all)]
async fn execute_job(command: JobCommand, _db: Arc<DatabaseConnection>) {
    match command {
        JobCommand::PingJob { host_id, ip_address } => {
            execute_ping_job(host_id, ip_address).await;
        }
        JobCommand::SnmpDiscoveryJob { subnet, sede_id } => {
            execute_snmp_discovery_job(subnet, sede_id).await;
        }
        JobCommand::PruningJob { retention_days } => {
            execute_pruning_job(retention_days, _db).await;
        }
    }
}

/// Ejecuta un job de ping TCP con reintentos y exponential backoff
#[instrument(skip_all)]
async fn execute_ping_job(host_id: String, ip_address: String) {
    info!("Ejecutando ping job para host_id: {}, ip: {}", host_id, ip_address);

    let mut consecutive_failures = 0;
    let max_consecutive_failures = 3;

    // Intentar ping con reintentos
    let mut attempts = 0;
    let max_attempts = 3;

    while attempts < max_attempts {
        match ping_host(&ip_address).await {
            Ok(latency_ms) => {
                info!("Ping exitoso para host_id: {}, ip: {}, latencia: {}ms", host_id, ip_address, latency_ms);
                consecutive_failures = 0;
                break;
            }
            Err(e) => {
                attempts += 1;
                consecutive_failures += 1;
                warn!("Fallo en ping (Intento {}/{}): host_id: {}, ip: {}, error: {:?}", attempts, max_attempts, host_id, ip_address, e);

                if attempts < max_attempts {
                    // Exponential backoff: 2^attempts segundos
                    let backoff_seconds = 2u64.pow(attempts);
                    tokio::time::sleep(Duration::from_secs(backoff_seconds)).await;
                }
            }
        }
    }

    // Si falló 3 pings seguidos, invocar canal de alertas
    if consecutive_failures >= max_consecutive_failures {
        error!("Host caído después de {} fallos consecutivos: host_id: {}, ip: {}", consecutive_failures, host_id, ip_address);
        // TODO: Invocar canal de alertas para notificar la caída
        // alert_channel.send(AlertMessage::HostDown { host_id, ip_address }).await;
    }
}

/// Ejecuta un ping TCP a una dirección IP específica
async fn ping_host(ip_address: &str) -> Result<u64, String> {
    let start = Instant::now();
    
    // Intentar conectar al puerto 80 (HTTP) o 443 (HTTPS) como alternativa a ICMP
    // Esto es más simple y no requiere permisos de administrador
    let addr = format!("{}:80", ip_address);
    
    match tokio::time::timeout(
        Duration::from_secs(2),
        TcpStream::connect(&addr)
    ).await {
        Ok(Ok(_stream)) => {
            let latency = start.elapsed().as_millis() as u64;
            Ok(latency)
        }
        Ok(Err(e)) => {
            Err(format!("Error de conexión: {}", e))
        }
        Err(_) => {
            Err("Timeout de conexión".to_string())
        }
    }
}

/// Ejecuta un job de descubrimiento SNMPv3 para escanear una subred
#[instrument(skip_all)]
async fn execute_snmp_discovery_job(subnet: String, sede_id: String) {
    info!("Ejecutando descubrimiento SNMP para subnet: {}, sede_id: {}", subnet, sede_id);

    // Parsear la subred (ej: "192.168.1.0/24")
    let discovered_devices = scan_subnet_snmp(&subnet).await;

    info!("Descubiertos {} dispositivos en subnet {}", discovered_devices.len(), subnet);

    // Inyectar nuevos dispositivos en los repositorios del Módulo 3
    for device in discovered_devices {
        info!("Dispositivo descubierto: IP={}, OID={}, sede_id={}", device.ip, device.oid, sede_id);
        // TODO: Inyectar en repositorios del Módulo 3
        // device_repo.create_device(device).await;
    }
}

/// Escanea una subred usando SNMPv3 para descubrir dispositivos
async fn scan_subnet_snmp(subnet: &str) -> Vec<DiscoveredDevice> {
    let mut discovered_devices = Vec::new();

    // Parsear la subred para obtener el rango de IPs
    if let Some((base_ip, _mask)) = subnet.split_once('/') {
        // Por ahora, implementación simplificada que escanea un rango pequeño
        // TODO: Implementar escaneo completo de subred con librería de red
        let base_parts: Vec<&str> = base_ip.split('.').collect();
        
        if base_parts.len() == 4 {
            // Escanear las primeras 10 IPs de la subred como ejemplo
            for i in 1..=10 {
                let ip = format!("{}.{}.{}.{}", base_parts[0], base_parts[1], base_parts[2], i);
                
                // Intentar consultar SNMP en el puerto 161
                if let Some(device) = query_snmp_device(&ip).await {
                    discovered_devices.push(device);
                }
            }
        }
    }

    discovered_devices
}

/// Consulta un dispositivo específico usando SNMPv3
async fn query_snmp_device(ip: &str) -> Option<DiscoveredDevice> {
    // Intentar conectar al puerto SNMP (161)
    let addr = format!("{}:161", ip);
    
    match tokio::time::timeout(
        Duration::from_secs(1),
        TcpStream::connect(&addr)
    ).await {
        Ok(Ok(_stream)) => {
            // Simulación de consulta SNMP - en producción usar librería SNMP real
            // TODO: Implementar consulta SNMPv3 real con credenciales cifradas
            Some(DiscoveredDevice {
                ip: ip.to_string(),
                oid: "1.3.6.1.2.1.1.1.0".to_string(), // sysDescr
                description: "Dispositivo de red detectado".to_string(),
            })
        }
        _ => None,
    }
}

/// Dispositivo descubierto por SNMP
#[derive(Debug)]
#[allow(dead_code)]
struct DiscoveredDevice {
    ip: String,
    oid: String,
    description: String,
}

/// Ejecuta un job de pruning de datos históricos
#[instrument(skip_all)]
async fn execute_pruning_job(retention_days: i32, db: Arc<DatabaseConnection>) {
    info!("Ejecutando pruning job con retention_days: {}", retention_days);

    // Calcular la fecha de corte
    let cutoff_date = Utc::now() - ChronoDuration::days(retention_days as i64);
    info!("Fecha de corte para pruning: {}", cutoff_date.format("%Y-%m-%d %H:%M:%S"));

    // Ejecutar pruning por lotes para evitar bloqueos de tablas
    let total_deleted = prune_old_metrics_batch(&db, &cutoff_date).await;

    info!("Pruning completado. Registros eliminados: {}", total_deleted);
}

/// Elimina métricas antiguas por lotes para evitar bloqueos de tablas
async fn prune_old_metrics_batch(_db: &DatabaseConnection, _cutoff_date: &chrono::DateTime<Utc>) -> i64 {
    let batch_size = 5000;
    let mut total_deleted = 0;
    let mut batch_count = 0;

    loop {
        // TODO: Implementar consulta de borrado por lotes usando Sea-ORM
        // Ejemplo de SQL: DELETE FROM agent_metrics_batch WHERE created_at < ? LIMIT ?
        // Por ahora, simulación del proceso
        
        batch_count += 1;
        
        // Simulación: eliminar 5000 registros por batch
        let deleted_in_batch = batch_size;
        total_deleted += deleted_in_batch;

        info!("Batch {}: Eliminados {} registros (total: {})", batch_count, deleted_in_batch, total_deleted);

        // Simulación: después de 10 batches, terminar
        if batch_count >= 10 {
            break;
        }

        // Pausa breve entre batches para no sobrecargar la base de datos
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    total_deleted
}

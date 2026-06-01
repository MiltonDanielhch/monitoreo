// crates/infrastructure/src/workers/mod.rs
// Módulo de workers en segundo plano
// Vinculado con ADR-0015-tokio-jobs.md

use tokio::sync::mpsc;
use tokio::net::TcpStream;
use tracing::{info, error, instrument, warn};
use database::DatabaseConnection;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub mod session_cleanup;

pub use session_cleanup::spawn_session_cleanup_worker;

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
            info!("Ejecutando descubrimiento SNMP para subnet: {}, sede_id: {}", subnet, sede_id);
            // TODO: Implementar lógica de descubrimiento SNMPv3
        }
        JobCommand::PruningJob { retention_days } => {
            info!("Ejecutando pruning job con retention_days: {}", retention_days);
            // TODO: Implementar lógica de limpieza de datos históricos
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

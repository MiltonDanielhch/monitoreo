// crates/infrastructure/src/workers/mod.rs
// Módulo de workers en segundo plano
// Vinculado con ADR-0015-tokio-jobs.md

use tokio::sync::mpsc;
use tracing::{info, instrument};
use database::DatabaseConnection;
use std::sync::Arc;

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
            info!("Ejecutando ping job para host_id: {}, ip: {}", host_id, ip_address);
            // TODO: Implementar lógica de ping con reintentos
            // Si falla 3 pings seguidos, invocar canal de alertas
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

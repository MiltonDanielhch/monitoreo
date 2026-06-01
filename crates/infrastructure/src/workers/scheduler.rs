// crates/infrastructure/src/workers/scheduler.rs
// Orquestador de tareas usando intervalos nativos de Tokio
// Vinculado con ADR-0015-tokio-jobs.md

use super::{JobCommand, WorkerContext};
use tokio::time::{interval, Duration};
use tracing::{info, error, instrument};
use chrono::Utc;

/// Spawnea el scheduler que programa los jobs automáticamente
pub fn spawn_scheduler(worker_context: WorkerContext) {
    tokio::spawn(async move {
        info!("Scheduler de tareas iniciado");

        // Spawnear cada tipo de scheduler en su propio task
        tokio::spawn(ping_scheduler(worker_context.clone()));
        tokio::spawn(snmp_discovery_scheduler(worker_context.clone()));
        tokio::spawn(pruning_scheduler(worker_context));

        info!("Todos los schedulers iniciados");
    });
}

/// Scheduler para jobs de ping (alta frecuencia: cada 30 segundos)
#[instrument(skip_all)]
async fn ping_scheduler(worker_context: WorkerContext) {
    info!("Scheduler de ping iniciado - Intervalo: 30 segundos");

    let mut ticker = interval(Duration::from_secs(30));

    loop {
        ticker.tick().await;

        // TODO: Obtener lista de hosts activos de la base de datos
        // Por ahora, simulación con un host de ejemplo
        let host_id = "example-host-1".to_string();
        let ip_address = "192.168.1.1".to_string();

        let job = JobCommand::PingJob { host_id, ip_address };

        if let Err(e) = worker_context.ping_sender().send(job).await {
            error!("Error al enviar job de ping: {}", e);
        }
    }
}

/// Scheduler para jobs de descubrimiento SNMP (cada 6 horas)
#[instrument(skip_all)]
async fn snmp_discovery_scheduler(worker_context: WorkerContext) {
    info!("Scheduler de descubrimiento SNMP iniciado - Intervalo: 6 horas");

    let mut ticker = interval(Duration::from_secs(6 * 60 * 60)); // 6 horas

    loop {
        ticker.tick().await;

        // TODO: Obtener lista de subredes a escanear de la base de datos
        // Por ahora, simulación con una subred de ejemplo
        let subnet = "192.168.1.0/24".to_string();
        let sede_id = "sede-trinidad".to_string();

        let job = JobCommand::SnmpDiscoveryJob { subnet, sede_id };

        if let Err(e) = worker_context.snmp_sender().send(job).await {
            error!("Error al enviar job de descubrimiento SNMP: {}", e);
        }
    }
}

/// Scheduler para jobs de pruning (02:00 AM hora de Bolivia)
#[instrument(skip_all)]
async fn pruning_scheduler(worker_context: WorkerContext) {
    info!("Scheduler de pruning iniciado - Ejecución: 02:00 AM hora de Bolivia");

    loop {
        // Calcular tiempo hasta las 02:00 AM hora de Bolivia (UTC-4)
        let now = Utc::now();
        let bolivia_time = now - chrono::Duration::hours(4);
        
        let target_time = bolivia_time.date_naive().and_hms_opt(2, 0, 0)
            .unwrap()
            .and_local_timezone(chrono_tz::Tz::America__La_Paz)
            .unwrap()
            .with_timezone(&Utc);

        let duration_until_target = if target_time > now {
            target_time - now
        } else {
            // Si ya pasó las 02:00 AM hoy, programar para mañana
            let target_time_tomorrow = target_time + chrono::Duration::days(1);
            target_time_tomorrow - now
        };

        let sleep_duration = duration_until_target.to_std().unwrap_or(Duration::from_secs(86400));
        
        info!("Próximo pruning programado en: {:?}", sleep_duration);
        tokio::time::sleep(sleep_duration).await;

        // Ejecutar job de pruning
        let retention_days = 90; // Mantener datos por 90 días
        let job = JobCommand::PruningJob { retention_days };

        if let Err(e) = worker_context.pruning_sender().send(job).await {
            error!("Error al enviar job de pruning: {}", e);
        }
    }
}

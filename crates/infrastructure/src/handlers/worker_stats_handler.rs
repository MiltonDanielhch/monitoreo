// crates/infrastructure/src/handlers/worker_stats_handler.rs
// Handlers para estadísticas de workers en segundo plano
// Vinculado con ADR-0015-tokio-jobs.md

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
};
use serde::Serialize;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Estado global de estadísticas de workers
#[derive(Clone)]
pub struct WorkerStats {
    /// Tareas de ping procesadas exitosamente
    pub ping_success: Arc<AtomicU64>,
    /// Tareas de ping fallidas
    pub ping_failures: Arc<AtomicU64>,
    /// Latencia promedio de ping (ms)
    pub ping_avg_latency: Arc<AtomicU64>,
    /// Tareas de SNMP procesadas exitosamente
    pub snmp_success: Arc<AtomicU64>,
    /// Tareas de SNMP fallidas
    pub snmp_failures: Arc<AtomicU64>,
    /// Dispositivos descubiertos
    pub devices_discovered: Arc<AtomicU64>,
    /// Registros purgados por pruning
    pub pruning_records_purged: Arc<AtomicU64>,
    /// Última ejecución de pruning
    pub last_pruning_run: Arc<AtomicU64>,
}

impl WorkerStats {
    /// Crea un nuevo WorkerStats con contadores inicializados en 0
    pub fn new() -> Self {
        Self {
            ping_success: Arc::new(AtomicU64::new(0)),
            ping_failures: Arc::new(AtomicU64::new(0)),
            ping_avg_latency: Arc::new(AtomicU64::new(0)),
            snmp_success: Arc::new(AtomicU64::new(0)),
            snmp_failures: Arc::new(AtomicU64::new(0)),
            devices_discovered: Arc::new(AtomicU64::new(0)),
            pruning_records_purged: Arc::new(AtomicU64::new(0)),
            last_pruning_run: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Incrementa el contador de ping exitosos
    pub fn increment_ping_success(&self) {
        self.ping_success.fetch_add(1, Ordering::Relaxed);
    }

    /// Incrementa el contador de ping fallidos
    pub fn increment_ping_failure(&self) {
        self.ping_failures.fetch_add(1, Ordering::Relaxed);
    }

    /// Actualiza la latencia promedio de ping
    pub fn update_ping_latency(&self, latency_ms: u64) {
        // Promedio móvil simple
        let current = self.ping_avg_latency.load(Ordering::Relaxed);
        let new_avg = (current + latency_ms) / 2;
        self.ping_avg_latency.store(new_avg, Ordering::Relaxed);
    }

    /// Incrementa el contador de SNMP exitosos
    pub fn increment_snmp_success(&self) {
        self.snmp_success.fetch_add(1, Ordering::Relaxed);
    }

    /// Incrementa el contador de SNMP fallidos
    pub fn increment_snmp_failure(&self) {
        self.snmp_failures.fetch_add(1, Ordering::Relaxed);
    }

    /// Incrementa el contador de dispositivos descubiertos
    pub fn increment_devices_discovered(&self, count: u64) {
        self.devices_discovered.fetch_add(count, Ordering::Relaxed);
    }

    /// Actualiza el contador de registros purgados
    pub fn update_pruning_records_purged(&self, count: u64) {
        self.pruning_records_purged.store(count, Ordering::Relaxed);
    }

    /// Actualiza la última ejecución de pruning (timestamp en segundos)
    pub fn update_last_pruning_run(&self) {
        self.last_pruning_run.store(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            Ordering::Relaxed,
        );
    }
}

impl Default for WorkerStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Response con estadísticas de workers
#[derive(Debug, Serialize)]
pub struct WorkerStatsResponse {
    /// Tareas de ping procesadas exitosamente
    pub ping_success: u64,
    /// Tareas de ping fallidas
    pub ping_failures: u64,
    /// Latencia promedio de ping (ms)
    pub ping_avg_latency: u64,
    /// Tareas de SNMP procesadas exitosamente
    pub snmp_success: u64,
    /// Tareas de SNMP fallidas
    pub snmp_failures: u64,
    /// Dispositivos descubiertos
    pub devices_discovered: u64,
    /// Registros purgados por pruning
    pub pruning_records_purged: u64,
    /// Última ejecución de pruning (timestamp en segundos)
    pub last_pruning_run: u64,
    /// Tasa de éxito de ping (porcentaje)
    pub ping_success_rate: f64,
    /// Tasa de éxito de SNMP (porcentaje)
    pub snmp_success_rate: f64,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Handler para obtener estadísticas de workers
pub async fn get_worker_stats(
    State(stats): State<Arc<WorkerStats>>,
) -> Result<Json<WorkerStatsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ping_success = stats.ping_success.load(Ordering::Relaxed);
    let ping_failures = stats.ping_failures.load(Ordering::Relaxed);
    let ping_avg_latency = stats.ping_avg_latency.load(Ordering::Relaxed);
    let snmp_success = stats.snmp_success.load(Ordering::Relaxed);
    let snmp_failures = stats.snmp_failures.load(Ordering::Relaxed);
    let devices_discovered = stats.devices_discovered.load(Ordering::Relaxed);
    let pruning_records_purged = stats.pruning_records_purged.load(Ordering::Relaxed);
    let last_pruning_run = stats.last_pruning_run.load(Ordering::Relaxed);

    // Calcular tasas de éxito
    let ping_total = ping_success + ping_failures;
    let ping_success_rate = if ping_total > 0 {
        (ping_success as f64 / ping_total as f64) * 100.0
    } else {
        0.0
    };

    let snmp_total = snmp_success + snmp_failures;
    let snmp_success_rate = if snmp_total > 0 {
        (snmp_success as f64 / snmp_total as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(WorkerStatsResponse {
        ping_success,
        ping_failures,
        ping_avg_latency,
        snmp_success,
        snmp_failures,
        devices_discovered,
        pruning_records_purged,
        last_pruning_run,
        ping_success_rate,
        snmp_success_rate,
    }))
}

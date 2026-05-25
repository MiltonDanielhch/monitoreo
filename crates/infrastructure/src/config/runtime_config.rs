// crates/infrastructure/src/config/runtime_config.rs
// Servicio de caché en memoria RAM para configuración del sistema
// Vinculado con ADR-0015 (Asincronía con Tokio Jobs)

use domain::{ThresholdSettings, Location};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct RuntimeConfig {
    inner: Arc<RwLock<RuntimeConfigInner>>,
}

#[derive(Clone)]
pub struct RuntimeConfigInner {
    pub thresholds: ThresholdSettings,
    pub locations: Vec<Location>,
    pub check_interval_seconds: u32,
}

impl RuntimeConfig {
    pub fn new(thresholds: ThresholdSettings, locations: Vec<Location>, check_interval_seconds: u32) -> Self {
        Self {
            inner: Arc::new(RwLock::new(RuntimeConfigInner {
                thresholds,
                locations,
                check_interval_seconds,
            })),
        }
    }

    pub async fn read(&self) -> RuntimeConfigInner {
        self.inner.read().await.clone()
    }

    pub async fn update_thresholds(&self, thresholds: ThresholdSettings) {
        let mut inner = self.inner.write().await;
        inner.thresholds = thresholds;
    }

    pub async fn update_locations(&self, locations: Vec<Location>) {
        let mut inner = self.inner.write().await;
        inner.locations = locations;
    }

    pub async fn update_check_interval(&self, interval: u32) {
        let mut inner = self.inner.write().await;
        inner.check_interval_seconds = interval;
    }

    pub async fn update_all(&self, thresholds: ThresholdSettings, locations: Vec<Location>, check_interval_seconds: u32) {
        let mut inner = self.inner.write().await;
        inner.thresholds = thresholds;
        inner.locations = locations;
        inner.check_interval_seconds = check_interval_seconds;
    }

    pub fn is_healthy(&self, ping_ms: f64, latency_ms: f64, packet_loss_percent: f64) -> bool {
        self.inner.blocking_read().thresholds.is_healthy(ping_ms, latency_ms, packet_loss_percent)
    }

    pub fn get_status(&self, ping_ms: f64, latency_ms: f64, packet_loss_percent: f64) -> &'static str {
        self.inner.blocking_read().thresholds.get_status(ping_ms, latency_ms, packet_loss_percent)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
}

impl From<&'static str> for HealthStatus {
    fn from(s: &'static str) -> Self {
        match s {
            "critical" => HealthStatus::Critical,
            "warning" => HealthStatus::Warning,
            _ => HealthStatus::Healthy,
        }
    }
}
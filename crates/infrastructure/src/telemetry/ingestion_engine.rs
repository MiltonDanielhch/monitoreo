// crates/infrastructure/src/telemetry/ingestion_engine.rs
// Motor de ingestión asíncrona con Tokio Channels
// Vinculado con ADR-0015-ingestion-asincrona.md

use database::TelemetryRepository;
use domain::models::telemetry::{TelemetryBatch, TelemetryPort};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tracing::{error, info, warn};

/// Mensaje del canal de ingestión
#[derive(Debug)]
pub enum IngestionMessage {
    /// Ingestar un solo lote de métricas
    Single(TelemetryBatch),
    /// Ingestar múltiples lotes de métricas
    Batch(Vec<TelemetryBatch>),
    /// Detener el motor
    Shutdown,
}

/// Motor de ingestión asíncrona
pub struct TelemetryIngestionEngine {
    tx: mpsc::Sender<IngestionMessage>,
    _handle: tokio::task::JoinHandle<()>,
}

impl TelemetryIngestionEngine {
    /// Crea un nuevo motor de ingestión
    pub fn new(repo: TelemetryRepository, batch_size: usize, flush_interval_secs: u64) -> Self {
        let (tx, rx) = mpsc::channel(1000);

        let handle = tokio::spawn(async move {
            Self::worker(repo, rx, batch_size, flush_interval_secs).await;
        });

        Self {
            tx,
            _handle: handle,
        }
    }

    /// Worker principal del motor de ingestión
    async fn worker(
        repo: TelemetryRepository,
        mut rx: mpsc::Receiver<IngestionMessage>,
        batch_size: usize,
        flush_interval_secs: u64,
    ) {
        let mut buffer: Vec<TelemetryBatch> = Vec::with_capacity(batch_size);
        let mut ticker = interval(Duration::from_secs(flush_interval_secs));
        ticker.tick().await; // Skip first tick

        info!("Telemetry ingestion engine started");

        loop {
            tokio::select! {
                // Recibir mensajes del canal
                msg = rx.recv() => {
                    match msg {
                        Some(IngestionMessage::Single(batch)) => {
                            buffer.push(batch);
                            if buffer.len() >= batch_size {
                                Self::flush_buffer(&repo, &mut buffer).await;
                            }
                        }
                        Some(IngestionMessage::Batch(batches)) => {
                            buffer.extend(batches);
                            if buffer.len() >= batch_size {
                                Self::flush_buffer(&repo, &mut buffer).await;
                            }
                        }
                        Some(IngestionMessage::Shutdown) => {
                            info!("Telemetry ingestion engine shutting down");
                            Self::flush_buffer(&repo, &mut buffer).await;
                            break;
                        }
                        None => {
                            warn!("Telemetry ingestion engine channel closed");
                            Self::flush_buffer(&repo, &mut buffer).await;
                            break;
                        }
                    }
                }
                // Flush periódico
                _ = ticker.tick() => {
                    if !buffer.is_empty() {
                        Self::flush_buffer(&repo, &mut buffer).await;
                    }
                }
            }
        }

        info!("Telemetry ingestion engine stopped");
    }

    /// Flush del buffer al repositorio
    async fn flush_buffer(repo: &TelemetryRepository, buffer: &mut Vec<TelemetryBatch>) {
        if buffer.is_empty() {
            return;
        }

        let batch_count = buffer.len();
        match repo.ingest_metrics_batch(buffer.clone()).await {
            Ok(_) => {
                info!("Flushed {} telemetry batches successfully", batch_count);
                buffer.clear();
            }
            Err(e) => {
                error!("Failed to flush telemetry batches: {}", e);
                // No limpiamos el buffer para reintentar en el siguiente ciclo
            }
        }
    }

    /// Envía un lote de métricas al motor de ingestión
    pub async fn ingest(&self, batch: TelemetryBatch) -> Result<(), String> {
        self.tx
            .send(IngestionMessage::Single(batch))
            .await
            .map_err(|e| format!("Failed to send telemetry batch: {}", e))
    }

    /// Envía múltiples lotes de métricas al motor de ingestión
    pub async fn ingest_batch(&self, batches: Vec<TelemetryBatch>) -> Result<(), String> {
        self.tx
            .send(IngestionMessage::Batch(batches))
            .await
            .map_err(|e| format!("Failed to send telemetry batches: {}", e))
    }

    /// Detiene el motor de ingestión
    pub async fn shutdown(self) {
        let _ = self.tx.send(IngestionMessage::Shutdown).await;
        self._handle.await.ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingestion_message_creation() {
        let batch = TelemetryBatch::new(
            "agent-123".to_string(),
            domain::models::telemetry::TelemetryMetrics::new(),
        );

        let msg = IngestionMessage::Single(batch);
        match msg {
            IngestionMessage::Single(_) => (),
            _ => panic!("Expected Single message"),
        }
    }
}

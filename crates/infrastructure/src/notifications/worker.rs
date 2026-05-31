// crates/infrastructure/src/notifications/worker.rs
// Worker de background para procesamiento de notificaciones
// Vinculado con ADR-0015-tokio-jobs.md
// Módulo 4: Motor de Notificaciones

use super::smtp_adapter::{SmtpAdapter, SmtpConfig};
use crate::AppState;
use database::{DatabaseConnection, NotificationRepository};
use domain::Result;
use std::sync::Arc;
use tokio::time::{interval, Duration};

/// Worker de notificaciones en segundo plano
/// Procesa logs pendientes de forma asíncrona sin bloquear el hilo web
pub struct NotificationWorker {
    #[allow(dead_code)]
    db: DatabaseConnection,
    notification_repo: NotificationRepository,
}

impl NotificationWorker {
    pub fn new(db: DatabaseConnection) -> Self {
        let notification_repo = NotificationRepository::new(db.clone());
        Self {
            db,
            notification_repo,
        }
    }

    /// Iniciar el worker en segundo plano
    /// Este método se ejecuta en un hilo separado y procesa notificaciones periódicamente
    pub async fn start(self) {
        tracing::info!("Iniciando Worker de Notificaciones en segundo plano");

        // Configurar intervalo de procesamiento (cada 30 segundos)
        let mut ticker = interval(Duration::from_secs(30));

        loop {
            ticker.tick().await;

            if let Err(e) = self.process_pending_notifications().await {
                tracing::error!("Error al procesar notificaciones pendientes: {}", e);
            }
        }
    }

    /// Procesar notificaciones pendientes de la base de datos
    async fn process_pending_notifications(&self) -> Result<()> {
        tracing::debug!("Buscando notificaciones pendientes...");

        // Obtener logs pendientes (máximo 10 por ciclo)
        let pending_logs = self
            .notification_repo
            .get_pending_logs(10)
            .await
            .map_err(|e| {
                tracing::error!("Error al obtener logs pendientes: {}", e);
                e
            })?;

        if pending_logs.is_empty() {
            tracing::debug!("No hay notificaciones pendientes para procesar");
            return Ok(());
        }

        tracing::info!("Procesando {} notificaciones pendientes", pending_logs.len());

        for log in pending_logs {
            let log_id = log.id.clone();

            // Procesar cada notificación
            match self.process_single_notification(&log).await {
                Ok(_) => {
                    tracing::info!("Notificación {} procesada exitosamente", log_id);
                }
                Err(e) => {
                    tracing::error!("Error al procesar notificación {}: {}", log_id, e);
                    
                    // Marcar como fallida o para reintentar
                    if log.attempt_count >= log.max_attempts {
                        let _ = self
                            .notification_repo
                            .mark_as_failed(&log_id, &e.to_string())
                            .await;
                    } else {
                        let _ = self.notification_repo.mark_for_retry(&log_id).await;
                    }
                }
            }
        }

        Ok(())
    }

    /// Procesar una notificación individual
    async fn process_single_notification(
        &self,
        log: &database::entities::notification_log_entity::Model,
    ) -> Result<()> {
        // Obtener el canal de notificación
        let channel = self
            .notification_repo
            .get_channel(&log.channel_id)
            .await?;

        // Obtener la plantilla
        let template_entity = self
            .notification_repo
            .get_template(&log.template_id)
            .await?;

        let template = self
            .notification_repo
            .entity_to_template(template_entity)?;

        // Parsear configuración SMTP del canal
        let smtp_config = SmtpConfig::from_json(&channel.config)
            .map_err(|e| domain::DomainError::Infrastructure(format!("Error en config SMTP: {}", e)))?;

        // Crear adaptador SMTP
        let smtp_adapter = SmtpAdapter::new(smtp_config);

        // Renderizar plantilla
        // Nota: En una implementación completa, aquí se cargarían los datos del payload
        // Por ahora, usamos la plantilla tal cual
        let (subject, body) = template.render(&Default::default())?;

        // Enviar correo
        smtp_adapter
            .send_email("noreply@beni.bo", &log.recipient, &subject, &body)
            .map_err(|e| domain::DomainError::Infrastructure(format!("Error al enviar correo: {}", e)))?;

        // Marcar como enviado
        self.notification_repo
            .mark_as_sent(&log.id)
            .await?;

        Ok(())
    }
}

/// Iniciar el worker de notificaciones desde el estado de la aplicación
pub fn start_notification_worker(state: Arc<AppState>) {
    tokio::spawn(async move {
        let worker = NotificationWorker::new(state.db.clone());
        worker.start().await;
    });
}

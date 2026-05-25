// crates/infrastructure/src/workers/session_cleanup.rs
// Worker en segundo plano para purga de sesiones expiradas
// Vinculado con ADR-0015-tokio-jobs.md

use database::AuthRepository;
use tokio::time::{interval, Duration};
use tracing::{info, error, instrument};

/// Inicia el worker de limpieza de sesiones en segundo plano
/// Se ejecuta periódicamente (cada hora) para eliminar sesiones expiradas
#[instrument(skip(auth_repo))]
pub fn spawn_session_cleanup_worker(auth_repo: AuthRepository) {
    tokio::spawn(async move {
        info!("Worker de limpieza de sesiones iniciado");

        // Intervalo de 1 hora (3600 segundos)
        let mut interval = interval(Duration::from_secs(3600));
        
        // Primera ejecución inmediata después de un pequeño delay
        tokio::time::sleep(Duration::from_secs(10)).await;
        
        loop {
            interval.tick().await;
            
            match cleanup_expired_sessions(&auth_repo).await {
                Ok(count) => {
                    if count > 0 {
                        info!("Sesiones expiradas eliminadas: {}", count);
                    }
                }
                Err(e) => {
                    error!("Error al limpiar sesiones expiradas: {:?}", e);
                }
            }
        }
    });
}

/// Elimina sesiones expiradas de la base de datos
#[instrument(skip(auth_repo))]
async fn cleanup_expired_sessions(auth_repo: &AuthRepository) -> Result<u64, String> {
    auth_repo.delete_expired_sessions()
        .await
        .map_err(|e| format!("Error al eliminar sesiones expiradas: {}", e))
}

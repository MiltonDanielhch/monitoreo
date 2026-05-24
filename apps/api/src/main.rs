// apps/api/src/main.rs
// Binario ejecutable - Orquestador y arranque del backend
// Vinculado con ADR-0003-stack-backend-rust-axum.md y ADR-0007-manejo-errores.md

// Este es el punto de entrada principal del servidor Axum
// Realiza la inyección de dependencias y el arranque del runtime Tokio

use database::establish_connection;
use infrastructure::{create_router, AppState};
use tokio::net::TcpListener;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Fallback seguro en caso de requerir valores por defecto nativos
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:Milton123@127.0.0.1:3306/redes_dev".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());

    let db_connection = establish_connection(&database_url).await
        .map_err(|e| anyhow::anyhow!("Fallo crítico al enlazar MySQL local: {}", e))?;

    let state = AppState { db: db_connection };
    let app = create_router(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Servidor Axum escuchando activamente en http://{}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}

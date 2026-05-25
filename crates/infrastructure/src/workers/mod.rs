// crates/infrastructure/src/workers/mod.rs
// Módulo de workers en segundo plano
// Vinculado con ADR-0015-tokio-jobs.md

pub mod session_cleanup;

pub use session_cleanup::spawn_session_cleanup_worker;

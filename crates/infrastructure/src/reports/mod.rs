// crates/infrastructure/src/reports/mod.rs
// Módulo de reportes y generación de documentos
// Vinculado con ADR-0012 (Renderizado de Documentos)

pub mod pdf_renderer;
pub mod crypto_signer;

pub use pdf_renderer::PdfRenderer;
pub use crypto_signer::{CryptoSigner, DocumentSignature};

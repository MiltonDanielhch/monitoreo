// crates/infrastructure/src/reports/crypto_signer.rs
// Firma criptográfica de integridad documental
// Vinculado con ADR-0009 (Criptografía y Seguridad)

use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Metadatos de firma criptográfica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSignature {
    /// Hash SHA-256 del documento
    pub document_hash: String,
    /// IP del operador que generó el documento
    pub operator_ip: String,
    /// Timestamp de generación (Unix epoch)
    pub generated_at: u64,
    /// ID del usuario que generó el documento
    pub generated_by: String,
    /// Versión del algoritmo de firma
    pub algorithm_version: String,
}

/// Firma criptográfica de documentos
pub struct CryptoSigner {
    /// IP del operador
    operator_ip: String,
}

impl CryptoSigner {
    /// Crear un nuevo firmador criptográfico
    pub fn new(operator_ip: String) -> Self {
        Self { operator_ip }
    }

    /// Calcular hash SHA-256 de los bytes del documento
    pub fn calculate_hash(&self, document_bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(document_bytes);
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Firmar documento con metadatos criptográficos
    pub fn sign_document(
        &self,
        document_bytes: &[u8],
        generated_by: String,
    ) -> DocumentSignature {
        let document_hash = self.calculate_hash(document_bytes);
        let generated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        DocumentSignature {
            document_hash,
            operator_ip: self.operator_ip.clone(),
            generated_at,
            generated_by,
            algorithm_version: "SHA-256-v1".to_string(),
        }
    }

    /// Verificar integridad del documento
    pub fn verify_integrity(
        &self,
        document_bytes: &[u8],
        signature: &DocumentSignature,
    ) -> bool {
        let current_hash = self.calculate_hash(document_bytes);
        current_hash == signature.document_hash
    }

    /// Inyectar metadatos de firma en el documento
    /// NOTA: Para PDFs reales, esto requeriría manipulación de metadatos PDF
    /// Por ahora, retornamos los metadatos como JSON para inyección posterior
    pub fn inject_metadata(&self, document_bytes: &[u8], signature: &DocumentSignature) -> Result<Vec<u8>, String> {
        // Implementación stub - en producción, esto inyectaría metadatos en el PDF
        let metadata_json = serde_json::to_string(signature)
            .map_err(|e| format!("Error al serializar metadatos: {}", e))?;
        
        // Retornar documento + metadatos (stub)
        let mut result = document_bytes.to_vec();
        result.extend(b"\n\nDOCUMENT METADATA:\n");
        result.extend(metadata_json.as_bytes());
        
        Ok(result)
    }
}

impl Default for CryptoSigner {
    fn default() -> Self {
        Self::new("127.0.0.1".to_string())
    }
}

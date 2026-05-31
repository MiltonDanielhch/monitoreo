// crates/infrastructure/src/notifications/smtp_adapter.rs
// Adaptador SMTP para envío de correos electrónicos
// Vinculado con ADR-0014-monitoreo-tareas-criticas.md
// Módulo 4: Motor de Notificaciones

use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use serde_json::Value;

/// Configuración SMTP extraída de la base de datos
#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub secure: bool, // true para TLS, false para STARTTLS
}

impl SmtpConfig {
    /// Crear configuración desde JSON almacenado en la base de datos
    pub fn from_json(config_json: &str) -> Result<Self, String> {
        let config: Value = serde_json::from_str(config_json)
            .map_err(|e| format!("Error al parsear config JSON: {}", e))?;

        let host = config
            .get("smtp_host")
            .and_then(|v| v.as_str())
            .ok_or("smtp_host no encontrado en config")?
            .to_string();

        let port = config
            .get("smtp_port")
            .and_then(|v| v.as_u64())
            .ok_or("smtp_port no encontrado en config")? as u16;

        let username = config
            .get("smtp_user")
            .and_then(|v| v.as_str())
            .ok_or("smtp_user no encontrado en config")?
            .to_string();

        let password = config
            .get("smtp_password")
            .and_then(|v| v.as_str())
            .ok_or("smtp_password no encontrado en config")?
            .to_string();

        let secure = config
            .get("smtp_secure")
            .and_then(|v| v.as_str())
            .unwrap_or("tls") == "tls";

        Ok(SmtpConfig {
            host,
            port,
            username,
            password,
            secure,
        })
    }
}

/// Adaptador SMTP para envío de correos
pub struct SmtpAdapter {
    config: SmtpConfig,
}

impl SmtpAdapter {
    pub fn new(config: SmtpConfig) -> Self {
        Self { config }
    }

    /// Enviar correo electrónico
    pub fn send_email(
        &self,
        from: &str,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), String> {
        // Crear el mensaje de correo
        let email = Message::builder()
            .from(from.parse().map_err(|e| format!("Error en from: {}", e))?)
            .to(to.parse().map_err(|e| format!("Error en to: {}", e))?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())
            .map_err(|e| format!("Error al construir mensaje: {}", e))?;

        // Configurar el transporte SMTP
        let creds = Credentials::new(self.config.username.clone(), self.config.password.clone());

        let mailer = if self.config.secure {
            // TLS
            SmtpTransport::relay(&self.config.host)
                .map_err(|e| format!("Error al crear relay TLS: {}", e))?
                .port(self.config.port)
                .credentials(creds)
                .build()
        } else {
            // STARTTLS
            SmtpTransport::starttls_relay(&self.config.host)
                .map_err(|e| format!("Error al crear relay STARTTLS: {}", e))?
                .port(self.config.port)
                .credentials(creds)
                .build()
        };

        // Enviar el correo
        mailer
            .send(&email)
            .map_err(|e| format!("Error al enviar correo: {}", e))?;

        tracing::info!("Correo enviado exitosamente a {}", to);
        Ok(())
    }

    /// Probar conexión SMTP
    pub fn test_connection(&self) -> Result<(), String> {
        let creds = Credentials::new(self.config.username.clone(), self.config.password.clone());

        let mailer = if self.config.secure {
            SmtpTransport::relay(&self.config.host)
                .map_err(|e| format!("Error al crear relay TLS: {}", e))?
                .port(self.config.port)
                .credentials(creds)
                .build()
        } else {
            SmtpTransport::starttls_relay(&self.config.host)
                .map_err(|e| format!("Error al crear relay STARTTLS: {}", e))?
                .port(self.config.port)
                .credentials(creds)
                .build()
        };

        // Intentar una conexión de prueba
        mailer
            .test_connection()
            .map_err(|e| format!("Error de conexión SMTP: {}", e))?;

        tracing::info!("Conexión SMTP probada exitosamente");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smtp_config_from_json() {
        let json = r#"{
            "smtp_host": "smtp.gmail.com",
            "smtp_port": 587,
            "smtp_user": "test@example.com",
            "smtp_password": "password123",
            "smtp_secure": "tls"
        }"#;

        let config = SmtpConfig::from_json(json).unwrap();
        assert_eq!(config.host, "smtp.gmail.com");
        assert_eq!(config.port, 587);
        assert_eq!(config.username, "test@example.com");
        assert!(config.secure);
    }
}

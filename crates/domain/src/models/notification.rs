// crates/domain/src/models/notification.rs
// Modelos de negocio puros para el sistema de notificaciones
// Vinculado con ADR-0001-arquitectura-hexagonal.md y ADR-0007-manejo-errores.md
// Módulo 4: Motor de Notificaciones en Segundo Plano

use crate::errors::{DomainError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Tipo de canal de notificación
/// El dominio solo entiende estos tipos abstractos, no protocolos concretos como SMTP
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChannelType {
    Email,
    Telegram,
    Webhook,
}

/// Tipo de evento que dispara una notificación
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationEventType {
    NodeDown,
    HighLatency,
    BandwidthSaturation,
    DeviceUnauthorized,
}

/// Estado de una notificación en el sistema
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum NotificationStatus {
    Pending,
    Sent,
    Failed,
    Retrying,
}

/// Datos de contexto para una alerta de nodo caído
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDownContext {
    pub host: String,
    pub location: String,
    pub latency: u32,
    pub detected_at: String,
}

/// Datos de contexto para una alerta de latencia alta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighLatencyContext {
    pub host: String,
    pub latency: u32,
    pub threshold: u32,
    pub location: String,
}

/// Datos de contexto para una alerta de saturación de ancho de banda
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthSaturationContext {
    pub host: String,
    pub usage: u8,
    pub current_gbps: f64,
    pub max_gbps: f64,
    pub location: String,
}

/// Datos de contexto para una alerta de dispositivo no autorizado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceUnauthorizedContext {
    pub mac_address: String,
    pub ip_address: String,
    pub port: u16,
    pub host: String,
}

/// Payload unificado de datos para una notificación
/// El dominio encapsula aquí toda la información necesaria sin conocer protocolos
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum NotificationPayload {
    #[serde(rename = "NODE_DOWN")]
    NodeDown(NodeDownContext),
    #[serde(rename = "HIGH_LATENCY")]
    HighLatency(HighLatencyContext),
    #[serde(rename = "BANDWIDTH_SATURATION")]
    BandwidthSaturation(BandwidthSaturationContext),
    #[serde(rename = "DEVICE_UNAUTHORIZED")]
    DeviceUnauthorized(DeviceUnauthorizedContext),
}

impl NotificationPayload {
    /// Extraer todos los placeholders disponibles para renderizado de plantillas
    pub fn to_template_vars(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        
        match self {
            NotificationPayload::NodeDown(ctx) => {
                vars.insert("host".to_string(), ctx.host.clone());
                vars.insert("location".to_string(), ctx.location.clone());
                vars.insert("latency".to_string(), ctx.latency.to_string());
                vars.insert("detected_at".to_string(), ctx.detected_at.clone());
            }
            NotificationPayload::HighLatency(ctx) => {
                vars.insert("host".to_string(), ctx.host.clone());
                vars.insert("latency".to_string(), ctx.latency.to_string());
                vars.insert("threshold".to_string(), ctx.threshold.to_string());
                vars.insert("location".to_string(), ctx.location.clone());
            }
            NotificationPayload::BandwidthSaturation(ctx) => {
                vars.insert("host".to_string(), ctx.host.clone());
                vars.insert("usage".to_string(), ctx.usage.to_string());
                vars.insert("current_gbps".to_string(), ctx.current_gbps.to_string());
                vars.insert("max_gbps".to_string(), ctx.max_gbps.to_string());
                vars.insert("location".to_string(), ctx.location.clone());
            }
            NotificationPayload::DeviceUnauthorized(ctx) => {
                vars.insert("mac_address".to_string(), ctx.mac_address.clone());
                vars.insert("ip_address".to_string(), ctx.ip_address.clone());
                vars.insert("port".to_string(), ctx.port.to_string());
                vars.insert("host".to_string(), ctx.host.clone());
            }
        }
        
        vars
    }

    /// Obtener el tipo de evento asociado al payload
    pub fn event_type(&self) -> NotificationEventType {
        match self {
            NotificationPayload::NodeDown(_) => NotificationEventType::NodeDown,
            NotificationPayload::HighLatency(_) => NotificationEventType::HighLatency,
            NotificationPayload::BandwidthSaturation(_) => NotificationEventType::BandwidthSaturation,
            NotificationPayload::DeviceUnauthorized(_) => NotificationEventType::DeviceUnauthorized,
        }
    }
}

/// Plantilla de notificación del dominio
/// Contiene la estructura lógica sin conocer almacenamiento físico
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTemplate {
    pub id: String,
    pub name: String,
    pub template_type: NotificationEventType,
    pub subject: String,
    pub body: String,
    pub is_active: bool,
}

impl NotificationTemplate {
    /// Validar que la plantilla tenga los campos mínimos requeridos
    pub fn validate(&self) -> Result<()> {
        if self.subject.trim().is_empty() {
            return Err(DomainError::TemplateRenderError(
                "El asunto de la plantilla no puede estar vacío".to_string(),
            ));
        }
        
        if self.body.trim().is_empty() {
            return Err(DomainError::TemplateRenderError(
                "El cuerpo de la plantilla no puede estar vacío".to_string(),
            ));
        }
        
        Ok(())
    }

    /// Renderizar la plantilla reemplazando placeholders con valores
    /// Esta es una implementación simple de reemplazo de {{placeholder}}
    pub fn render(&self, vars: &HashMap<String, String>) -> Result<(String, String)> {
        let rendered_subject = self.render_text(&self.subject, vars)?;
        let rendered_body = self.render_text(&self.body, vars)?;
        
        Ok((rendered_subject, rendered_body))
    }

    fn render_text(&self, text: &str, vars: &HashMap<String, String>) -> Result<String> {
        let mut result = text.to_string();
        
        for (key, value) in vars {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        Ok(result)
    }
}

/// Solicitud de envío de notificación desde el dominio
/// Contrato puro que no conoce implementaciones de transporte
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRequest {
    pub channel_id: String,
    pub template_id: String,
    pub recipient: String,
    pub payload: NotificationPayload,
}

impl NotificationRequest {
    /// Validar que la solicitud tenga todos los campos requeridos
    pub fn validate(&self) -> Result<()> {
        if self.channel_id.trim().is_empty() {
            return Err(DomainError::InvalidRecipient(
                "El ID del canal no puede estar vacío".to_string(),
            ));
        }
        
        if self.template_id.trim().is_empty() {
            return Err(DomainError::TemplateNotFound);
        }
        
        if self.recipient.trim().is_empty() {
            return Err(DomainError::InvalidRecipient(
                "El destinatario no puede estar vacío".to_string(),
            ));
        }
        
        // Validación básica de formato de email si es canal de email
        // El dominio puede hacer validaciones de formato sin conocer SMTP
        if self.recipient.contains('@') && !self.recipient.contains('@') {
            return Err(DomainError::InvalidRecipient(
                "Formato de destinatario inválido".to_string(),
            ));
        }
        
        Ok(())
    }
}

/// Resultado de un intento de envío de notificación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationResult {
    pub status: NotificationStatus,
    pub error_message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_rendering() {
        let template = NotificationTemplate {
            id: "test".to_string(),
            name: "Test".to_string(),
            template_type: NotificationEventType::NodeDown,
            subject: "Alerta: {{host}} caído".to_string(),
            body: "El nodo {{host}} en {{location}} está offline".to_string(),
            is_active: true,
        };

        let mut vars = HashMap::new();
        vars.insert("host".to_string(), "router-01".to_string());
        vars.insert("location".to_string(), "Trinidad".to_string());

        let (subject, body) = template.render(&vars).unwrap();
        assert_eq!(subject, "Alerta: router-01 caído");
        assert_eq!(body, "El nodo router-01 en Trinidad está offline");
    }

    #[test]
    fn test_payload_to_template_vars() {
        let payload = NotificationPayload::NodeDown(NodeDownContext {
            host: "router-01".to_string(),
            location: "Trinidad".to_string(),
            latency: 0,
            detected_at: "2024-01-01 10:00:00".to_string(),
        });

        let vars = payload.to_template_vars();
        assert_eq!(vars.get("host"), Some(&"router-01".to_string()));
        assert_eq!(vars.get("location"), Some(&"Trinidad".to_string()));
    }

    #[test]
    fn test_notification_request_validation() {
        let request = NotificationRequest {
            channel_id: "channel-1".to_string(),
            template_id: "template-1".to_string(),
            recipient: "admin@example.com".to_string(),
            payload: NotificationPayload::NodeDown(NodeDownContext {
                host: "router-01".to_string(),
                location: "Trinidad".to_string(),
                latency: 0,
                detected_at: "2024-01-01 10:00:00".to_string(),
            }),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_notification_request_invalid_recipient() {
        let request = NotificationRequest {
            channel_id: "channel-1".to_string(),
            template_id: "template-1".to_string(),
            recipient: "".to_string(),
            payload: NotificationPayload::NodeDown(NodeDownContext {
                host: "router-01".to_string(),
                location: "Trinidad".to_string(),
                latency: 0,
                detected_at: "2024-01-01 10:00:00".to_string(),
            }),
        };

        assert!(request.validate().is_err());
    }
}

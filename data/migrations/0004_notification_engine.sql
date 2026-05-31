-- data/migrations/0004_notification_engine.sql
-- Módulo 4: Motor de Notificaciones en Segundo Plano
-- Vinculado con ADR-0004 (Persistencia MySQL) y ADR-0005 (Migraciones)
-- Código 3026 - Lab 3030

-- Tabla de canales de notificación (Email, Telegram, etc.)
CREATE TABLE IF NOT EXISTS notification_channels (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUIDv7 del canal de notificación',
    name VARCHAR(100) NOT NULL COMMENT 'Nombre descriptivo del canal (ej: Email Operativo)',
    channel_type ENUM('EMAIL', 'TELEGRAM', 'WEBHOOK') NOT NULL COMMENT 'Tipo de canal de comunicación',
    config JSON NOT NULL COMMENT 'Configuración del canal (SMTP settings, bot token, etc.)',
    is_active BOOLEAN DEFAULT TRUE COMMENT 'Estado del canal (activo/inactivo)',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL COMMENT 'Soft delete - ADR-0004',
    INDEX idx_channel_type (channel_type),
    INDEX idx_is_active (is_active),
    INDEX idx_deleted_at (deleted_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='Canales de envío de notificaciones';

-- Tabla de plantillas de notificación
CREATE TABLE IF NOT EXISTS notification_templates (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUIDv7 de la plantilla',
    name VARCHAR(100) NOT NULL COMMENT 'Nombre de la plantilla (ej: Alerta Caída Nodo)',
    template_type ENUM('NODE_DOWN', 'HIGH_LATENCY', 'BANDWIDTH_SATURATION', 'DEVICE_UNAUTHORIZED') NOT NULL COMMENT 'Tipo de evento que dispara la plantilla',
    subject VARCHAR(255) NOT NULL COMMENT 'Asunto del correo/mensaje',
    body TEXT NOT NULL COMMENT 'Cuerpo del mensaje con placeholders ({{host}}, {{latency}}, etc.)',
    is_active BOOLEAN DEFAULT TRUE COMMENT 'Estado de la plantilla',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL COMMENT 'Soft delete - ADR-0004',
    INDEX idx_template_type (template_type),
    INDEX idx_is_active (is_active),
    INDEX idx_deleted_at (deleted_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='Plantillas preconfiguradas de notificaciones';

-- Tabla de historial de logs de notificaciones
CREATE TABLE IF NOT EXISTS notification_logs (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUIDv7 del log de notificación',
    channel_id VARCHAR(36) NOT NULL COMMENT 'ID del canal utilizado',
    template_id VARCHAR(36) NOT NULL COMMENT 'ID de la plantilla utilizada',
    recipient VARCHAR(255) NOT NULL COMMENT 'Destinatario (email, chat_id, webhook URL)',
    status ENUM('PENDING', 'SENT', 'FAILED', 'RETRYING') DEFAULT 'PENDING' COMMENT 'Estado del envío',
    attempt_count INT DEFAULT 0 COMMENT 'Número de intentos de reenvío',
    max_attempts INT DEFAULT 3 COMMENT 'Máximo de intentos permitidos',
    error_message TEXT NULL COMMENT 'Mensaje de error si falló el envío',
    sent_at TIMESTAMP NULL COMMENT 'Timestamp cuando se envió exitosamente',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL COMMENT 'Soft delete - ADR-0004',
    FOREIGN KEY (channel_id) REFERENCES notification_channels(id) ON DELETE CASCADE,
    FOREIGN KEY (template_id) REFERENCES notification_templates(id) ON DELETE CASCADE,
    INDEX idx_channel_id (channel_id),
    INDEX idx_template_id (template_id),
    INDEX idx_status (status),
    INDEX idx_created_at (created_at),
    INDEX idx_deleted_at (deleted_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='Historial de envíos y reintentos de notificaciones';

-- Inserción de datos semilla (Seeds) - ADR-0005
-- Canal de email por defecto
INSERT INTO notification_channels (id, name, channel_type, config, is_active) VALUES
('550e8400-e29b-41d4-a716-446655440001', 'Email Operativo', 'EMAIL', 
 '{"smtp_host":"smtp.gmail.com","smtp_port":587,"smtp_user":"noreply@beni.bo","smtp_secure":"tls"}', TRUE);

-- Plantillas por defecto para alertas comunes
INSERT INTO notification_templates (id, name, template_type, subject, body, is_active) VALUES
('550e8400-e29b-41d4-a716-446655440002', 'Alerta Caída de Nodo', 'NODE_DOWN',
 '⚠️ ALERTA CRÍTICA: Nodo {{host}} Caído',
 'El nodo {{host}} ubicado en {{location}} ha sido detectado como OFFLINE.\n\nÚltima latencia reportada: {{latency}}ms\nHora de detección: {{detected_at}}\n\nPor favor, verifique la conectividad inmediatamente.', TRUE),

('550e8400-e29b-41d4-a716-446655440003', 'Alerta Latencia Alta', 'HIGH_LATENCY',
 '⚡ ALERTA: Latencia Elevada en {{host}}',
 'El nodo {{host}} está experimentando latencia superior al umbral crítico.\n\nLatencia actual: {{latency}}ms\nUmbral crítico: {{threshold}}ms\nUbicación: {{location}}', TRUE),

('550e8400-e29b-41d4-a716-446655440004', 'Alerta Saturación Ancho de Banda', 'BANDWIDTH_SATURATION',
 '📊 ALERTA: Saturación de Ancho de Banda en {{host}}',
 'El nodo {{host}} ha alcanzado el {{usage}}% de su capacidad de ancho de banda.\n\nConsumo actual: {{current_gbps}} Gbps\nCapacidad total: {{max_gbps}} Gbps\nUbicación: {{location}}', TRUE),

('550e8400-e29b-41d4-a716-446655440005', 'Alerta Dispositivo No Autorizado', 'DEVICE_UNAUTHORIZED',
 '🚨 ALERTA DE SEGURIDAD: Dispositivo No Autorizado Detectado',
 'Se ha detectado un dispositivo no autorizado en la red.\n\nMAC Address: {{mac_address}}\nIP Address: {{ip_address}}\nPuerto de conexión: {{port}}\nNodo de detección: {{host}}\n\nPor favor, investigue este incidente de inmediato.', TRUE);

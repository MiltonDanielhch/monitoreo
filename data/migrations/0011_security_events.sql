-- data/migrations/0011_security_events.sql
-- Módulo 11: Detección de Intrusiones y Seguridad
-- Tabla de eventos de seguridad para detección de intrusiones
-- Vinculado con ADR-0004 (Persistencia con Sea-ORM) y ADR-0005 (Diseño de esquema relacional)

-- Crear tabla de eventos de seguridad
CREATE TABLE IF NOT EXISTS security_events (
    id VARCHAR(255) PRIMARY KEY,
    event_type VARCHAR(100) NOT NULL COMMENT 'Tipo de intrusión: port_scan, ddos, unauthorized_access, malware_detection, phishing, data_exfiltration, other',
    severity VARCHAR(20) NOT NULL COMMENT 'Severidad: critical, high, medium, low',
    status VARCHAR(50) NOT NULL DEFAULT 'detected' COMMENT 'Estado: detected, investigating, resolved, false_positive',
    source_ip VARCHAR(45) NOT NULL COMMENT 'IP origen del ataque (IPv4 o IPv6)',
    source_mac VARCHAR(17) NULL COMMENT 'MAC address origen (formato XX:XX:XX:XX:XX:XX)',
    target_device_id VARCHAR(255) NULL COMMENT 'ID del dispositivo objetivo',
    target_sede_id VARCHAR(255) NULL COMMENT 'ID de la sede objetivo',
    description TEXT NOT NULL COMMENT 'Descripción del evento de seguridad',
    metadata TEXT NULL COMMENT 'JSON con detalles adicionales del evento',
    detected_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Timestamp de detección del evento',
    resolved_at DATETIME NULL COMMENT 'Timestamp de resolución del evento',
    resolved_by VARCHAR(255) NULL COMMENT 'Usuario que resolvió el evento',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Timestamp de creación del registro',
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT 'Timestamp de última actualización'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='Eventos de seguridad y detección de intrusiones';

-- Índices para optimizar consultas frecuentes
CREATE INDEX idx_security_events_severity ON security_events(severity);
CREATE INDEX idx_security_events_status ON security_events(status);
CREATE INDEX idx_security_events_detected_at ON security_events(detected_at DESC);
CREATE INDEX idx_security_events_source_ip ON security_events(source_ip);
CREATE INDEX idx_security_events_target_device ON security_events(target_device_id);
CREATE INDEX idx_security_events_target_sede ON security_events(target_sede_id);
CREATE INDEX idx_security_events_event_type ON security_events(event_type);

-- Índice compuesto para consultas de filtrado común
CREATE INDEX idx_security_events_severity_status ON security_events(severity, status);
CREATE INDEX idx_security_events_detected_at_status ON security_events(detected_at DESC, status);

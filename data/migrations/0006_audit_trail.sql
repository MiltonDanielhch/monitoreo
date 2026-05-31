-- Módulo 6: Auditoría Dinámica Inmutable
-- Slice 6.1: Esquema Estricto Append-Only (MySQL Workbench)
-- Tabla de auditoría append-only - Solo escritura, sin UPDATE ni DELETE
-- Vinculado con ADR-0004-persistencia-sea-orm.md y ADR-0009-auditoria-inmutable.md

CREATE TABLE IF NOT EXISTS audit_trail (
    id VARCHAR(36) PRIMARY KEY,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    user_id VARCHAR(36) COMMENT 'Usuario que realizó la acción',
    action VARCHAR(50) NOT NULL COMMENT 'Tipo de acción: CREATE, UPDATE, DELETE, LOGIN, LOGOUT, etc.',
    entity_type VARCHAR(100) NOT NULL COMMENT 'Tipo de entidad afectada: User, Device, Alert, etc.',
    entity_id VARCHAR(36) COMMENT 'ID de la entidad afectada',
    old_value JSON COMMENT 'Estado anterior de la entidad (JSON)',
    new_value JSON COMMENT 'Nuevo estado de la entidad (JSON)',
    ip_address VARCHAR(45) COMMENT 'Dirección IP del cliente',
    user_agent VARCHAR(500) COMMENT 'User agent del navegador',
    metadata JSON COMMENT 'Información adicional contextual',
    INDEX idx_timestamp (timestamp),
    INDEX idx_user_id (user_id),
    INDEX idx_entity (entity_type, entity_id),
    INDEX idx_action (action),
    CONSTRAINT fk_audit_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

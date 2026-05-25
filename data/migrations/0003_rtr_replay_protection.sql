-- Migración para agregar tabla de tokens de refresh usados (RTR replay protection)
-- Vinculado con ADR-0006-rbac-sessions-audit.md
-- Base de datos: redes_dev

CREATE TABLE IF NOT EXISTS used_refresh_tokens (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUIDv7',
    token_hash VARCHAR(64) NOT NULL UNIQUE COMMENT 'Hash SHA-256 del token ya usado',
    user_id VARCHAR(36) NOT NULL COMMENT 'ID del usuario propietario',
    used_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Timestamp cuando fue usado',
    INDEX idx_used_refresh_tokens_hash (token_hash),
    INDEX idx_used_refresh_tokens_user_id (user_id),
    INDEX idx_used_refresh_tokens_expires (used_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
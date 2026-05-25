-- Migración para cambiar user_sessions de TIMESTAMP a DATETIME
-- Vinculado con ADR-0005 (Migraciones)
-- Base de datos: redes_dev

-- Eliminar tabla user_sessions
DROP TABLE IF EXISTS user_sessions;

-- Recrear tabla user_sessions con DATETIME en lugar de TIMESTAMP
CREATE TABLE IF NOT EXISTS user_sessions (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUIDv7 para indexación eficiente',
    user_id VARCHAR(36) NOT NULL COMMENT 'ID del usuario propietario de la sesión',
    refresh_token_hash VARCHAR(64) NOT NULL COMMENT 'Hash SHA-256 del refresh token opaco',
    expires_at DATETIME NOT NULL COMMENT 'Fecha de expiración del refresh token',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    INDEX idx_user_sessions_user_id (user_id),
    INDEX idx_user_sessions_refresh_token_hash (refresh_token_hash),
    INDEX idx_user_sessions_expires_at (expires_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Migración inicial para autenticación y sesiones
-- Vinculado con ADR-0004 (MySQL), ADR-0005 (Migraciones), ADR-0008 (PASETO)
-- Base de datos: redes_dev

-- Crear tabla roles
CREATE TABLE IF NOT EXISTS roles (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUIDv7 para indexación eficiente',
    name VARCHAR(50) NOT NULL UNIQUE COMMENT 'Nombre del rol (ADMIN, OPERATOR, MONITOR)',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Crear tabla users
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUIDv7 para indexación eficiente',
    email VARCHAR(255) NOT NULL UNIQUE COMMENT 'Correo electrónico del usuario',
    password_hash VARCHAR(255) NOT NULL COMMENT 'Hash Argon2id de la contraseña',
    role_id VARCHAR(36) NOT NULL COMMENT 'ID del rol del usuario',
    is_active BOOLEAN DEFAULT TRUE COMMENT 'Estado del usuario (activo/suspendido)',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE RESTRICT ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Crear tabla user_sessions
CREATE TABLE IF NOT EXISTS user_sessions (
    id VARCHAR(36) PRIMARY KEY COMMENT 'UUIDv7 para indexación eficiente',
    user_id VARCHAR(36) NOT NULL COMMENT 'ID del usuario propietario de la sesión',
    refresh_token_hash VARCHAR(64) NOT NULL COMMENT 'Hash SHA-256 del refresh token opaco',
    expires_at TIMESTAMP NOT NULL COMMENT 'Fecha de expiración del refresh token',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    INDEX idx_user_sessions_user_id (user_id),
    INDEX idx_user_sessions_refresh_token_hash (refresh_token_hash),
    INDEX idx_user_sessions_expires_at (expires_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Seed para usuario admin
-- Vinculado con ADR-0005 (Seeding)
-- Base de datos: redes_dev

-- Insertar usuario admin: admin@lab3030.bo
-- Contraseña: Admin123! (hash generado con Argon2id)
INSERT INTO users (id, email, password_hash, role_id, is_active) VALUES
('018f0000-0000-0000-0000-000000000010', 'admin@lab3030.bo', '$argon2id$v=19$m=19456,t=2,p=1$USD4MhLoZEqPv06yTH7ElA$5J+G5PGv5HByWpc98BojG3GpWsEz3gkgmL5PbU0KfKk', '018f0000-0000-0000-0000-000000000001', TRUE)
ON DUPLICATE KEY UPDATE 
    password_hash=VALUES(password_hash),
    role_id=VALUES(role_id),
    is_active=VALUES(is_active);

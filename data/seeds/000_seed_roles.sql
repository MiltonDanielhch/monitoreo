-- Seed para roles
-- Vinculado con ADR-0005 (Seeding)
-- Base de datos: redes_dev

-- Insertar roles básicos
INSERT INTO roles (id, name) VALUES
('018f0000-0000-0000-0000-000000000001', 'ADMIN'),
('018f0000-0000-0000-0000-000000000002', 'OPERATOR'),
('018f0000-0000-0000-0000-000000000003', 'MONITOR')
ON DUPLICATE KEY UPDATE 
    name=VALUES(name);

-- Seed para Slice 2.1: Sedes del Beni
-- Vinculado con ADR-0004 (Persistencia MySQL) y ADR-0005 (Migraciones)
-- Base de datos: redes_dev

INSERT INTO locations (id, name, code, region, latitude, longitude, is_active) VALUES
('018f0000-0000-0000-0000-000000000020', 'Trinidad', 'TDD', 'Beni', -14.8333, -64.9000, TRUE),
('018f0000-0000-0000-0000-000000000021', 'Riberalta', 'RBT', 'Beni', -10.9833, -66.1000, TRUE),
('018f0000-0000-0000-0000-000000000022', 'Guayaramerín', 'GYA', 'Beni', -10.8167, -65.7833, TRUE),
('018f0000-0000-0000-0000-000000000023', 'San Ignacio de Moxos', 'SIM', 'Beni', -14.5333, -65.7500, TRUE),
('018f0000-0000-0000-0000-000000000024', 'Santa Ana de Yacuma', 'SAY', 'Beni', -13.7500, -67.1833, TRUE);
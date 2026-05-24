# ==============================================================================
# 🛠️ JUSTFILE OPERATIVO POR MÓDULOS — CÓDIGO 3026 (Docker Edition)
# Sistema de Monitoreo de Infraestructura Regional - Gobernación del Beni
# ==============================================================================

set shell := ["sh", "-c"]

# Muestra la lista de comandos estructurados por módulo
default:
    @just --list

# ------------------------------------------------------------------------------
# 📦 MÓDULO 0 & 1: GÉNESIS Y DOMINIO PURO
# ------------------------------------------------------------------------------

# Valida que el toolchain de Rust, Docker y pnpm estén listos en el sistema
doctor:
    @echo "=== [CÓDIGO 3026] Verificando Entorno ==="
    @rustc --version || (echo "❌ Rust no instalado"; exit 1)
    @docker --version || (echo "❌ Docker no instalado"; exit 1)
    @pnpm --version || (echo "❌ pnpm no instalado"; exit 1)
    @echo "✅ Entorno listo para iniciar."

# Comprobar la sintaxis y pureza del dominio sin dependencias externas
check-domain:
    cargo check -p domain
    cargo clippy -p domain -- -D warnings

# Ejecutar las pruebas unitarias de las reglas de negocio inmutables
test-domain:
    cargo test -p domain

# ------------------------------------------------------------------------------
# 🗄️ MÓDULO 2 & 3: BASE DE DATOS, MIGRACIONES Y REPOSITORIOS
# ------------------------------------------------------------------------------

# Levantar el contenedor oficial de la base de datos de desarrollo (PostgreSQL)
db-up:
    docker run --name redes-db-dev -e POSTGRES_DB=redes_beni -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=secret -p 5432:5432 -d postgres:16-alpine

# Verificar si el contenedor de la base de datos está corriendo de forma correcta
db-status:
    docker ps --filter name=redes-db-dev

# Ver los logs del motor PostgreSQL en tiempo real para debugging de queries
db-logs:
    docker logs -f redes-db-dev

# Ejecutar las migraciones de Sea-ORM / SQLx leyendo las credenciales de entorno
db-migrate:
    #!/usr/bin/env bash
    if [ -f .env.local ]; then export $(grep -v '^#' .env.local | xargs); fi; \
    cargo run -p database --bin migrate

# Comprobar compilación del módulo de persistencia y repositorios
check-db:
    cargo check -p database

# ------------------------------------------------------------------------------
# 🔐 MÓDULO 4: AUTENTICACIÓN Y CRIPTOGRAFÍA (PASETO v4 + ARGON2ID)
# ------------------------------------------------------------------------------

# Comprobar el módulo de seguridad, firmas y hashes de contraseñas
check-auth:
    cargo check -p auth
    cargo clippy -p auth -- -D warnings

# Ejecutar los tests criptográficos de tokens PASETO y Argon2id (OWASP 2025)
test-auth:
    cargo test -p auth

# ------------------------------------------------------------------------------
# 🌐 MÓDULO 5 & 6: CORE API (AXUM 0.8) Y AUDITORÍA
# ------------------------------------------------------------------------------

# Iniciar el monitor continuo inteligente Bacon para el Backend
watch-backend:
    bacon

# Compilar y ejecutar el servidor API central con las variables del entorno local
run-api:
    #!/usr/bin/env bash
    if [ -f .env.local ]; then export $(grep -v '^#' .env.local | xargs); fi; \
    cargo run --bin api

# Comprobar la sintaxis de toda la capa de infraestructura web
check-infra:
    cargo check -p infrastructure

# ------------------------------------------------------------------------------
# 🖥️ MÓDULO 7 & 8: FRONTEND (SVELTE 5) Y TELEMETRÍA/ALERTAS EN TIEMPO REAL
# ------------------------------------------------------------------------------

# Levantar el servidor de desarrollo interactivo de la App Web (SvelteKit 2)
run-web:
    cd apps/web && pnpm dev

# Forzar el chequeo estricto de tipos de TypeScript y Runes en el Frontend
check-web:
    cd apps/web && pnpm check

# Compilar y correr el Agente de Monitoreo distribuido para las sedes remotas
run-agent:
    #!/usr/bin/env bash
    if [ -f .env.local ]; then export $(grep -v '^#' .env.local | xargs); fi; \
    cargo run --bin agent

# ------------------------------------------------------------------------------
# 📊 MÓDULO 9: REPORTES / EXPORT (PDF GENERATION)
# ------------------------------------------------------------------------------

# Ejecutar pruebas de integración específicas para la exportación de PDFs firmados
test-reporting:
    #!/usr/bin/env bash
    if [ -f .env.local ]; then export $(grep -v '^#' .env.local | xargs); fi; \
    cargo test --package infrastructure --reporting

# ------------------------------------------------------------------------------
# 🚀 MÓDULO 10: DEPLOY Y AUDITORÍA GLOBAL DE CALIDAD (GATEKEEPER)
# ------------------------------------------------------------------------------

# Formatear el código de manera consistente en todo el monorepo antes del commit
format-all:
    cargo fmt --all
    cd apps/web && pnpm exec prettier --write .

# Análisis global de sintaxis, advertencias y tipado completo de la plataforma
check-project:
    cargo check --workspace --all-targets
    cargo clippy --workspace --all-targets -- -D warnings
    cd apps/web && pnpm check

# Correr toda la suite de pruebas unitarias y de integración del ecosistema
test-all:
    #!/usr/bin/env bash
    if [ -f .env.local ]; then export $(grep -v '^#' .env.local | xargs); fi; \
    cargo test --workspace

# Auditar la seguridad de las dependencias contra vulnerabilidades conocidas
audit-security:
    cargo deny check
    cargo audit
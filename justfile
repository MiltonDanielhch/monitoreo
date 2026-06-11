# Comandos de automatización rápida — Laboratorio 3030 / Código 3026

# Configurar shell para Windows
set shell := ["powershell", "-c"]

# Monitorear continuamente errores sintácticos del backend en tiempo real usando Bacon
watch-backend:
    bacon

# Levantar el servidor de la API leyendo las variables locales de entorno sobre el metal
run-api:
    Get-Content .env.local | ForEach-Object { if ($_ -match '^([^=]+)=(.*)$') { [Environment]::SetEnvironmentVariable($matches[1], $matches[2]) } }; cargo run --bin api

# Lanzar el entorno de desarrollo local rápido para el frontend en Svelte 5 (Vite)
run-web:
    cd apps/web; pnpm dev

# Comprobación de tipos integral en todo el monorepo de forma simultánea
check-all:
    cargo check --workspace
    cd apps/web; pnpm check

# --- Comandos de Base de Datos (ADR-0005) ---

# Obtener URL de base de datos desde .env.local
get-db-url:
    powershell -Command "Get-Content .env.local | Select-String 'DATABASE_URL' | ForEach-Object { $_.ToString().Split('=')[1] }"

# Ejecutar migraciones y seeds de desarrollo
db-migrate:
    powershell -File ./scripts/db-migrate.ps1

# Ejecutar migraciones y seeds en producción (con confirmación)
db-migrate-prod:
    powershell -File ./scripts/db-migrate-prod.ps1

# Ver estado de migraciones aplicadas
db-status:
    powershell -File ./scripts/db-status.ps1

# Resetear base de datos local (peligroso - solo desarrollo)
db-reset:
    powershell -File ./scripts/db-reset.ps1

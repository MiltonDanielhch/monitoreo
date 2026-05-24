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

# ==========================================
# ETAPA 1: Planificador (cargo-chef)
# ==========================================
FROM rust:1.85-alpine AS planner
WORKDIR /app
RUN apk add --no-cache musl-dev gcc
RUN cargo install cargo-chef --version 0.1.68

# Copiar manifiestos para analizar dependencias del monorepo
COPY Cargo.toml Cargo.lock ./
COPY crates/domain/Cargo.toml ./crates/domain/
COPY crates/database/Cargo.toml ./crates/database/
COPY crates/infrastructure/Cargo.toml ./crates/infrastructure/
COPY crates/shared_types/Cargo.toml ./crates/shared_types/
COPY apps/api/Cargo.toml ./apps/api/

RUN cargo chef prepare --recipe-path recipe.json

# ==========================================
# ETAPA 2: Constructor de Dependencias (Caché)
# ==========================================
FROM rust:1.85-alpine AS builder
WORKDIR /app
RUN apk add --no-cache musl-dev gcc libressl-dev
RUN cargo install cargo-chef --version 0.1.68

# Copiar la receta analizada en la Etapa 1
COPY --from=planner /app/recipe.json recipe.json

# Compilar solo las dependencias en modo Release (Capas de caché de Docker)
RUN cargo chef cook --release --recipe-path recipe.json

# ==========================================
# ETAPA 3: Compilación del Código Fuente
# ==========================================
COPY . .
# Forzar la compilación estática del binario nativo de Axum
RUN cargo build --release --bin api

# ==========================================
# ETAPA 4: Imagen de Ejecución Inmaculada (Runtime)
# ==========================================
# Usamos distroless/cc para máxima seguridad y mínimo tamaño (contiene musl y certificados SSL)
FROM gcr.io/distroless/cc-debian12:latest AS runtime
WORKDIR /app

# Copiar el binario compilado desde la etapa constructora
COPY --from=builder /app/target/release/api /app/server

# Copiar archivos de configuración globales necesarios o assets estáticos
COPY .env.production /app/.env

# Coolify leerá este puerto para mapear el tráfico web
EXPOSE 8000

# Comando de ejecución por defecto
CMD ["/app/server"]

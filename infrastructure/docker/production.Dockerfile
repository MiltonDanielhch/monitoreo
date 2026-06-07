# syntax=docker/dockerfile:1

# ==========================================
# Builder - Compilación Rust/Axum
# ==========================================
FROM rust:bookworm AS builder
WORKDIR /app

# Instalar dependencias para compilación
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copiar código completo del workspace
COPY . .

# Compilar en modo Release
RUN cargo build --release --bin api

# ==========================================
# Runtime - Imagen distroless mínima
# ==========================================
FROM gcr.io/distroless/cc-debian12:latest AS runtime
WORKDIR /app

# Copiar solo el binario compilado
COPY --from=builder /app/target/release/api /app/server

EXPOSE 8000

CMD ["/app/server"]

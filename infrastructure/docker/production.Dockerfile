# ==========================================
# Builder - Compilación directa sin cargo-chef
# ==========================================
FROM rust:1.85-alpine AS builder
WORKDIR /app

# Instalar dependencias para compilación
RUN apk add --no-cache musl-dev gcc libressl-dev

# Copiar manifiestos primero para caché de dependencias
COPY Cargo.toml ./
COPY crates/domain/Cargo.toml ./crates/domain/
COPY crates/database/Cargo.toml ./crates/database/
COPY crates/infrastructure/Cargo.toml ./crates/infrastructure/
COPY crates/shared_types/Cargo.toml ./crates/shared_types/
COPY apps/api/Cargo.toml ./apps/api/

# Crear dummy main.rs si no existe para caching de deps
RUN mkdir -p apps/api/src && \
    echo "fn main() {}" > apps/api/src/main.rs

# Descargar dependencias (caché)
RUN cargo fetch

# Copiar código fuente
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

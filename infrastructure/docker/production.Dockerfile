# syntax=docker/dockerfile:1

# ==========================================
# Fase 1: Builder (Estrategia de Caché Avanzada)
# ==========================================
FROM rust:bookworm AS builder
WORKDIR /app

# 1. Instalar dependencias del sistema nativas
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 2. Copiar los manifiestos raíz
COPY Cargo.toml Cargo.lock ./

# 3. Copiar los Cargo.toml de todos los componentes
COPY apps/api/Cargo.toml ./apps/api/Cargo.toml
COPY crates/domain/Cargo.toml ./crates/domain/Cargo.toml
COPY crates/infrastructure/Cargo.toml ./crates/infrastructure/Cargo.toml
COPY crates/database/Cargo.toml ./crates/database/Cargo.toml
COPY crates/shared_types/Cargo.toml ./crates/shared_types/Cargo.toml

# 4. Crear los esqueletos iniciales para compilar dependencias pesadas
RUN mkdir -p apps/api/src \
    && mkdir -p crates/domain/src \
    && mkdir -p crates/infrastructure/src \
    && mkdir -p crates/database/src \
    && mkdir -p crates/shared_types/src \
    && echo "fn main() { println!(\"Skeleton\"); }" > apps/api/src/main.rs \
    && echo "// Skeleton" > crates/domain/src/lib.rs \
    && echo "// Skeleton" > crates/infrastructure/src/lib.rs \
    && echo "// Skeleton" > crates/database/src/lib.rs \
    && echo "// Skeleton" > crates/shared_types/src/lib.rs

# 5. Compilar las dependencias externas (Este paso ya está CACHEADO, irá a 0 segundos)
RUN cargo build --release --bin api

# 6. Copiar TODO tu código fuente real (Sobrescribirá los esqueletos falsos)
COPY . .

# 7. ¡El secreto!: Eliminamos explícitamente solo los artefactos viejos de la API
# y actualizamos los metadatos de las librerías locales para forzar a Cargo a recompilarlas con tu lógica real
RUN rm -f target/release/deps/api* target/release/api* \
    && touch apps/api/src/main.rs \
    && touch crates/domain/src/lib.rs \
    && touch crates/infrastructure/src/lib.rs \
    && touch crates/database/src/lib.rs \
    && touch crates/shared_types/src/lib.rs

# 8. Compilación final de tu código de negocio
RUN cargo build --release --bin api

# ==========================================
# Fase 2: Runtime - Imagen distroless mínima
# ==========================================
FROM gcr.io/distroless/cc-debian12:latest AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/api /app/server

EXPOSE 8000

CMD ["/app/server"]
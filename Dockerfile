# RustPlayer — Dockerfile para build de producción Linux
#
# NOTA IMPORTANTE: RustPlayer es una aplicación de escritorio con GUI OpenGL.
# Este Dockerfile es ÚNICAMENTE para compilación en CI/CD y no para ejecutar
# el reproductor dentro de Docker (requiere display, OpenGL, audio del host).
#
# Uso:
#   docker build -t rustplayer-builder .
#   docker run --rm -v $(pwd)/artifacts:/out rustplayer-builder
#   # El binario queda en ./artifacts/rustplayer
#
# O con docker compose:
#   docker compose run --rm build-linux

# ── Stage 1: Dependencias de sistema ────────────────────────────────────────
FROM fedora:40 AS deps

RUN dnf install -y \
    # Build tools
    gcc \
    pkg-config \
    make \
    # libmpv (motor de reproducción)
    mpv-libs \
    mpv-libs-devel \
    # Audio/video
    ffmpeg \
    ffmpeg-devel \
    # SSL (para ureq / HTTP)
    openssl-devel \
    # X11 / Wayland (para eframe nativo)
    libX11-devel \
    libXcursor-devel \
    libXrandr-devel \
    libXi-devel \
    libGL-devel \
    wayland-devel \
    wayland-protocols-devel \
    && dnf clean all

# ── Stage 2: Instalar Rust ───────────────────────────────────────────────────
FROM deps AS rust-base

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=stable

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --no-modify-path --profile minimal --default-toolchain ${RUST_VERSION} \
    && rustup component add clippy rustfmt

# ── Stage 3: Cachear dependencias de Cargo ──────────────────────────────────
FROM rust-base AS cargo-deps

WORKDIR /build

# Copiar solo Cargo.toml y Cargo.lock para cachear deps
COPY Cargo.toml Cargo.lock ./

# Crear un main.rs dummy para compilar solo las dependencias
RUN mkdir -p src && echo "fn main() {}" > src/main.rs \
    && cargo build --release 2>/dev/null || true \
    && rm -f src/main.rs target/release/rustplayer*

# ── Stage 4: Compilar el proyecto ───────────────────────────────────────────
FROM cargo-deps AS builder

COPY . .

ARG BUILD_VERSION=dev
ENV RUSTFLAGS="-C target-cpu=native"

RUN cargo build --release \
    && strip target/release/rustplayer

# ── Stage 5: Artefacto final (solo el binario) ──────────────────────────────
FROM scratch AS artifact

COPY --from=builder /build/target/release/rustplayer /rustplayer

# ── Stage alternativo: Ubuntu para mayor compatibilidad ─────────────────────
FROM ubuntu:22.04 AS builder-ubuntu

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    gcc \
    pkg-config \
    libmpv-dev \
    libssl-dev \
    libx11-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    libgl1-mesa-dev \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --no-modify-path --profile minimal

WORKDIR /build
COPY . .

RUN cargo build --release && strip target/release/rustplayer

CMD ["cp", "target/release/rustplayer", "/out/rustplayer"]

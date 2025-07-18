FROM node:18

# OCI standard labels for image metadata
LABEL org.opencontainers.image.source="https://github.com/AABelkhiria/ashnotes-app"
LABEL org.opencontainers.image.description="Build environment for the AshNotes Tauri app. Includes Rust, Node.js, Tauri CLI, and all system dependencies for a .deb package build."
LABEL org.opencontainers.image.licenses="MIT"

# Arguments for user and group IDs, defaulted for GitHub Actions runners
ARG USER_UID=1001
ARG GROUP_GID=127

# Set environment variables to non-interactive to prevent prompts during build
ENV DEBIAN_FRONTEND=noninteractive

# Install essential build tools and Tauri's system dependencies for Debian/Ubuntu
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    jq \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --gid ${GROUP_GID} runner \
    && useradd --uid ${USER_UID} --gid ${GROUP_GID} --create-home --shell /bin/bash runner

# Switch to the non-root user
USER runner

# Install Rust and Cargo using rustup
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable

# Install the Tauri CLI using Cargo
RUN cargo install tauri-cli --version 2.0.0-rc.18 --locked

# Set the working directory for your application
WORKDIR /app

# Use Ubuntu as the base image
FROM lagmoellertim/essentia:latest-tensorflow

# Install system dependencies and Rust
RUN apt-get update && apt-get install -y \
    curl build-essential pkg-config && \
    rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

# Set workdir
WORKDIR /app

# Copy the crate source
COPY . .

# Build the crate (release by default)
#RUN cargo build --release
WORKDIR /app/essentia_core

RUN cargo run
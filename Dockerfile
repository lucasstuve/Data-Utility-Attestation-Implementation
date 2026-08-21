FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential \
        pkg-config \
        libssl-dev \
        curl \
        git \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/root/.rustup \
    CARGO_HOME=/root/.cargo \
    PATH=/root/.cargo/bin:/root/.risc0/bin:$PATH

# Rust itself; the exact version comes from rust-toolchain.toml once we're in /app
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none

# RISC Zero toolchain: rzup, then rzup pulls r0vm + cargo-risczero + the
# riscv32im target matching the risc0-zkvm version pinned in Cargo.toml
RUN curl -L https://risczero.com/install | bash
RUN rzup install

WORKDIR /app
COPY . .

# Normalize line endings in case the build context came from a Windows
# checkout (CRLF breaks bash's `set -euo pipefail`)
RUN find . -name "*.sh" -type f -exec sed -i 's/\r$//' {} \;

# Resolves rust-toolchain.toml and builds host + benchmarks (and, via
# methods/build.rs, cross-compiles the guest) so the image is ready to run.
RUN cargo build --release -p host -p benchmarks

ENV RISC0_DEV_MODE=0
CMD ["bash"]

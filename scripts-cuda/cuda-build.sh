#!/usr/bin/env bash
set -euo pipefail

export PATH=/usr/local/cuda/bin:$HOME/.cargo/bin:$PATH
export LD_LIBRARY_PATH=/usr/local/cuda/lib64${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}
export CC=gcc-12
export CXX=g++-12
export NVCC_CCBIN=g++-12
export RISC0_CUDA_ARCH="${RISC0_CUDA_ARCH:-sm_86}"

cargo build -p host --release --features cuda
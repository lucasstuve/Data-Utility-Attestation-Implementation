#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "ERROR: $1" >&2
  exit 1
}

echo "Checking NVIDIA driver..."
command -v nvidia-smi >/dev/null 2>&1 || fail "nvidia-smi not found"
nvidia-smi >/dev/null 2>&1 || fail "nvidia-smi failed"

echo "Checking CUDA toolkit..."
command -v nvcc >/dev/null 2>&1 || fail "nvcc not found"
nvcc --version || fail "nvcc failed"

echo "Checking Rust..."
command -v cargo >/dev/null 2>&1 || fail "cargo not found"
command -v rustc >/dev/null 2>&1 || fail "rustc not found"
rustc --version
cargo --version

echo "Checking C/C++ toolchain..."
command -v gcc-12 >/dev/null 2>&1 || fail "gcc-12 not found"
command -v g++-12 >/dev/null 2>&1 || fail "g++-12 not found"

echo "Checking CUDA path..."
[ -d /usr/local/cuda ] || fail "/usr/local/cuda not found"
[ -x /usr/local/cuda/bin/nvcc ] || fail "/usr/local/cuda/bin/nvcc not found"

echo "Checking GPU arch setting..."
: "${RISC0_CUDA_ARCH:=sm_86}"
echo "RISC0_CUDA_ARCH=${RISC0_CUDA_ARCH}"

echo "Environment check passed."
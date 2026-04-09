#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# SET-UP Copied from the official RISC0 documentation, with small fixes

# install driver install tool
sudo apt update
sudo apt install ubuntu-drivers-common -y

# install drivers
sudo ubuntu-drivers install

# install compiling tools for risc0
sudo apt install build-essential libssl-dev gcc-12 g++-12 -y

# install the cuda toolkit to compile the required codebase
sudo apt install cuda-toolkit -y

# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# setup your paths
echo 'export PATH=/usr/local/cuda/bin:$HOME/.cargo/bin:$PATH' >> ~/.bashrc
echo 'export LD_LIBRARY_PATH=/usr/local/cuda/lib64:$LD_LIBRARY_PATH' >> ~/.bashrc

# force nvcc to use gcc/g++ instead of clang
echo 'export CC=gcc-12' >> ~/.bashrc
echo 'export CXX=g++-12' >> ~/.bashrc
echo 'export NVCC_CCBIN=g++-12' >> ~/.bashrc

# load rust for this script run
source "$HOME/.cargo/env"
source ~/.bashrc

# clone the risc0 repo
git clone https://github.com/risc0/risc0.git || true
cd risc0

# install the r0vm
cargo run --bin rzup install

echo "Set-up finished!"
echo "If NVIDIA drivers were newly installed, reboot before building your project."
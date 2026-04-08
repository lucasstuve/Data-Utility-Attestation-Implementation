#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"


# SET-UP Copied for the offical RISC0-Documentation: 

# install driver install tool
sudo apt install ubuntu-drivers-common
# install drivers
sudo ubuntu-drivers install
# install compiling tools for risc0
sudo apt install build-essential libssl-dev -y
# install the cuda toolkit to compile the required codebase
sudo apt install cuda-toolkit -y
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# setup your paths
echo 'export PATH=/usr/local/cuda/bin:$PATH' >> ~/.bashrc
echo 'export LD_LIBRARY_PATH=/usr/local/cuda/lib64:$LD_LIBRARY_PATH' >> ~/.bashrc
source ~/.bashrc
# clone the risc0 repo
git clone https://github.com/risc0/risc0.git
cd risc0
# install the r0vm
cargo run --bin rzup install

echo "Set-up finished!"
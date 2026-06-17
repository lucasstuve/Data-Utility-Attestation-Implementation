#!/usr/bin/env bash
set -euo pipefail

# USED for debugging, before running Cuda tests.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

DATA_SETS=("user-batch.json")

cd "$PROJECT_ROOT"
export RISC0_DEV_MODE=0

echo -e "\e[1mGenerating benchmark test data...\e[0m"
cargo run -p benchmarks --release

for DATA_SET in "${DATA_SETS[@]}"; do 
echo -e "\e[1mSTART end-to-end run for:\e[0m"
echo -e "\e[31m $DATA_SET\e[0m"
cargo run -p host --release -- "$DATA_SET"
done
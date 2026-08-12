#!/usr/bin/env bash
set -euo pipefail

# USED for debugging, before running Cuda tests.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

DATA_SETS=("user-batch.json")   # ONLY for demonstration, in the CUDA setting under ./scripts-cuda this data is used: "test-data-10-MB.json" "test-data-100MB.json" "test-data-1000-MB.json" "vw-batch.json"

cd "$PROJECT_ROOT"
export RISC0_DEV_MODE=0

#echo -e "\e[1mGenerating benchmark test data...\e[0m"
# cargo run -p benchmarks --release   # Data generation is skipped, for faster code execution test/demonstration 

for DATA_SET in "${DATA_SETS[@]}"; do 
echo -e "\e[1mSTART end-to-end run for:\e[0m"
echo -e "\e[31m $DATA_SET\e[0m"
cargo run -p host --release -- "$DATA_SET"
done
#!/usr/bin/env bash
set -euo pipefail

# USED for debugging, before running Cuda tests.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"


DATA_SETS=("test-data-10-MB.json" "test-data-100MB.json" "test-data-1000-MB.json" "vw-batch.json" )

cd "$PROJECT_ROOT"
export RISC0_DEV_MODE=0
export RISC0_SEGMENT_LIMIT_PO2="${RISC0_SEGMENT_LIMIT_PO2:-19}"

echo -e "\e[1mGenerating benchmark test data...\e[0m"
cargo run -p benchmarks --release

for DATA_SET in "${DATA_SETS[@]}"; do 
echo -e "\e[1mSTART benchmark for:\e[0m"
echo -e "\e[31m $DATA_SET\e[0m"
#echo "Start benchmark for: $DATA_SET"
RUSTFLAGS="-C target-cpu=native" cargo run -p host --release --features cuda -- "$DATA_SET" 
done
#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

DATA_SETS=("test-data-1KB.json" "test-data-10KB.json" "test-data-100KB.json" "test-data-1000KB.json")
QUERY='CREATE SCHEMA VehicleData (dataFieldName string, value float); assert ALL VehicleData (dataFieldName == "profiles.targetSOCPercentage" AND value < 50.0 ); assert VehicleData #win:time(50 min);  assert (COUNT(value) > 20);'
RESULTS_FILE="filter-window-aggregate-benchmark-cuda.csv"

cd "$PROJECT_ROOT"
export RISC0_DEV_MODE=0

for DATA_SET in "${DATA_SETS[@]}"; do 
echo -e "\e[1mSTART benchmark for:\e[0m"
echo -e "\e[31m $DATA_SET\e[0m"
#echo "Start benchmark for: $DATA_SET"
RUSTFLAGS="-C target-cpu=native" cargo run -p host --release --features cuda -- "$DATA_SET" "$QUERY" "$RESULTS_FILE"
done
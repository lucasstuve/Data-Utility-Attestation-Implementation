#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

DATA_SETS=("test-data-1000MB.json")
QUERY='CREATE SCHEMA VehicleData (dataFieldName string, value int); assert ALL VehicleData (dataFieldName == "currentSOCInPct" AND value < 50 ); assert (COUNT(value) > 200); '
RESULTS_FILE="extensive-benchmark-1GB.csv"

cd "$PROJECT_ROOT"

export RISC0_DEV_MODE=0

for DATA_SET in "${DATA_SETS[@]}"; do 
echo -e "\e[1mSTART benchmark for:\e[0m"
echo -e "\e[31m $DATA_SET\e[0m"
#echo "Start benchmark for: $DATA_SET"
RUSTFLAGS="-C target-cpu=native" cargo run -p host --release --features cuda -- "$DATA_SET" "$QUERY" "$RESULTS_FILE"
done
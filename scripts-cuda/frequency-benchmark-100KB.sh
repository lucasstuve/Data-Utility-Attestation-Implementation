#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

DATA_SETS=("test-data-5-percent.json" "test-data-10-percent.json" "test-data-15-percent.json" "test-data-20-percent.json" "test-data-25-percent.json" "test-data-30-percent.json" "test-data-35-percent.json" "test-data-40-percent.json" "test-data-45-percent.json" "test-data-50-percent.json" "test-data-55-percent.json" "test-data-60-percent.json" "test-data-65-percent.json" "test-data-70-percent.json" "test-data-75-percent.json" "test-data-80-percent.json" "test-data-85-percent.json" "test-data-90-percent.json" "test-data-95-percent.json" "test-data-100-percent.json")
QUERY='CREATE SCHEMA VehicleData (dataFieldName string, value float,  timestampUtc string ); assert ALL VehicleData (dataFieldName == "profiles.targetSOCPercentage" AND value < 50.0 );'
RESULTS_FILE="frequency-benchmark-cuda.csv"

cd "$PROJECT_ROOT"
export RISC0_DEV_MODE=0

for DATA_SET in "${DATA_SETS[@]}"; do 
echo -e "\e[1mSTART benchmark for:\e[0m"
echo -e "\e[31m $DATA_SET\e[0m"
#echo "Start benchmark for: $DATA_SET"
RUSTFLAGS="-C target-cpu=native" cargo run -p host --release --features cuda -- "$DATA_SET" "$QUERY" "$RESULTS_FILE"
done
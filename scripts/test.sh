#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

DATA_SETS=("test-data-10KB.json")
QUERY='CREATE SCHEMA VehicleData (dataFieldName string, value float); assert ALL VehicleData (dataFieldName == "mileage" AND value > 120.0 );'
RESULTS_FILE="test.csv"

cd "$PROJECT_ROOT"
export RISC0_DEV_MODE=1

for DATA_SET in "${DATA_SETS[@]}"; do 
echo -e "\e[1mSTART benchmark for:\e[0m"
echo -e "\e[31m $DATA_SET\e[0m"
#echo "Start benchmark for: $DATA_SET"
cargo run -p host -- "$DATA_SET" "$QUERY" "$RESULTS_FILE"
done
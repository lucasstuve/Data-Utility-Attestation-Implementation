#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

DATA_SETS=("100-VW-Events.json")
QUERY='CREATE SCHEMA VehicleData (dataFieldName string, value int); assert ALL VehicleData (dataFieldName == "profiles.targetSOCPercentage" AND value < 50);'
RESULTS_FILE="shell-script-test.csv"

cd "$PROJECT_ROOT"

for DATA_SET in "${DATA_SETS[@]}"; do 
echo "Start benchmark for: $DATA_SET"
cargo run -p host -- "$DATA_SET" "$QUERY" "$RESULTS_FILE"
done
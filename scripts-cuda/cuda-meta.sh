#!/usr/bin/env bash
set -euo pipefail

echo "Start 1st benchmark: filter-benchmark.sh"
bash filter-benchmark.sh || "1: failed"

echo "Start 2nd bechmark: filter-aggregate-benchmark.sh" 
bash filter-aggregate-benchmark.sh || "2: failed"

echo "Start 3rd benchmark: filter-aggregate-real-data.sh" 
bash filter-aggregate-real-data.sh || "3: failed"

echo "Start 4th bechmark: filter-pattern-benchmark.sh" 
bash filter-pattern-benchmark.sh || "4: failed"

echo "Start 5th bechmark: filter-session-benchmark.sh"
bash filter-session-benchmark.sh || "5: failed"

echo "Start 6th benchmark: filter-window-aggregate-benchmark.sh"
bash filter-window-aggregate-benchmark.sh || "6: failed"

echo "Start 7th benchmark: filter-window-benchmark.sh" 
bash filter-window-benchmark.sh || "7: failed"

echo "Start 8th benchmark: frequency-benchmark.sh" 
bash frequency-benchmark.sh || "8: failed"

#echo "Start 9th benchmark: extensive-benchmark-100MB.sh"
#bash extensive-benchmark-100MB.sh || "9: failed"

#echo "Start 10th benchmark: extenstive-benchmark-1GB.sh"
#bash extensive-benchmark-1000MB.sh || "10: failed"

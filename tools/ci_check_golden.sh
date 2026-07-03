#!/usr/bin/env bash
set -euo pipefail

# Minimal golden MIR check for CI/local use

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT_DIR"

PAIRS=(
  "docs/development/testing/fixtures/golden/simple_return.hako docs/development/testing/golden/simple_return.mir.txt"
)

for pair in "${PAIRS[@]}"; do
  in_file="${pair%% *}"
  golden_file="${pair##* }"
  if [ ! -f "$in_file" ]; then
    echo "[GOLDEN] Missing input: $in_file" >&2
    exit 1
  fi
  if [ ! -f "$golden_file" ]; then
    echo "[GOLDEN] Missing golden: $golden_file" >&2
    exit 1
  fi
  echo "[GOLDEN] Checking: $in_file vs $golden_file"
  bash ./tools/compare_mir.sh "$in_file" "$golden_file"
done

echo "All golden MIR snapshots match."

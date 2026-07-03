#!/usr/bin/env bash
set -euo pipefail

# Run Hakorune VM with stats enabled and save JSON output.
# Usage: tools/engineering/run_vm_stats.sh <hako_file> [output_json]

if [ $# -lt 1 ]; then
  echo "Usage: $0 <hako_file> [output_json]" >&2
  exit 1
fi

HAKO_FILE="$1"
OUT_JSON="${2:-vm_stats.json}"

if [ ! -f "$HAKO_FILE" ]; then
  echo "File not found: $HAKO_FILE" >&2
  exit 1
fi

if [ -n "${HAKO_BIN:-}" ] && [ -z "${NYASH_BIN:-}" ]; then
  NYASH_BIN="$HAKO_BIN"
fi
HAKORUNE_BIN="./target/release/hakorune"
LEGACY_NYASH_BIN="./target/release/nyash"
NYASH_BIN="${NYASH_BIN:-$HAKORUNE_BIN}"
if [ ! -x "$NYASH_BIN" ] && [ -x "$LEGACY_NYASH_BIN" ]; then
  NYASH_BIN="$LEGACY_NYASH_BIN"
fi
if [ ! -x "$NYASH_BIN" ]; then
  echo "Building hakorune in release mode..." >&2
  cargo build --release --bin hakorune -q
  NYASH_BIN="$HAKORUNE_BIN"
fi
export HAKO_BIN="${HAKO_BIN:-$NYASH_BIN}"

echo "Running: $NYASH_BIN --backend vm --vm-stats --vm-stats-json $HAKO_FILE" >&2
tmp_json="${OUT_JSON}.tmp.$$"
trap 'rm -f "$tmp_json"' EXIT
NYASH_VM_STATS=1 NYASH_VM_STATS_JSON=1 "$NYASH_BIN" --backend vm --vm-stats --vm-stats-json "$HAKO_FILE" > "$tmp_json"
mv "$tmp_json" "$OUT_JSON"
echo "Stats written to: $OUT_JSON" >&2

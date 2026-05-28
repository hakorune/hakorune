#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-192-TYPED-OBJECT-RUNTIME-FAST-LANE-KEEPER-MEASUREMENT.md"
TMP_DIR="$(mktemp -d /tmp/hakorune_row192_fast_lane_keeper.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

grep -q '^measurement_contract=accepted$' "$DOC"
grep -q '^runtime_fast_lane_keeper=1$' "$DOC"
grep -q '^keeper_effect=accepted$' "$DOC"
grep -q '^winner_claim=0$' "$DOC"
grep -q '^replacement_active=0$' "$DOC"
grep -q '^hook_installed=0$' "$DOC"
grep -q '^global_allocator=0$' "$DOC"
grep -q '^summary=ok$' "$DOC"

"$ROOT_DIR/tools/allocator/typed_object_runtime_fast_lane_keeper_measurement.py" \
  --sample-count 1 \
  --out "$REPORT"

grep -q '^output_contract=typed-object-runtime-fast-lane-keeper-measurement-v0$' "$REPORT"
grep -q '^input_contract=typed-object-runtime-single-thread-fast-lane-v0$' "$REPORT"
grep -q '^keeper_effect=accepted$' "$REPORT"
grep -q '^runtime_fast_lane_keeper=1$' "$REPORT"
grep -q '^winner_claim=0$' "$REPORT"
grep -q '^replacement_active=0$' "$REPORT"
grep -q '^hook_installed=0$' "$REPORT"
grep -q '^global_allocator=0$' "$REPORT"
grep -q '^summary=ok$' "$REPORT"

cat "$REPORT"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TMP_DIR="$(mktemp -d /tmp/hakorune_typed_object_lock_cost.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

"$ROOT_DIR/tools/allocator/typed_object_helper_lock_cost_probe.py" \
  --iterations 500000 > "$REPORT"

grep -q '^output_contract=typed-object-helper-lock-cost-probe-v0$' "$REPORT"
grep -q '^input_contract=hako-mimalloc-field-array-runtime-boundary-probe-v0$' "$REPORT"
grep -q '^dominant_helper_subowner=lock_global_slab$' "$REPORT"
grep -q '^recommended_next=runtime_single_thread_fast_lane$' "$REPORT"
grep -q '^optimization_open=0$' "$REPORT"
grep -q '^winner_claim=0$' "$REPORT"
grep -q '^replacement_active=0$' "$REPORT"
grep -q '^hook_installed=0$' "$REPORT"
grep -q '^global_allocator=0$' "$REPORT"
grep -q '^summary=ok$' "$REPORT"

cat "$REPORT"

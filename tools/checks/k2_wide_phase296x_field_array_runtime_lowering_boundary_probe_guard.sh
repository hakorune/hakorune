#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_field_array_boundary.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR_JSON="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

NYASH_FEATURES="${NYASH_FEATURES:-rune}" \
NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$MIR_JSON" "$APP" >/dev/null

"$ROOT_DIR/tools/allocator/hako_mimalloc_field_array_runtime_boundary_probe.py" \
  --mir-json "$MIR_JSON" > "$REPORT"

grep -q '^output_contract=hako-mimalloc-field-array-runtime-boundary-probe-v0$' "$REPORT"
grep -q '^selected_boundary=typed_object_field_helper_lowering$' "$REPORT"
grep -q '^next_diagnostic=typed_object_field_helper_fast_lane_selection$' "$REPORT"
grep -q '^optimization_open=0$' "$REPORT"
grep -q '^winner_claim=0$' "$REPORT"
grep -q '^replacement_active=0$' "$REPORT"
grep -q '^hook_installed=0$' "$REPORT"
grep -q '^global_allocator=0$' "$REPORT"
grep -q '^summary=ok$' "$REPORT"

cat "$REPORT"

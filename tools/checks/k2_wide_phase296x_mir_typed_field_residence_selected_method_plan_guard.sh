#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-195-MIR-TYPED-FIELD-RESIDENCE-SELECTED-METHOD-PLAN.md"
TMP_DIR="$(mktemp -d /tmp/hakorune_row195_mir_residence_plan.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR_JSON="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

grep -q '^selected_method_plan=accepted$' "$DOC"
grep -q '^selected_method=HakoAllocPageModel.acquire_usize/1$' "$DOC"
grep -q '^transform_open=0$' "$DOC"
grep -q '^writeback_field_count_positive=1$' "$DOC"
grep -q '^helper_load_on_first_use_count_positive=1$' "$DOC"
grep -q '^by_name_special_case=0$' "$DOC"
grep -q '^summary=ok$' "$DOC"

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR_JSON" \
    "$APP" >/tmp/hakorune_row195_mir_emit.log

"$ROOT_DIR/tools/allocator/mir_typed_field_residence_selected_method_plan.py" \
  --mir-json "$MIR_JSON" > "$REPORT"

grep -q '^output_contract=mir-typed-field-residence-selected-method-plan-v0$' "$REPORT"
grep -q '^input_contract=mir-typed-field-residence-inventory-v0$' "$REPORT"
grep -q '^selected_method=HakoAllocPageModel.acquire_usize/1$' "$REPORT"
grep -q '^residence_kind=method_receiver_cache_writeback$' "$REPORT"
grep -q '^next_step=mir_typed_field_residence_selected_method_keeper$' "$REPORT"
grep -q '^transform_open=0$' "$REPORT"
grep -q '^by_name_special_case=0$' "$REPORT"
grep -q '^winner_claim=0$' "$REPORT"
grep -q '^replacement_active=0$' "$REPORT"
grep -q '^hook_installed=0$' "$REPORT"
grep -q '^global_allocator=0$' "$REPORT"
grep -q '^summary=ok$' "$REPORT"

if ! awk -F= '$1 == "writeback_field_count" { exit !($2 + 0 > 0) }' "$REPORT"; then
  echo "[row195] writeback_field_count must be positive" >&2
  cat "$REPORT" >&2
  exit 1
fi

if ! awk -F= '$1 == "helper_load_on_first_use_count" { exit !($2 + 0 > 0) }' "$REPORT"; then
  echo "[row195] helper_load_on_first_use_count must be positive" >&2
  cat "$REPORT" >&2
  exit 1
fi

cat "$REPORT"

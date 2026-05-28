#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-219-MIR-TYPED-FIELD-DIRECT-OP-GUARD-SURFACE.md"
TOOL="$ROOT_DIR/tools/allocator/mir_typed_field_direct_op_guard_surface.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row219_direct_op_guard.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR_JSON="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local line="$2"
  if ! grep -Fqx "$line" "$file"; then
    echo "[row219-direct-op-guard] missing line in $file: $line" >&2
    exit 1
  fi
}

for line in \
  "output_contract=mir-typed-field-direct-op-guard-surface-v0" \
  "input_contract=mir-typed-field-direct-op-net-inventory-v0" \
  "selected_method=HakoAllocPageModel.acquire_usize/1" \
  "candidate_total=21" \
  "projected_net_helper_call_delta=21" \
  "unsigned_set_nonnegative_guard_count=8" \
  "set_status_trap_count=8" \
  "helper_free_direct_op_required=1" \
  "fallback_silent_success=0" \
  "implementation_open=0" \
  "selected_next=mir_typed_field_direct_op_selected_method_keeper" \
  "by_name_special_case=0" \
  "winner_claim=0" \
  "summary=ok"; do
  require_line "$DOC" "$line"
done

if [ ! -x "$ROOT_DIR/target/release/hakorune" ]; then
  cargo build --release --bin hakorune >/tmp/hakorune_row219_hakorune_build.log
fi

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR_JSON" \
    "$APP" >/tmp/hakorune_row219_mir_emit.log

"$TOOL" --mir-json "$MIR_JSON" > "$REPORT"

for line in \
  "output_contract=mir-typed-field-direct-op-guard-surface-v0" \
  "input_contract=mir-typed-field-direct-op-net-inventory-v0" \
  "selected_method=HakoAllocPageModel.acquire_usize/1" \
  "candidate_field_get_count=13" \
  "candidate_field_set_count=8" \
  "candidate_total=21" \
  "projected_net_helper_call_delta=21" \
  "candidate_usize_count=17" \
  "candidate_handle_count=2" \
  "unsigned_set_nonnegative_guard_count=8" \
  "set_status_trap_count=8" \
  "helper_free_direct_op_required=1" \
  "slot_constant_required=1" \
  "typed_object_plan_required=1" \
  "fallback_silent_success=0" \
  "residence_transform_open=0" \
  "direct_op_transform_open=0" \
  "implementation_open=0" \
  "projected_symbol_2=nyash.object.exact_slot_get_u64_hii" \
  "projected_symbol_2_count=9" \
  "projected_symbol_3=nyash.object.exact_slot_set_u64_hiu" \
  "projected_symbol_3_count=8" \
  "selected_next=mir_typed_field_direct_op_selected_method_keeper" \
  "by_name_special_case=0" \
  "winner_claim=0" \
  "replacement_active=0" \
  "hook_installed=0" \
  "global_allocator=0" \
  "summary=ok"; do
  require_line "$REPORT" "$line"
done

cat "$REPORT"

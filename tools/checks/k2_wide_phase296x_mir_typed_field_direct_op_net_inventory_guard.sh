#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-218-MIR-TYPED-FIELD-DIRECT-OP-NET-INVENTORY.md"
TOOL="$ROOT_DIR/tools/allocator/mir_typed_field_direct_op_net_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row218_direct_op_inventory.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

MIR_JSON="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local line="$2"
  if ! grep -Fqx "$line" "$file"; then
    echo "[row218-direct-op-inventory] missing line in $file: $line" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[row218-direct-op-inventory] $key must be present and positive" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line "$DOC" "output_contract=mir-typed-field-direct-op-net-inventory-v0"
require_line "$DOC" "input_contract=typed-object-exact-slot-owner-refresh-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_next=mir_typed_field_direct_op_guard_surface"
require_line "$DOC" "inventory_only=1"
require_line "$DOC" "projected_net_helper_call_delta_positive=1"
require_line "$DOC" "dynamic_projected_net_helper_call_delta_positive=1"
require_line "$DOC" "residence_inserted_load_writeback_delta_used=0"
require_line "$DOC" "direct_op_transform_open=0"
require_line "$DOC" "by_name_special_case=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "summary=ok"

if [ ! -x "$ROOT_DIR/target/release/hakorune" ]; then
  cargo build --release --bin hakorune >/tmp/hakorune_row218_hakorune_build.log
fi

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR_JSON" \
    "$APP" >/tmp/hakorune_row218_mir_emit.log

"$TOOL" --mir-json "$MIR_JSON" > "$REPORT"

require_line "$REPORT" "output_contract=mir-typed-field-direct-op-net-inventory-v0"
require_line "$REPORT" "input_contract=typed-object-exact-slot-owner-refresh-v0"
require_line "$REPORT" "hot_method_count=5"
require_line "$REPORT" "missing_hot_method_count=0"
require_line "$REPORT" "planned_added_helper_calls=0"
require_line "$REPORT" "projected_added_helper_call_count=0"
require_line "$REPORT" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "selected_next=mir_typed_field_direct_op_guard_surface"
require_line "$REPORT" "inventory_only=1"
require_line "$REPORT" "projected_net_helper_call_delta_positive=1"
require_line "$REPORT" "dynamic_projected_net_helper_call_delta_positive=1"
require_line "$REPORT" "selected_method_required=1"
require_line "$REPORT" "projected_exact_helper_symbol_coverage_matches_mir_storage_counts=1"
require_line "$REPORT" "residence_inserted_load_writeback_delta_used=0"
require_line "$REPORT" "residence_transform_open=0"
require_line "$REPORT" "direct_op_transform_open=0"
require_line "$REPORT" "previous_residence_zero_net_guard=1"
require_line "$REPORT" "by_name_special_case=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

require_positive_key "$REPORT" "projected_erased_exact_helper_call_count"
require_positive_key "$REPORT" "projected_net_helper_call_delta"
require_positive_key "$REPORT" "dynamic_projected_net_helper_call_delta"
require_positive_key "$REPORT" "selected_method_net_helper_call_delta"
require_positive_key "$REPORT" "selected_method_dynamic_net_helper_call_delta"

cat "$REPORT"

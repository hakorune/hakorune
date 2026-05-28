#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-247-PAGE-MODEL-HOTPATH-SHAPE-OWNER-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-246-PAGE-MODEL-HOTPATH-IR-SHAPE-DIFF-INVENTORY.md"
TOOL="$ROOT_DIR/tools/allocator/page_model_hotpath_shape_owner_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row247_page_model_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

INV="$TMP_DIR/inventory.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row247-page-model-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-model-hotpath-shape-owner-selection-v0"
require_line "$DOC" "selected_owner=page_model_acquire_usize_copy_materialization_probe"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "summary=ok"

cat >"$INV" <<'REPORT'
output_contract=page-model-hotpath-ir-shape-diff-inventory-v0
input_contract=weighted-exact-slot-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_pct=8.52
selected_method_field_op_count=21
selected_method_copy_count=31
selected_method_call_count=3
selected_method_shape_owner=copy_materialization
recent_selected_method_rmw_keeper_already_applied=1
direct_op_previous_rejected=1
page_queue_recent_nonkeeper_retry_closed=1
ir_shape_diff_inventory_only=1
summary=ok
REPORT

python3 "$TOOL" --inventory-report "$INV" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=page-model-hotpath-shape-owner-selection-v0"
require_line "$REPORT" "input_contract=page-model-hotpath-ir-shape-diff-inventory-v0"
require_line "$REPORT" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "selected_method_shape_owner=copy_materialization"
require_line "$REPORT" "selected_owner=page_model_acquire_usize_copy_materialization_probe"
require_line "$REPORT" "selected_reason=selected_method_shape_owner_copy_materialization"
require_line "$REPORT" "next_diagnostic=page_model_acquire_usize_copy_materialization_probe"
require_line "$REPORT" "rejected_owner=page_model_same_block_rmw_retry"
require_line "$REPORT" "rejected_owner_1=page_model_direct_op_retry"
require_line "$REPORT" "rejected_owner_2=page_queue_retry"
require_line "$REPORT" "implementation_open=0"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-263-PAGE-MODEL-HOTPATH-SHAPE-OWNER-SELECTION-AFTER-RESULT-CAPSULE-RESET.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-262-PAGE-MODEL-HOTPATH-IR-SHAPE-DIFF-REFRESH-AFTER-RESULT-CAPSULE-RESET.md"
TOOL="$ROOT_DIR/tools/allocator/page_model_hotpath_shape_owner_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row263_page_model_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

INV="$TMP_DIR/inventory.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row263-page-model-owner-after-reset] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-model-hotpath-shape-owner-selection-v0"
require_line "$DOC" "selected_owner=page_model_release_known_live_field_traffic_probe"
require_line "$DOC" "selected_owner_method=HakoAllocPageModel.releaseLocalKnownLive/1"
require_line "$DOC" "selected_method_prior_no_material_effect_row=296x-252"
require_line "$DOC" "rejected_owner_3=page_model_acquire_usize_copy_materialization_retry"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$INV" <<'REPORT'
output_contract=page-model-hotpath-ir-shape-diff-inventory-v0
input_contract=weighted-exact-slot-owner-selection-after-result-capsule-reset-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_pct=9.05
selected_method_field_op_count=21
selected_method_copy_count=31
selected_method_call_count=3
selected_method_shape_owner=copy_materialization
selected_method_prior_no_material_effect_row=296x-252
method_1_symbol=HakoAllocPageModel.releaseLocalKnownLive/1
method_1_pct=4.14
method_1_field_get_count=7
method_1_field_set_count=5
method_1_copy_count=13
method_1_call_count=2
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
require_line "$REPORT" "selected_owner=page_model_release_known_live_field_traffic_probe"
require_line "$REPORT" "selected_owner_method=HakoAllocPageModel.releaseLocalKnownLive/1"
require_line "$REPORT" "selected_reason=prior_acquire_copy_materialization_no_material_effect_select_next_page_model_method"
require_line "$REPORT" "next_diagnostic=page_model_release_known_live_field_traffic_probe"
require_line "$REPORT" "selected_method_prior_no_material_effect_row=296x-252"
require_line "$REPORT" "selected_owner_method_pct=4.14"
require_line "$REPORT" "selected_owner_method_field_get_count=7"
require_line "$REPORT" "selected_owner_method_field_set_count=5"
require_line "$REPORT" "selected_owner_method_copy_count=13"
require_line "$REPORT" "selected_owner_method_call_count=2"
require_line "$REPORT" "rejected_owner_3=page_model_acquire_usize_copy_materialization_retry"
require_line "$REPORT" "rejected_reason_3=prior_receiver_forwarding_no_material_effect_requires_different_page_model_owner"
require_line "$REPORT" "implementation_open=0"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

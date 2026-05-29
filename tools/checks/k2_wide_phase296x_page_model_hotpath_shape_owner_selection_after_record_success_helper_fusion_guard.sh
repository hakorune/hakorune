#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-290-PAGE-MODEL-HOTPATH-SHAPE-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-289-PAGE-MODEL-HOTPATH-IR-SHAPE-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
TOOL="$ROOT_DIR/tools/allocator/page_model_hotpath_shape_owner_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row290_page_model_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

INV="$TMP_DIR/inventory.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row290-page-model-owner-after-record-success] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-model-hotpath-shape-owner-selection-v0"
require_line "$DOC" "selected_owner=post_page_model_hotpath_owner_refresh_after_record_success_helper_fusion"
require_line "$DOC" "selected_owner_method=none"
require_line "$DOC" "selected_reason=prior_acquire_copy_and_release_known_live_no_effect_select_owner_refresh"
require_line "$DOC" "selected_method_prior_no_material_effect_row=296x-252"
require_line "$DOC" "fallback_method=HakoAllocPageModel.releaseLocalKnownLive/1"
require_line "$DOC" "fallback_method_prior_no_effect_row=296x-268"
require_line "$DOC" "rejected_owner_3=page_model_acquire_usize_copy_materialization_retry"
require_line "$DOC" "rejected_owner_4=page_model_release_known_live_field_traffic_probe"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$INV" <<'REPORT'
output_contract=page-model-hotpath-ir-shape-diff-inventory-v0
input_contract=post-facade-inventory-owner-refresh-after-record-success-helper-fusion-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_pct=6.31
selected_method_field_op_count=21
selected_method_copy_count=31
selected_method_call_count=3
selected_method_shape_owner=copy_materialization
selected_method_prior_no_material_effect_row=296x-252
method_1_symbol=HakoAllocPageModel.releaseLocalKnownLive/1
method_1_pct=2.89
method_1_field_get_count=7
method_1_field_set_count=5
method_1_copy_count=13
method_1_call_count=2
method_1_prior_no_effect_row=296x-268
recent_selected_method_rmw_keeper_already_applied=1
direct_op_previous_rejected=1
page_queue_recent_nonkeeper_retry_closed=1
ir_shape_diff_inventory_only=1
summary=ok
REPORT

python3 "$TOOL" --context after-record-success-helper-fusion --inventory-report "$INV" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=page-model-hotpath-shape-owner-selection-v0"
require_line "$REPORT" "input_contract=page-model-hotpath-ir-shape-diff-inventory-v0"
require_line "$REPORT" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "selected_method_shape_owner=copy_materialization"
require_line "$REPORT" "selected_owner=post_page_model_hotpath_owner_refresh_after_record_success_helper_fusion"
require_line "$REPORT" "selected_owner_method=none"
require_line "$REPORT" "selected_reason=prior_acquire_copy_and_release_known_live_no_effect_select_owner_refresh"
require_line "$REPORT" "next_diagnostic=post_page_model_hotpath_owner_refresh_after_record_success_helper_fusion"
require_line "$REPORT" "selected_method_prior_no_material_effect_row=296x-252"
require_line "$REPORT" "fallback_method=HakoAllocPageModel.releaseLocalKnownLive/1"
require_line "$REPORT" "fallback_method_prior_no_effect_row=296x-268"
require_line "$REPORT" "rejected_owner_3=page_model_acquire_usize_copy_materialization_retry"
require_line "$REPORT" "rejected_owner_4=page_model_release_known_live_field_traffic_probe"
require_line "$REPORT" "implementation_open=0"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

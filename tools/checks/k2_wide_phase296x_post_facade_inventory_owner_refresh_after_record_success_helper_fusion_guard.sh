#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-288-POST-FACADE-INVENTORY-OWNER-REFRESH-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-287-FACADE-FIELD-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
SOURCE="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-284-POST-RECORD-SUCCESS-HELPER-FUSION-OWNER-REFRESH.md"
TOOL="$ROOT_DIR/tools/allocator/post_facade_inventory_owner_refresh_after_rollback.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row288_owner_refresh.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row288-post-facade-owner-refresh] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$SOURCE" "Status: Landed"
require_line "$DOC" "output_contract=post-facade-inventory-owner-refresh-after-record-success-helper-fusion-v0"
require_line "$DOC" "input_contract=facade-field-owner-selection-after-record-success-helper-fusion-v0"
require_line "$DOC" "source_exact_slot_get_set_pct=50.97"
require_line "$DOC" "excluded_family_0=object_lifecycle_facade"
require_line "$DOC" "excluded_reason_0=facade_positive_net_surface_already_exercised"
require_line "$DOC" "excluded_family_1=page_queue_helpers"
require_line "$DOC" "excluded_reason_1=recent_nonkeeper_requires_fresh_shape_before_retry"
require_line "$DOC" "selected_family=page_model_hotpath"
require_line "$DOC" "selected_family_pct=11.73"
require_line "$DOC" "selected_owner=page_model_hotpath_ir_shape_inventory_after_record_success_helper_fusion"
require_line "$DOC" "next_diagnostic=page_model_hotpath_ir_shape_inventory_after_record_success_helper_fusion"
require_line "$DOC" "weighted_hot_candidate_score_required=1"
require_line "$DOC" "ir_shape_diff_required_before_next_keeper=1"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

python3 "$TOOL" \
  --context after-record-success-helper-fusion \
  --owner-refresh-report "$SOURCE" \
  --facade-selection-report "$PREV" \
  --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=post-facade-inventory-owner-refresh-after-record-success-helper-fusion-v0"
require_line "$REPORT" "input_contract=facade-field-owner-selection-after-record-success-helper-fusion-v0"
require_line "$REPORT" "source_exact_slot_get_set_pct=50.97"
require_line "$REPORT" "selected_family=page_model_hotpath"
require_line "$REPORT" "selected_family_pct=11.73"
require_line "$REPORT" "selected_owner=page_model_hotpath_ir_shape_inventory_after_record_success_helper_fusion"
require_line "$REPORT" "next_diagnostic=page_model_hotpath_ir_shape_inventory_after_record_success_helper_fusion"
require_line "$REPORT" "family_0_name=page_queue_helpers"
require_line "$REPORT" "family_1_name=object_lifecycle_facade"
require_line "$REPORT" "family_2_name=page_model_hotpath"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

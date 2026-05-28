#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-254-WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RECEIVER-FORWARDING.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-253-POST-RECEIVER-FORWARDING-OWNER-REFRESH.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row254-weighted-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=weighted-exact-slot-owner-selection-after-receiver-forwarding-v0"
require_line "$DOC" "dominant_family=page_queue_helpers"
require_line "$DOC" "page_queue_immediate_retry_blocked=1"
require_line "$DOC" "top_unblocked_family=page_model_hotpath"
require_line "$DOC" "page_model_recent_no_material_effect_row=296x-252"
require_line "$DOC" "page_model_immediate_retry_blocked=1"
require_line "$DOC" "combined_result_capsule_pct=16.42"
require_line "$DOC" "selected_family=result_capsule_family"
require_line "$DOC" "selected_owner=result_capsule_ir_shape_diff_inventory"
require_line "$DOC" "next_diagnostic=result_capsule_ir_shape_diff_inventory"
require_line "$DOC" "rejected_owner=page_queue_immediate_retry"
require_line "$DOC" "rejected_owner_1=page_model_immediate_retry"
require_line "$DOC" "rejected_owner_2=implementation_without_ir_shape_diff"
require_line "$DOC" "weighted_hot_candidate_score_required=1"
require_line "$DOC" "ir_shape_diff_required_before_next_keeper=1"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat <<REPORT
output_contract=weighted-exact-slot-owner-selection-after-receiver-forwarding-v0
selected_owner=result_capsule_ir_shape_diff_inventory
next_diagnostic=result_capsule_ir_shape_diff_inventory
page_queue_immediate_retry_blocked=1
page_model_immediate_retry_blocked=1
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

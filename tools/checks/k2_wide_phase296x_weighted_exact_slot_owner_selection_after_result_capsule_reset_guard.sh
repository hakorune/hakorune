#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-261-WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RESULT-CAPSULE-RESET.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-260-POST-RESULT-CAPSULE-RESET-BATCHING-OWNER-REFRESH.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row261-weighted-owner-after-result-capsule-reset] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$PREV" "output_contract=post-result-capsule-reset-batching-owner-refresh-v0"
require_line "$PREV" "dominant_family=page_model_hotpath"
require_line "$PREV" "dominant_family_pct=16.81"
require_line "$PREV" "recent_nonkeeper_family=page_queue_helpers"
require_line "$PREV" "recent_nonkeeper_row=296x-241"
require_line "$PREV" "ir_shape_diff_required_before_next_keeper=1"

require_line "$DOC" "output_contract=weighted-exact-slot-owner-selection-after-result-capsule-reset-v0"
require_line "$DOC" "input_contract=post-result-capsule-reset-batching-owner-refresh-v0"
require_line "$DOC" "dominant_family=page_model_hotpath"
require_line "$DOC" "dominant_family_pct=16.81"
require_line "$DOC" "dominant_family_is_recent_nonkeeper=0"
require_line "$DOC" "recent_nonkeeper_family=page_queue_helpers"
require_line "$DOC" "recent_nonkeeper_row=296x-241"
require_line "$DOC" "page_queue_immediate_retry_blocked=1"
require_line "$DOC" "top_unblocked_family=page_model_hotpath"
require_line "$DOC" "page_model_recent_no_material_effect_row=296x-252"
require_line "$DOC" "page_model_immediate_implementation_blocked=1"
require_line "$DOC" "page_model_ir_shape_refresh_required=1"
require_line "$DOC" "selected_family=page_model_hotpath"
require_line "$DOC" "selected_owner=page_model_hotpath_ir_shape_diff_refresh"
require_line "$DOC" "selected_reason=dominant_page_model_family_requires_fresh_ir_shape_after_prior_no_material_receiver_forwarding"
require_line "$DOC" "next_diagnostic=page_model_hotpath_ir_shape_diff_refresh"
require_line "$DOC" "rejected_owner=page_queue_immediate_retry"
require_line "$DOC" "rejected_owner_1=page_model_immediate_implementation"
require_line "$DOC" "rejected_owner_2=implementation_without_ir_shape_diff"
require_line "$DOC" "weighted_hot_candidate_score_required=1"
require_line "$DOC" "ir_shape_diff_required_before_next_keeper=1"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

echo "[row261-weighted-owner-after-result-capsule-reset] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-260-POST-RESULT-CAPSULE-RESET-BATCHING-OWNER-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-259-RESULT-CAPSULE-RESET-FIELD-BATCHING-MEASUREMENT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row260-post-result-capsule-reset-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=post-result-capsule-reset-batching-owner-refresh-v0"
require_line "$DOC" "input_contract=result-capsule-reset-field-batching-measurement-v0"
require_line "$DOC" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$DOC" "attribution_source=perf_callgraph"
require_line "$DOC" "callgraph_attribution_available=1"
require_line "$DOC" "perf_sample_count=125"
require_line "$DOC" "exact_slot_get_set_pct=52.67"
require_line "$DOC" "attributed_callsite_count=29"
require_line "$DOC" "top_callsite_pct=5.06"
require_line "$DOC" "top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2"
require_line "$DOC" "top_callsite_helper=nyash.object.exact_slot_get_i64_hii"
require_line "$DOC" "dominant_family=page_model_hotpath"
require_line "$DOC" "dominant_family_pct=16.81"
require_line "$DOC" "recent_nonkeeper_family=page_queue_helpers"
require_line "$DOC" "recent_nonkeeper_row=296x-241"
require_line "$DOC" "recent_nonkeeper_family_blocked_for_immediate_keeper=1"
require_line "$DOC" "top_unblocked_family=page_model_hotpath"
require_line "$DOC" "family_0_name=page_model_hotpath"
require_line "$DOC" "family_1_name=page_queue_helpers"
require_line "$DOC" "family_2_name=object_lifecycle_facade"
require_line "$DOC" "result_capsule_combined_pct=9.00"
require_line "$DOC" "result_capsule_reset_batch_helper_pct=1.80"
require_line "$DOC" "static_candidate_count_only_rejected=1"
require_line "$DOC" "weighted_hot_candidate_score_required=1"
require_line "$DOC" "ir_shape_diff_required_before_next_keeper=1"
require_line "$DOC" "selected_boundary=weighted_exact_slot_owner_selection"
require_line "$DOC" "next_diagnostic=weighted_exact_slot_owner_selection"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat <<REPORT
output_contract=post-result-capsule-reset-batching-owner-refresh-v0
exact_slot_get_set_pct=52.67
dominant_family=page_model_hotpath
dominant_family_pct=16.81
result_capsule_combined_pct=9.00
selected_boundary=weighted_exact_slot_owner_selection
next_diagnostic=weighted_exact_slot_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-284-POST-RECORD-SUCCESS-HELPER-FUSION-OWNER-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-283-RECORD-SUCCESS-HELPER-FUSION-MEASUREMENT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row284-post-record-success-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=post-record-success-helper-fusion-owner-refresh-v0"
require_line "$DOC" "input_contract=record-success-helper-fusion-measurement-v0"
require_line "$DOC" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$DOC" "attribution_source=perf_callgraph"
require_line "$DOC" "callgraph_attribution_available=1"
require_line "$DOC" "exact_slot_get_set_pct=50.97"
require_line "$DOC" "attributed_callsite_count=30"
require_line "$DOC" "top_callsite_pct=3.13"
require_line "$DOC" "top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2"
require_line "$DOC" "dominant_family=page_queue_helpers"
require_line "$DOC" "dominant_family_pct=14.32"
require_line "$DOC" "recent_nonkeeper_family=page_queue_helpers"
require_line "$DOC" "recent_nonkeeper_row=296x-241"
require_line "$DOC" "recent_nonkeeper_family_blocked_for_immediate_keeper=1"
require_line "$DOC" "top_unblocked_family=object_lifecycle_facade"
require_line "$DOC" "top_unblocked_family_pct=13.47"
require_line "$DOC" "family_0_name=page_queue_helpers"
require_line "$DOC" "family_1_name=object_lifecycle_facade"
require_line "$DOC" "family_2_name=page_model_hotpath"
require_line "$DOC" "helper_7_symbol=nyash.object.exact_slot_record_alloc_success_hii"
require_line "$DOC" "helper_8_symbol=nyash.object.exact_slot_record_release_success_hiii"
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
output_contract=post-record-success-helper-fusion-owner-refresh-v0
exact_slot_get_set_pct=50.97
dominant_family=page_queue_helpers
dominant_family_pct=14.32
recent_nonkeeper_family=page_queue_helpers
top_unblocked_family=object_lifecycle_facade
selected_boundary=weighted_exact_slot_owner_selection
next_diagnostic=weighted_exact_slot_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-270-POST-RELEASE-KNOWN-LIVE-RMW-ROLLBACK-OWNER-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-269-PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-ROLLBACK.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row270-post-release-known-live-rmw-rollback-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=post-release-known-live-rmw-rollback-owner-refresh-v0"
require_line "$DOC" "input_contract=page-model-release-known-live-single-use-rmw-rollback-v0"
require_line "$DOC" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$DOC" "attribution_source=perf_callgraph"
require_line "$DOC" "callgraph_attribution_available=1"
require_line "$DOC" "exact_slot_get_set_pct=49.64"
require_line "$DOC" "attributed_callsite_count=26"
require_line "$DOC" "top_callsite_pct=6.09"
require_line "$DOC" "top_callsite_symbol=HakoAllocObjectLifecycleAllocResult.recordSuccess/1"
require_line "$DOC" "top_callsite_helper=nyash.object.exact_slot_set_i64_hii"
require_line "$DOC" "dominant_family=object_lifecycle_facade"
require_line "$DOC" "dominant_family_pct=15.68"
require_line "$DOC" "recent_nonkeeper_family=page_model_hotpath"
require_line "$DOC" "recent_nonkeeper_row=296x-268"
require_line "$DOC" "recent_nonkeeper_family_blocked_for_immediate_keeper=1"
require_line "$DOC" "top_unblocked_family=object_lifecycle_facade"
require_line "$DOC" "family_0_name=object_lifecycle_facade"
require_line "$DOC" "family_1_name=page_model_hotpath"
require_line "$DOC" "family_2_name=alloc_result_capsule"
require_line "$DOC" "family_3_name=page_queue_helpers"
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
output_contract=post-release-known-live-rmw-rollback-owner-refresh-v0
exact_slot_get_set_pct=49.64
dominant_family=object_lifecycle_facade
dominant_family_pct=15.68
recent_nonkeeper_family=page_model_hotpath
selected_boundary=weighted_exact_slot_owner_selection
next_diagnostic=weighted_exact_slot_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

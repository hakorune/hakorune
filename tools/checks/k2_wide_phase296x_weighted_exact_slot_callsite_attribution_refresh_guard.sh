#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-244-WEIGHTED-EXACT-SLOT-CALLSITE-ATTRIBUTION-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-243-POST-PAGE-QUEUE-ROLLBACK-OWNER-REFRESH.md"
TOOL="$ROOT_DIR/tools/allocator/weighted_exact_slot_callsite_attribution_refresh.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row244_weighted_attr.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PERF_REPORT="$TMP_DIR/perf.report"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row244-weighted-attr] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=weighted-exact-slot-callsite-attribution-refresh-v0"
require_line "$DOC" "dominant_family=page_queue_helpers"
require_line "$DOC" "dominant_family_is_recent_nonkeeper=1"
require_line "$DOC" "recent_nonkeeper_family_blocked_for_immediate_keeper=1"
require_line "$DOC" "top_unblocked_family=page_model_hotpath"
require_line "$DOC" "weighted_hot_candidate_score_required=1"
require_line "$DOC" "ir_shape_diff_required_before_next_keeper=1"
require_line "$DOC" "selected_boundary=weighted_exact_slot_owner_selection"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$PERF_REPORT" <<'REPORT'
    17.17%  app.exe  app.exe               [.] nyash.object.exact_slot_set_i64_hii
               |--3.52%--HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
               |--3.42%--HakoAllocObjectLifecycleAllocResult.reset/0
               |--2.57%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--2.56%--HakoAllocObjectLifecyclePageQueue.beginSelection/0
               |--2.53%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--1.71%--HakoAllocObjectLifecycleReleaseResult.reset/0
                --0.86%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
    15.51%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
               |--4.60%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--4.18%--HakoAllocPageModel.acquire_usize/1
               |--3.39%--HakoAllocPageModel.isRetired/0
               |--1.66%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
               |--0.85%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
                --0.83%--HakoAllocPageModel.isDecommitted/0
    11.38%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
               |--3.66%--HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
               |--3.44%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--1.74%--HakoAllocPageModel.acquire_usize/1
               |--1.70%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
                --0.85%--HakoAllocPageModel.releaseLocalKnownLive/1
     4.23%  app.exe  app.exe               [.] nyash.object.exact_slot_set_u64_hiu
               |--1.70%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--0.86%--HakoAllocPageModel.acquire_usize/1
               |--0.85%--HakoAllocPageModel.resetToFresh/0
                --0.82%--HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
     3.44%  app.exe  app.exe               [.] nyash.object.exact_slot_get_handle_hii
               |--1.74%--HakoAllocPageModel.acquire_usize/1
               |--0.85%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
                --0.85%--HakoAllocPageModel.releaseLocalKnownLive/1
     2.56%  app.exe  app.exe               [.] nyash.object.exact_slot_set_handle_hii
               |--0.86%--HakoAllocObjectLifecyclePageQueue.beginSelection/0
               |--0.85%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
                --0.85%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
REPORT

python3 "$TOOL" \
  --perf-report "$PERF_REPORT" \
  --input-contract post-page-queue-rollback-owner-refresh-v0 \
  --recent-nonkeeper-family page_queue_helpers \
  --recent-nonkeeper-row 296x-241 \
  --family-candidate page_queue_helpers=21 \
  --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=weighted-exact-slot-callsite-attribution-refresh-v0"
require_line "$REPORT" "input_contract=post-page-queue-rollback-owner-refresh-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "exact_slot_get_set_pct=54.29"
require_line "$REPORT" "attributed_callsite_count=28"
require_line "$REPORT" "top_callsite_pct=4.60"
require_line "$REPORT" "top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "$REPORT" "dominant_family=page_queue_helpers"
require_line "$REPORT" "dominant_family_pct=16.45"
require_line "$REPORT" "recent_nonkeeper_family=page_queue_helpers"
require_line "$REPORT" "recent_nonkeeper_candidate_count=21"
require_line "$REPORT" "recent_nonkeeper_hot_per_candidate_pct=0.78"
require_line "$REPORT" "dominant_family_is_recent_nonkeeper=1"
require_line "$REPORT" "recent_nonkeeper_family_blocked_for_immediate_keeper=1"
require_line "$REPORT" "top_unblocked_family=page_model_hotpath"
require_line "$REPORT" "top_unblocked_family_pct=15.29"
require_line "$REPORT" "static_candidate_count_only_rejected=1"
require_line "$REPORT" "weighted_hot_candidate_score_required=1"
require_line "$REPORT" "ir_shape_diff_required_before_next_keeper=1"
require_line "$REPORT" "selected_boundary=weighted_exact_slot_owner_selection"
require_line "$REPORT" "next_diagnostic=weighted_exact_slot_owner_selection"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

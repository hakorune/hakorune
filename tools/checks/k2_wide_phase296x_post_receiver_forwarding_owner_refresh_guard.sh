#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-253-POST-RECEIVER-FORWARDING-OWNER-REFRESH.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-252-SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-MEASUREMENT.md"
OWNER_TOOL="$ROOT_DIR/tools/allocator/post_page_queue_rollback_owner_refresh.py"
WEIGHTED_TOOL="$ROOT_DIR/tools/allocator/weighted_exact_slot_callsite_attribution_refresh.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row253_owner_refresh.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PERF="$TMP_DIR/perf.report"
OWNER="$TMP_DIR/owner.out"
WEIGHTED="$TMP_DIR/weighted.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row253-owner-refresh] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=post-receiver-forwarding-owner-refresh-v0"
require_line "$DOC" "input_contract=selected-method-receiver-block-entry-copy-forwarding-measurement-v0"
require_line "$DOC" "perf_exact_slot_get_set_pct=52.64"
require_line "$DOC" "perf_array_total_pct=31.88"
require_line "$DOC" "selected_boundary=weighted_exact_slot_callsite_attribution_refresh"
require_line "$DOC" "next_diagnostic=weighted_exact_slot_owner_selection"
require_line "$DOC" "recent_nonkeeper_family_blocked_for_immediate_keeper=1"
require_line "$DOC" "top_unblocked_family=page_model_hotpath"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "summary=ok"

cat >"$PERF" <<'REPORT'
    17.81%  app.exe  app.exe               [.] nyash.object.exact_slot_set_i64_hii
            |--3.55%--HakoAllocObjectLifecycleReleaseResult.recordRequest/2
            |--2.78%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
            |--2.47%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
            |--2.45%--HakoAllocObjectLifecycleAllocResult.reset/0
            |--2.34%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
            |--2.09%--HakoAllocObjectLifecycleReleaseResult.reset/0
            |--1.06%--HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
             --1.06%--HakoAllocObjectLifecyclePageQueue.beginSelection/0
    14.55%  app.exe  app.exe               [.] core::hash::BuildHasher::hash_one
            |--11.21%--nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
             --3.34%--nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64
    13.77%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
    10.43%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
            |--1.78%--HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
            |--1.76%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
            |--1.76%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
            |--1.42%--HakoAllocPageModel.freeCount/0
            |--1.22%--HakoAllocPageModel.acquire_usize/1
            |--1.06%--HakoAllocPageModel.releaseLocalKnownLive/1
            |--0.71%--HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
             --0.71%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
     8.15%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
            |--3.20%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
            |--2.49%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
            |--0.83%--HakoAllocPageModel.isDecommitted/0
            |--0.68%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
             --0.66%--HakoAllocPageModel.acquire_usize/1
     7.36%  app.exe  app.exe               [.] nyash.object.exact_slot_set_u64_hiu
            |--1.75%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
            |--1.73%--HakoAllocPageModel.releaseLocalKnownLive/1
            |--1.39%--HakoAllocPageModel.acquire_usize/1
            |--1.07%--HakoAllocPageModel.resetToFresh/0
             --0.71%--HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
     7.05%  app.exe  app.exe               [.] nyash.object.exact_slot_get_handle_hii
            |--2.48%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
            |--1.37%--HakoAllocPageModel.acquire_usize/1
             --1.06%--HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
     4.08%  app.exe  app.exe               [.] nyash.object.exact_slot_rmw_add_u64_hiii
     2.14%  app.exe  app.exe               [.] nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64
     1.84%  app.exe  app.exe               [.] nyash.object.exact_slot_set_handle_hii
             --1.49%--HakoAllocObjectLifecyclePageQueue.beginSelection/0
     1.79%  app.exe  app.exe               [.] HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
     1.42%  app.exe  app.exe               [.] <core::hash::sip::Hasher<S> as core::hash::Hasher>::write
     1.07%  app.exe  app.exe               [.] HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
     1.05%  app.exe  app.exe               [.] HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
     1.02%  app.exe  app.exe               [.] HakoAllocPageModel.releaseLocalKnownLive/1
     0.71%  app.exe  app.exe               [.] nyash.array.slot_load_store_i64_hihi
     0.71%  app.exe  app.exe               [.] HakoAllocObjectLifecycleAllocResult.recordSuccess/1
     0.71%  app.exe  app.exe               [.] HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
     0.71%  app.exe  app.exe               [.] HakoAllocPageModel.acquire_usize/1
REPORT

python3 "$OWNER_TOOL" --perf-report "$PERF" --out "$OWNER" >/dev/null
python3 "$WEIGHTED_TOOL" \
  --perf-report "$PERF" \
  --out "$WEIGHTED" \
  --input-contract post-receiver-forwarding-owner-refresh-v0 \
  --recent-nonkeeper-family page_queue_helpers \
  --recent-nonkeeper-row 296x-241 \
  --family-candidate page_queue_helpers=21 \
  --family-candidate page_model_hotpath=9 \
  --family-candidate object_lifecycle_facade=4 >/dev/null

require_line "$OWNER" "output_contract=post-page-queue-rollback-owner-refresh-v0"
require_line "$OWNER" "perf_exact_slot_get_set_pct=52.64"
require_line "$OWNER" "perf_array_total_pct=31.88"
require_line "$OWNER" "selected_boundary=weighted_exact_slot_callsite_attribution_refresh"
require_line "$OWNER" "next_diagnostic=weighted_exact_slot_callsite_attribution_refresh"
require_line "$OWNER" "summary=ok"

require_line "$WEIGHTED" "output_contract=weighted-exact-slot-callsite-attribution-refresh-v0"
require_line "$WEIGHTED" "input_contract=post-receiver-forwarding-owner-refresh-v0"
require_line "$WEIGHTED" "exact_slot_get_set_pct=52.64"
require_line "$WEIGHTED" "dominant_family=page_queue_helpers"
require_line "$WEIGHTED" "dominant_family_is_recent_nonkeeper=1"
require_line "$WEIGHTED" "recent_nonkeeper_family_blocked_for_immediate_keeper=1"
require_line "$WEIGHTED" "top_unblocked_family=page_model_hotpath"
require_line "$WEIGHTED" "selected_boundary=weighted_exact_slot_owner_selection"
require_line "$WEIGHTED" "next_diagnostic=weighted_exact_slot_owner_selection"
require_line "$WEIGHTED" "summary=ok"

cat "$WEIGHTED"

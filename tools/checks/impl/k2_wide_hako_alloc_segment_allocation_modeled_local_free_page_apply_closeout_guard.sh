#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-page-apply-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-page-apply-closeout-ssot.md"
PAGE_APPLY_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-page-apply-ssot.md"
SCALAR_CLOSEOUT_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-scalar-lane-closeout-ssot.md"
PAGE_PILOT_SSOT="docs/development/current/main/design/mimalloc-page-free-list-pilot-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_115A="docs/development/current/main/phases/phase-293x/293x-614-MIMAP-115A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-PAGE-MODEL-APPLY-ROUTE.md"
CARD_117A="docs/development/current/main/phases/phase-293x/293x-616-MIMAP-117A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-PAGE-APPLY-CLOSEOUT-GUARD.md"
CARD_118A="docs/development/current/main/phases/phase-293x/293x-617-MIMAP-118A-POST-LOCAL-FREE-PAGE-APPLY-CLOSEOUT-ROW-SELECTION.md"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako"
PAGE_OWNER="lang/src/hako_alloc/memory/page_box.hako"
APP="apps/hako-alloc-segment-allocation-modeled-local-free-page-model-apply-proof/main.hako"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-page-model-apply-proof/test.sh"
GUARD_115A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_page_model_apply_guard.sh"
SCALAR_CLOSEOUT_GUARD="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_scalar_lane_closeout_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_page_apply_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_page_apply_closeout_guard.sh"

echo "[$TAG] checking MIMAP-117A segment allocation modeled local-free page-apply closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$PAGE_APPLY_SSOT" \
  "$SCALAR_CLOSEOUT_SSOT" \
  "$PAGE_PILOT_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_115A" \
  "$CARD_117A" \
  "$CARD_118A" \
  "$OWNER" \
  "$PAGE_OWNER" \
  "$APP" \
  "$APP_TEST" \
  "$GUARD_115A" \
  "$SCALAR_CLOSEOUT_GUARD" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_115A" "$SCALAR_CLOSEOUT_GUARD" "$SELF_SCRIPT" "$IMPL_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_115A" "MIMAP-115A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_117A" "MIMAP-117A card must be landed"
guard_expect_in_file "$TAG" "MIMAP-118A" "$CARD_118A" "MIMAP-118A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-117A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$PAGE_APPLY_SSOT" "MIMAP-115A page-apply SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$SCALAR_CLOSEOUT_SSOT" "MIMAP-113A scalar closeout SSOT must stay accepted"
guard_expect_in_file "$TAG" "releaseLocal" "$PAGE_PILOT_SSOT" "page/free-list pilot must keep releaseLocal contract"
guard_expect_in_file "$TAG" "MIMAP-115A" "$SSOT" "closeout SSOT must include page-apply row"
guard_expect_in_file "$TAG" "MIMAP-118A post-local-free-page-apply-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-115A" "$GRANULARITY" "granularity SSOT must describe MIMAP-115A"
guard_expect_in_file "$TAG" "MIMAP-117A" "$GRANULARITY" "granularity SSOT must describe MIMAP-117A"
guard_expect_in_file "$TAG" "MIMAP-118A" "$GRANULARITY" "granularity SSOT must describe MIMAP-118A"
guard_expect_in_file "$TAG" "MIMAP-117A segment allocation modeled local-free page-apply closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-118A post-local-free-page-apply-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-118A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-115A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-115A"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-allocation-modeled-local-free-page-apply-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-117A closeout row"
guard_expect_in_file "$TAG" "$GUARD_115A" "$INDEX" "check index must list MIMAP-115A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-117A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_page_apply_box = "memory/segment_allocation_modeled_local_free_page_apply_box.hako"' "$MODULE" "page apply owner must stay exported"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_page_apply_box.hako` owns' "$MEMORY_README" "memory README must name MIMAP-115A owner"
guard_expect_in_file "$TAG" "recordLocalFreePageApply" "$OWNER" "page apply owner route must stay present"
guard_expect_in_file "$TAG" "releaseLocal" "$OWNER" "page apply owner must call page model releaseLocal"
guard_expect_in_file "$TAG" "blockIsLive" "$OWNER" "page apply owner must keep live-block preflight"
guard_expect_in_file "$TAG" "would_directly_mutate_page_arrays" "$OWNER" "page apply owner must expose direct page-array inactive flag"
guard_expect_in_file "$TAG" "releaseLocal" "$PAGE_OWNER" "page owner must keep releaseLocal"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledLocalFreePageApply" "$APP" "proof must construct page apply owner"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "page-apply closeout must keep raw pointer/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)|\\.set\\(' "$OWNER" >/tmp/"$TAG".page_array_leak 2>&1; then
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  guard_fail "$TAG" "page-apply closeout must keep direct page array mutation out of the route owner"
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "page-apply closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-local-free-page-model-apply-proof|LocalFreePageApply|recordLocalFreePageApply' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "page-apply app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

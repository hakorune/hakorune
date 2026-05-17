#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-reuse-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-reuse-closeout-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_126A="docs/development/current/main/phases/phase-293x/293x-632-MIMAP-126A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-ROUTE.md"
CARD_127A="docs/development/current/main/phases/phase-293x/293x-633-MIMAP-127A-POST-LOCAL-FREE-REUSE-ROW-SELECTION.md"
CARD_128A="docs/development/current/main/phases/phase-293x/293x-634-MIMAP-128A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-CLOSEOUT-GUARD.md"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_box.hako"
INTEGRATION_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako"
PAGE_OWNER="lang/src/hako_alloc/memory/page_box.hako"
APP="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-proof/main.hako"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-proof/test.sh"
GUARD_126A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_closeout_guard.sh"

echo "[$TAG] checking MIMAP-128A segment allocation modeled local-free reuse closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_126A" \
  "$CARD_127A" \
  "$CARD_128A" \
  "$OWNER" \
  "$INTEGRATION_OWNER" \
  "$PAGE_OWNER" \
  "$APP" \
  "$APP_TEST" \
  "$GUARD_126A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_126A" "$SELF_SCRIPT" "$IMPL_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_126A" "MIMAP-126A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_127A" "MIMAP-127A card must be landed"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_128A" "MIMAP-128A card must be landed"
guard_expect_in_file "$TAG" "MIMAP-129A" "$CARD_128A" "MIMAP-128A must select the next row"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-128A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "MIMAP-126A" "$SSOT" "closeout SSOT must include reuse row"
guard_expect_in_file "$TAG" "MIMAP-129A post-local-free-reuse-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-126A" "$GRANULARITY" "granularity SSOT must describe MIMAP-126A"
guard_expect_in_file "$TAG" "MIMAP-128A" "$GRANULARITY" "granularity SSOT must describe MIMAP-128A"
guard_expect_in_file "$TAG" "MIMAP-129A" "$GRANULARITY" "granularity SSOT must describe MIMAP-129A"
guard_expect_in_file "$TAG" "MIMAP-128A segment allocation modeled local-free reuse closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-129A post-local-free-reuse-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-129A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-126A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-126A"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-allocation-modeled-local-free-reuse-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-128A closeout row"
guard_expect_in_file "$TAG" "$GUARD_126A" "$INDEX" "check index must list MIMAP-126A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-128A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_box = "memory/segment_allocation_modeled_local_free_reuse_box.hako"' "$MODULE" "reuse owner must stay exported"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_reuse_box.hako` owns' "$MEMORY_README" "memory README must name MIMAP-126A owner"
guard_expect_in_file "$TAG" "integrateAndReuseLocalFree" "$OWNER" "reuse owner route must stay present"
guard_expect_in_file "$TAG" "integrateLocalFree" "$OWNER" "reuse owner must compose integration"
guard_expect_in_file "$TAG" "page[.]acquire" "$OWNER" "reuse owner must call page acquire"
guard_expect_in_file "$TAG" "local_free_collect_count" "$OWNER" "reuse owner must observe local-free collection"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledLocalFreeReuse" "$APP" "proof must construct reuse owner"
guard_expect_in_file "$TAG" "acquire[(]requested_size[)]" "$PAGE_OWNER" "page model acquire route must stay present"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "reuse closeout must keep raw pointer/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)|\\.set\\(' "$OWNER" >/tmp/"$TAG".page_array_leak 2>&1; then
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  guard_fail "$TAG" "reuse closeout must keep direct page array mutation out of the route owner"
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_TEST" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "reuse closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-local-free-reuse-proof|LocalFreeReuse|integrateAndReuseLocalFree' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "reuse app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

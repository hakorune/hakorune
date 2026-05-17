#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-integration-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-integration-closeout-ssot.md"
INTEGRATION_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-integration-ssot.md"
PAGE_APPLY_CLOSEOUT_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-page-apply-closeout-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_119A="docs/development/current/main/phases/phase-293x/293x-618-MIMAP-119A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-INTEGRATION-ROUTE.md"
CARD_121A="docs/development/current/main/phases/phase-293x/293x-620-MIMAP-121A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-INTEGRATION-CLOSEOUT-GUARD.md"
CARD_122A="docs/development/current/main/phases/phase-293x/293x-621-MIMAP-122A-POST-LOCAL-FREE-INTEGRATION-CLOSEOUT-ROW-SELECTION.md"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_integration_box.hako"
PAGE_APPLY_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_page_apply_box.hako"
APP="apps/hako-alloc-segment-allocation-modeled-local-free-integration-proof/main.hako"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-integration-proof/test.sh"
GUARD_119A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_integration_closeout_guard.sh"

echo "[$TAG] checking MIMAP-121A segment allocation modeled local-free integration closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$INTEGRATION_SSOT" \
  "$PAGE_APPLY_CLOSEOUT_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_119A" \
  "$CARD_121A" \
  "$CARD_122A" \
  "$OWNER" \
  "$PAGE_APPLY_OWNER" \
  "$APP" \
  "$APP_TEST" \
  "$GUARD_119A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_119A" "$SELF_SCRIPT" "$IMPL_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_119A" "MIMAP-119A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_121A" "MIMAP-121A card must be landed"
guard_expect_in_file "$TAG" "MIMAP-122A" "$CARD_122A" "MIMAP-122A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-121A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INTEGRATION_SSOT" "MIMAP-119A integration SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$PAGE_APPLY_CLOSEOUT_SSOT" "MIMAP-117A page-apply closeout SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-119A" "$SSOT" "closeout SSOT must include integration row"
guard_expect_in_file "$TAG" "MIMAP-122A post-local-free-integration-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-119A" "$GRANULARITY" "granularity SSOT must describe MIMAP-119A"
guard_expect_in_file "$TAG" "MIMAP-121A" "$GRANULARITY" "granularity SSOT must describe MIMAP-121A"
guard_expect_in_file "$TAG" "MIMAP-122A" "$GRANULARITY" "granularity SSOT must describe MIMAP-122A"
guard_expect_in_file "$TAG" "MIMAP-121A segment allocation modeled local-free integration closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-122A post-local-free-integration-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-122A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-119A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-119A"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-allocation-modeled-local-free-integration-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-121A closeout row"
guard_expect_in_file "$TAG" "$GUARD_119A" "$INDEX" "check index must list MIMAP-119A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-121A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_integration_box = "memory/segment_allocation_modeled_local_free_integration_box.hako"' "$MODULE" "integration owner must stay exported"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_integration_box.hako` owns' "$MEMORY_README" "memory README must name MIMAP-119A owner"
guard_expect_in_file "$TAG" "integrateLocalFree" "$OWNER" "integration owner route must stay present"
guard_expect_in_file "$TAG" "recordLocalFreeCandidate" "$OWNER" "integration owner must compose candidate ledger"
guard_expect_in_file "$TAG" "recordLocalFreeApplyPlan" "$OWNER" "integration owner must compose apply plan ledger"
guard_expect_in_file "$TAG" "recordLocalFreePageApply" "$OWNER" "integration owner must compose page apply"
guard_expect_in_file "$TAG" "would_directly_mutate_page_arrays" "$OWNER" "integration report must expose direct page-array inactive flag"
guard_expect_in_file "$TAG" "releaseLocal" "$PAGE_APPLY_OWNER" "page apply owner must keep releaseLocal route"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledLocalFreeIntegration" "$APP" "proof must construct integration owner"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "integration closeout must keep raw pointer/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)|\\.set\\(' "$OWNER" >/tmp/"$TAG".page_array_leak 2>&1; then
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  guard_fail "$TAG" "integration closeout must keep direct page array mutation out of the route owner"
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "integration closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-local-free-integration-proof|LocalFreeIntegration|integrateLocalFree' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "integration app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

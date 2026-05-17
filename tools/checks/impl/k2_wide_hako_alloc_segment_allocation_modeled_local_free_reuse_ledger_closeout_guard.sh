#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-closeout-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_130A="docs/development/current/main/phases/phase-293x/293x-636-MIMAP-130A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-LEDGER-ROUTE.md"
CARD_131A="docs/development/current/main/phases/phase-293x/293x-637-MIMAP-131A-POST-LOCAL-FREE-REUSE-LEDGER-ROW-SELECTION.md"
CARD_132A="docs/development/current/main/phases/phase-293x/293x-638-MIMAP-132A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-LEDGER-CLOSEOUT-GUARD.md"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
REUSE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_box.hako"
BUMP_LEDGER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako"
APP="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-proof/test.sh"
GUARD_130A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_closeout_guard.sh"

echo "[$TAG] checking MIMAP-132A segment allocation modeled local-free reuse ledger closeout"

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
  "$CARD_130A" \
  "$CARD_131A" \
  "$CARD_132A" \
  "$OWNER" \
  "$REUSE_OWNER" \
  "$BUMP_LEDGER" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$GUARD_130A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_130A" "$SELF_SCRIPT" "$IMPL_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_130A" "MIMAP-130A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_131A" "MIMAP-131A card must be landed"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_132A" "MIMAP-132A card must be landed"
guard_expect_in_file "$TAG" "MIMAP-133A" "$CARD_132A" "MIMAP-132A must select the next row"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-132A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "MIMAP-130A" "$SSOT" "closeout SSOT must include reuse ledger row"
guard_expect_in_file "$TAG" "MIMAP-133A post-local-free-reuse-ledger-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-130A" "$GRANULARITY" "granularity SSOT must describe MIMAP-130A"
guard_expect_in_file "$TAG" "MIMAP-132A" "$GRANULARITY" "granularity SSOT must describe MIMAP-132A"
guard_expect_in_file "$TAG" "MIMAP-133A" "$GRANULARITY" "granularity SSOT must describe MIMAP-133A"
guard_expect_in_file "$TAG" "MIMAP-132A segment allocation modeled local-free reuse ledger closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-133A post-local-free-reuse-ledger-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-133A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-130A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-130A"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-132A closeout row"
guard_expect_in_file "$TAG" "$GUARD_130A" "$INDEX" "check index must list MIMAP-130A route guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-132A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_ledger_box = "memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"' "$MODULE" "reuse ledger owner must stay exported"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_reuse_ledger_box.hako` owns' "$MEMORY_README" "memory README must name MIMAP-130A owner"
guard_expect_in_file "$TAG" "recordLocalFreeReuse" "$OWNER" "reuse ledger record route must stay present"
guard_expect_in_file "$TAG" "makeReuseToken" "$OWNER" "reuse ledger token derivation must stay present"
guard_expect_in_file "$TAG" "local_free_reuse_ledger_present" "$OWNER" "reuse ledger report must expose presence flag"
guard_expect_in_file "$TAG" "reused_block_id" "$OWNER" "reuse ledger must stay keyed by reused block"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledLocalFreeReuseLedger" "$APP" "proof must construct reuse ledger owner"

if rg -n 'segment_allocation_modeled_ledger_box|recordModeledConsume|releaseModeledToken' "$OWNER" >/tmp/"$TAG".bump_ledger_leak 2>&1; then
  cat /tmp/"$TAG".bump_ledger_leak >&2
  rm -f /tmp/"$TAG".bump_ledger_leak
  guard_fail "$TAG" "reuse ledger closeout must not widen or depend on the bump-shaped modeled ledger"
fi
rm -f /tmp/"$TAG".bump_ledger_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "reuse ledger closeout must keep raw pointer/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)|\\.set\\(' "$OWNER" >/tmp/"$TAG".page_array_leak 2>&1; then
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  guard_fail "$TAG" "reuse ledger closeout must keep direct page array mutation out of the route owner"
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "reuse ledger closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-proof|LocalFreeReuseLedger|recordLocalFreeReuse' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "reuse ledger app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

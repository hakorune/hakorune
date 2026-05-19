#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-closeout-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_138A="docs/development/current/main/phases/phase-293x/293x-644-MIMAP-138A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-ROUTE.md"
CARD_139A="docs/development/current/main/phases/phase-293x/293x-645-MIMAP-139A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-LEDGER-RELEASE-APPLY-CLOSEOUT-GUARD.md"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
RELEASE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"
APP="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-proof/test.sh"
GUARD_138A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_closeout_guard.sh"

echo "[$TAG] checking MIMAP-139A segment allocation modeled local-free reuse ledger release apply closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$MEMORY_README" \
  "$CARD_138A" \
  "$CARD_139A" \
  "$OWNER" \
  "$RELEASE_OWNER" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$GUARD_138A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_138A" "$SELF_SCRIPT" "$IMPL_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_138A" "MIMAP-138A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_139A" "MIMAP-139A card must be landed"
guard_expect_in_file "$TAG" "MIMAP-140A" "$CARD_139A" "MIMAP-139A must select the next row"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-139A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "MIMAP-138A" "$SSOT" "closeout SSOT must include release apply row"
guard_expect_in_file "$TAG" "MIMAP-140A post-local-free-reuse-ledger-release-apply-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-138A" "$GRANULARITY" "granularity SSOT must describe MIMAP-138A"
guard_expect_in_file "$TAG" "MIMAP-139A" "$GRANULARITY" "granularity SSOT must describe MIMAP-139A"
guard_expect_in_file "$TAG" "MIMAP-140A" "$GRANULARITY" "granularity SSOT must describe MIMAP-140A"
guard_expect_in_file "$TAG" "MIMAP-139A segment allocation modeled local-free reuse ledger release apply closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-140A post-local-free-reuse-ledger-release-apply-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-140A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-138A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-138A"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-139A closeout row"
guard_expect_in_file "$TAG" "$GUARD_138A" "$INDEX" "check index must list MIMAP-138A route guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-139A closeout guard"
guard_expect_in_file "$TAG" "MIMAP-130A, MIMAP-138A, MIMAP-142A" "$MEMORY_README" "memory README must name release apply owner row"
guard_expect_in_file "$TAG" "applyReuseLedgerRelease" "$OWNER" "release apply route must stay present"
guard_expect_in_file "$TAG" "local_free_reuse_ledger_release_apply_present" "$OWNER" "release apply report must expose presence flag"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerReleaseApplyReport" "$OWNER" "release apply report type must stay present"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerRelease" "$APP" "proof must construct release facts owner"

if rg -n 'segment_allocation_modeled_ledger_box|recordModeledConsume|releaseModeledToken' "$OWNER" >/tmp/"$TAG".bump_ledger_leak 2>&1; then
  cat /tmp/"$TAG".bump_ledger_leak >&2
  rm -f /tmp/"$TAG".bump_ledger_leak
  guard_fail "$TAG" "release apply closeout must not widen or depend on the bump-shaped modeled ledger"
fi
rm -f /tmp/"$TAG".bump_ledger_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "release apply closeout must keep raw pointer/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)' "$OWNER" >/tmp/"$TAG".page_array_leak 2>&1; then
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  guard_fail "$TAG" "release apply closeout must keep direct page array mutation out of the route owner"
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "release apply closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-apply-proof|LocalFreeReuseLedgerReleaseApply|applyReuseLedgerRelease' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "release apply app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

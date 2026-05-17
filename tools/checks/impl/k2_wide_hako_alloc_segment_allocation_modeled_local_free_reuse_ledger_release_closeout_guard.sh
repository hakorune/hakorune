#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-closeout-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_134A="docs/development/current/main/phases/phase-293x/293x-640-MIMAP-134A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-LEDGER-RELEASE-ROUTE.md"
CARD_135A="docs/development/current/main/phases/phase-293x/293x-641-MIMAP-135A-POST-LOCAL-FREE-REUSE-LEDGER-RELEASE-ROW-SELECTION.md"
CARD_136A="docs/development/current/main/phases/phase-293x/293x-642-MIMAP-136A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-REUSE-LEDGER-RELEASE-CLOSEOUT-GUARD.md"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"
SOURCE_LEDGER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako"
BUMP_LEDGER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako"
APP="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-proof/main.hako"
APP_README="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-proof/README.md"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-proof/test.sh"
GUARD_134A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_closeout_guard.sh"

echo "[$TAG] checking MIMAP-136A segment allocation modeled local-free reuse ledger release closeout"

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
  "$CARD_134A" \
  "$CARD_135A" \
  "$CARD_136A" \
  "$OWNER" \
  "$SOURCE_LEDGER" \
  "$BUMP_LEDGER" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$GUARD_134A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_134A" "$SELF_SCRIPT" "$IMPL_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_134A" "MIMAP-134A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_135A" "MIMAP-135A card must be landed"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_136A" "MIMAP-136A card must be landed"
guard_expect_in_file "$TAG" "MIMAP-137A" "$CARD_136A" "MIMAP-136A must select the next row"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-136A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "MIMAP-134A" "$SSOT" "closeout SSOT must include release route row"
guard_expect_in_file "$TAG" "MIMAP-137A post-local-free-reuse-ledger-release-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-134A" "$GRANULARITY" "granularity SSOT must describe MIMAP-134A"
guard_expect_in_file "$TAG" "MIMAP-136A" "$GRANULARITY" "granularity SSOT must describe MIMAP-136A"
guard_expect_in_file "$TAG" "MIMAP-137A" "$GRANULARITY" "granularity SSOT must describe MIMAP-137A"
guard_expect_in_file "$TAG" "MIMAP-136A segment allocation modeled local-free reuse ledger release closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-137A post-local-free-reuse-ledger-release-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-137A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-134A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-134A"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-136A closeout row"
guard_expect_in_file "$TAG" "$GUARD_134A" "$INDEX" "check index must list MIMAP-134A route guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-136A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_reuse_ledger_release_box = "memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako"' "$MODULE" "release owner must stay exported"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_reuse_ledger_release_box.hako` owns' "$MEMORY_README" "memory README must name MIMAP-134A release owner"
guard_expect_in_file "$TAG" "recordReuseLedgerRelease" "$OWNER" "release route must stay present"
guard_expect_in_file "$TAG" "local_free_reuse_ledger_release_present" "$OWNER" "release report must expose presence flag"
guard_expect_in_file "$TAG" "modeled_reuse_token" "$OWNER" "release owner must stay keyed by modeled reuse token"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledLocalFreeReuseLedgerRelease" "$APP" "proof must construct release owner"

if rg -n 'segment_allocation_modeled_ledger_box|recordModeledConsume|releaseModeledToken' "$OWNER" >/tmp/"$TAG".bump_ledger_leak 2>&1; then
  cat /tmp/"$TAG".bump_ledger_leak >&2
  rm -f /tmp/"$TAG".bump_ledger_leak
  guard_fail "$TAG" "release closeout must not widen or depend on the bump-shaped modeled ledger"
fi
rm -f /tmp/"$TAG".bump_ledger_leak

if rg -n 'source_ledger\.|reuse_ledger\.live_flags|reuse_ledger\.set|recordLocalFreeReuse[[:space:]]*=' "$OWNER" >/tmp/"$TAG".source_ledger_leak 2>&1; then
  cat /tmp/"$TAG".source_ledger_leak >&2
  rm -f /tmp/"$TAG".source_ledger_leak
  guard_fail "$TAG" "release closeout must keep mutation of the source reuse ledger out of the route owner"
fi
rm -f /tmp/"$TAG".source_ledger_leak

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "release closeout must keep raw pointer/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'page\.(free|local_free|block_used)' "$OWNER" >/tmp/"$TAG".page_array_leak 2>&1; then
  cat /tmp/"$TAG".page_array_leak >&2
  rm -f /tmp/"$TAG".page_array_leak
  guard_fail "$TAG" "release closeout must keep direct page array mutation out of the route owner"
fi
rm -f /tmp/"$TAG".page_array_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "release closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-local-free-reuse-ledger-release-proof|LocalFreeReuseLedgerRelease|recordReuseLedgerRelease' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "release app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-ledger-release-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-release-closeout-ssot.md"
RELEASE_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-release-ssot.md"
LEDGER_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_097A="docs/development/current/main/phases/phase-293x/293x-594-MIMAP-097A-SEGMENT-ALLOCATION-MODELED-LEDGER-RELEASE-ROUTE.md"
CARD_098A="docs/development/current/main/phases/phase-293x/293x-595-MIMAP-098A-SEGMENT-ALLOCATION-MODELED-LEDGER-RELEASE-CLOSEOUT-GUARD.md"
CARD_099A="docs/development/current/main/phases/phase-293x/293x-596-MIMAP-099A-POST-SEGMENT-ALLOCATION-MODELED-RELEASE-ROW-SELECTION.md"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako"
APP="apps/hako-alloc-segment-allocation-modeled-ledger-release-proof/main.hako"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-ledger-release-proof/test.sh"
GUARD_097A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_release_closeout_guard.sh"

echo "[$TAG] checking MIMAP-098A segment allocation modeled ledger release closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$RELEASE_SSOT" \
  "$LEDGER_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_097A" \
  "$CARD_098A" \
  "$CARD_099A" \
  "$OWNER" \
  "$APP" \
  "$APP_TEST" \
  "$GUARD_097A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_097A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_097A" "MIMAP-097A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_098A" "MIMAP-098A card must be landed"
guard_expect_in_file "$TAG" "MIMAP-099A" "$CARD_099A" "MIMAP-099A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-098A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$RELEASE_SSOT" "MIMAP-097A release SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$LEDGER_SSOT" "MIMAP-094A ledger SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-097A" "$SSOT" "closeout SSOT must include modeled release row"
guard_expect_in_file "$TAG" "MIMAP-099A post-segment-allocation-modeled-release row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-097A" "$GRANULARITY" "granularity SSOT must describe MIMAP-097A"
guard_expect_in_file "$TAG" "MIMAP-098A" "$GRANULARITY" "granularity SSOT must describe MIMAP-098A"
guard_expect_in_file "$TAG" "MIMAP-099A" "$GRANULARITY" "granularity SSOT must describe MIMAP-099A"
guard_expect_in_file "$TAG" "MIMAP-097A segment allocation modeled ledger release route" "$JOINT" "joint order must name modeled release row"
guard_expect_in_file "$TAG" "MIMAP-098A segment allocation modeled ledger release closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-099A post-segment-allocation-modeled-release row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-099A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-097A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-097A"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-allocation-modeled-ledger-release-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-098A closeout row"
guard_expect_in_file "$TAG" "$GUARD_097A" "$INDEX" "check index must list MIMAP-097A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-098A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_ledger_box = "memory/segment_allocation_modeled_ledger_box.hako"' "$MODULE" "modeled ledger owner must stay exported"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_ledger_box.hako` owns MIMAP-097A' "$MEMORY_README" "memory README must name MIMAP-097A owner"
guard_expect_in_file "$TAG" "box HakoAllocSegmentAllocationModeledLedgerReleaseReport" "$OWNER" "release report box must stay present"
guard_expect_in_file "$TAG" "releaseModeledToken" "$OWNER" "modeled release owner method must stay present"
guard_expect_in_file "$TAG" "findAnyIndex" "$OWNER" "any-token lookup must stay present"
guard_expect_in_file "$TAG" "would_execute_real_segment_free" "$OWNER" "owner must expose real-free inactive flag"
guard_expect_in_file "$TAG" "would_use_raw_pointer" "$OWNER" "owner must expose raw pointer inactive flag"
guard_expect_in_file "$TAG" "would_use_segment_map" "$OWNER" "owner must expose segment-map inactive flag"
guard_expect_in_file "$TAG" "would_allocate_arena_backing" "$OWNER" "owner must expose arena inactive flag"
guard_expect_in_file "$TAG" "would_execute_atomic_bitmap" "$OWNER" "owner must expose atomic bitmap inactive flag"
guard_expect_in_file "$TAG" "would_call_osvm" "$OWNER" "owner must expose OSVM inactive flag"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledLedger" "$APP" "proof must construct modeled ledger owner"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "modeled release closeout must keep execution/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "modeled release closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-ledger-release-proof|HakoAllocSegmentAllocationModeledLedgerRelease|releaseModeledToken' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "modeled release app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

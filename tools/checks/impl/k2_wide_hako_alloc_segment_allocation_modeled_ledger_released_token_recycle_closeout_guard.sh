#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-closeout-ssot.md"
RECYCLE_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-ssot.md"
RELEASE_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-release-ssot.md"
LEDGER_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
GUARD_MANIFEST_INCLUDE="tools/checks/manifests/guard_rows/hako_alloc_closeout.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_100A="docs/development/current/main/phases/phase-293x/293x-597-MIMAP-100A-SEGMENT-ALLOCATION-MODELED-LEDGER-RELEASED-TOKEN-RECYCLE-ROUTE.md"
CARD_101A="docs/development/current/main/phases/phase-293x/293x-598-MIMAP-101A-SEGMENT-ALLOCATION-MODELED-LEDGER-RELEASED-TOKEN-RECYCLE-CLOSEOUT-GUARD.md"
CARD_102A="docs/development/current/main/phases/phase-293x/293x-599-MIMAP-102A-POST-SEGMENT-ALLOCATION-MODELED-RECYCLE-ROW-SELECTION.md"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako"
HELPER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_report_box.hako"
APP="apps/hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-proof/main.hako"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-proof/test.sh"
GUARD_100A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_released_token_recycle_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_released_token_recycle_closeout_guard.sh"

echo "[$TAG] checking MIMAP-101A segment allocation modeled ledger released-token recycle closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$RECYCLE_SSOT" \
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
  "$GUARD_MANIFEST_INCLUDE" \
  "$CARD_100A" \
  "$CARD_101A" \
  "$CARD_102A" \
  "$OWNER" \
  "$HELPER" \
  "$APP" \
  "$APP_TEST" \
  "$GUARD_100A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_100A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_100A" "MIMAP-100A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_101A" "MIMAP-101A card must be landed"
guard_expect_in_file "$TAG" "MIMAP-102A" "$CARD_102A" "MIMAP-102A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-101A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$RECYCLE_SSOT" "MIMAP-100A recycle SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$RELEASE_SSOT" "MIMAP-097A release SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$LEDGER_SSOT" "MIMAP-094A ledger SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-100A" "$SSOT" "closeout SSOT must include released-token recycle row"
guard_expect_in_file "$TAG" "MIMAP-102A post-segment-allocation-modeled-recycle row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-100A" "$GRANULARITY" "granularity SSOT must describe MIMAP-100A"
guard_expect_in_file "$TAG" "MIMAP-101A" "$GRANULARITY" "granularity SSOT must describe MIMAP-101A"
guard_expect_in_file "$TAG" "MIMAP-102A" "$GRANULARITY" "granularity SSOT must describe MIMAP-102A"
guard_expect_in_file "$TAG" "MIMAP-100A segment allocation modeled ledger released-token recycle route" "$JOINT" "joint order must name released-token recycle row"
guard_expect_in_file "$TAG" "MIMAP-101A segment allocation modeled ledger released-token recycle closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-102A post-segment-allocation-modeled-recycle row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-102A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-100A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-100A"
guard_expect_fixed_in_file "$TAG" "id = \"hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-closeout\"" "$GUARD_MANIFEST_INCLUDE" "guard manifest must include MIMAP-101A closeout row"
guard_expect_in_file "$TAG" "$GUARD_100A" "$INDEX" "check index must list MIMAP-100A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-101A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_ledger_box = "memory/segment_allocation_modeled_ledger_box.hako"' "$MODULE" "modeled ledger owner must stay exported"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_ledger_report_box = "memory/segment_allocation_modeled_ledger_report_box.hako"' "$MODULE" "modeled ledger report helper must stay exported"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_ledger_box.hako` owns MIMAP-100A' "$MEMORY_README" "memory README must name MIMAP-100A owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_ledger_box.hako` owns MIMAP-094A' "$MEMORY_README" "memory README must name MIMAP-094A owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_ledger_report_box.hako' "$MEMORY_README" "memory README must name the report helper"
guard_expect_in_file "$TAG" "releaseModeledToken" "$OWNER" "modeled release owner method must stay present"
guard_expect_in_file "$TAG" "findIndex" "$OWNER" "live-token lookup must stay present"
guard_expect_in_file "$TAG" "findAnyIndex" "$OWNER" "historical-token lookup must stay present"
guard_expect_in_file "$TAG" "historical_index" "$OWNER" "release lookup must keep historical duplicate diagnostics"
guard_expect_fixed_in_file "$TAG" "report(accepted, reason, row_index, existing_index, segment_id, page_id, old_page_used, page_capacity, request_blocks, new_page_used, remaining_blocks, modeled_block_start, modeled_allocation_token)" "$OWNER" "ledger owner must expose direct report builder"
guard_expect_fixed_in_file "$TAG" "releaseReport(did_release, reason, row_index, modeled_allocation_token, segment_id, page_id, modeled_block_start, live_before, live_after)" "$OWNER" "ledger owner must expose direct release report builder"
guard_expect_fixed_in_file "$TAG" "releaseRejectUnsupportedRequirement(requirement, modeled_allocation_token)" "$OWNER" "ledger owner must expose direct release reject helper"
if rg -n 'report_surface|HakoAllocSegmentAllocationModeledLedgerReportSurface' "$OWNER" >/tmp/"$TAG".surface_leak 2>&1; then
  cat /tmp/"$TAG".surface_leak >&2
  rm -f /tmp/"$TAG".surface_leak
  guard_fail "$TAG" "ledger owner must not keep report surface dispatch"
fi
rm -f /tmp/"$TAG".surface_leak
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledLedger" "$APP" "proof must construct modeled ledger owner"
guard_expect_in_file "$TAG" "duplicate_after_recycle" "$APP" "proof must keep live duplicate rejection after recycle"
guard_expect_in_file "$TAG" "release_recycled" "$APP" "proof must release recycled row"
guard_expect_in_file "$TAG" "box HakoAllocSegmentAllocationModeledLedgerReleaseReport" "$HELPER" "report helper must own release report box"
guard_expect_in_file "$TAG" "box HakoAllocSegmentAllocationModeledLedgerReport" "$HELPER" "report helper must own report box"
guard_expect_in_file "$TAG" "modeled_release_present" "$HELPER" "report helper must own release capsule fields"
guard_expect_in_file "$TAG" "modeled_ledger_present" "$HELPER" "report helper must own report capsule fields"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "released-token recycle closeout must keep execution/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "released-token recycle closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-ledger-released-token-recycle-proof|ReleasedTokenRecycle|releasedTokenRecycle' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "released-token recycle app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

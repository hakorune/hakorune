#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-ledger-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-closeout-ssot.md"
LEDGER_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-ledger-ssot.md"
CONSUME_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-consume-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
GUARD_MANIFEST_INCLUDE="tools/checks/manifests/guard_rows/hako_alloc_closeout.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
HELPER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_report_box.hako"
CARD_094A="docs/development/current/main/phases/phase-293x/293x-591-MIMAP-094A-SEGMENT-ALLOCATION-MODELED-LEDGER-ROUTE.md"
CARD_095A="docs/development/current/main/phases/phase-293x/293x-592-MIMAP-095A-SEGMENT-ALLOCATION-MODELED-LEDGER-CLOSEOUT-GUARD.md"
CARD_096A="docs/development/current/main/phases/phase-293x/293x-593-MIMAP-096A-POST-SEGMENT-ALLOCATION-MODELED-LEDGER-ROW-SELECTION.md"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_ledger_box.hako"
APP="apps/hako-alloc-segment-allocation-modeled-ledger-proof/main.hako"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-ledger-proof/test.sh"
GUARD_094A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_closeout_guard.sh"

echo "[$TAG] checking MIMAP-095A segment allocation modeled ledger closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$LEDGER_SSOT" \
  "$CONSUME_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$GUARD_MANIFEST_INCLUDE" \
  "$HELPER" \
  "$CARD_094A" \
  "$CARD_095A" \
  "$CARD_096A" \
  "$OWNER" \
  "$APP" \
  "$APP_TEST" \
  "$GUARD_094A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_094A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_094A" "MIMAP-094A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_095A" "MIMAP-095A card must be landed"
guard_expect_in_file "$TAG" "MIMAP-096A" "$CARD_096A" "MIMAP-096A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-095A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$LEDGER_SSOT" "MIMAP-094A ledger SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$CONSUME_SSOT" "MIMAP-091A consume SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-094A" "$SSOT" "closeout SSOT must include modeled ledger row"
guard_expect_in_file "$TAG" "MIMAP-096A post-segment-allocation-modeled-ledger row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-094A" "$GRANULARITY" "granularity SSOT must describe MIMAP-094A"
guard_expect_in_file "$TAG" "MIMAP-095A" "$GRANULARITY" "granularity SSOT must describe MIMAP-095A"
guard_expect_in_file "$TAG" "MIMAP-094A segment allocation modeled ledger route" "$JOINT" "joint order must name modeled ledger row"
guard_expect_in_file "$TAG" "MIMAP-095A segment allocation modeled ledger closeout guard" "$JOINT" "joint order must name closeout row"

guard_expect_in_file "$TAG" "id = \"MIMAP-094A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-094A"
guard_expect_fixed_in_file "$TAG" "id = \"hako-alloc-segment-allocation-modeled-ledger-closeout\"" "$GUARD_MANIFEST_INCLUDE" "guard manifest must include MIMAP-095A closeout row"
guard_expect_in_file "$TAG" "$GUARD_094A" "$INDEX" "check index must list MIMAP-094A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-095A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_ledger_box = "memory/segment_allocation_modeled_ledger_box.hako"' "$MODULE" "modeled ledger owner must stay exported"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_ledger_box.hako` owns MIMAP-094A' "$MEMORY_README" "memory README must name MIMAP-094A owner"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_ledger_report_box = "memory/segment_allocation_modeled_ledger_report_box.hako"' "$MODULE" "modeled ledger report helper must stay exported"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_ledger_report_box.hako` owns MIMAP-094A report capsules' "$MEMORY_README" "memory README must name modeled ledger report helper"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.segment_allocation_modeled_ledger_report_box as HakoAllocSegmentAllocationModeledLedgerReportBox' "$OWNER" "modeled ledger owner must import report helper"
guard_expect_fixed_in_file "$TAG" 'report(accepted, reason, row_index, existing_index, segment_id, page_id, old_page_used, page_capacity, request_blocks, new_page_used, remaining_blocks, modeled_block_start, modeled_allocation_token)' "$OWNER" "modeled ledger owner must expose direct report builder"
guard_expect_fixed_in_file "$TAG" 'releaseReport(did_release, reason, row_index, modeled_allocation_token, segment_id, page_id, modeled_block_start, live_before, live_after)' "$OWNER" "modeled ledger owner must expose direct release report builder"
guard_expect_fixed_in_file "$TAG" 'rejectUnsupportedRequirement(requirement, segment_id, page_id, old_page_used, page_capacity, request_blocks, new_page_used, remaining_blocks, modeled_block_start, modeled_allocation_token)' "$OWNER" "modeled ledger owner must expose direct reject helper"
guard_expect_fixed_in_file "$TAG" 'releaseRejectUnsupportedRequirement(requirement, modeled_allocation_token)' "$OWNER" "modeled ledger owner must expose direct release reject helper"
if rg -n 'report_surface|HakoAllocSegmentAllocationModeledLedgerReportSurface' "$OWNER" >/tmp/"$TAG".surface_leak 2>&1; then
  cat /tmp/"$TAG".surface_leak >&2
  rm -f /tmp/"$TAG".surface_leak
  guard_fail "$TAG" "modeled ledger owner must not keep report surface dispatch"
fi
rm -f /tmp/"$TAG".surface_leak
guard_expect_in_file "$TAG" "box HakoAllocSegmentAllocationModeledLedgerReport" "$HELPER" "report box must live in helper"
guard_expect_in_file "$TAG" "box HakoAllocSegmentAllocationModeledLedger" "$OWNER" "modeled ledger owner must stay present"
guard_expect_in_file "$TAG" "recordModeledConsume" "$OWNER" "modeled ledger owner must keep recordModeledConsume"
guard_expect_in_file "$TAG" "findIndex" "$OWNER" "modeled ledger owner must keep token lookup"
guard_expect_in_file "$TAG" "makeModeledToken" "$OWNER" "modeled ledger owner must keep token helper"
guard_expect_in_file "$TAG" "would_execute_real_segment_allocation" "$HELPER" "report helper must expose real execution inactive flag"
guard_expect_in_file "$TAG" "would_use_raw_pointer" "$HELPER" "report helper must expose raw pointer inactive flag"
guard_expect_in_file "$TAG" "would_use_segment_map" "$HELPER" "report helper must expose segment-map inactive flag"
guard_expect_in_file "$TAG" "would_allocate_arena_backing" "$HELPER" "report helper must expose arena inactive flag"
guard_expect_in_file "$TAG" "would_execute_atomic_bitmap" "$HELPER" "report helper must expose atomic bitmap inactive flag"
guard_expect_in_file "$TAG" "would_call_osvm" "$HELPER" "report helper must expose OSVM inactive flag"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledLedger" "$APP" "proof must construct modeled ledger owner"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "modeled ledger closeout must keep execution/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "modeled ledger closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-ledger-proof|HakoAllocSegmentAllocationModeledLedger|segment_allocation_modeled_ledger' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "modeled ledger app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-consume-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-consume-closeout-ssot.md"
CONSUME_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-consume-ssot.md"
READINESS_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-readiness-scalar-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_091A="docs/development/current/main/phases/phase-293x/293x-588-MIMAP-091A-SEGMENT-ALLOCATION-MODELED-CONSUME-ROUTE.md"
CARD_092A="docs/development/current/main/phases/phase-293x/293x-589-MIMAP-092A-SEGMENT-ALLOCATION-MODELED-CONSUME-CLOSEOUT-GUARD.md"
CARD_093A="docs/development/current/main/phases/phase-293x/293x-590-MIMAP-093A-POST-SEGMENT-ALLOCATION-MODELED-CONSUME-ROW-SELECTION.md"
OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_consume_box.hako"
APP="apps/hako-alloc-segment-allocation-modeled-consume-proof/main.hako"
APP_TEST="apps/hako-alloc-segment-allocation-modeled-consume-proof/test.sh"
GUARD_091A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_consume_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_consume_closeout_guard.sh"

echo "[$TAG] checking MIMAP-092A segment allocation modeled consume closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$CONSUME_SSOT" \
  "$READINESS_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_091A" \
  "$CARD_092A" \
  "$CARD_093A" \
  "$OWNER" \
  "$APP" \
  "$APP_TEST" \
  "$GUARD_091A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_091A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_091A" "MIMAP-091A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_092A" "MIMAP-092A card must be landed"
guard_expect_in_file "$TAG" "Status:" "$CARD_093A" "MIMAP-093A selection card must have status"
guard_expect_in_file "$TAG" "MIMAP-093A" "$CARD_093A" "MIMAP-093A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-092A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$CONSUME_SSOT" "MIMAP-091A consume SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$READINESS_SSOT" "MIMAP-088A readiness SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-091A" "$SSOT" "closeout SSOT must include modeled consume row"
guard_expect_in_file "$TAG" "MIMAP-093A post-segment-allocation-modeled-consume row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-091A" "$GRANULARITY" "granularity SSOT must describe MIMAP-091A"
guard_expect_in_file "$TAG" "MIMAP-092A" "$GRANULARITY" "granularity SSOT must describe MIMAP-092A"
guard_expect_in_file "$TAG" "MIMAP-091A segment allocation modeled consume route" "$JOINT" "joint order must name modeled consume row"
guard_expect_in_file "$TAG" "MIMAP-092A segment allocation modeled consume closeout guard" "$JOINT" "joint order must name closeout row"

guard_expect_in_file "$TAG" "id = \"MIMAP-091A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-091A"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-allocation-modeled-consume-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-092A closeout row"
guard_expect_in_file "$TAG" "$GUARD_091A" "$INDEX" "check index must list MIMAP-091A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-092A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_consume_box = "memory/segment_allocation_modeled_consume_box.hako"' "$MODULE" "modeled consume owner must stay exported"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_consume_box.hako` owns MIMAP-091A' "$MEMORY_README" "memory README must name MIMAP-091A owner"
guard_expect_in_file "$TAG" "box HakoAllocSegmentAllocationModeledConsumeReport" "$OWNER" "report box must stay present"
guard_expect_in_file "$TAG" "box HakoAllocSegmentAllocationModeledConsume" "$OWNER" "modeled consume owner must stay present"
guard_expect_in_file "$TAG" "consumeReadiness" "$OWNER" "modeled consume owner must keep consumeReadiness"
guard_expect_in_file "$TAG" "makeModeledToken" "$OWNER" "modeled consume owner must keep token helper"
guard_expect_in_file "$TAG" "would_execute_real_segment_allocation" "$OWNER" "owner must expose real execution inactive flag"
guard_expect_in_file "$TAG" "would_use_raw_pointer" "$OWNER" "owner must expose raw pointer inactive flag"
guard_expect_in_file "$TAG" "would_use_segment_map" "$OWNER" "owner must expose segment-map inactive flag"
guard_expect_in_file "$TAG" "would_allocate_arena_backing" "$OWNER" "owner must expose arena inactive flag"
guard_expect_in_file "$TAG" "would_execute_atomic_bitmap" "$OWNER" "owner must expose atomic bitmap inactive flag"
guard_expect_in_file "$TAG" "would_call_osvm" "$OWNER" "owner must expose OSVM inactive flag"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledConsume" "$APP" "proof must construct modeled consume owner"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "modeled consume closeout must keep execution/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "modeled consume closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-consume-proof|HakoAllocSegmentAllocationModeledConsume|segment_allocation_modeled_consume' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "modeled consume app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

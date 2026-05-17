#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-readiness-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-readiness-scalar-closeout-ssot.md"
READINESS_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-readiness-scalar-ssot.md"
MEMBERSHIP_SSOT="docs/development/current/main/design/hako-alloc-segment-page-membership-scalar-ssot.md"
SEGMENT_STATE_SSOT="docs/development/current/main/design/hako-alloc-segment-lifecycle-scalar-state-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_088A="docs/development/current/main/phases/phase-293x/293x-585-MIMAP-088A-SEGMENT-ALLOCATION-READINESS-SCALAR-CONTRACT.md"
CARD_089A="docs/development/current/main/phases/phase-293x/293x-586-MIMAP-089A-SEGMENT-ALLOCATION-READINESS-CLOSEOUT-GUARD.md"
CARD_090A="docs/development/current/main/phases/phase-293x/293x-587-MIMAP-090A-POST-SEGMENT-ALLOCATION-READINESS-ROW-SELECTION.md"
OWNER="lang/src/hako_alloc/memory/segment_allocation_readiness_scalar_box.hako"
APP="apps/hako-alloc-segment-allocation-readiness-scalar-proof/main.hako"
APP_TEST="apps/hako-alloc-segment-allocation-readiness-scalar-proof/test.sh"
GUARD_088A="tools/checks/k2_wide_hako_alloc_segment_allocation_readiness_scalar_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_readiness_closeout_guard.sh"

echo "[$TAG] checking MIMAP-089A segment allocation readiness closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$READINESS_SSOT" \
  "$MEMBERSHIP_SSOT" \
  "$SEGMENT_STATE_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_088A" \
  "$CARD_089A" \
  "$CARD_090A" \
  "$OWNER" \
  "$APP" \
  "$APP_TEST" \
  "$GUARD_088A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_088A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_088A" "MIMAP-088A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_089A" "MIMAP-089A card must be landed"
guard_expect_in_file "$TAG" "Status:" "$CARD_090A" "MIMAP-090A selection card must have status"
guard_expect_in_file "$TAG" "MIMAP-090A" "$CARD_090A" "MIMAP-090A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-089A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$READINESS_SSOT" "MIMAP-088A readiness SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$MEMBERSHIP_SSOT" "membership SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$SEGMENT_STATE_SSOT" "segment state SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-088A" "$SSOT" "closeout SSOT must include readiness row"
guard_expect_in_file "$TAG" "MIMAP-090A post-segment-allocation-readiness row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-088A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-088A"
guard_expect_in_file "$TAG" "MIMAP-089A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-089A"
guard_expect_in_file "$TAG" "MIMAP-088A segment allocation readiness scalar contract" "$JOINT" "joint order must name readiness row"
guard_expect_in_file "$TAG" "MIMAP-089A segment allocation readiness closeout guard" "$JOINT" "joint order must name closeout row"

guard_expect_in_file "$TAG" "id = \"MIMAP-088A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-088A"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-allocation-readiness-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-089A closeout row"
guard_expect_in_file "$TAG" "$GUARD_088A" "$INDEX" "check index must list MIMAP-088A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-089A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_allocation_readiness_scalar_box = "memory/segment_allocation_readiness_scalar_box.hako"' "$MODULE" "readiness owner must stay exported"
guard_expect_in_file "$TAG" 'segment_allocation_readiness_scalar_box.hako` owns MIMAP-088A' "$MEMORY_README" "memory README must name MIMAP-088A owner"
guard_expect_in_file "$TAG" "box HakoAllocSegmentAllocationReadinessScalarReport" "$OWNER" "report box must stay present"
guard_expect_in_file "$TAG" "box HakoAllocSegmentAllocationReadinessScalar" "$OWNER" "readiness owner must stay present"
guard_expect_in_file "$TAG" "classifyReadiness" "$OWNER" "readiness owner must keep classifyReadiness"
guard_expect_in_file "$TAG" "supportsAllocationReadiness" "$OWNER" "readiness owner must keep state policy"
guard_expect_in_file "$TAG" "would_execute_segment_allocation" "$OWNER" "owner must expose execution inactive flag"
guard_expect_in_file "$TAG" "would_use_raw_pointer" "$OWNER" "owner must expose raw pointer inactive flag"
guard_expect_in_file "$TAG" "would_use_segment_map" "$OWNER" "owner must expose segment-map inactive flag"
guard_expect_in_file "$TAG" "would_allocate_arena_backing" "$OWNER" "owner must expose arena inactive flag"
guard_expect_in_file "$TAG" "would_execute_atomic_bitmap" "$OWNER" "owner must expose atomic bitmap inactive flag"
guard_expect_in_file "$TAG" "would_call_osvm" "$OWNER" "owner must expose OSVM inactive flag"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationReadinessScalar" "$APP" "proof must construct readiness owner"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "segment allocation readiness closeout must keep execution/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "segment allocation readiness closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-readiness-scalar-proof|HakoAllocSegmentAllocationReadinessScalar|segment_allocation_readiness_scalar' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "segment allocation readiness app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

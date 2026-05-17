#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-allocation-modeled-local-free-scalar-lane-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-scalar-lane-closeout-ssot.md"
RELEASED_SPAN_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-released-span-ledger-ssot.md"
CANDIDATE_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-ssot.md"
APPLY_PLAN_SSOT="docs/development/current/main/design/hako-alloc-segment-allocation-modeled-local-free-apply-plan-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
GUARD_MANIFEST="tools/checks/guard_rows.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_107A="docs/development/current/main/phases/phase-293x/293x-606-MIMAP-107A-SEGMENT-ALLOCATION-MODELED-RELEASED-SPAN-LEDGER-ROUTE.md"
CARD_109A="docs/development/current/main/phases/phase-293x/293x-608-MIMAP-109A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-CANDIDATE-LEDGER-ROUTE.md"
CARD_111A="docs/development/current/main/phases/phase-293x/293x-610-MIMAP-111A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-APPLY-PLAN-ROUTE.md"
CARD_113A="docs/development/current/main/phases/phase-293x/293x-612-MIMAP-113A-SEGMENT-ALLOCATION-MODELED-LOCAL-FREE-SCALAR-LANE-CLOSEOUT-GUARD.md"
CARD_114A="docs/development/current/main/phases/phase-293x/293x-613-MIMAP-114A-POST-LOCAL-FREE-SCALAR-CLOSEOUT-ROW-SELECTION.md"
RELEASED_SPAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_released_span_ledger_box.hako"
CANDIDATE_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako"
APPLY_PLAN_OWNER="lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_apply_plan_box.hako"
RELEASED_SPAN_APP="apps/hako-alloc-segment-allocation-modeled-released-span-ledger-proof/main.hako"
CANDIDATE_APP="apps/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-proof/main.hako"
APPLY_PLAN_APP="apps/hako-alloc-segment-allocation-modeled-local-free-apply-plan-proof/main.hako"
RELEASED_SPAN_APP_TEST="apps/hako-alloc-segment-allocation-modeled-released-span-ledger-proof/test.sh"
CANDIDATE_APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-candidate-ledger-proof/test.sh"
APPLY_PLAN_APP_TEST="apps/hako-alloc-segment-allocation-modeled-local-free-apply-plan-proof/test.sh"
GUARD_107A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_released_span_ledger_guard.sh"
GUARD_109A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_candidate_ledger_guard.sh"
GUARD_111A="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_apply_plan_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_scalar_lane_closeout_guard.sh"
IMPL_SCRIPT="tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_scalar_lane_closeout_guard.sh"

echo "[$TAG] checking MIMAP-113A segment allocation modeled local-free scalar lane closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$RELEASED_SPAN_SSOT" \
  "$CANDIDATE_SSOT" \
  "$APPLY_PLAN_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$GUARD_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_107A" \
  "$CARD_109A" \
  "$CARD_111A" \
  "$CARD_113A" \
  "$CARD_114A" \
  "$RELEASED_SPAN_OWNER" \
  "$CANDIDATE_OWNER" \
  "$APPLY_PLAN_OWNER" \
  "$RELEASED_SPAN_APP" \
  "$CANDIDATE_APP" \
  "$APPLY_PLAN_APP" \
  "$RELEASED_SPAN_APP_TEST" \
  "$CANDIDATE_APP_TEST" \
  "$APPLY_PLAN_APP_TEST" \
  "$GUARD_107A" \
  "$GUARD_109A" \
  "$GUARD_111A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT"

guard_require_exec_files \
  "$TAG" \
  "$RELEASED_SPAN_APP_TEST" \
  "$CANDIDATE_APP_TEST" \
  "$APPLY_PLAN_APP_TEST" \
  "$GUARD_107A" \
  "$GUARD_109A" \
  "$GUARD_111A" \
  "$SELF_SCRIPT" \
  "$IMPL_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_107A" "MIMAP-107A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_109A" "MIMAP-109A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_111A" "MIMAP-111A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_113A" "MIMAP-113A card must be landed"
guard_expect_in_file "$TAG" "MIMAP-114A" "$CARD_114A" "MIMAP-114A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-113A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$RELEASED_SPAN_SSOT" "MIMAP-107A SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$CANDIDATE_SSOT" "MIMAP-109A SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$APPLY_PLAN_SSOT" "MIMAP-111A SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-107A" "$SSOT" "closeout SSOT must include released-span row"
guard_expect_in_file "$TAG" "MIMAP-109A" "$SSOT" "closeout SSOT must include candidate row"
guard_expect_in_file "$TAG" "MIMAP-111A" "$SSOT" "closeout SSOT must include apply-plan row"
guard_expect_in_file "$TAG" "MIMAP-114A post-local-free-scalar-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-107A" "$GRANULARITY" "granularity SSOT must describe MIMAP-107A"
guard_expect_in_file "$TAG" "MIMAP-109A" "$GRANULARITY" "granularity SSOT must describe MIMAP-109A"
guard_expect_in_file "$TAG" "MIMAP-111A" "$GRANULARITY" "granularity SSOT must describe MIMAP-111A"
guard_expect_in_file "$TAG" "MIMAP-113A" "$GRANULARITY" "granularity SSOT must describe MIMAP-113A"
guard_expect_in_file "$TAG" "MIMAP-114A" "$GRANULARITY" "granularity SSOT must describe MIMAP-114A"
guard_expect_in_file "$TAG" "MIMAP-113A segment allocation modeled local-free scalar lane closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-114A post-local-free-scalar-closeout row selection" "$JOINT" "joint order must name next row"
guard_expect_in_file "$TAG" "MIMAP-114A" "$TASKBOARD" "taskboard must name selected next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-107A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-107A"
guard_expect_in_file "$TAG" "id = \"MIMAP-109A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-109A"
guard_expect_in_file "$TAG" "id = \"MIMAP-111A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-111A"
guard_expect_in_file "$TAG" "id = \"hako-alloc-segment-allocation-modeled-local-free-scalar-lane-closeout\"" "$GUARD_MANIFEST" "guard manifest must include MIMAP-113A closeout row"
guard_expect_in_file "$TAG" "$GUARD_107A" "$INDEX" "check index must list MIMAP-107A guard"
guard_expect_in_file "$TAG" "$GUARD_109A" "$INDEX" "check index must list MIMAP-109A guard"
guard_expect_in_file "$TAG" "$GUARD_111A" "$INDEX" "check index must list MIMAP-111A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-113A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_released_span_ledger_box = "memory/segment_allocation_modeled_released_span_ledger_box.hako"' "$MODULE" "released-span ledger owner must stay exported"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_candidate_ledger_box = "memory/segment_allocation_modeled_local_free_candidate_ledger_box.hako"' "$MODULE" "candidate ledger owner must stay exported"
guard_expect_in_file "$TAG" 'memory.segment_allocation_modeled_local_free_apply_plan_box = "memory/segment_allocation_modeled_local_free_apply_plan_box.hako"' "$MODULE" "apply-plan owner must stay exported"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_released_span_ledger_box.hako` owns MIMAP-107A' "$MEMORY_README" "memory README must name MIMAP-107A owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_candidate_ledger_box.hako` owns' "$MEMORY_README" "memory README must name MIMAP-109A owner"
guard_expect_in_file "$TAG" 'segment_allocation_modeled_local_free_apply_plan_box.hako` owns' "$MEMORY_README" "memory README must name MIMAP-111A owner"
guard_expect_in_file "$TAG" "recordReleasedSpan" "$RELEASED_SPAN_OWNER" "released-span owner route must stay present"
guard_expect_in_file "$TAG" "recordLocalFreeCandidate" "$CANDIDATE_OWNER" "candidate owner route must stay present"
guard_expect_in_file "$TAG" "recordLocalFreeApplyPlan" "$APPLY_PLAN_OWNER" "apply-plan owner route must stay present"
guard_expect_in_file "$TAG" "would_mutate_free_list" "$APPLY_PLAN_OWNER" "apply-plan owner must expose free-list inactive flag"
guard_expect_in_file "$TAG" "would_mutate_page_state" "$APPLY_PLAN_OWNER" "apply-plan owner must expose page-state inactive flag"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledReleasedSpanLedger" "$RELEASED_SPAN_APP" "released-span proof must construct released-span owner"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledLocalFreeCandidateLedger" "$CANDIDATE_APP" "candidate proof must construct candidate owner"
guard_expect_in_file "$TAG" "HakoAllocSegmentAllocationModeledLocalFreeApplyPlan" "$APPLY_PLAN_APP" "apply-plan proof must construct apply-plan owner"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|claimBitmap|unclaimBitmap|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|allocateSegment[[:space:]]*\(|freeSegment[[:space:]]*\(|mutateFreeList|freeList|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$RELEASED_SPAN_OWNER" "$CANDIDATE_OWNER" "$APPLY_PLAN_OWNER" "$RELEASED_SPAN_APP" "$CANDIDATE_APP" "$APPLY_PLAN_APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "local-free scalar lane closeout must keep execution/free-list/page-state/concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$RELEASED_SPAN_OWNER" "$CANDIDATE_OWNER" "$APPLY_PLAN_OWNER" "$RELEASED_SPAN_APP" "$CANDIDATE_APP" "$APPLY_PLAN_APP" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "local-free scalar lane closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-allocation-modeled-(released-span-ledger|local-free-candidate-ledger|local-free-apply-plan)-proof|LocalFreeApplyPlan|LocalFreeCandidateLedger|ReleasedSpanLedger|recordLocalFreeApplyPlan|recordLocalFreeCandidate|recordReleasedSpan' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "local-free scalar lane app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-lifecycle-scalar-state-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-lifecycle-scalar-state-closeout-ssot.md"
STATE_SSOT="docs/development/current/main/design/hako-alloc-segment-lifecycle-scalar-state-ssot.md"
LIFECYCLE_BLUEPRINT="docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_082A="docs/development/current/main/phases/phase-293x/293x-569-MIMAP-082A-SEGMENT-LIFECYCLE-SCALAR-STATE-CONTRACT.md"
CARD_083A="docs/development/current/main/phases/phase-293x/293x-570-MIMAP-083A-SEGMENT-LIFECYCLE-SCALAR-STATE-CLOSEOUT-GUARD.md"
CARD_084A="docs/development/current/main/phases/phase-293x/293x-571-MIMAP-084A-POST-SEGMENT-LIFECYCLE-CLOSEOUT-ROW-SELECTION.md"
OWNER="lang/src/hako_alloc/memory/segment_lifecycle_scalar_state_box.hako"
APP="apps/hako-alloc-segment-lifecycle-scalar-state-proof/main.hako"
APP_TEST="apps/hako-alloc-segment-lifecycle-scalar-state-proof/test.sh"
GUARD_082A="tools/checks/k2_wide_hako_alloc_segment_lifecycle_scalar_state_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_lifecycle_scalar_state_closeout_guard.sh"

echo "[$TAG] checking MIMAP-083A segment lifecycle scalar state closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$STATE_SSOT" \
  "$LIFECYCLE_BLUEPRINT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_082A" \
  "$CARD_083A" \
  "$CARD_084A" \
  "$OWNER" \
  "$APP" \
  "$APP_TEST" \
  "$GUARD_082A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_082A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_082A" "MIMAP-082A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_083A" "MIMAP-083A card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_084A" "MIMAP-084A must be selected after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-083A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$STATE_SSOT" "MIMAP-082A state SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-082A" "$SSOT" "closeout SSOT must include state row"
guard_expect_in_file "$TAG" "MIMAP-084A post-segment-lifecycle-closeout row selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "Segment Lifecycle" "$LIFECYCLE_BLUEPRINT" "lifecycle blueprint must keep segment lifecycle"

guard_expect_in_file "$TAG" "MIMAP-083A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-083A"
guard_expect_in_file "$TAG" "MIMAP-084A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-084A"
guard_expect_in_file "$TAG" "MIMAP-083A segment lifecycle scalar state closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-084A post-segment-lifecycle-closeout row selection" "$JOINT" "joint order must name next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-082A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-082A"
guard_expect_in_file "$TAG" "$GUARD_082A" "$INDEX" "check index must list MIMAP-082A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-083A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_lifecycle_scalar_state_box = "memory/segment_lifecycle_scalar_state_box.hako"' "$MODULE" "state owner must stay exported"
guard_expect_in_file "$TAG" 'segment_lifecycle_scalar_state_box.hako` owns MIMAP-082A' "$MEMORY_README" "memory README must name MIMAP-082A owner"
guard_expect_in_file "$TAG" "box HakoAllocSegmentLifecycleScalarStateReport" "$OWNER" "report box must stay present"
guard_expect_in_file "$TAG" "box HakoAllocSegmentLifecycleScalarState" "$OWNER" "state owner must stay present"
guard_expect_in_file "$TAG" "classifyTransition" "$OWNER" "state owner must keep classifyTransition"
guard_expect_in_file "$TAG" "transitionId" "$OWNER" "state owner must keep transition id SSOT"
guard_expect_in_file "$TAG" "would_use_raw_pointer" "$OWNER" "owner must expose raw pointer inactive flag"
guard_expect_in_file "$TAG" "would_execute_atomic_bitmap" "$OWNER" "owner must expose atomic bitmap inactive flag"
guard_expect_in_file "$TAG" "would_call_osvm" "$OWNER" "owner must expose OSVM inactive flag"
guard_expect_in_file "$TAG" "would_run_thread" "$OWNER" "owner must expose thread inactive flag"
guard_expect_in_file "$TAG" "would_activate_provider" "$OWNER" "owner must expose provider inactive flag"
guard_expect_in_file "$TAG" "HakoAllocSegmentLifecycleScalarState" "$APP" "proof must construct state owner"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "segment lifecycle closeout must keep source-concurrency/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "segment lifecycle closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-lifecycle-scalar-state-proof|HakoAllocSegmentLifecycleScalarState|segment_lifecycle_scalar_state' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "segment lifecycle app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"


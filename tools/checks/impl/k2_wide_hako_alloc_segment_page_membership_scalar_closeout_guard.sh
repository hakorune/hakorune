#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-page-membership-scalar-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-page-membership-scalar-closeout-ssot.md"
MEMBERSHIP_SSOT="docs/development/current/main/design/hako-alloc-segment-page-membership-scalar-ssot.md"
SEGMENT_STATE_SSOT="docs/development/current/main/design/hako-alloc-segment-lifecycle-scalar-state-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_085A="docs/development/current/main/phases/phase-293x/293x-572-MIMAP-085A-SEGMENT-PAGE-MEMBERSHIP-SCALAR-CONTRACT.md"
CARD_086A="docs/development/current/main/phases/phase-293x/293x-573-MIMAP-086A-SEGMENT-PAGE-MEMBERSHIP-CLOSEOUT-GUARD.md"
CARD_087A="docs/development/current/main/phases/phase-293x/293x-574-MIMAP-087A-POST-SEGMENT-PAGE-MEMBERSHIP-CLOSEOUT-ROW-SELECTION.md"
OWNER="lang/src/hako_alloc/memory/segment_page_membership_scalar_box.hako"
APP="apps/hako-alloc-segment-page-membership-scalar-proof/main.hako"
APP_TEST="apps/hako-alloc-segment-page-membership-scalar-proof/test.sh"
GUARD_085A="tools/checks/k2_wide_hako_alloc_segment_page_membership_scalar_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_page_membership_scalar_closeout_guard.sh"

echo "[$TAG] checking MIMAP-086A segment page membership scalar closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$MEMBERSHIP_SSOT" \
  "$SEGMENT_STATE_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_085A" \
  "$CARD_086A" \
  "$CARD_087A" \
  "$OWNER" \
  "$APP" \
  "$APP_TEST" \
  "$GUARD_085A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_085A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_085A" "MIMAP-085A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_086A" "MIMAP-086A card must be landed"
guard_expect_in_file "$TAG" "Status:" "$CARD_087A" "MIMAP-087A selection card must have status"
guard_expect_in_file "$TAG" "MIMAP-087A" "$CARD_087A" "MIMAP-087A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-086A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$MEMBERSHIP_SSOT" "MIMAP-085A membership SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$SEGMENT_STATE_SSOT" "segment state SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-085A" "$SSOT" "closeout SSOT must include membership row"
guard_expect_in_file "$TAG" "MIMAP-087A post-segment-page-membership-closeout row selection" "$SSOT" "closeout SSOT must name next row"

guard_expect_in_file "$TAG" "MIMAP-086A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-086A"
guard_expect_in_file "$TAG" "MIMAP-087A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-087A"
guard_expect_in_file "$TAG" "MIMAP-086A segment page membership closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-087A post-segment-page-membership-closeout row selection" "$JOINT" "joint order must name next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-085A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-085A"
guard_expect_in_file "$TAG" "$GUARD_085A" "$INDEX" "check index must list MIMAP-085A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-086A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_page_membership_scalar_box = "memory/segment_page_membership_scalar_box.hako"' "$MODULE" "membership owner must stay exported"
guard_expect_in_file "$TAG" 'segment_page_membership_scalar_box.hako` owns MIMAP-085A' "$MEMORY_README" "memory README must name MIMAP-085A owner"
guard_expect_in_file "$TAG" "box HakoAllocSegmentPageMembershipScalarReport" "$OWNER" "report box must stay present"
guard_expect_in_file "$TAG" "box HakoAllocSegmentPageMembershipScalar" "$OWNER" "membership owner must stay present"
guard_expect_in_file "$TAG" "classifyMembership" "$OWNER" "membership owner must keep classifyMembership"
guard_expect_in_file "$TAG" "supportsMembership" "$OWNER" "membership owner must keep state policy"
guard_expect_in_file "$TAG" "would_use_raw_pointer" "$OWNER" "owner must expose raw pointer inactive flag"
guard_expect_in_file "$TAG" "would_use_segment_map" "$OWNER" "owner must expose segment-map inactive flag"
guard_expect_in_file "$TAG" "would_allocate_arena_backing" "$OWNER" "owner must expose arena inactive flag"
guard_expect_in_file "$TAG" "would_execute_atomic_bitmap" "$OWNER" "owner must expose atomic bitmap inactive flag"
guard_expect_in_file "$TAG" "would_call_osvm" "$OWNER" "owner must expose OSVM inactive flag"
guard_expect_in_file "$TAG" "HakoAllocSegmentPageMembershipScalar" "$APP" "proof must construct membership owner"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|SegmentMap|lookupSegment|pointer_member|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "segment/page membership closeout must keep source-concurrency/segment-map/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "segment/page membership closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-page-membership-scalar-proof|HakoAllocSegmentPageMembershipScalar|segment_page_membership_scalar' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "segment/page membership app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-segment-arena-bitmap-inventory-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-segment-arena-bitmap-inventory-closeout-ssot.md"
INVENTORY_SSOT="docs/development/current/main/design/hako-alloc-segment-arena-bitmap-inventory-ssot.md"
GAP_LEDGER="docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md"
CONCEPT_INVENTORY="docs/development/current/main/investigations/mimalloc-source-concept-inventory.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_079A="docs/development/current/main/phases/phase-293x/293x-566-MIMAP-079A-SEGMENT-ARENA-BITMAP-BOUNDARY-INVENTORY.md"
CARD_080A="docs/development/current/main/phases/phase-293x/293x-567-MIMAP-080A-SEGMENT-ARENA-BITMAP-INVENTORY-CLOSEOUT-GUARD.md"
CARD_081A="docs/development/current/main/phases/phase-293x/293x-568-MIMAP-081A-POST-SEGMENT-ARENA-BITMAP-INVENTORY-ROW-SELECTION.md"
OWNER="lang/src/hako_alloc/memory/segment_arena_bitmap_inventory_box.hako"
APP="apps/hako-alloc-segment-arena-bitmap-inventory-proof/main.hako"
APP_TEST="apps/hako-alloc-segment-arena-bitmap-inventory-proof/test.sh"
GUARD_079A="tools/checks/k2_wide_hako_alloc_segment_arena_bitmap_inventory_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_segment_arena_bitmap_inventory_closeout_guard.sh"

echo "[$TAG] checking MIMAP-080A segment arena bitmap inventory closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$INVENTORY_SSOT" \
  "$GAP_LEDGER" \
  "$CONCEPT_INVENTORY" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_079A" \
  "$CARD_080A" \
  "$CARD_081A" \
  "$OWNER" \
  "$APP" \
  "$APP_TEST" \
  "$GUARD_079A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$GUARD_079A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_079A" "MIMAP-079A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_080A" "MIMAP-080A card must be landed"
guard_expect_in_file "$TAG" "Status:" "$CARD_081A" "MIMAP-081A selection card must have status"
guard_expect_in_file "$TAG" "MIMAP-081A" "$CARD_081A" "MIMAP-081A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-080A closeout SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$INVENTORY_SSOT" "MIMAP-079A inventory SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-079A" "$SSOT" "closeout SSOT must include inventory row"
guard_expect_in_file "$TAG" "MIMAP-081A post-segment-arena-bitmap-inventory row selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "GAP-BITMAP" "$GAP_LEDGER" "gap ledger must keep bitmap representation gap"
guard_expect_in_file "$TAG" "mi_segment_t" "$CONCEPT_INVENTORY" "source concept inventory must keep segment concept"
guard_expect_in_file "$TAG" "Arena allocation" "$CONCEPT_INVENTORY" "source concept inventory must keep arena concept"
guard_expect_in_file "$TAG" "Bitmap helpers" "$CONCEPT_INVENTORY" "source concept inventory must keep bitmap concept"

guard_expect_in_file "$TAG" "MIMAP-080A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-080A"
guard_expect_in_file "$TAG" "MIMAP-081A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-081A"
guard_expect_in_file "$TAG" "MIMAP-080A segment arena bitmap inventory closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "MIMAP-081A post-segment-arena-bitmap-inventory row selection" "$JOINT" "joint order must name next row"

guard_expect_in_file "$TAG" "id = \"MIMAP-079A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-079A"
guard_expect_in_file "$TAG" "$GUARD_079A" "$INDEX" "check index must list MIMAP-079A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-080A closeout guard"
guard_expect_in_file "$TAG" 'memory.segment_arena_bitmap_inventory_box = "memory/segment_arena_bitmap_inventory_box.hako"' "$MODULE" "inventory owner must stay exported"
guard_expect_in_file "$TAG" 'segment_arena_bitmap_inventory_box.hako` owns MIMAP-079A' "$MEMORY_README" "memory README must name MIMAP-079A owner"
guard_expect_in_file "$TAG" "box HakoAllocSegmentArenaBitmapInventoryReport" "$OWNER" "report box must stay present"
guard_expect_in_file "$TAG" "box HakoAllocSegmentArenaBitmapInventory" "$OWNER" "inventory owner must stay present"
guard_expect_in_file "$TAG" "classifyBoundary" "$OWNER" "inventory owner must keep classifyBoundary"
guard_expect_in_file "$TAG" "would_use_raw_pointer" "$OWNER" "owner must expose raw pointer inactive flag"
guard_expect_in_file "$TAG" "would_execute_atomic_bitmap" "$OWNER" "owner must expose atomic bitmap inactive flag"
guard_expect_in_file "$TAG" "would_call_osvm" "$OWNER" "owner must expose OSVM inactive flag"
guard_expect_in_file "$TAG" "would_activate_provider" "$OWNER" "owner must expose provider inactive flag"
guard_expect_in_file "$TAG" "HakoAllocSegmentArenaBitmapInventory" "$APP" "proof must construct inventory"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP" >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "segment/arena/bitmap closeout must keep scheduling/source-concurrency/atomics/page-source/OS release inactive"
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" >/tmp/"$TAG".provider_leak 2>&1; then
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  guard_fail "$TAG" "segment/arena/bitmap closeout must keep provider/hook/replacement inactive"
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-segment-arena-bitmap-inventory-proof|HakoAllocSegmentArenaBitmapInventory|segment_arena_bitmap_inventory' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "segment/arena/bitmap app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider_sentinel 2>&1 || {
  cat /tmp/"$TAG".provider_sentinel >&2
  rm -f /tmp/"$TAG".provider_sentinel
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider_sentinel

echo "[$TAG] ok"

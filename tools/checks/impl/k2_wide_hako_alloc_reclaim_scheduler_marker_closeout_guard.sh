#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-scheduler-marker-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-marker-closeout-ssot.md"
BOUNDARY_SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-boundary-inventory-ssot.md"
MARKER_SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-marker-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_063A="docs/development/current/main/phases/phase-293x/293x-550-MIMAP-063A-RECLAIM-SCHEDULER-BOUNDARY-INVENTORY.md"
CARD_064A="docs/development/current/main/phases/phase-293x/293x-551-MIMAP-064A-RECLAIM-SCHEDULER-REQUEST-MARKER-CONTRACT.md"
CARD_065A="docs/development/current/main/phases/phase-293x/293x-552-MIMAP-065A-RECLAIM-SCHEDULER-MARKER-CLOSEOUT-GUARD.md"
CARD_066A="docs/development/current/main/phases/phase-293x/293x-553-MIMAP-066A-POST-SCHEDULER-MARKER-ROW-SELECTION.md"
OWNER_064A="lang/src/hako_alloc/memory/reclaim_scheduler_request_marker_box.hako"
APP_064A="apps/hako-alloc-reclaim-scheduler-request-marker-proof/main.hako"
GUARD_063A="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_boundary_inventory_guard.sh"
GUARD_064A="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_marker_closeout_guard.sh"

echo "[$TAG] checking MIMAP-065A scheduler marker closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$BOUNDARY_SSOT" \
  "$MARKER_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_063A" \
  "$CARD_064A" \
  "$CARD_065A" \
  "$CARD_066A" \
  "$OWNER_064A" \
  "$APP_064A" \
  "$GUARD_063A" \
  "$GUARD_064A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$GUARD_063A" "$GUARD_064A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_063A" "MIMAP-063A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_064A" "MIMAP-064A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_065A" "MIMAP-065A card must be landed"
guard_expect_in_file "$TAG" "Status:" "$CARD_066A" "MIMAP-066A selection card must have status"
guard_expect_in_file "$TAG" "MIMAP-066A" "$CARD_066A" "MIMAP-066A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-065A SSOT must be accepted"
guard_expect_in_file "$TAG" "MIMAP-063A" "$SSOT" "closeout SSOT must include boundary row"
guard_expect_in_file "$TAG" "MIMAP-064A" "$SSOT" "closeout SSOT must include marker row"
guard_expect_in_file "$TAG" "MIMAP-066A post-scheduler-marker row selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "Decision: accepted" "$BOUNDARY_SSOT" "boundary SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$MARKER_SSOT" "marker SSOT must stay accepted"
guard_expect_in_file "$TAG" "id = \"MIMAP-064A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-064A"
guard_expect_in_file "$TAG" "MIMAP-065A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-065A"
guard_expect_in_file "$TAG" "MIMAP-065A reclaim scheduler marker closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "$GUARD_063A" "$INDEX" "check index must list MIMAP-063A guard"
guard_expect_in_file "$TAG" "$GUARD_064A" "$INDEX" "check index must list MIMAP-064A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-065A guard"
guard_expect_in_file "$TAG" 'memory.reclaim_scheduler_request_marker_box = "memory/reclaim_scheduler_request_marker_box.hako"' "$MODULE" "MIMAP-064A owner must stay exported"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_marker_box.hako` owns MIMAP-064A' "$MEMORY_README" "memory README must name MIMAP-064A owner"

if rg -n 'hako-alloc-reclaim-scheduler|HakoAllocReclaimScheduler|reclaim_scheduler' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "scheduler app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|#\[global_allocator\]|GlobalAlloc|replace_allocator' \
  "$OWNER_064A" "$APP_064A" >/tmp/"$TAG".stop_line_leak 2>&1; then
  cat /tmp/"$TAG".stop_line_leak >&2
  rm -f /tmp/"$TAG".stop_line_leak
  guard_fail "$TAG" "scheduler marker closeout must keep scheduling/source-concurrency/replacement inactive"
fi
rm -f /tmp/"$TAG".stop_line_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider 2>&1 || {
  cat /tmp/"$TAG".provider >&2
  rm -f /tmp/"$TAG".provider
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider

echo "[$TAG] ok"

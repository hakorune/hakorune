#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-scheduler-request-ledger-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-closeout-ssot.md"
BOUNDARY_SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-boundary-inventory-ssot.md"
MARKER_SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-marker-ssot.md"
LEDGER_SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-ssot.md"
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
CARD_068A="docs/development/current/main/phases/phase-293x/293x-555-MIMAP-068A-RECLAIM-SCHEDULER-REQUEST-LEDGER-ROUTE.md"
CARD_069A="docs/development/current/main/phases/phase-293x/293x-556-MIMAP-069A-RECLAIM-SCHEDULER-REQUEST-LEDGER-CLOSEOUT-GUARD.md"
CARD_070A="docs/development/current/main/phases/phase-293x/293x-557-MIMAP-070A-POST-SCHEDULER-LEDGER-ROW-SELECTION.md"
OWNER_064A="lang/src/hako_alloc/memory/reclaim_scheduler_request_marker_box.hako"
OWNER_068A="lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_box.hako"
APP_068A="apps/hako-alloc-reclaim-scheduler-request-ledger-proof/main.hako"
APP_README_068A="apps/hako-alloc-reclaim-scheduler-request-ledger-proof/README.md"
APP_TEST_068A="apps/hako-alloc-reclaim-scheduler-request-ledger-proof/test.sh"
GUARD_063A="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_boundary_inventory_guard.sh"
GUARD_064A="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard.sh"
GUARD_065A="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_marker_closeout_guard.sh"
GUARD_068A="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_closeout_guard.sh"

echo "[$TAG] checking MIMAP-069A scheduler request ledger closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$BOUNDARY_SSOT" \
  "$MARKER_SSOT" \
  "$LEDGER_SSOT" \
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
  "$CARD_068A" \
  "$CARD_069A" \
  "$CARD_070A" \
  "$OWNER_064A" \
  "$OWNER_068A" \
  "$APP_068A" \
  "$APP_README_068A" \
  "$APP_TEST_068A" \
  "$GUARD_063A" \
  "$GUARD_064A" \
  "$GUARD_065A" \
  "$GUARD_068A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$GUARD_063A" "$GUARD_064A" "$GUARD_065A" "$GUARD_068A" "$APP_TEST_068A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_063A" "MIMAP-063A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_064A" "MIMAP-064A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_065A" "MIMAP-065A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_068A" "MIMAP-068A must be landed before closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_069A" "MIMAP-069A card must be landed"
guard_expect_in_file "$TAG" "Status:" "$CARD_070A" "MIMAP-070A selection card must have status"
guard_expect_in_file "$TAG" "MIMAP-070A" "$CARD_070A" "MIMAP-070A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-069A SSOT must be accepted"
guard_expect_in_file "$TAG" "MIMAP-063A" "$SSOT" "closeout SSOT must include boundary row"
guard_expect_in_file "$TAG" "MIMAP-064A" "$SSOT" "closeout SSOT must include marker row"
guard_expect_in_file "$TAG" "MIMAP-065A" "$SSOT" "closeout SSOT must include marker closeout row"
guard_expect_in_file "$TAG" "MIMAP-068A" "$SSOT" "closeout SSOT must include ledger row"
guard_expect_in_file "$TAG" "MIMAP-070A post-scheduler-ledger row selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "Decision: accepted" "$BOUNDARY_SSOT" "boundary SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$MARKER_SSOT" "marker SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$LEDGER_SSOT" "ledger SSOT must stay accepted"
guard_expect_in_file "$TAG" "id = \"MIMAP-068A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-068A"
guard_expect_in_file "$TAG" "MIMAP-069A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-069A"
guard_expect_in_file "$TAG" "MIMAP-069A reclaim scheduler request ledger closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "$GUARD_063A" "$INDEX" "check index must list MIMAP-063A guard"
guard_expect_in_file "$TAG" "$GUARD_064A" "$INDEX" "check index must list MIMAP-064A guard"
guard_expect_in_file "$TAG" "$GUARD_065A" "$INDEX" "check index must list MIMAP-065A guard"
guard_expect_in_file "$TAG" "$GUARD_068A" "$INDEX" "check index must list MIMAP-068A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-069A guard"
guard_expect_in_file "$TAG" 'memory.reclaim_scheduler_request_marker_box = "memory/reclaim_scheduler_request_marker_box.hako"' "$MODULE" "MIMAP-064A owner must stay exported"
guard_expect_in_file "$TAG" 'memory.reclaim_scheduler_request_ledger_box = "memory/reclaim_scheduler_request_ledger_box.hako"' "$MODULE" "MIMAP-068A owner must stay exported"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_marker_box.hako` owns MIMAP-064A' "$MEMORY_README" "memory README must name MIMAP-064A owner"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_ledger_box.hako` owns MIMAP-068A' "$MEMORY_README" "memory README must name MIMAP-068A owner"

if rg -n 'hako-alloc-reclaim-scheduler|HakoAllocReclaimScheduler|reclaim_scheduler' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "scheduler app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|#\[global_allocator\]|GlobalAlloc|replace_allocator|hako_osvm_(unreserve|release)' \
  "$OWNER_064A" "$OWNER_068A" "$APP_068A" >/tmp/"$TAG".stop_line_leak 2>&1; then
  cat /tmp/"$TAG".stop_line_leak >&2
  rm -f /tmp/"$TAG".stop_line_leak
  guard_fail "$TAG" "scheduler ledger closeout must keep scheduling/source-concurrency/replacement inactive"
fi
rm -f /tmp/"$TAG".stop_line_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider 2>&1 || {
  cat /tmp/"$TAG".provider >&2
  rm -f /tmp/"$TAG".provider
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider

echo "[$TAG] ok"

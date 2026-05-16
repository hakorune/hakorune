#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-scheduler-boundary-inventory"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-boundary-inventory-ssot.md"
CONCURRENCY_SSOT="docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
CARD_063A="docs/development/current/main/phases/phase-293x/293x-550-MIMAP-063A-RECLAIM-SCHEDULER-BOUNDARY-INVENTORY.md"
CARD_064A="docs/development/current/main/phases/phase-293x/293x-551-MIMAP-064A-RECLAIM-SCHEDULER-REQUEST-MARKER-CONTRACT.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_boundary_inventory_guard.sh"

echo "[$TAG] checking MIMAP-063A reclaim scheduler boundary inventory"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$CONCURRENCY_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$CARD_063A" \
  "$CARD_064A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_063A" "MIMAP-063A card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_064A" "MIMAP-064A must be selected after inventory"
guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-063A SSOT must be accepted"
guard_expect_in_file "$TAG" "allocator-internal scheduler boundary" "$SSOT" "SSOT must name allocator-internal boundary"
guard_expect_in_file "$TAG" "source concurrency surface" "$SSOT" "SSOT must keep source concurrency separate"
guard_expect_in_file "$TAG" "MIMAP-064A reclaim scheduler request marker contract" "$SSOT" "SSOT must name next marker row"
guard_expect_in_file "$TAG" "source-level worker_local remains closed" "$SSOT" "SSOT must keep worker_local source surface closed"
guard_expect_in_file "$TAG" "MIMAP-063A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-063A"
guard_expect_in_file "$TAG" "MIMAP-063A reclaim scheduler boundary inventory" "$JOINT" "joint order must name current row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-063A guard"
guard_expect_in_file "$TAG" "MIMAP-WORKER-001" "$CONCURRENCY_SSOT" "concurrency substrate SSOT must keep worker substrate row visible"
guard_expect_in_file "$TAG" 'Do not use `ChannelBox` for allocator remote-free queues.' "$CONCURRENCY_SSOT" "allocator queues must stay separate from Channel"

if rg -n 'hako-alloc-reclaim-scheduler|HakoAllocReclaimScheduler|reclaim_scheduler' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "scheduler boundary matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'spawn[[:space:]]*\(|thread::|#\[global_allocator\]|GlobalAlloc|replace_allocator' \
  lang/src/hako_alloc lang/c-abi/shims >/tmp/"$TAG".execution_leak 2>&1; then
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  guard_fail "$TAG" "real scheduling or host allocator replacement must remain inactive"
fi
rm -f /tmp/"$TAG".execution_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider 2>&1 || {
  cat /tmp/"$TAG".provider >&2
  rm -f /tmp/"$TAG".provider
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider

echo "[$TAG] ok"

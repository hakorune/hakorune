#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-scheduler-scalar-lane-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-scalar-lane-closeout-ssot.md"
BOUNDARY_SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-boundary-inventory-ssot.md"
MARKER_SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-marker-ssot.md"
LEDGER_SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-ssot.md"
CONSUME_SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-consume-ssot.md"
ROUNDTRIP_SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-ssot.md"
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
CARD_067A="docs/development/current/main/phases/phase-293x/293x-554-MIMAP-067A-RECLAIM-SCHEDULER-SUBSTRATE-PROPOSAL-OR-PARK.md"
CARD_068A="docs/development/current/main/phases/phase-293x/293x-555-MIMAP-068A-RECLAIM-SCHEDULER-REQUEST-LEDGER-ROUTE.md"
CARD_069A="docs/development/current/main/phases/phase-293x/293x-556-MIMAP-069A-RECLAIM-SCHEDULER-REQUEST-LEDGER-CLOSEOUT-GUARD.md"
CARD_071A="docs/development/current/main/phases/phase-293x/293x-558-MIMAP-071A-RECLAIM-SCHEDULER-REQUEST-LEDGER-CONSUME-ROUTE.md"
CARD_072A="docs/development/current/main/phases/phase-293x/293x-559-MIMAP-072A-RECLAIM-SCHEDULER-LEDGER-CONSUME-CLOSEOUT-GUARD.md"
CARD_074A="docs/development/current/main/phases/phase-293x/293x-561-MIMAP-074A-RECLAIM-SCHEDULER-REQUEST-LEDGER-ROUNDTRIP-ROUTE.md"
CARD_075A="docs/development/current/main/phases/phase-293x/293x-562-MIMAP-075A-RECLAIM-SCHEDULER-REQUEST-LEDGER-ROUNDTRIP-CLOSEOUT-GUARD.md"
CARD_077A="docs/development/current/main/phases/phase-293x/293x-564-MIMAP-077A-RECLAIM-SCHEDULER-SCALAR-LANE-CLOSEOUT-GUARD.md"
CARD_078A="docs/development/current/main/phases/phase-293x/293x-565-MIMAP-078A-POST-SCHEDULER-SCALAR-CLOSEOUT-ROW-SELECTION.md"
MARKER_OWNER="lang/src/hako_alloc/memory/reclaim_scheduler_request_marker_box.hako"
LEDGER_OWNER="lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_box.hako"
ROUNDTRIP_OWNER="lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_roundtrip_box.hako"
APP_MARKER="apps/hako-alloc-reclaim-scheduler-request-marker-proof/main.hako"
APP_LEDGER="apps/hako-alloc-reclaim-scheduler-request-ledger-proof/main.hako"
APP_CONSUME="apps/hako-alloc-reclaim-scheduler-request-ledger-consume-proof/main.hako"
APP_ROUNDTRIP="apps/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof/main.hako"
GUARD_MARKER="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard.sh"
GUARD_MARKER_CLOSEOUT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_marker_closeout_guard.sh"
GUARD_LEDGER="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_guard.sh"
GUARD_LEDGER_CLOSEOUT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_closeout_guard.sh"
GUARD_CONSUME="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_guard.sh"
GUARD_CONSUME_CLOSEOUT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_closeout_guard.sh"
GUARD_ROUNDTRIP="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_guard.sh"
GUARD_ROUNDTRIP_CLOSEOUT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_closeout_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_scalar_lane_closeout_guard.sh"

echo "[$TAG] checking MIMAP-077A reclaim scheduler scalar lane closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$BOUNDARY_SSOT" \
  "$MARKER_SSOT" \
  "$LEDGER_SSOT" \
  "$CONSUME_SSOT" \
  "$ROUNDTRIP_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_063A" "$CARD_064A" "$CARD_065A" "$CARD_067A" "$CARD_068A" "$CARD_069A" \
  "$CARD_071A" "$CARD_072A" "$CARD_074A" "$CARD_075A" "$CARD_077A" "$CARD_078A" \
  "$MARKER_OWNER" "$LEDGER_OWNER" "$ROUNDTRIP_OWNER" \
  "$APP_MARKER" "$APP_LEDGER" "$APP_CONSUME" "$APP_ROUNDTRIP" \
  "$GUARD_MARKER" "$GUARD_MARKER_CLOSEOUT" "$GUARD_LEDGER" "$GUARD_LEDGER_CLOSEOUT" \
  "$GUARD_CONSUME" "$GUARD_CONSUME_CLOSEOUT" "$GUARD_ROUNDTRIP" "$GUARD_ROUNDTRIP_CLOSEOUT" \
  "$SELF_SCRIPT"

guard_require_exec_files \
  "$TAG" \
  "$GUARD_MARKER" "$GUARD_MARKER_CLOSEOUT" "$GUARD_LEDGER" "$GUARD_LEDGER_CLOSEOUT" \
  "$GUARD_CONSUME" "$GUARD_CONSUME_CLOSEOUT" "$GUARD_ROUNDTRIP" "$GUARD_ROUNDTRIP_CLOSEOUT" \
  "$SELF_SCRIPT"

for card in "$CARD_063A" "$CARD_064A" "$CARD_065A" "$CARD_067A" "$CARD_068A" "$CARD_069A" "$CARD_071A" "$CARD_072A" "$CARD_074A" "$CARD_075A" "$CARD_077A"; do
  guard_expect_in_file "$TAG" "Status: landed" "$card" "$card must be landed before scheduler scalar closeout"
done
guard_expect_in_file "$TAG" "Status:" "$CARD_078A" "MIMAP-078A selection card must have status"
guard_expect_in_file "$TAG" "MIMAP-078A" "$CARD_078A" "MIMAP-078A selection card must stay present after scheduler scalar closeout"

for ssot in "$SSOT" "$BOUNDARY_SSOT" "$MARKER_SSOT" "$LEDGER_SSOT" "$CONSUME_SSOT" "$ROUNDTRIP_SSOT"; do
  guard_expect_in_file "$TAG" "Decision: accepted" "$ssot" "$ssot must be accepted"
done

for row in MIMAP-063A MIMAP-064A MIMAP-065A MIMAP-067A MIMAP-068A MIMAP-069A MIMAP-071A MIMAP-072A MIMAP-074A MIMAP-075A; do
  guard_expect_in_file "$TAG" "$row" "$SSOT" "scalar lane closeout SSOT must include $row"
done
guard_expect_in_file "$TAG" "MIMAP-078A post-scheduler-scalar-closeout row selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "MIMAP-077A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-077A"
guard_expect_in_file "$TAG" "MIMAP-077A reclaim scheduler scalar lane closeout guard" "$JOINT" "joint order must name closeout row"

for proof_id in MIMAP-064A MIMAP-068A MIMAP-071A MIMAP-074A; do
  guard_expect_in_file "$TAG" "id = \"$proof_id\"" "$PROOF_MANIFEST" "proof manifest must include $proof_id"
done

for guard in "$GUARD_MARKER" "$GUARD_MARKER_CLOSEOUT" "$GUARD_LEDGER" "$GUARD_LEDGER_CLOSEOUT" "$GUARD_CONSUME" "$GUARD_CONSUME_CLOSEOUT" "$GUARD_ROUNDTRIP" "$GUARD_ROUNDTRIP_CLOSEOUT" "$SELF_SCRIPT"; do
  guard_expect_in_file "$TAG" "$guard" "$INDEX" "check index must list $guard"
done

guard_expect_in_file "$TAG" 'memory.reclaim_scheduler_request_marker_box = "memory/reclaim_scheduler_request_marker_box.hako"' "$MODULE" "marker owner must stay exported"
guard_expect_in_file "$TAG" 'memory.reclaim_scheduler_request_ledger_box = "memory/reclaim_scheduler_request_ledger_box.hako"' "$MODULE" "ledger owner must stay exported"
guard_expect_in_file "$TAG" 'memory.reclaim_scheduler_request_ledger_roundtrip_box = "memory/reclaim_scheduler_request_ledger_roundtrip_box.hako"' "$MODULE" "roundtrip owner must stay exported"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_marker_box.hako` owns MIMAP-064A' "$MEMORY_README" "memory README must name marker owner"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_ledger_box.hako` owns MIMAP-068A' "$MEMORY_README" "memory README must name ledger record owner"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_ledger_box.hako` also owns MIMAP-071A' "$MEMORY_README" "memory README must name ledger consume owner"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_ledger_roundtrip_box.hako` owns MIMAP-074A' "$MEMORY_README" "memory README must name roundtrip owner"

if rg -n 'hako-alloc-reclaim-scheduler|HakoAllocReclaimScheduler|reclaim_scheduler' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "scheduler app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|#\[global_allocator\]|GlobalAlloc|replace_allocator|hako_osvm_(unreserve|release)' \
  "$MARKER_OWNER" "$LEDGER_OWNER" "$ROUNDTRIP_OWNER" "$APP_MARKER" "$APP_LEDGER" "$APP_CONSUME" "$APP_ROUNDTRIP" >/tmp/"$TAG".stop_line_leak 2>&1; then
  cat /tmp/"$TAG".stop_line_leak >&2
  rm -f /tmp/"$TAG".stop_line_leak
  guard_fail "$TAG" "scheduler scalar lane closeout must keep scheduling/source-concurrency/replacement inactive"
fi
rm -f /tmp/"$TAG".stop_line_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider 2>&1 || {
  cat /tmp/"$TAG".provider >&2
  rm -f /tmp/"$TAG".provider
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-scheduler-request-ledger-consume-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-consume-closeout-ssot.md"
LEDGER_SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-ssot.md"
CONSUME_SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-consume-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md"
GRANULARITY="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD_068A="docs/development/current/main/phases/phase-293x/293x-555-MIMAP-068A-RECLAIM-SCHEDULER-REQUEST-LEDGER-ROUTE.md"
CARD_069A="docs/development/current/main/phases/phase-293x/293x-556-MIMAP-069A-RECLAIM-SCHEDULER-REQUEST-LEDGER-CLOSEOUT-GUARD.md"
CARD_071A="docs/development/current/main/phases/phase-293x/293x-558-MIMAP-071A-RECLAIM-SCHEDULER-REQUEST-LEDGER-CONSUME-ROUTE.md"
CARD_072A="docs/development/current/main/phases/phase-293x/293x-559-MIMAP-072A-RECLAIM-SCHEDULER-LEDGER-CONSUME-CLOSEOUT-GUARD.md"
CARD_073A="docs/development/current/main/phases/phase-293x/293x-560-MIMAP-073A-POST-SCHEDULER-CONSUME-ROW-SELECTION.md"
OWNER="lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_box.hako"
APP_068A="apps/hako-alloc-reclaim-scheduler-request-ledger-proof/main.hako"
APP_071A="apps/hako-alloc-reclaim-scheduler-request-ledger-consume-proof/main.hako"
APP_TEST_071A="apps/hako-alloc-reclaim-scheduler-request-ledger-consume-proof/test.sh"
GUARD_068A="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_guard.sh"
GUARD_069A="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_closeout_guard.sh"
GUARD_071A="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_closeout_guard.sh"

echo "[$TAG] checking MIMAP-072A scheduler request ledger consume closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
  "$LEDGER_SSOT" \
  "$CONSUME_SSOT" \
  "$TASKBOARD" \
  "$GRANULARITY" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD_068A" \
  "$CARD_069A" \
  "$CARD_071A" \
  "$CARD_072A" \
  "$CARD_073A" \
  "$OWNER" \
  "$APP_068A" \
  "$APP_071A" \
  "$APP_TEST_071A" \
  "$GUARD_068A" \
  "$GUARD_069A" \
  "$GUARD_071A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$GUARD_068A" "$GUARD_069A" "$GUARD_071A" "$APP_TEST_071A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_068A" "MIMAP-068A must be landed before consume closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_069A" "MIMAP-069A must be landed before consume closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_071A" "MIMAP-071A must be landed before consume closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_072A" "MIMAP-072A card must be landed"
guard_expect_in_file "$TAG" "Status:" "$CARD_073A" "MIMAP-073A selection card must have status"
guard_expect_in_file "$TAG" "MIMAP-073A" "$CARD_073A" "MIMAP-073A selection card must stay present after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-072A SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$LEDGER_SSOT" "ledger SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$CONSUME_SSOT" "consume SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-068A" "$SSOT" "closeout SSOT must include record row"
guard_expect_in_file "$TAG" "MIMAP-071A" "$SSOT" "closeout SSOT must include consume row"
guard_expect_in_file "$TAG" "MIMAP-073A post-scheduler-consume row selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "id = \"MIMAP-071A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-071A"
guard_expect_in_file "$TAG" "MIMAP-072A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-072A"
guard_expect_in_file "$TAG" "MIMAP-072A reclaim scheduler ledger consume closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "$GUARD_068A" "$INDEX" "check index must list MIMAP-068A guard"
guard_expect_in_file "$TAG" "$GUARD_069A" "$INDEX" "check index must list MIMAP-069A guard"
guard_expect_in_file "$TAG" "$GUARD_071A" "$INDEX" "check index must list MIMAP-071A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-072A guard"
guard_expect_in_file "$TAG" 'memory.reclaim_scheduler_request_ledger_box = "memory/reclaim_scheduler_request_ledger_box.hako"' "$MODULE" "ledger owner must stay exported"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_ledger_box.hako` owns MIMAP-068A' "$MEMORY_README" "memory README must name MIMAP-068A owner"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_ledger_box.hako` also owns MIMAP-071A' "$MEMORY_README" "memory README must name MIMAP-071A owner"

if rg -n 'hako-alloc-reclaim-scheduler|HakoAllocReclaimScheduler|reclaim_scheduler' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "scheduler app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|#\[global_allocator\]|GlobalAlloc|replace_allocator|hako_osvm_(unreserve|release)' \
  "$OWNER" "$APP_068A" "$APP_071A" >/tmp/"$TAG".stop_line_leak 2>&1; then
  cat /tmp/"$TAG".stop_line_leak >&2
  rm -f /tmp/"$TAG".stop_line_leak
  guard_fail "$TAG" "scheduler ledger consume closeout must keep scheduling/source-concurrency/replacement inactive"
fi
rm -f /tmp/"$TAG".stop_line_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider 2>&1 || {
  cat /tmp/"$TAG".provider >&2
  rm -f /tmp/"$TAG".provider
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider

echo "[$TAG] ok"

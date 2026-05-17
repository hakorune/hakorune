#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-scheduler-request-ledger-roundtrip-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

SSOT="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-closeout-ssot.md"
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
CARD_068A="docs/development/current/main/phases/phase-293x/293x-555-MIMAP-068A-RECLAIM-SCHEDULER-REQUEST-LEDGER-ROUTE.md"
CARD_071A="docs/development/current/main/phases/phase-293x/293x-558-MIMAP-071A-RECLAIM-SCHEDULER-REQUEST-LEDGER-CONSUME-ROUTE.md"
CARD_074A="docs/development/current/main/phases/phase-293x/293x-561-MIMAP-074A-RECLAIM-SCHEDULER-REQUEST-LEDGER-ROUNDTRIP-ROUTE.md"
CARD_075A="docs/development/current/main/phases/phase-293x/293x-562-MIMAP-075A-RECLAIM-SCHEDULER-REQUEST-LEDGER-ROUNDTRIP-CLOSEOUT-GUARD.md"
CARD_076A="docs/development/current/main/phases/phase-293x/293x-563-MIMAP-076A-POST-SCHEDULER-ROUNDTRIP-ROW-SELECTION.md"
LEDGER_OWNER="lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_box.hako"
OWNER="lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_roundtrip_box.hako"
APP_074A="apps/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof/main.hako"
APP_TEST_074A="apps/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof/test.sh"
GUARD_074A="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_closeout_guard.sh"

echo "[$TAG] checking MIMAP-075A scheduler request ledger roundtrip closeout"

guard_require_files \
  "$TAG" \
  "$SSOT" \
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
  "$CARD_068A" \
  "$CARD_071A" \
  "$CARD_074A" \
  "$CARD_075A" \
  "$CARD_076A" \
  "$LEDGER_OWNER" \
  "$OWNER" \
  "$APP_074A" \
  "$APP_TEST_074A" \
  "$GUARD_074A" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$GUARD_074A" "$APP_TEST_074A" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" "Status: landed" "$CARD_068A" "MIMAP-068A must be landed before roundtrip closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_071A" "MIMAP-071A must be landed before roundtrip closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_074A" "MIMAP-074A must be landed before roundtrip closeout"
guard_expect_in_file "$TAG" "Status: landed" "$CARD_075A" "MIMAP-075A card must be landed"
guard_expect_in_file "$TAG" "Status: selected current" "$CARD_076A" "MIMAP-076A must be selected after closeout"

guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-075A SSOT must be accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$LEDGER_SSOT" "ledger SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$CONSUME_SSOT" "consume SSOT must stay accepted"
guard_expect_in_file "$TAG" "Decision: accepted" "$ROUNDTRIP_SSOT" "roundtrip SSOT must stay accepted"
guard_expect_in_file "$TAG" "MIMAP-068A" "$SSOT" "closeout SSOT must include record row"
guard_expect_in_file "$TAG" "MIMAP-071A" "$SSOT" "closeout SSOT must include consume row"
guard_expect_in_file "$TAG" "MIMAP-074A" "$SSOT" "closeout SSOT must include roundtrip row"
guard_expect_in_file "$TAG" "MIMAP-076A post-scheduler-roundtrip row selection" "$SSOT" "closeout SSOT must name next row"
guard_expect_in_file "$TAG" "id = \"MIMAP-074A\"" "$PROOF_MANIFEST" "proof manifest must include MIMAP-074A"
guard_expect_in_file "$TAG" "MIMAP-075A granularity" "$GRANULARITY" "granularity SSOT must describe MIMAP-075A"
guard_expect_in_file "$TAG" "MIMAP-075A reclaim scheduler request ledger roundtrip closeout guard" "$JOINT" "joint order must name closeout row"
guard_expect_in_file "$TAG" "$GUARD_074A" "$INDEX" "check index must list MIMAP-074A guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list MIMAP-075A guard"
guard_expect_in_file "$TAG" 'memory.reclaim_scheduler_request_ledger_box = "memory/reclaim_scheduler_request_ledger_box.hako"' "$MODULE" "ledger owner must stay exported"
guard_expect_in_file "$TAG" 'memory.reclaim_scheduler_request_ledger_roundtrip_box = "memory/reclaim_scheduler_request_ledger_roundtrip_box.hako"' "$MODULE" "roundtrip owner must stay exported"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_ledger_box.hako` owns MIMAP-068A' "$MEMORY_README" "memory README must name MIMAP-068A owner"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_ledger_box.hako` also owns MIMAP-071A' "$MEMORY_README" "memory README must name MIMAP-071A owner"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_ledger_roundtrip_box.hako` owns MIMAP-074A' "$MEMORY_README" "memory README must name MIMAP-074A owner"
guard_expect_in_file "$TAG" "HakoAllocReclaimSchedulerRequestLedger" "$OWNER" "roundtrip owner must compose ledger"
guard_expect_in_file "$TAG" "recordAndConsumeSchedulerRequest" "$OWNER" "roundtrip owner must expose route"

if rg -n 'hako-alloc-reclaim-scheduler|HakoAllocReclaimScheduler|reclaim_scheduler' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "scheduler app/owner matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|#\[global_allocator\]|GlobalAlloc|replace_allocator|hako_osvm_(unreserve|release)' \
  "$LEDGER_OWNER" "$OWNER" "$APP_074A" >/tmp/"$TAG".stop_line_leak 2>&1; then
  cat /tmp/"$TAG".stop_line_leak >&2
  rm -f /tmp/"$TAG".stop_line_leak
  guard_fail "$TAG" "scheduler ledger roundtrip closeout must keep scheduling/source-concurrency/replacement inactive"
fi
rm -f /tmp/"$TAG".stop_line_leak

bash tools/checks/allocator_provider_inactive_sentinel_guard.sh >/tmp/"$TAG".provider 2>&1 || {
  cat /tmp/"$TAG".provider >&2
  rm -f /tmp/"$TAG".provider
  guard_fail "$TAG" "allocator provider inactive sentinel failed"
}
rm -f /tmp/"$TAG".provider

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-scheduler-request-ledger-consume"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-reclaim-scheduler-request-ledger-consume-proof/main.hako"
APP_README="apps/hako-alloc-reclaim-scheduler-request-ledger-consume-proof/README.md"
APP_TEST="apps/hako-alloc-reclaim-scheduler-request-ledger-consume-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-558-MIMAP-071A-RECLAIM-SCHEDULER-REQUEST-LEDGER-CONSUME-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-consume-ssot.md"
LEDGER_DESIGN="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_guard.sh"

printf '[%s] checking MIMAP-071A reclaim scheduler request ledger consume route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$LEDGER_DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-071A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-071A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$LEDGER_DESIGN" "MIMAP-068A ledger design must stay accepted"
guard_expect_in_file "$TAG" 'MIMAP-071A granularity' "$PLAN" "granularity SSOT must describe MIMAP-071A"
guard_expect_in_file "$TAG" 'MIMAP-071A reclaim scheduler request ledger consume route' "$JOINT" "joint order must name MIMAP-071A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-071A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-071A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-071A"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_ledger_box.hako` also owns MIMAP-071A' "$MEMORY_README" "memory README must define MIMAP-071A owner"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimSchedulerRequestLedgerConsumeReport' "$OWNER" "MIMAP-071A consume report must exist"
guard_expect_in_file "$TAG" 'consumePendingRequest' "$OWNER" "MIMAP-071A consume method must exist"
guard_expect_in_file "$TAG" 'consumeReport' "$OWNER" "MIMAP-071A consume report helper must exist"
guard_expect_in_file "$TAG" 'HakoAllocReclaimSchedulerRequestLedger' "$APP" "MIMAP-071A proof must construct owner"
guard_expect_in_file "$TAG" 'check "mimap071a reclaim scheduler request ledger consume"' "$APP" "MIMAP-071A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|drainRemote[[:space:]]*\(|drain_remote[[:space:]]*\(|collectOne[[:space:]]*\(|pushRetry[[:space:]]*\(|peekHead[[:space:]]*\(|peekNext[[:space:]]*\(|releaseLocal[[:space:]]*\(|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-071A must not add real scheduling, source concurrency, atomics, pointer drain, page-source/OS release seams, or page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-071A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-reclaim-scheduler-request-ledger-consume-proof|HakoAllocReclaimSchedulerRequestLedger|reclaim_scheduler_request_ledger' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-071A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_reclaim_scheduler_request_ledger_consume_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-071A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap071a_reclaim_scheduler.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap071a.mir.json"
exe_out="$tmp_dir/mimap071a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-reclaim-scheduler-request-ledger-consume-proof' "$vm_log"
rg -F -q 'no_pending=0,1,0,0,-1,-1' "$vm_log"
rg -F -q 'recorded=1,0,1,200' "$vm_log"
rg -F -q 'mismatch=0,2,1,1,200,200' "$vm_log"
rg -F -q 'consumed=1,0,1,0,200,-1' "$vm_log"
rg -F -q 'after=0,1,0,0,-1,-1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'record_counts=1,1,0' "$vm_log"
rg -F -q 'consume_counts=1,3,0,-1,200,1' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "HakoAllocReclaimSchedulerRequestLedger.consumePendingRequest/1",
    "HakoAllocReclaimSchedulerRequestLedger.consumeReport/6",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
if plans.get("HakoAllocReclaimSchedulerRequestLedgerConsumeReport") is None:
    raise SystemExit("missing typed object plan: HakoAllocReclaimSchedulerRequestLedgerConsumeReport")

fields = {field.get("name"): field for field in plans["HakoAllocReclaimSchedulerRequestLedgerConsumeReport"].get("fields", [])}
required_fields = {
    "did_consume",
    "reason",
    "requested_page_id",
    "consumed_page_id",
    "pending_before",
    "pending_after",
    "pending_page_before",
    "pending_page_after",
    "would_schedule_thread",
    "would_spawn_worker",
    "would_touch_source_concurrency",
    "would_call_page_source",
    "would_unreserve",
    "would_release_osvm",
    "would_activate_provider",
    "would_host_allocator_swap",
}
missing_fields = sorted(name for name in required_fields if name not in fields)
if missing_fields:
    raise SystemExit(f"missing consume report fields: {missing_fields}")

for name in required_fields:
    field = fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad consume report field {name}: {field}")

print("[mimap071a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-reclaim-scheduler-request-ledger-consume-proof' "$run_log"
rg -F -q 'no_pending=0,1,0,0,-1,-1' "$run_log"
rg -F -q 'recorded=1,0,1,200' "$run_log"
rg -F -q 'mismatch=0,2,1,1,200,200' "$run_log"
rg -F -q 'consumed=1,0,1,0,200,-1' "$run_log"
rg -F -q 'after=0,1,0,0,-1,-1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'record_counts=1,1,0' "$run_log"
rg -F -q 'consume_counts=1,3,0,-1,200,1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

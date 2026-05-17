#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-scheduler-request-ledger-roundtrip"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof/main.hako"
APP_README="apps/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof/README.md"
APP_TEST="apps/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-561-MIMAP-074A-RECLAIM-SCHEDULER-REQUEST-LEDGER-ROUNDTRIP-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-roundtrip-ssot.md"
LEDGER_DESIGN="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-ssot.md"
CONSUME_DESIGN="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-ledger-consume-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
LEDGER_OWNER="lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_box.hako"
OWNER="lang/src/hako_alloc/memory/reclaim_scheduler_request_ledger_roundtrip_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_guard.sh"

printf '[%s] checking MIMAP-074A reclaim scheduler request ledger roundtrip route\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$LEDGER_DESIGN" \
  "$CONSUME_DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MEMORY_README" \
  "$MODULE" \
  "$LEDGER_OWNER" \
  "$OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-074A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-074A design must be accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$LEDGER_DESIGN" "MIMAP-068A ledger design must stay accepted"
guard_expect_in_file "$TAG" 'Decision: accepted' "$CONSUME_DESIGN" "MIMAP-071A consume design must stay accepted"
guard_expect_in_file "$TAG" 'MIMAP-074A granularity' "$PLAN" "granularity SSOT must describe MIMAP-074A"
guard_expect_in_file "$TAG" 'MIMAP-074A reclaim scheduler request ledger roundtrip route' "$JOINT" "joint order must name MIMAP-074A"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-074A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-074A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-074A"
guard_expect_in_file "$TAG" 'memory.reclaim_scheduler_request_ledger_roundtrip_box' "$MODULE" "hako module must export MIMAP-074A owner"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_ledger_roundtrip_box.hako` owns MIMAP-074A' "$MEMORY_README" "memory README must define MIMAP-074A owner"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimSchedulerRequestLedgerRoundtripReport' "$OWNER" "MIMAP-074A roundtrip report must exist"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimSchedulerRequestLedgerRoundtrip' "$OWNER" "MIMAP-074A owner must exist"
guard_expect_in_file "$TAG" 'recordAndConsumeSchedulerRequest' "$OWNER" "MIMAP-074A roundtrip method must exist"
guard_expect_in_file "$TAG" 'HakoAllocReclaimSchedulerRequestLedger' "$OWNER" "MIMAP-074A must compose the ledger"
guard_expect_in_file "$TAG" 'HakoAllocReclaimSchedulerRequestLedgerRoundtrip' "$APP" "MIMAP-074A proof must construct owner"
guard_expect_in_file "$TAG" 'check "mimap074a reclaim scheduler request ledger roundtrip"' "$APP" "MIMAP-074A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|wake|sleep|runQueue|run_queue|drainRemote[[:space:]]*\(|drain_remote[[:space:]]*\(|collectOne[[:space:]]*\(|pushRetry[[:space:]]*\(|peekHead[[:space:]]*\(|peekNext[[:space:]]*\(|releaseLocal[[:space:]]*\(|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-074A must not add real scheduling, source concurrency, atomics, pointer drain, page-source/OS release seams, or page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-074A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof|HakoAllocReclaimSchedulerRequestLedgerRoundtrip|reclaim_scheduler_request_ledger_roundtrip' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-074A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_reclaim_scheduler_request_ledger_roundtrip_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-074A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap074a_reclaim_scheduler.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap074a.mir.json"
exe_out="$tmp_dir/mimap074a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof' "$vm_log"
rg -F -q 'success=1,1,0,1,0,300,1,1,1,1,2' "$vm_log"
rg -F -q 'disabled=0,0,2,2,0,0' "$vm_log"
rg -F -q 'blocked=0,0,1,1,0,0' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'route_counts=3,1,2,302,1' "$vm_log"
rg -F -q 'ledger_counts=3,1,2,1,0,-1' "$vm_log"
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
    "HakoAllocReclaimSchedulerRequestLedgerRoundtrip.recordAndConsumeSchedulerRequest/9",
    "HakoAllocReclaimSchedulerRequestLedgerRoundtrip.newReport/1",
    "HakoAllocReclaimSchedulerRequestLedgerRoundtrip.remember/3",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
if plans.get("HakoAllocReclaimSchedulerRequestLedgerRoundtripReport") is None:
    raise SystemExit("missing typed object plan: HakoAllocReclaimSchedulerRequestLedgerRoundtripReport")
if plans.get("HakoAllocReclaimSchedulerRequestLedgerRoundtrip") is None:
    raise SystemExit("missing typed object plan: HakoAllocReclaimSchedulerRequestLedgerRoundtrip")

fields = {field.get("name"): field for field in plans["HakoAllocReclaimSchedulerRequestLedgerRoundtripReport"].get("fields", [])}
required_fields = {
    "did_record_request",
    "did_consume",
    "reason",
    "page_id",
    "pending_after_record",
    "pending_after_consume",
    "record_reason",
    "consume_reason",
    "record_marker",
    "consumed_page_id",
    "marker_did_complete",
    "marker_did_drain",
    "marker_did_transfer",
    "marker_owner_after",
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
    raise SystemExit(f"missing roundtrip report fields: {missing_fields}")

for name in required_fields:
    field = fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad roundtrip report field {name}: {field}")

print("[mimap074a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-reclaim-scheduler-request-ledger-roundtrip-proof' "$run_log"
rg -F -q 'success=1,1,0,1,0,300,1,1,1,1,2' "$run_log"
rg -F -q 'disabled=0,0,2,2,0,0' "$run_log"
rg -F -q 'blocked=0,0,1,1,0,0' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'route_counts=3,1,2,302,1' "$run_log"
rg -F -q 'ledger_counts=3,1,2,1,0,-1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

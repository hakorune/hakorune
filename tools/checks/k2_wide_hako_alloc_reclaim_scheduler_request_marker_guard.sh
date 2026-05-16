#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-scheduler-request-marker"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-reclaim-scheduler-request-marker-proof/main.hako"
APP_README="apps/hako-alloc-reclaim-scheduler-request-marker-proof/README.md"
APP_TEST="apps/hako-alloc-reclaim-scheduler-request-marker-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-551-MIMAP-064A-RECLAIM-SCHEDULER-REQUEST-MARKER-CONTRACT.md"
DESIGN="docs/development/current/main/design/hako-alloc-reclaim-scheduler-request-marker-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/reclaim_scheduler_request_marker_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard.sh"

printf '[%s] checking MIMAP-064A reclaim scheduler request marker contract\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
  "$PLAN" \
  "$JOINT" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-064A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-064A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-064A granularity' "$PLAN" "granularity SSOT must describe MIMAP-064A"
guard_expect_in_file "$TAG" 'MIMAP-064A reclaim scheduler request marker contract' "$JOINT" "joint order must name current row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-064A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-064A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-064A"
guard_expect_in_file "$TAG" 'memory.reclaim_scheduler_request_marker_box = "memory/reclaim_scheduler_request_marker_box.hako"' "$MODULE" "hako_alloc module must export MIMAP-064A owner"
guard_expect_in_file "$TAG" 'reclaim_scheduler_request_marker_box.hako` owns MIMAP-064A' "$MEMORY_README" "memory README must define MIMAP-064A owner"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimSchedulerRequestMarkerReport' "$OWNER" "MIMAP-064A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimSchedulerRequestMarker' "$OWNER" "MIMAP-064A marker owner must exist"
guard_expect_in_file "$TAG" 'classifySchedulerRequest' "$OWNER" "MIMAP-064A owner must expose marker method"
guard_expect_in_file "$TAG" 'HakoAllocReclaimCompletionMarker' "$OWNER" "MIMAP-064A owner must compose completion marker"
guard_expect_in_file "$TAG" 'would_schedule_thread: i64 = 0' "$OWNER" "real scheduling must stay inactive"
guard_expect_in_file "$TAG" 'would_spawn_worker: i64 = 0' "$OWNER" "worker spawning must stay inactive"
guard_expect_in_file "$TAG" 'would_touch_source_concurrency: i64 = 0' "$OWNER" "source concurrency must stay inactive"
guard_expect_in_file "$TAG" 'would_call_page_source: i64 = 0' "$OWNER" "page-source calls must stay inactive"
guard_expect_in_file "$TAG" 'would_unreserve: i64 = 0' "$OWNER" "unreserve must stay inactive"
guard_expect_in_file "$TAG" 'would_release_osvm: i64 = 0' "$OWNER" "OS release must stay inactive"
guard_expect_in_file "$TAG" 'would_activate_provider: i64 = 0' "$OWNER" "provider activation must stay inactive"
guard_expect_in_file "$TAG" 'would_host_allocator_swap: i64 = 0' "$OWNER" "host allocator replacement must stay inactive"
guard_expect_in_file "$TAG" 'HakoAllocReclaimSchedulerRequestMarker' "$APP" "MIMAP-064A proof must construct owner"
guard_expect_in_file "$TAG" 'check "mimap064a reclaim scheduler request marker"' "$APP" "MIMAP-064A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|worker_local|ChannelBox|TaskGroupBox|nowait|sync[[:space:]]+box|context[[:space:]]|drainRemote[[:space:]]*\(|drain_remote[[:space:]]*\(|collectOne[[:space:]]*\(|pushRetry[[:space:]]*\(|peekHead[[:space:]]*\(|peekNext[[:space:]]*\(|releaseLocal[[:space:]]*\(|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-064A must not add real scheduling, source concurrency, atomics, pointer drain, page-source/OS release seams, or page release" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|replace_allocator' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-064A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-reclaim-scheduler-request-marker-proof|HakoAllocReclaimSchedulerRequestMarker|reclaim_scheduler_request_marker' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-064A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-064A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap064a_reclaim_scheduler.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap064a.mir.json"
exe_out="$tmp_dir/mimap064a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-reclaim-scheduler-request-marker-proof' "$vm_log"
rg -F -q 'request_drained=1,0,1,1,1,1,2' "$vm_log"
rg -F -q 'request_direct=1,0,1,1,0,4' "$vm_log"
rg -F -q 'disabled=0,2,0,1' "$vm_log"
rg -F -q 'blocked=0,1,0,1,3' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=4,2,2,93,1,0' "$vm_log"
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
    "HakoAllocReclaimSchedulerRequestMarker.classifySchedulerRequest/9",
    "HakoAllocReclaimSchedulerRequestMarker.report/12",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in ("HakoAllocReclaimSchedulerRequestMarkerReport", "HakoAllocReclaimSchedulerRequestMarker"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {field.get("name"): field for field in plans["HakoAllocReclaimSchedulerRequestMarkerReport"].get("fields", [])}
required_fields = {
    "should_request_schedule",
    "reason",
    "page_id",
    "request_marker",
    "did_complete",
    "completion_marker",
    "completion_reason",
    "integration_reason",
    "did_drain",
    "did_transfer",
    "pending_after",
    "owner_after",
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
    raise SystemExit(f"missing report fields: {missing_fields}")

for name in required_fields:
    field = fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad report field {name}: {field}")

print("[mimap064a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-reclaim-scheduler-request-marker-proof' "$run_log"
rg -F -q 'request_drained=1,0,1,1,1,1,2' "$run_log"
rg -F -q 'request_direct=1,0,1,1,0,4' "$run_log"
rg -F -q 'disabled=0,2,0,1' "$run_log"
rg -F -q 'blocked=0,1,0,1,3' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=4,2,2,93,1,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

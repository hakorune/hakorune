#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-remote-free-drain-execution"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-reclaim-remote-free-drain-execution-proof/main.hako"
APP_README="apps/hako-alloc-reclaim-remote-free-drain-execution-proof/README.md"
APP_TEST="apps/hako-alloc-reclaim-remote-free-drain-execution-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-544-MIMAP-057A-RECLAIM-REMOTE-FREE-DRAIN-FIRST-EXECUTION-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-reclaim-remote-free-drain-execution-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/reclaim_remote_free_drain_execution_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_remote_free_drain_execution_guard.sh"

printf '[%s] checking MIMAP-057A reclaim remote-free drain execution route\n' "$TAG"

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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-057A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-057A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-057A granularity' "$PLAN" "granularity SSOT must describe MIMAP-057A"
guard_expect_in_file "$TAG" 'MIMAP-057A reclaim remote-free drain first execution route' "$JOINT" "joint order must name current row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-057A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-057A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-057A"
guard_expect_in_file "$TAG" 'memory.reclaim_remote_free_drain_execution_box = "memory/reclaim_remote_free_drain_execution_box.hako"' "$MODULE" "hako_alloc module must export MIMAP-057A owner"
guard_expect_in_file "$TAG" 'reclaim_remote_free_drain_execution_box.hako` owns MIMAP-057A' "$MEMORY_README" "memory README must define MIMAP-057A owner"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimRemoteFreeDrainExecutionReport' "$OWNER" "MIMAP-057A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimRemoteFreeDrainExecution' "$OWNER" "MIMAP-057A execution owner must exist"
guard_expect_in_file "$TAG" 'drainOne' "$OWNER" "MIMAP-057A owner must expose drainOne"
guard_expect_in_file "$TAG" 'HakoAllocReclaimRemoteFreeDrainContract' "$OWNER" "MIMAP-057A owner must compose MIMAP-056A contract"
guard_expect_in_file "$TAG" 'did_change_modeled_pending: i64 = 0' "$OWNER" "modeled pending update must be explicit"
guard_expect_in_file "$TAG" 'would_schedule_thread: i64 = 0' "$OWNER" "thread scheduling must stay inactive"
guard_expect_in_file "$TAG" 'would_call_page_source: i64 = 0' "$OWNER" "page-source calls must stay inactive"
guard_expect_in_file "$TAG" 'would_unreserve: i64 = 0' "$OWNER" "unreserve must stay inactive"
guard_expect_in_file "$TAG" 'would_release_osvm: i64 = 0' "$OWNER" "OS release must stay inactive"
guard_expect_in_file "$TAG" 'would_activate_provider: i64 = 0' "$OWNER" "provider activation must stay inactive"
guard_expect_in_file "$TAG" 'would_execute_full_reclaim: i64 = 0' "$OWNER" "full reclaim must stay inactive"
guard_expect_in_file "$TAG" 'would_change_production_page_owner: i64 = 0' "$OWNER" "production owner mutation must stay inactive"
guard_expect_in_file "$TAG" 'would_call_page_release: i64 = 0' "$OWNER" "page-local release must stay inactive"
guard_expect_in_file "$TAG" 'HakoAllocReclaimRemoteFreeDrainExecution' "$APP" "MIMAP-057A proof must construct owner"
guard_expect_in_file "$TAG" 'check "mimap057a reclaim remote free drain execution"' "$APP" "MIMAP-057A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|drainRemote[[:space:]]*\(|drain_remote[[:space:]]*\(|collectOne[[:space:]]*\(|pushRetry[[:space:]]*\(|peekHead[[:space:]]*\(|peekNext[[:space:]]*\(|releaseLocal[[:space:]]*\(|changeOwner[[:space:]]*\(|setOwner[[:space:]]*\(|adoptOwner[[:space:]]*\(|executeReclaim[[:space:]]*\(|reclaimPage[[:space:]]*\(|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-057A must not add real atomics, pointer drain, page release, full reclaim, owner mutation, or page-source/OS release seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-057A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-reclaim-remote-free-drain-execution-proof|HakoAllocReclaimRemoteFreeDrainExecution|reclaim_remote_free_drain_execution' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-057A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_reclaim_remote_free_drain_execution_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-057A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap057a_reclaim_drain_exec.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap057a.mir.json"
exe_out="$tmp_dir/mimap057a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-reclaim-remote-free-drain-execution-proof' "$vm_log"
rg -F -q 'drained=1,0,3,2,1,1' "$vm_log"
rg -F -q 'no_work=0,1,0,0' "$vm_log"
rg -F -q 'over_budget=0,3,1,5' "$vm_log"
rg -F -q 'inconsistent=0,2,3' "$vm_log"
rg -F -q 'bad_budget=0,2,4,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=5,1,1,2,1,64,2,1' "$vm_log"
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
    "HakoAllocReclaimRemoteFreeDrainExecution.drainOne/4",
    "HakoAllocReclaimRemoteFreeDrainExecution.report/11",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in ("HakoAllocReclaimRemoteFreeDrainExecutionReport", "HakoAllocReclaimRemoteFreeDrainExecution"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {field.get("name"): field for field in plans["HakoAllocReclaimRemoteFreeDrainExecutionReport"].get("fields", [])}
required_fields = {
    "did_drain",
    "reason",
    "page_id",
    "pending_before",
    "pending_after",
    "remote_head",
    "drain_budget",
    "contract_reason",
    "needs_drain",
    "bounded_drain_possible",
    "did_change_modeled_pending",
    "would_schedule_thread",
    "would_call_page_source",
    "would_unreserve",
    "would_release_osvm",
    "would_activate_provider",
    "would_execute_full_reclaim",
    "would_change_production_page_owner",
    "would_call_page_release",
}
missing_fields = sorted(name for name in required_fields if name not in fields)
if missing_fields:
    raise SystemExit(f"missing report fields: {missing_fields}")

for name in required_fields:
    field = fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad report field {name}: {field}")

print("[mimap057a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-reclaim-remote-free-drain-execution-proof' "$run_log"
rg -F -q 'drained=1,0,3,2,1,1' "$run_log"
rg -F -q 'no_work=0,1,0,0' "$run_log"
rg -F -q 'over_budget=0,3,1,5' "$run_log"
rg -F -q 'inconsistent=0,2,3' "$run_log"
rg -F -q 'bad_budget=0,2,4,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=5,1,1,2,1,64,2,1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

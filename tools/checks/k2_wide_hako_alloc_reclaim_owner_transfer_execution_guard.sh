#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-owner-transfer-execution"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-reclaim-owner-transfer-execution-proof/main.hako"
APP_README="apps/hako-alloc-reclaim-owner-transfer-execution-proof/README.md"
APP_TEST="apps/hako-alloc-reclaim-owner-transfer-execution-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-542-MIMAP-055A-RECLAIM-OWNER-TRANSFER-FIRST-EXECUTION-ROUTE.md"
DESIGN="docs/development/current/main/design/hako-alloc-reclaim-owner-transfer-execution-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/reclaim_owner_transfer_execution_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_execution_guard.sh"

printf '[%s] checking MIMAP-055A reclaim owner-transfer execution route\n' "$TAG"

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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-055A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-055A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-055A granularity' "$PLAN" "granularity SSOT must describe MIMAP-055A"
guard_expect_in_file "$TAG" 'MIMAP-055A reclaim owner-transfer first execution route' "$JOINT" "joint order must name current row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-055A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-055A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-055A"
guard_expect_in_file "$TAG" 'memory.reclaim_owner_transfer_execution_box = "memory/reclaim_owner_transfer_execution_box.hako"' "$MODULE" "hako_alloc module must export MIMAP-055A owner"
guard_expect_in_file "$TAG" 'reclaim_owner_transfer_execution_box.hako` owns MIMAP-055A' "$MEMORY_README" "memory README must define MIMAP-055A owner"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimOwnerTransferExecutionReport' "$OWNER" "MIMAP-055A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimOwnerTransferExecution' "$OWNER" "MIMAP-055A executor owner must exist"
guard_expect_in_file "$TAG" 'executeTransfer' "$OWNER" "MIMAP-055A owner must expose executeTransfer"
guard_expect_in_file "$TAG" 'HakoAllocReclaimOwnerTransferContract' "$OWNER" "MIMAP-055A owner must compose readiness contract"
guard_expect_in_file "$TAG" 'HakoAllocReclaimAtomicClaimContract' "$OWNER" "MIMAP-055A owner must compose claim contract"
guard_expect_in_file "$TAG" 'did_change_modeled_owner: i64 = 0' "$OWNER" "modeled owner update must be explicit"
guard_expect_in_file "$TAG" 'would_change_production_page_owner: i64 = 0' "$OWNER" "production owner mutation must stay inactive"
guard_expect_in_file "$TAG" 'would_execute_full_reclaim: i64 = 0' "$OWNER" "full reclaim must stay inactive"
guard_expect_in_file "$TAG" 'would_drain_remote_free: i64 = 0' "$OWNER" "remote-free drain must stay inactive"
guard_expect_in_file "$TAG" 'would_schedule_thread: i64 = 0' "$OWNER" "thread scheduling must stay inactive"
guard_expect_in_file "$TAG" 'would_call_page_source: i64 = 0' "$OWNER" "page-source calls must stay inactive"
guard_expect_in_file "$TAG" 'would_unreserve: i64 = 0' "$OWNER" "unreserve must stay inactive"
guard_expect_in_file "$TAG" 'would_release_osvm: i64 = 0' "$OWNER" "OS release must stay inactive"
guard_expect_in_file "$TAG" 'would_activate_provider: i64 = 0' "$OWNER" "provider activation must stay inactive"
guard_expect_in_file "$TAG" 'HakoAllocReclaimOwnerTransferExecution' "$APP" "MIMAP-055A proof must construct owner"
guard_expect_in_file "$TAG" 'check "mimap055a reclaim owner transfer execution"' "$APP" "MIMAP-055A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|drainRemote[[:space:]]*\(|drain_remote[[:space:]]*\(|changeOwner[[:space:]]*\(|setOwner[[:space:]]*\(|adoptOwner[[:space:]]*\(|executeReclaim[[:space:]]*\(|reclaimPage[[:space:]]*\(|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-055A must not add real atomics, full reclaim, remote-free drain, production owner mutation, or page-source/OS release seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-055A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-reclaim-owner-transfer-execution-proof|HakoAllocReclaimOwnerTransferExecution|reclaim_owner_transfer_execution' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-055A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_reclaim_owner_transfer_execution_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-055A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap055a_reclaim_exec.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap055a.mir.json"
exe_out="$tmp_dir/mimap055a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-reclaim-owner-transfer-execution-proof' "$vm_log"
rg -F -q 'success=1,0,1,1,2,1' "$vm_log"
rg -F -q 'contract_block=0,1,3,0,1' "$vm_log"
rg -F -q 'claim_block=0,2,1,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=3,1,1,1,2,42,2' "$vm_log"
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
    "HakoAllocReclaimOwnerTransferExecution.executeTransfer/10",
    "HakoAllocReclaimOwnerTransferExecution.report/12",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in ("HakoAllocReclaimOwnerTransferExecutionReport", "HakoAllocReclaimOwnerTransferExecution"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {field.get("name"): field for field in plans["HakoAllocReclaimOwnerTransferExecutionReport"].get("fields", [])}
required_fields = {
    "did_transfer",
    "reason",
    "page_id",
    "expected_owner",
    "observed_owner",
    "claimant_owner",
    "owner_after",
    "contract_ready",
    "contract_reason",
    "claim_ready",
    "claim_reason",
    "did_change_modeled_owner",
    "would_change_production_page_owner",
    "would_execute_full_reclaim",
    "would_drain_remote_free",
    "would_schedule_thread",
    "would_call_page_source",
    "would_unreserve",
    "would_release_osvm",
    "would_activate_provider",
}
missing_fields = sorted(name for name in required_fields if name not in fields)
if missing_fields:
    raise SystemExit(f"missing report fields: {missing_fields}")

for name in required_fields:
    field = fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad report field {name}: {field}")

print("[mimap055a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-reclaim-owner-transfer-execution-proof' "$run_log"
rg -F -q 'success=1,0,1,1,2,1' "$run_log"
rg -F -q 'contract_block=0,1,3,0,1' "$run_log"
rg -F -q 'claim_block=0,2,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=3,1,1,1,2,42,2' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-owner-transfer-contract"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-reclaim-owner-transfer-contract-proof/main.hako"
APP_README="apps/hako-alloc-reclaim-owner-transfer-contract-proof/README.md"
APP_TEST="apps/hako-alloc-reclaim-owner-transfer-contract-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-535-MIMAP-051A-RECLAIM-OWNER-TRANSFER-CONTRACT-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-reclaim-owner-transfer-contract-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/reclaim_owner_transfer_contract_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_contract_guard.sh"

printf '[%s] checking MIMAP-051A reclaim owner-transfer contract inventory\n' "$TAG"

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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-051A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-051A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-051A granularity' "$PLAN" "granularity SSOT must describe MIMAP-051A"
guard_expect_in_file "$TAG" 'MIMAP-051A reclaim owner-transfer contract inventory' "$JOINT" "joint order must name current row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-051A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-051A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-051A"
guard_expect_in_file "$TAG" 'memory.reclaim_owner_transfer_contract_box = "memory/reclaim_owner_transfer_contract_box.hako"' "$MODULE" "hako_alloc module must export MIMAP-051A owner"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimOwnerTransferContractReport' "$OWNER" "MIMAP-051A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimOwnerTransferContract' "$OWNER" "MIMAP-051A contract owner must exist"
guard_expect_in_file "$TAG" 'classifyTransfer' "$OWNER" "MIMAP-051A owner must expose classifyTransfer"
guard_expect_in_file "$TAG" 'would_schedule_thread: i64 = 0' "$OWNER" "thread scheduling must stay inactive"
guard_expect_in_file "$TAG" 'would_atomic_claim: i64 = 0' "$OWNER" "atomic claim must stay inactive"
guard_expect_in_file "$TAG" 'would_drain_remote_free: i64 = 0' "$OWNER" "remote-free drain must stay inactive"
guard_expect_in_file "$TAG" 'would_change_page_owner: i64 = 0' "$OWNER" "owner mutation must stay inactive"
guard_expect_in_file "$TAG" 'would_execute_reclaim: i64 = 0' "$OWNER" "reclaim execution must stay inactive"
guard_expect_in_file "$TAG" 'would_call_page_source: i64 = 0' "$OWNER" "page-source calls must stay inactive"
guard_expect_in_file "$TAG" 'would_unreserve: i64 = 0' "$OWNER" "unreserve must stay inactive"
guard_expect_in_file "$TAG" 'would_release_osvm: i64 = 0' "$OWNER" "OS release must stay inactive"
guard_expect_in_file "$TAG" 'reclaim_owner_transfer_contract_box.hako` owns MIMAP-051A' "$MEMORY_README" "memory README must define MIMAP-051A owner"
guard_expect_in_file "$TAG" 'HakoAllocReclaimOwnerTransferContract' "$APP" "MIMAP-051A proof must construct owner"
guard_expect_in_file "$TAG" 'check "mimap051a reclaim owner transfer contract"' "$APP" "MIMAP-051A proof must use labelled check block"

if rg -n 'AtomicCoreBox|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|drainRemote[[:space:]]*\(|drain_remote[[:space:]]*\(|changeOwner[[:space:]]*\(|setOwner[[:space:]]*\(|adoptOwner[[:space:]]*\(|executeReclaim[[:space:]]*\(|reclaimPage[[:space:]]*\(|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-051A inventory must not execute reclaim, atomics, remote-free drain, owner mutation, or page-source/OS release seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|random_source|entropy_source|hako_random|hako_entropy' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-051A must not add env/provider/hook/replacement/random behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-reclaim-owner-transfer-contract-proof|HakoAllocReclaimOwnerTransferContract|reclaim_owner_transfer_contract' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-051A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_reclaim_owner_transfer_contract_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-051A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap051a_reclaim_contract.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap051a.mir.json"
exe_out="$tmp_dir/mimap051a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-reclaim-owner-transfer-contract-proof' "$vm_log"
rg -F -q 'unknown=0,1,1' "$vm_log"
rg -F -q 'active=0,3,3' "$vm_log"
rg -F -q 'remote=0,4,4' "$vm_log"
rg -F -q 'no_backing=0,6,1' "$vm_log"
rg -F -q 'decommitted=0,5,5,4' "$vm_log"
rg -F -q 'ready=1,0,1,1,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=6,1,5,2,1,1,1,35,0' "$vm_log"
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
    "HakoAllocReclaimOwnerTransferContract.classifyTransfer/9",
    "HakoAllocReclaimOwnerTransferContract.decision/19",
    "HakoAllocReclaimOwnerTransferContract.reasonFromOwner/1",
    "HakoAllocReclaimOwnerTransferContract.reasonFromReclaim/1",
    "HakoAllocReclaimOwnerTransferContract.updateBlockCounts/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in ("HakoAllocReclaimOwnerTransferContractReport", "HakoAllocReclaimOwnerTransferContract"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {field.get("name"): field for field in plans["HakoAllocReclaimOwnerTransferContractReport"].get("fields", [])}
required_fields = {
    "contract_ready",
    "reason",
    "page_id",
    "owner_thread_id",
    "observer_thread_id",
    "owner_known",
    "owner_active",
    "same_thread",
    "abandoned",
    "owner_adoption_candidate",
    "reclaim_candidate",
    "requires_owner_adoption",
    "remote_free_pending",
    "decommitted",
    "backing_bytes",
    "page_empty",
    "retired",
    "owner_reason",
    "reclaim_reason",
    "would_schedule_thread",
    "would_atomic_claim",
    "would_drain_remote_free",
    "would_change_page_owner",
    "would_execute_reclaim",
    "would_call_page_source",
    "would_unreserve",
    "would_release_osvm",
}
missing_fields = sorted(name for name in required_fields if name not in fields)
if missing_fields:
    raise SystemExit(f"missing report fields: {missing_fields}")

for name in required_fields:
    field = fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad report field {name}: {field}")

print("[mimap051a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-reclaim-owner-transfer-contract-proof' "$run_log"
rg -F -q 'ready=1,0,1,1,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=6,1,5,2,1,1,1,35,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

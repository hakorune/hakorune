#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-reclaim-atomic-claim-contract"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-reclaim-atomic-claim-contract-proof/main.hako"
APP_README="apps/hako-alloc-reclaim-atomic-claim-contract-proof/README.md"
APP_TEST="apps/hako-alloc-reclaim-atomic-claim-contract-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-541-MIMAP-054A-RECLAIM-ATOMIC-CLAIM-CONTRACT.md"
DESIGN="docs/development/current/main/design/hako-alloc-reclaim-atomic-claim-contract-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
JOINT="docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/reclaim_atomic_claim_contract_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_reclaim_atomic_claim_contract_guard.sh"

printf '[%s] checking MIMAP-054A reclaim atomic-claim contract\n' "$TAG"

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

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-054A card must be landed"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "MIMAP-054A design must be accepted"
guard_expect_in_file "$TAG" 'MIMAP-054A granularity' "$PLAN" "granularity SSOT must describe MIMAP-054A"
guard_expect_in_file "$TAG" 'MIMAP-054A reclaim atomic-claim contract' "$JOINT" "joint order must name current row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-054A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-054A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-054A"
guard_expect_in_file "$TAG" 'memory.reclaim_atomic_claim_contract_box = "memory/reclaim_atomic_claim_contract_box.hako"' "$MODULE" "hako_alloc module must export MIMAP-054A owner"
guard_expect_in_file "$TAG" 'reclaim_atomic_claim_contract_box.hako` owns MIMAP-054A' "$MEMORY_README" "memory README must define MIMAP-054A owner"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimAtomicClaimContractReport' "$OWNER" "MIMAP-054A report box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocReclaimAtomicClaimContract' "$OWNER" "MIMAP-054A contract owner must exist"
guard_expect_in_file "$TAG" 'attemptClaim' "$OWNER" "MIMAP-054A owner must expose attemptClaim"
guard_expect_in_file "$TAG" 'would_execute_reclaim: i64 = 0' "$OWNER" "reclaim execution must stay inactive"
guard_expect_in_file "$TAG" 'would_change_page_owner: i64 = 0' "$OWNER" "production owner mutation must stay inactive"
guard_expect_in_file "$TAG" 'would_atomic_claim: i64 = 0' "$OWNER" "real atomic claim must stay inactive"
guard_expect_in_file "$TAG" 'would_drain_remote_free: i64 = 0' "$OWNER" "remote-free drain must stay inactive"
guard_expect_in_file "$TAG" 'would_schedule_thread: i64 = 0' "$OWNER" "thread scheduling must stay inactive"
guard_expect_in_file "$TAG" 'would_call_page_source: i64 = 0' "$OWNER" "page-source calls must stay inactive"
guard_expect_in_file "$TAG" 'would_unreserve: i64 = 0' "$OWNER" "unreserve must stay inactive"
guard_expect_in_file "$TAG" 'would_release_osvm: i64 = 0' "$OWNER" "OS release must stay inactive"
guard_expect_in_file "$TAG" 'HakoAllocReclaimAtomicClaimContract' "$APP" "MIMAP-054A proof must construct owner"
guard_expect_in_file "$TAG" 'check "mimap054a reclaim atomic claim contract"' "$APP" "MIMAP-054A proof must use labelled check block"

if rg -n 'AtomicCoreBox|hako_atomic|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|drainRemote[[:space:]]*\(|drain_remote[[:space:]]*\(|changeOwner[[:space:]]*\(|setOwner[[:space:]]*\(|adoptOwner[[:space:]]*\(|executeReclaim[[:space:]]*\(|reclaimPage[[:space:]]*\(|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-054A contract must not execute reclaim, real atomics, remote-free drain, owner mutation, or page-source/OS release seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-054A must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-reclaim-atomic-claim-contract-proof|HakoAllocReclaimAtomicClaimContract|reclaim_atomic_claim_contract' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-054A app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_reclaim_atomic_claim_contract_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: MIMAP-054A guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap054a_reclaim_claim.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap054a.mir.json"
exe_out="$tmp_dir/mimap054a.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-reclaim-atomic-claim-contract-proof' "$vm_log"
rg -F -q 'success=1,1,0,20' "$vm_log"
rg -F -q 'mismatch=0,0,1,11' "$vm_log"
rg -F -q 'bad_expected=0,0,2,10' "$vm_log"
rg -F -q 'bad_claimant=0,0,3,10' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=4,1,1,1,1,3,10' "$vm_log"
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
    "HakoAllocReclaimAtomicClaimContract.attemptClaim/3",
    "HakoAllocReclaimAtomicClaimContract.report/7",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in ("HakoAllocReclaimAtomicClaimContractReport", "HakoAllocReclaimAtomicClaimContract"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {field.get("name"): field for field in plans["HakoAllocReclaimAtomicClaimContractReport"].get("fields", [])}
required_fields = {
    "claim_ready",
    "claim_succeeded",
    "reason",
    "expected_owner",
    "observed_owner",
    "claimant_owner",
    "owner_after",
    "would_execute_reclaim",
    "would_change_page_owner",
    "would_atomic_claim",
    "would_drain_remote_free",
    "would_schedule_thread",
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

print("[mimap054a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-reclaim-atomic-claim-contract-proof' "$run_log"
rg -F -q 'success=1,1,0,20' "$run_log"
rg -F -q 'mismatch=0,0,1,11' "$run_log"
rg -F -q 'bad_expected=0,0,2,10' "$run_log"
rg -F -q 'bad_claimant=0,0,3,10' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=4,1,1,1,1,3,10' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

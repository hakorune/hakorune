#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-purge-candidate-policy-inventory"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-purge-candidate-policy-inventory-proof/main.hako"
APP_README="apps/hako-alloc-purge-candidate-policy-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-purge-candidate-policy-inventory-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-257-M211-PURGE-CANDIDATE-POLICY-INVENTORY.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/purge_candidate_policy_box.hako"
LIFECYCLE="lang/src/hako_alloc/memory/page_lifecycle_invariant_box.hako"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_purge_candidate_policy_inventory_guard.sh"

echo "[$TAG] checking M211 purge candidate policy inventory"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$OWNER" \
  "$LIFECYCLE" \
  "$CURRENT_STATE" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M211 card must be complete"
guard_expect_in_file "$TAG" 'M211 status:' "$PLAN" "mimalloc plan must record M211 status"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M211 guard"
guard_expect_in_file "$TAG" 'id = "M211"' "$PROOF_MANIFEST" "proof app manifest must list M211"
guard_expect_in_file "$TAG" 'latest_card = "293x-257-M211-PURGE-CANDIDATE-POLICY-INVENTORY"' "$CURRENT_STATE" "current state must point at M211 as latest card"
guard_expect_in_file "$TAG" 'current_blocker_token = "M212 bounded purge/decommit scheduler small path"' "$CURRENT_STATE" "current state must advance to M212"
guard_expect_in_file "$TAG" 'memory.purge_candidate_policy_box = "memory/purge_candidate_policy_box.hako"' "$MODULE" "hako_alloc module must export M211 owner"
guard_expect_in_file "$TAG" 'box HakoAllocPurgeCandidateDecision' "$OWNER" "M211 decision box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocPurgeCandidatePolicyInventory' "$OWNER" "M211 policy inventory box must exist"
guard_expect_in_file "$TAG" 'classifyLifecycleReport' "$OWNER" "M211 inventory must expose classifyLifecycleReport"
guard_expect_in_file "$TAG" 'would_schedule_decommit: i64 = 0' "$OWNER" "M211 scheduler execution must stay inactive"
guard_expect_in_file "$TAG" 'would_decommit: i64 = 0' "$OWNER" "M211 decommit execution must stay inactive"
guard_expect_in_file "$TAG" 'would_recommit: i64 = 0' "$OWNER" "M211 recommit execution must stay inactive"
guard_expect_in_file "$TAG" 'would_unreserve: i64 = 0' "$OWNER" "M211 unreserve must stay inactive"
guard_expect_in_file "$TAG" 'would_release_osvm: i64 = 0' "$OWNER" "M211 OS release must stay inactive"
guard_expect_in_file "$TAG" 'purge_candidate_policy_box.hako` owns M211 purge candidate policy inventory' "$MEMORY_README" "memory README must define M211 owner"
guard_expect_in_file "$TAG" 'HakoAllocPurgeCandidatePolicyInventory' "$APP" "M211 proof must construct candidate policy inventory"
guard_expect_in_file "$TAG" 'check "m211 purge candidate policy inventory"' "$APP" "M211 proof must use labelled check block"

if rg -n 'observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|acquire[[:space:]]*\(|releaseLocal[[:space:]]*\(|reactivate[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(' \
  "$OWNER" >/tmp/"$TAG".mutation_leak 2>&1; then
  echo "[$TAG] ERROR: M211 owner must not observe, scan, mutate, schedule, or source memory" >&2
  cat /tmp/"$TAG".mutation_leak >&2
  rm -f /tmp/"$TAG".mutation_leak
  exit 1
fi
rm -f /tmp/"$TAG".mutation_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|provider_|install_hook|global_allocator|InlineRecord|ArrayStorage|PlanProbe|AutoUse|compiler' \
  "$OWNER" "$APP_README" >/tmp/"$TAG".forbidden_vocab 2>&1; then
  echo "[$TAG] ERROR: M211 owner/readme must stay options/provider/backend vocabulary free" >&2
  cat /tmp/"$TAG".forbidden_vocab >&2
  rm -f /tmp/"$TAG".forbidden_vocab
  exit 1
fi
rm -f /tmp/"$TAG".forbidden_vocab

if rg -n 'hako-alloc-purge-candidate-policy-inventory-proof|HakoAllocPurgeCandidateDecision|HakoAllocPurgeCandidatePolicyInventory|classifyLifecycleReport' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M211 app/policy matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_purge_candidate_policy_inventory_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M211 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m211_hako_alloc_purge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m211.mir.json"
exe_out="$tmp_dir/m211.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-purge-candidate-policy-inventory-proof' "$vm_log"
rg -F -q 'null=0,1,-1,0' "$vm_log"
rg -F -q 'missing=0,2,60,0' "$vm_log"
rg -F -q 'active=0,3,10,1,1' "$vm_log"
rg -F -q 'retired_busy=0,6,20,0' "$vm_log"
rg -F -q 'ready=1,0,30,2,1,4096' "$vm_log"
rg -F -q 'decommitted=0,4,40,3,1,1' "$vm_log"
rg -F -q 'recommitted=0,5,50,4,1' "$vm_log"
rg -F -q 'would=0,0,0,0,0' "$vm_log"
rg -F -q 'counts=7,1,6,1,1,1,1,1,1,50,5' "$vm_log"
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
    "HakoAllocPurgeCandidatePolicyInventory.classifyLifecycleReport/1",
    "HakoAllocPurgeCandidatePolicyInventory.decision/15",
    "HakoAllocPurgeCandidatePolicyInventory.reject/3",
    "M211LifecycleReportFactory.report/17",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in ("HakoAllocPurgeCandidateDecision", "HakoAllocPurgeCandidatePolicyInventory"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

decision_fields = {
    field.get("name"): field
    for field in plans["HakoAllocPurgeCandidateDecision"].get("fields", [])
}
required_fields = {
    "eligible",
    "reason",
    "page_id",
    "lifecycle_state",
    "active",
    "retired",
    "decommitted",
    "recommitted",
    "decommit_candidate",
    "recommit_required",
    "duplicate_decommit_blocked",
    "has_backing",
    "backing_bytes",
    "marked_generations",
    "recommitted_generations",
    "would_schedule_decommit",
    "would_decommit",
    "would_recommit",
    "would_unreserve",
    "would_release_osvm",
}
missing_fields = sorted(name for name in required_fields if name not in decision_fields)
if missing_fields:
    raise SystemExit(f"missing purge candidate decision fields: {missing_fields}")

for name in required_fields:
    field = decision_fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad purge candidate field {name}: {field}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

owner_fn = functions["HakoAllocPurgeCandidatePolicyInventory.classifyLifecycleReport/1"]
for callee in iter_calls(owner_fn):
    name = callee.get("name")
    if name in {"observeHeapPage", "selectHeapPage", "attemptHeapPage", "acquire", "releaseLocal", "reactivate", "decommitPage", "commitPage"}:
        raise SystemExit(f"classifyLifecycleReport must not call behavior method: {callee}")

print("[m211-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-purge-candidate-policy-inventory-proof' "$run_log"
rg -F -q 'null=0,1,-1,0' "$run_log"
rg -F -q 'missing=0,2,60,0' "$run_log"
rg -F -q 'active=0,3,10,1,1' "$run_log"
rg -F -q 'retired_busy=0,6,20,0' "$run_log"
rg -F -q 'ready=1,0,30,2,1,4096' "$run_log"
rg -F -q 'decommitted=0,4,40,3,1,1' "$run_log"
rg -F -q 'recommitted=0,5,50,4,1' "$run_log"
rg -F -q 'would=0,0,0,0,0' "$run_log"
rg -F -q 'counts=7,1,6,1,1,1,1,1,1,50,5' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

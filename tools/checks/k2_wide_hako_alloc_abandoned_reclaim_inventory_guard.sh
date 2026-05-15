#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-abandoned-reclaim-inventory"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-abandoned-reclaim-inventory-proof/main.hako"
APP_README="apps/hako-alloc-abandoned-reclaim-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-abandoned-reclaim-inventory-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-261-M213-ABANDONED-RECLAIM-INVENTORY.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/abandoned_reclaim_inventory_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh"

echo "[$TAG] checking M213 abandoned/reclaim inventory"

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
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M213 card must be complete"
guard_expect_in_file "$TAG" 'M213 status:' "$PLAN" "mimalloc plan must record M213 status"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M213 guard"
guard_expect_in_file "$TAG" 'id = "M213"' "$PROOF_MANIFEST" "proof app manifest must list M213"
guard_expect_in_file "$TAG" 'memory.abandoned_reclaim_inventory_box = "memory/abandoned_reclaim_inventory_box.hako"' "$MODULE" "hako_alloc module must export M213 owner"
guard_expect_in_file "$TAG" 'box HakoAllocAbandonedReclaimDecision' "$OWNER" "M213 decision box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocAbandonedReclaimInventory' "$OWNER" "M213 inventory box must exist"
guard_expect_in_file "$TAG" 'classifyPage' "$OWNER" "M213 inventory must expose classifyPage"
guard_expect_in_file "$TAG" 'would_schedule_reclaim: i64 = 0' "$OWNER" "M213 reclaim scheduling must stay inactive"
guard_expect_in_file "$TAG" 'would_reclaim: i64 = 0' "$OWNER" "M213 reclaim execution must stay inactive"
guard_expect_in_file "$TAG" 'would_atomic_claim: i64 = 0' "$OWNER" "M213 atomic claim must stay inactive"
guard_expect_in_file "$TAG" 'would_decommit: i64 = 0' "$OWNER" "M213 decommit execution must stay inactive"
guard_expect_in_file "$TAG" 'would_unreserve: i64 = 0' "$OWNER" "M213 unreserve must stay inactive"
guard_expect_in_file "$TAG" 'would_release_osvm: i64 = 0' "$OWNER" "M213 OS release must stay inactive"
guard_expect_in_file "$TAG" 'abandoned_reclaim_inventory_box.hako` owns M213 abandoned/reclaim inventory' "$MEMORY_README" "memory README must define M213 owner"
guard_expect_in_file "$TAG" 'HakoAllocAbandonedReclaimInventory' "$APP" "M213 proof must construct reclaim inventory"
guard_expect_in_file "$TAG" 'check "m213 abandoned reclaim inventory"' "$APP" "M213 proof must use labelled check block"

if rg -n 'AtomicCoreBox|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(' \
  "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: M213 inventory must not schedule threads, use atomics, observe, execute, or call page-source/OS release seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|provider_|install_hook|global_allocator|InlineRecord|ArrayStorage|PlanProbe|AutoUse|compiler' \
  "$OWNER" "$APP_README" >/tmp/"$TAG".forbidden_vocab 2>&1; then
  echo "[$TAG] ERROR: M213 owner/readme must stay options/provider/backend vocabulary free" >&2
  cat /tmp/"$TAG".forbidden_vocab >&2
  rm -f /tmp/"$TAG".forbidden_vocab
  exit 1
fi
rm -f /tmp/"$TAG".forbidden_vocab

if rg -n 'hako-alloc-abandoned-reclaim-inventory-proof|HakoAllocAbandonedReclaimDecision|HakoAllocAbandonedReclaimInventory|abandoned_reclaim_inventory' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M213 app/inventory matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_abandoned_reclaim_inventory_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M213 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m213_hako_alloc_reclaim.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m213.mir.json"
exe_out="$tmp_dir/m213.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-abandoned-reclaim-inventory-proof' "$vm_log"
rg -F -q 'missing=0,1,10,0' "$vm_log"
rg -F -q 'active_owner=0,2,0,1' "$vm_log"
rg -F -q 'same_owner=0,2,2,2' "$vm_log"
rg -F -q 'remote_pending=0,3,3' "$vm_log"
rg -F -q 'decommitted=0,4,1' "$vm_log"
rg -F -q 'live=1,0,1,1,1,0' "$vm_log"
rg -F -q 'retired=1,0,1,1,1' "$vm_log"
rg -F -q 'would=0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=7,2,5,1,2,1,1,1,1,1,16,0' "$vm_log"
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
    "HakoAllocAbandonedReclaimInventory.classifyPage/9",
    "HakoAllocAbandonedReclaimInventory.decision/16",
    "HakoAllocAbandonedReclaimInventory.reject/12",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in ("HakoAllocAbandonedReclaimDecision", "HakoAllocAbandonedReclaimInventory"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

decision_fields = {
    field.get("name"): field
    for field in plans["HakoAllocAbandonedReclaimDecision"].get("fields", [])
}
required_fields = {
    "eligible",
    "reason",
    "page_id",
    "owner_thread_id",
    "observer_thread_id",
    "abandoned",
    "reclaim_candidate",
    "requires_owner_adoption",
    "can_forward_to_purge_candidate",
    "has_backing",
    "owner_active",
    "remote_free_pending",
    "page_empty",
    "retired",
    "decommitted",
    "backing_bytes",
    "would_schedule_reclaim",
    "would_reclaim",
    "would_atomic_claim",
    "would_decommit",
    "would_unreserve",
    "would_release_osvm",
}
missing_fields = sorted(name for name in required_fields if name not in decision_fields)
if missing_fields:
    raise SystemExit(f"missing abandoned reclaim fields: {missing_fields}")

for name in required_fields:
    field = decision_fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad abandoned reclaim field {name}: {field}")

print("[m213-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-abandoned-reclaim-inventory-proof' "$run_log"
rg -F -q 'missing=0,1,10,0' "$run_log"
rg -F -q 'active_owner=0,2,0,1' "$run_log"
rg -F -q 'same_owner=0,2,2,2' "$run_log"
rg -F -q 'remote_pending=0,3,3' "$run_log"
rg -F -q 'decommitted=0,4,1' "$run_log"
rg -F -q 'live=1,0,1,1,1,0' "$run_log"
rg -F -q 'retired=1,0,1,1,1' "$run_log"
rg -F -q 'would=0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=7,2,5,1,2,1,1,1,1,1,16,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

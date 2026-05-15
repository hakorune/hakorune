#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-thread-heap-owner-inventory"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-thread-heap-owner-inventory-proof/main.hako"
APP_README="apps/hako-alloc-thread-heap-owner-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-thread-heap-owner-inventory-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-268-M215-THREAD-HEAP-OWNER-TOKEN-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-thread-heap-owner-inventory-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/thread_heap_owner_inventory_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_thread_heap_owner_inventory_guard.sh"

printf '[%s] checking M215 thread heap owner-token inventory\n' "$TAG"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$DESIGN" \
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

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M215 card must be complete"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "M215 design must be accepted"
guard_expect_in_file "$TAG" 'M215 status:' "$PLAN" "mimalloc plan must record M215 status"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M215 guard"
guard_expect_in_file "$TAG" 'id = "M215"' "$PROOF_MANIFEST" "proof app manifest must list M215"
guard_expect_in_file "$TAG" 'memory.thread_heap_owner_inventory_box = "memory/thread_heap_owner_inventory_box.hako"' "$MODULE" "hako_alloc module must export M215 owner"
guard_expect_in_file "$TAG" 'box HakoAllocThreadHeapOwnerDecision' "$OWNER" "M215 decision box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocThreadHeapOwnerInventory' "$OWNER" "M215 inventory box must exist"
guard_expect_in_file "$TAG" 'classifyOwner' "$OWNER" "M215 inventory must expose classifyOwner"
guard_expect_in_file "$TAG" 'would_schedule_thread: i64 = 0' "$OWNER" "M215 scheduling must stay inactive"
guard_expect_in_file "$TAG" 'would_atomic_claim: i64 = 0' "$OWNER" "M215 atomic claim must stay inactive"
guard_expect_in_file "$TAG" 'would_drain_remote_free: i64 = 0' "$OWNER" "M215 remote-free drain must stay inactive"
guard_expect_in_file "$TAG" 'would_change_page_owner: i64 = 0' "$OWNER" "M215 owner mutation must stay inactive"
guard_expect_in_file "$TAG" 'would_execute_reclaim: i64 = 0' "$OWNER" "M215 reclaim execution must stay inactive"
guard_expect_in_file "$TAG" 'would_call_page_source: i64 = 0' "$OWNER" "M215 page-source calls must stay inactive"
guard_expect_in_file "$TAG" 'would_unreserve: i64 = 0' "$OWNER" "M215 unreserve must stay inactive"
guard_expect_in_file "$TAG" 'would_release_osvm: i64 = 0' "$OWNER" "M215 OS release must stay inactive"
guard_expect_in_file "$TAG" 'thread_heap_owner_inventory_box.hako` owns M215 thread heap owner-token inventory' "$MEMORY_README" "memory README must define M215 owner"
guard_expect_in_file "$TAG" 'HakoAllocThreadHeapOwnerInventory' "$APP" "M215 proof must construct owner inventory"
guard_expect_in_file "$TAG" 'check "m215 thread heap owner inventory"' "$APP" "M215 proof must use labelled check block"

if rg -n 'AtomicCoreBox|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(' \
  "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: M215 inventory must not schedule threads, use atomics, execute reclaim, or call page-source/OS release seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".provider_leak 2>&1; then
  echo "[$TAG] ERROR: M215 must not add env/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".provider_leak >&2
  rm -f /tmp/"$TAG".provider_leak
  exit 1
fi
rm -f /tmp/"$TAG".provider_leak

if rg -n 'hako-alloc-thread-heap-owner-inventory-proof|HakoAllocThreadHeapOwnerDecision|HakoAllocThreadHeapOwnerInventory|thread_heap_owner_inventory' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M215 app/inventory matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_thread_heap_owner_inventory_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M215 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m215_thread_owner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m215.mir.json"
exe_out="$tmp_dir/m215.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-thread-heap-owner-inventory-proof' "$vm_log"
rg -F -q 'unknown=0,1,0' "$vm_log"
rg -F -q 'same=0,2,1' "$vm_log"
rg -F -q 'active=0,3,1' "$vm_log"
rg -F -q 'remote=0,4,4' "$vm_log"
rg -F -q 'decommitted=0,5,1' "$vm_log"
rg -F -q 'candidate=1,0,1,1' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1,1,25,0' "$vm_log"
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
    "HakoAllocThreadHeapOwnerInventory.classifyOwner/6",
    "HakoAllocThreadHeapOwnerInventory.decision/12",
    "HakoAllocThreadHeapOwnerInventory.reject/10",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in ("HakoAllocThreadHeapOwnerDecision", "HakoAllocThreadHeapOwnerInventory"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fields = {field.get("name"): field for field in plans["HakoAllocThreadHeapOwnerDecision"].get("fields", [])}
required_fields = {
    "eligible",
    "reason",
    "page_id",
    "owner_thread_id",
    "observer_thread_id",
    "owner_known",
    "owner_active",
    "same_thread",
    "abandoned",
    "remote_free_pending",
    "decommitted",
    "adoption_candidate",
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
    raise SystemExit(f"missing owner fields: {missing_fields}")

for name in required_fields:
    field = fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad owner field {name}: {field}")

print("[m215-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-thread-heap-owner-inventory-proof' "$run_log"
rg -F -q 'unknown=0,1,0' "$run_log"
rg -F -q 'same=0,2,1' "$run_log"
rg -F -q 'active=0,3,1' "$run_log"
rg -F -q 'remote=0,4,4' "$run_log"
rg -F -q 'decommitted=0,5,1' "$run_log"
rg -F -q 'candidate=1,0,1,1' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0,0' "$run_log"
rg -F -q 'counts=6,1,5,1,1,1,1,1,1,25,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

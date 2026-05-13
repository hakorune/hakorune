#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-purge-policy-inventory"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-purge-policy-inventory-proof/main.hako"
APP_README="apps/hako-alloc-purge-policy-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-purge-policy-inventory-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-232-M192-PURGE-DECOMMIT-POLICY-INVENTORY.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
POLICY="lang/src/hako_alloc/memory/purge_policy_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
PAGE_SOURCE="lang/src/hako_alloc/memory/page_source_policy_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_purge_policy_inventory_guard.sh"

echo "[$TAG] checking M192 purge/decommit policy inventory"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$PHASE_README" \
  "$TASKBOARD" \
  "$INDEX" \
  "$POLICY" \
  "$MODULE" \
  "$MEMORY_README" \
  "$PAGE_SOURCE" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M192 card must be complete"
guard_expect_in_file "$TAG" 'M192 status:' "$PLAN" "mimalloc plan must record M192 status"
guard_expect_in_file "$TAG" '`293x-232`' "$PHASE_README" "phase README must list M192 row"
guard_expect_in_file "$TAG" '\[x\] `293x-232`' "$TASKBOARD" "taskboard must mark M192 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M192 guard"

guard_expect_in_file "$TAG" 'memory.purge_policy_box = "memory/purge_policy_box.hako"' "$MODULE" "hako_alloc module must export purge_policy_box"
guard_expect_in_file "$TAG" 'box HakoAllocPurgeDecision' "$POLICY" "purge decision box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocPurgePolicyInventory' "$POLICY" "purge policy inventory box must exist"
guard_expect_in_file "$TAG" 'classifyLocalPage' "$POLICY" "purge policy must expose classifyLocalPage"
guard_expect_in_file "$TAG" 'would_decommit: i64 = 0' "$POLICY" "M192 must keep decommit execution inactive"
guard_expect_in_file "$TAG" 'would_unreserve: i64 = 0' "$POLICY" "M192 must keep unreserve execution inactive"
guard_expect_in_file "$TAG" 'would_release_osvm: i64 = 0' "$POLICY" "M192 must keep OS release inactive"
guard_expect_in_file "$TAG" 'purge_policy_box.hako` owns M192 purge/decommit policy inventory' "$MEMORY_README" "memory README must define purge owner"

if rg -n 'using selfhost\.hako_alloc\.memory\.page_source_policy_box|HakoAllocPageSourcePolicy\.|OsVmCoreBox\.|decommitPage[[:space:]]*\(|pageSourceDecommit|reservePage[[:space:]]*\(|commitPage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(' \
  "$POLICY" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: M192 policy inventory must not call page source or OS release surfaces" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-purge-policy-inventory-proof|HakoAllocPurgeDecision|HakoAllocPurgePolicyInventory|classifyLocalPage' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M192 app/policy matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_purge_policy_inventory_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M192 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m192_hako_alloc_purge.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m192.mir.json"
exe_out="$tmp_dir/m192.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-purge-policy-inventory-proof' "$vm_log"
rg -F -q 'missing=0,1,1,0' "$vm_log"
rg -F -q 'live=0,2,0,1' "$vm_log"
rg -F -q 'not_retired=0,3,1,0' "$vm_log"
rg -F -q 'ready=1,0,1,1,1,4096' "$vm_log"
rg -F -q 'would=0,0,0' "$vm_log"
rg -F -q 'counts=1,3,1,1,1,1' "$vm_log"
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
    "HakoAllocPurgePolicyInventory.classifyLocalPage/3",
    "HakoAllocPurgePolicyInventory.decision/6",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in ("HakoAllocPurgeDecision", "HakoAllocPurgePolicyInventory"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

decision_fields = {
    field.get("name"): field
    for field in plans["HakoAllocPurgeDecision"].get("fields", [])
}
required_fields = {
    "eligible",
    "reason",
    "page_empty",
    "retired",
    "has_backing",
    "committed_bytes",
    "would_decommit",
    "would_unreserve",
    "would_release_osvm",
}
missing_fields = sorted(name for name in required_fields if name not in decision_fields)
if missing_fields:
    raise SystemExit(f"missing purge decision fields: {missing_fields}")

print("[m192-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-purge-policy-inventory-proof' "$run_log"
rg -F -q 'missing=0,1,1,0' "$run_log"
rg -F -q 'live=0,2,0,1' "$run_log"
rg -F -q 'not_retired=0,3,1,0' "$run_log"
rg -F -q 'ready=1,0,1,1,1,4096' "$run_log"
rg -F -q 'would=0,0,0' "$run_log"
rg -F -q 'counts=1,3,1,1,1,1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-options-inventory"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-options-inventory-proof/main.hako"
APP_README="apps/hako-alloc-options-inventory-proof/README.md"
APP_TEST="apps/hako-alloc-options-inventory-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-266-M214-ALLOCATOR-OPTIONS-DEFAULTS-INVENTORY.md"
DESIGN="docs/development/current/main/design/hako-alloc-options-inventory-ssot.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/options_inventory_box.hako"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_options_inventory_guard.sh"

printf '[%s] checking M214 allocator options/defaults inventory\n' "$TAG"

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
  "$CURRENT_STATE" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M214 card must be complete"
guard_expect_in_file "$TAG" 'Decision: accepted' "$DESIGN" "M214 design must be accepted"
guard_expect_in_file "$TAG" 'M214 status:' "$PLAN" "mimalloc plan must record M214 status"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M214 guard"
guard_expect_in_file "$TAG" 'id = "M214"' "$PROOF_MANIFEST" "proof app manifest must list M214"
guard_expect_in_file "$TAG" 'latest_card = "293x-266-M214-ALLOCATOR-OPTIONS-DEFAULTS-INVENTORY"' "$CURRENT_STATE" "current state must point at M214 as latest card"
guard_expect_in_file "$TAG" 'current_blocker_token = "D208 mimalloc migration closeout check"' "$CURRENT_STATE" "current state must advance to D208"
guard_expect_in_file "$TAG" 'memory.options_inventory_box = "memory/options_inventory_box.hako"' "$MODULE" "hako_alloc module must export M214 owner"
guard_expect_in_file "$TAG" 'box HakoAllocOptionDefaultFact' "$OWNER" "M214 option fact box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocOptionsInventorySnapshot' "$OWNER" "M214 snapshot box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocOptionsInventory' "$OWNER" "M214 inventory box must exist"
guard_expect_in_file "$TAG" 'describeOption' "$OWNER" "M214 inventory must expose describeOption"
guard_expect_in_file "$TAG" 'mutable_options_enabled: i64 = 0' "$OWNER" "M214 mutable options must stay inactive"
guard_expect_in_file "$TAG" 'env_toggles_added: i64 = 0' "$OWNER" "M214 env toggles must stay inactive"
guard_expect_in_file "$TAG" 'would_change_allocation_policy: i64 = 0' "$OWNER" "M214 allocation policy changes must stay inactive"
guard_expect_in_file "$TAG" 'would_select_provider: i64 = 0' "$OWNER" "M214 provider selection must stay inactive"
guard_expect_in_file "$TAG" 'would_install_hook: i64 = 0' "$OWNER" "M214 hook install must stay inactive"
guard_expect_in_file "$TAG" 'would_replace_process_allocator: i64 = 0' "$OWNER" "M214 allocator replacement must stay inactive"
guard_expect_in_file "$TAG" 'would_execute_reclaim: i64 = 0' "$OWNER" "M214 reclaim execution must stay inactive"
guard_expect_in_file "$TAG" 'options_inventory_box.hako` owns M214 allocator options/defaults inventory' "$MEMORY_README" "memory README must define M214 owner"
guard_expect_in_file "$TAG" 'HakoAllocOptionsInventory' "$APP" "M214 proof must construct options inventory"
guard_expect_in_file "$TAG" 'check "m214 allocator options defaults inventory"' "$APP" "M214 proof must use labelled check block"

if rg -n 'NYASH_|HAKO_|std::env|env::|getenv|readFile|openFile|global_allocator|GlobalAlloc|AllocatorProviderRegistry|selectProvider' \
  "$OWNER" "$APP" "$APP_README" >/tmp/"$TAG".option_leak 2>&1; then
  echo "[$TAG] ERROR: M214 must not add env/config/provider/hook/replacement behavior" >&2
  cat /tmp/"$TAG".option_leak >&2
  rm -f /tmp/"$TAG".option_leak
  exit 1
fi
rm -f /tmp/"$TAG".option_leak

if rg -n 'AtomicCoreBox|cas_i64|fetch_add|spawn[[:space:]]*\(|thread::|observeHeapPage[[:space:]]*\(|selectHeapPage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(' \
  "$OWNER" >/tmp/"$TAG".execution_leak 2>&1; then
  echo "[$TAG] ERROR: M214 inventory must not schedule threads, use atomics, execute purge/reclaim, or call page-source/OS release seams" >&2
  cat /tmp/"$TAG".execution_leak >&2
  rm -f /tmp/"$TAG".execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".execution_leak

if rg -n 'hako-alloc-options-inventory-proof|HakoAllocOptionDefaultFact|HakoAllocOptionsInventory|options_inventory' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M214 app/inventory matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_options_inventory_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M214 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m214_hako_alloc_options.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m214.mir.json"
exe_out="$tmp_dir/m214.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-options-inventory-proof' "$vm_log"
rg -F -q 'known=1,1,1,1,1' "$vm_log"
rg -F -q 'defaults=0,1,4096,0,0' "$vm_log"
rg -F -q 'unknown=0,99,-1' "$vm_log"
rg -F -q 'snapshot=1,6,5,1,99' "$vm_log"
rg -F -q 'inactive=0,0,0,0,0,0,0' "$vm_log"
rg -F -q 'fact_inactive=0,0,0,0,0,0,0' "$vm_log"
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
    "HakoAllocOptionsInventory.describeOption/1",
    "HakoAllocOptionsInventory.fact/5",
    "HakoAllocOptionsInventory.snapshot/0",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for name in ("HakoAllocOptionDefaultFact", "HakoAllocOptionsInventorySnapshot", "HakoAllocOptionsInventory"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

fact_fields = {field.get("name"): field for field in plans["HakoAllocOptionDefaultFact"].get("fields", [])}
required_fields = {
    "known",
    "option_id",
    "default_value",
    "default_is_static",
    "default_source",
    "requires_backend_support",
    "affects_allocation_policy",
    "mutable_options_enabled",
    "env_toggles_added",
    "would_change_allocation_policy",
    "would_select_provider",
    "would_install_hook",
    "would_replace_process_allocator",
    "would_execute_reclaim",
}
missing_fields = sorted(name for name in required_fields if name not in fact_fields)
if missing_fields:
    raise SystemExit(f"missing option fields: {missing_fields}")

for name in required_fields:
    field = fact_fields.get(name)
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad option field {name}: {field}")

print("[m214-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-options-inventory-proof' "$run_log"
rg -F -q 'known=1,1,1,1,1' "$run_log"
rg -F -q 'defaults=0,1,4096,0,0' "$run_log"
rg -F -q 'unknown=0,99,-1' "$run_log"
rg -F -q 'snapshot=1,6,5,1,99' "$run_log"
rg -F -q 'inactive=0,0,0,0,0,0,0' "$run_log"
rg -F -q 'fact_inactive=0,0,0,0,0,0,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

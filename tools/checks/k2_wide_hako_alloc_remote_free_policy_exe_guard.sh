#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-remote-free-policy-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-remote-free-policy-proof/main.hako"
APP_README="apps/hako-alloc-remote-free-policy-proof/README.md"
FACADE="lang/src/hako_alloc/memory/allocator_facade_box.hako"
REMOTE_POLICY="lang/src/hako_alloc/memory/remote_free_policy_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-100-M48-ALLOCATOR-REMOTE-FREE-POLICY-PROOF.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M48 hako_alloc remote-free policy EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$FACADE" "$REMOTE_POLICY" "$CARD" "$TASKBOARD"

if rg -n 'hako-alloc-remote-free-policy-proof|HakoAllocProductionFacade|HakoAllocRemoteFreePolicy|AllocatorRemoteFreePolicy' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: hako_alloc remote-free policy matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".inactive_pointer_rows 2>&1; then
  echo "[$TAG] ERROR: pointer atomic fetch_add rows must stay inactive in M48" >&2
  cat /tmp/"$TAG".inactive_pointer_rows >&2
  rm -f /tmp/"$TAG".inactive_pointer_rows
  exit 1
fi
rm -f /tmp/"$TAG".inactive_pointer_rows

rg -F -q 'using selfhost.hako_alloc.memory.allocator_facade_box as HakoAllocFacade' "$APP"
rg -F -q 'box HakoAllocProductionFacade' "$FACADE"
rg -F -q 'remotePushRetry' "$FACADE"
rg -F -q 'static box HakoAllocRemoteFreePolicy' "$REMOTE_POLICY"

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m48_hako_alloc_remote_free.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m48.mir.json"
exe_out="$tmp_dir/m48.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required_functions = {
    "main",
    "HakoAllocProductionFacade.allocate/1",
    "HakoAllocProductionFacade.release/1",
    "HakoAllocProductionFacade.remoteInitHead/1",
    "HakoAllocProductionFacade.remotePushRetry/3",
    "HakoAllocProductionFacade.remotePeekHead/1",
    "HakoAllocProductionFacade.remotePeekNext/1",
    "HakoAllocRemoteFreePolicy.initHead/1",
    "HakoAllocRemoteFreePolicy.pushRetry/3",
    "HakoAllocRemoteFreePolicy.peekHead/1",
    "HakoAllocRemoteFreePolicy.peekNext/1",
}
missing = sorted(name for name in required_functions if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
if plans.get("HakoAllocProductionFacade") is None:
    raise SystemExit("missing facade typed object plan")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def require_method(fn, box_name, name):
    for callee in iter_calls(fn):
        if (
            callee.get("type") == "Method"
            and callee.get("box_name") == box_name
            and callee.get("name") == name
        ):
            return
    raise SystemExit(f"missing method call {box_name}.{name} in {fn.get('name')}")

main = functions["main"]
for name in (
    "allocate",
    "release",
    "remoteInitHead",
    "remotePushRetry",
    "remotePeekHead",
    "remotePeekNext",
):
    require_method(main, "HakoAllocProductionFacade", name)

def require_global(owner, owner_name, symbol):
    routes = owner.get("metadata", {}).get("global_call_routes", [])
    for route in routes:
        if (
            route.get("symbol") == symbol
            and route.get("target_shape") == "generic_i64_body"
            and route.get("proof") == "typed_global_call_generic_i64"
            and route.get("return_shape") == "ScalarI64"
        ):
            return
    raise SystemExit(f"missing generic-i64 route in {owner_name} -> {symbol}: {routes}")

for owner_name, symbol in (
    ("HakoAllocProductionFacade.remoteInitHead/1", "HakoAllocRemoteFreePolicy.initHead/1"),
    ("HakoAllocProductionFacade.remotePushRetry/3", "HakoAllocRemoteFreePolicy.pushRetry/3"),
    ("HakoAllocProductionFacade.remotePeekHead/1", "HakoAllocRemoteFreePolicy.peekHead/1"),
    ("HakoAllocProductionFacade.remotePeekNext/1", "HakoAllocRemoteFreePolicy.peekNext/1"),
):
    require_global(functions[owner_name], owner_name, symbol)

def require_extern(owner, owner_name, symbol, route_id, core_op, arity, ret, demand, effects):
    routes = owner.get("metadata", {}).get("extern_call_routes", [])
    for route in routes:
        if (
            route.get("route_id") == route_id
            and route.get("core_op") == core_op
            and route.get("symbol") == symbol
            and route.get("return_shape") == ret
            and route.get("value_demand") == demand
            and route.get("effects") == effects
        ):
            break
    else:
        raise SystemExit(f"missing extern route in {owner_name} for {symbol}: {routes}")

    plans = owner.get("metadata", {}).get("lowering_plan", [])
    for plan in plans:
        if (
            plan.get("source") == "extern_call_routes"
            and plan.get("source_route_id") == route_id
            and plan.get("arity") == arity
            and plan.get("symbol") == symbol
        ):
            return
    raise SystemExit(f"missing lowering plan in {owner_name} for {symbol}: {plans}")

for symbol, route_id, core_op, arity, ret, demand, effects in (
    (
        "hako_mem_alloc",
        "extern.hako_mem.alloc",
        "HakoMemAlloc",
        1,
        "native_ptr_nullable",
        "native_ptr_nullable",
        ["hako.mem.alloc"],
    ),
    (
        "hako_mem_free",
        "extern.hako_mem.free",
        "HakoMemFree",
        1,
        "void_sentinel_i64_zero",
        "scalar_i64",
        ["hako.mem.free"],
    ),
):
    require_extern(main, "main", symbol, route_id, core_op, arity, ret, demand, effects)

require_extern(
    functions["HakoAllocRemoteFreePolicy.initHead/1"],
    "HakoAllocRemoteFreePolicy.initHead/1",
    "hako_atomic_ptr_store_ordered",
    "extern.hako_atomic.ptr_store_ordered",
    "HakoAtomicPtrStoreOrdered",
    3,
    "scalar_i64",
    "native_ptr_nullable",
    ["hako.atomic.ptr_store"],
)

for owner_name in (
    "HakoAllocRemoteFreePolicy.pushRetry/3",
    "HakoAllocRemoteFreePolicy.peekHead/1",
    "HakoAllocRemoteFreePolicy.peekNext/1",
):
    require_extern(
        functions[owner_name],
        owner_name,
        "hako_atomic_ptr_load_ordered",
        "extern.hako_atomic.ptr_load_ordered",
        "HakoAtomicPtrLoadOrdered",
        2,
        "native_ptr_nullable",
        "native_ptr_nullable",
        ["hako.atomic.ptr_load"],
    )

require_extern(
    functions["HakoAllocRemoteFreePolicy.pushRetry/3"],
    "HakoAllocRemoteFreePolicy.pushRetry/3",
    "hako_atomic_ptr_store_ordered",
    "extern.hako_atomic.ptr_store_ordered",
    "HakoAtomicPtrStoreOrdered",
    3,
    "scalar_i64",
    "native_ptr_nullable",
    ["hako.atomic.ptr_store"],
)
require_extern(
    functions["HakoAllocRemoteFreePolicy.pushRetry/3"],
    "HakoAllocRemoteFreePolicy.pushRetry/3",
    "hako_atomic_ptr_cas_ordered",
    "extern.hako_atomic.ptr_cas_ordered",
    "HakoAtomicPtrCasOrdered",
    5,
    "native_ptr_nullable",
    "native_ptr_nullable",
    ["hako.atomic.ptr_cas"],
)

print("[m48-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_store_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_load_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_cas_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-remote-free-policy-proof' "$run_log"
rg -F -q 'facade=1,1,0' "$run_log"
rg -F -q 'init=0' "$run_log"
rg -F -q 'retries=0,1' "$run_log"
rg -F -q 'shape=1,1,1,1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '| `M48 allocator remote-free policy proof` | `live-narrow` |' "$TASKBOARD"
rg -F -q 'M48 Allocator Remote-Free Policy Proof' "$CARD"
rg -F -q 'HakoAllocProductionFacade' "$APP_README"
rg -F -q 'HakoAllocRemoteFreePolicy' "$APP_README"
rg -F -q 'k2_wide_hako_alloc_remote_free_policy_exe_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"

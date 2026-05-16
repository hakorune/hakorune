#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-page-source-policy-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-page-source-policy-proof/main.hako"
APP_README="apps/hako-alloc-page-source-policy-proof/README.md"
FACADE="lang/src/hako_alloc/memory/allocator_facade_box.hako"
PAGE_SOURCE_POLICY="lang/src/hako_alloc/memory/page_source_policy_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-101-M49-ALLOCATOR-OSVM-PAGE-SOURCE-PROOF.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M49 hako_alloc page-source policy EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$FACADE" "$PAGE_SOURCE_POLICY" "$CARD" "$TASKBOARD"

if rg -n 'hako-alloc-page-source-policy-proof|HakoAllocProductionFacade|HakoAllocPageSourcePolicy|AllocatorPageSourcePolicy' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: hako_alloc page-source policy matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

if rg -n 'unreservePage|unreserve_bytes_i64|hako_osvm_unreserve' \
  "$APP" >/tmp/"$TAG".inactive_osvm_rows 2>&1; then
  echo "[$TAG] ERROR: M49 proof app must stay reserve/commit/decommit-only" >&2
  cat /tmp/"$TAG".inactive_osvm_rows >&2
  rm -f /tmp/"$TAG".inactive_osvm_rows
  exit 1
fi
rm -f /tmp/"$TAG".inactive_osvm_rows

rg -F -q 'using selfhost.hako_alloc.memory.allocator_facade_box as HakoAllocFacade' "$APP"
rg -F -q 'box HakoAllocProductionFacade' "$FACADE"
rg -F -q 'pageSourceReserve' "$FACADE"
rg -F -q 'static box HakoAllocPageSourcePolicy' "$PAGE_SOURCE_POLICY"

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m49_hako_alloc_page_source.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m49.mir.json"
exe_out="$tmp_dir/m49.exe"
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
    "HakoAllocProductionFacade.pageSourceReserve/1",
    "HakoAllocProductionFacade.pageSourceCommit/2",
    "HakoAllocProductionFacade.pageSourceDecommit/2",
    "HakoAllocPageSourcePolicy.reservePage/1",
    "HakoAllocPageSourcePolicy.commitPage/2",
    "HakoAllocPageSourcePolicy.decommitPage/2",
    "OsVmCoreBox.reserve_bytes_i64/1",
    "OsVmCoreBox.commit_bytes_i64/2",
    "OsVmCoreBox.decommit_bytes_i64/2",
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
for name in ("pageSourceReserve", "pageSourceCommit", "pageSourceDecommit"):
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
    ("HakoAllocProductionFacade.pageSourceReserve/1", "HakoAllocPageSourcePolicy.reservePage/1"),
    ("HakoAllocProductionFacade.pageSourceCommit/2", "HakoAllocPageSourcePolicy.commitPage/2"),
    ("HakoAllocProductionFacade.pageSourceDecommit/2", "HakoAllocPageSourcePolicy.decommitPage/2"),
    ("HakoAllocPageSourcePolicy.reservePage/1", "OsVmCoreBox.reserve_bytes_i64/1"),
    ("HakoAllocPageSourcePolicy.commitPage/2", "OsVmCoreBox.commit_bytes_i64/2"),
    ("HakoAllocPageSourcePolicy.decommitPage/2", "OsVmCoreBox.decommit_bytes_i64/2"),
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

for fn_name, route_id, core_op, symbol, arity, ret, demand, effects in (
    (
        "OsVmCoreBox.reserve_bytes_i64/1",
        "extern.hako_osvm.reserve_bytes_i64",
        "HakoOsvmReserveBytesI64",
        "hako_osvm_reserve_bytes_i64",
        1,
        "native_ptr_nullable",
        "native_ptr_nullable",
        ["hako.osvm.reserve"],
    ),
    (
        "OsVmCoreBox.commit_bytes_i64/2",
        "extern.hako_osvm.commit_bytes_i64",
        "HakoOsvmCommitBytesI64",
        "hako_osvm_commit_bytes_i64",
        2,
        "scalar_i64",
        "runtime_i64",
        ["hako.osvm.commit"],
    ),
    (
        "OsVmCoreBox.decommit_bytes_i64/2",
        "extern.hako_osvm.decommit_bytes_i64",
        "HakoOsvmDecommitBytesI64",
        "hako_osvm_decommit_bytes_i64",
        2,
        "scalar_i64",
        "runtime_i64",
        ["hako.osvm.decommit"],
    ),
):
    require_extern(functions[fn_name], fn_name, symbol, route_id, core_op, arity, ret, demand, effects)

print("[m49-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-page-source-policy-proof' "$run_log"
rg -F -q 'page=4096 reserved=1' "$run_log"
rg -F -q 'commit=0 decommit=0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '| `M49 allocator OSVM page-source proof` | `live-narrow` |' "$TASKBOARD"
rg -F -q 'M49 Allocator OSVM Page-Source Proof' "$CARD"
rg -F -q 'HakoAllocProductionFacade' "$APP_README"
rg -F -q 'HakoAllocPageSourcePolicy' "$APP_README"
rg -F -q 'k2_wide_hako_alloc_page_source_policy_exe_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"

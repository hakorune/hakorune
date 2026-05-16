#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-page-source-unreserve-adapter"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-page-source-unreserve-adapter-proof/main.hako"
APP_README="apps/hako-alloc-page-source-unreserve-adapter-proof/README.md"
APP_TEST="apps/hako-alloc-page-source-unreserve-adapter-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-459-MIMAP-033A-PAGE-SOURCE-UNRESERVE-ADAPTER.md"
INDEX="docs/tools/check-scripts-index.md"
POLICY="lang/src/hako_alloc/memory/page_source_policy_box.hako"
ADAPTER="lang/src/hako_alloc/memory/purge_page_source_unreserve_adapter_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_page_source_unreserve_adapter_guard.sh"

echo "[$TAG] running MIMAP-033A page-source unreserve adapter guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$INDEX" \
  "$POLICY" \
  "$ADAPTER" \
  "$MODULE" \
  "$MEMORY_README" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: landed' "$CARD" "MIMAP-033A card must be landed before the guard is green"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-033A guard"
guard_expect_in_file "$TAG" 'memory.purge_page_source_unreserve_adapter_box = "memory/purge_page_source_unreserve_adapter_box.hako"' "$MODULE" "hako_alloc module must export page-source unreserve adapter"
guard_expect_in_file "$TAG" 'unreservePage\(base, bytes\)' "$POLICY" "page-source policy must own unreservePage"
guard_expect_in_file "$TAG" 'OsVmCoreBox\.unreserve_bytes_i64\(base, bytes\)' "$POLICY" "page-source policy must delegate to OsVmCoreBox unreserve"
guard_expect_in_file "$TAG" 'box HakoAllocPageSourceUnreserveAdapter' "$ADAPTER" "page-source unreserve adapter box must exist"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy\.unreservePage\(base, bytes\)' "$ADAPTER" "adapter must delegate only to page-source unreserve"
guard_expect_in_file "$TAG" 'purge_page_source_unreserve_adapter_box.hako` owns MIMAP-033A page-source' "$MEMORY_README" "memory README must define MIMAP-033A owner"

if rg -n 'HakoAllocPageSourcePolicy\.(reservePage|commitPage|decommitPage)|hako_osvm_|recommitPage|provider[[:space:]]*\(|global_allocator' \
  "$ADAPTER" >/tmp/"$TAG".forbidden_adapter 2>&1; then
  echo "[$TAG] ERROR: MIMAP-033A adapter must not reserve, commit, decommit, recommit, or call OSVM/provider directly" >&2
  cat /tmp/"$TAG".forbidden_adapter >&2
  rm -f /tmp/"$TAG".forbidden_adapter
  exit 1
fi
rm -f /tmp/"$TAG".forbidden_adapter

if rg -n 'OsVmCoreBox|hako_osvm_|HakoAllocProductionFacade|objectLifecycle|Huge' \
  "$APP" >/tmp/"$TAG".forbidden_app 2>&1; then
  echo "[$TAG] ERROR: proof app must go through page-source policy / adapter, not direct OSVM or facade huge owners" >&2
  cat /tmp/"$TAG".forbidden_app >&2
  rm -f /tmp/"$TAG".forbidden_app
  exit 1
fi
rm -f /tmp/"$TAG".forbidden_app

if rg -n 'HakoAllocPageSourceUnreserveAdapter|hako-alloc-page-source-unreserve-adapter-proof' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-033A app/adapter matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'HakoAllocPageSourceUnreserveAdapter|unreservePage[[:space:]]*\(' \
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_*.hako >/tmp/"$TAG".facade_leak 2>&1; then
  echo "[$TAG] ERROR: facade huge owners must not adopt unreserve in MIMAP-033A" >&2
  cat /tmp/"$TAG".facade_leak >&2
  rm -f /tmp/"$TAG".facade_leak
  exit 1
fi
rm -f /tmp/"$TAG".facade_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap033a_page_source_unreserve.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap033a.mir.json"
exe_out="$tmp_dir/mimap033a.exe"
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
required = {
    "main",
    "HakoAllocPageSourcePolicy.reservePage/1",
    "HakoAllocPageSourcePolicy.commitPage/2",
    "HakoAllocPageSourcePolicy.decommitPage/2",
    "HakoAllocPageSourcePolicy.unreservePage/2",
    "HakoAllocPageSourceUnreserveAdapter.unreservePage/2",
    "OsVmCoreBox.reserve_bytes_i64/1",
    "OsVmCoreBox.commit_bytes_i64/2",
    "OsVmCoreBox.decommit_bytes_i64/2",
    "OsVmCoreBox.unreserve_bytes_i64/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
if plans.get("HakoAllocPageSourceUnreserveAdapter") is None:
    raise SystemExit("missing typed object plan: HakoAllocPageSourceUnreserveAdapter")

def require_global(owner_name, symbol):
    owner = functions[owner_name]
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
    ("main", "HakoAllocPageSourcePolicy.reservePage/1"),
    ("main", "HakoAllocPageSourcePolicy.commitPage/2"),
    ("main", "HakoAllocPageSourcePolicy.decommitPage/2"),
    ("HakoAllocPageSourceUnreserveAdapter.unreservePage/2", "HakoAllocPageSourcePolicy.unreservePage/2"),
    ("HakoAllocPageSourcePolicy.reservePage/1", "OsVmCoreBox.reserve_bytes_i64/1"),
    ("HakoAllocPageSourcePolicy.commitPage/2", "OsVmCoreBox.commit_bytes_i64/2"),
    ("HakoAllocPageSourcePolicy.decommitPage/2", "OsVmCoreBox.decommit_bytes_i64/2"),
    ("HakoAllocPageSourcePolicy.unreservePage/2", "OsVmCoreBox.unreserve_bytes_i64/2"),
):
    require_global(owner_name, symbol)

def require_extern(owner_name, route_id, core_op, symbol, arity, ret, demand, effects):
    owner = functions[owner_name]
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
    (
        "OsVmCoreBox.unreserve_bytes_i64/2",
        "extern.hako_osvm.unreserve_bytes_i64",
        "HakoOsvmUnreserveBytesI64",
        "hako_osvm_unreserve_bytes_i64",
        2,
        "scalar_i64",
        "runtime_i64",
        ["hako.osvm.unreserve"],
    ),
):
    require_extern(fn_name, route_id, core_op, symbol, arity, ret, demand, effects)

print("[mimap033a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_unreserve_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-page-source-unreserve-adapter-proof' "$run_log"
rg -F -q 'page=4096 reserved=1' "$run_log"
rg -F -q 'route=0,0,0' "$run_log"
rg -F -q 'adapter=1,1,0,0,4096' "$run_log"
rg -F -q 'stop=facade=0,provider=0,replacement=0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

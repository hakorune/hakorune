#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-ptr-atomic-load-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-ptr-atomic-load-proof/main.hako"
APP_README="apps/mimalloc-ptr-atomic-load-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-091-M39-NATIVE-PTR-ATOMIC-LOAD-ROUTE-PROOF.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M39 native pointer atomic load EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$CARD" "$TASKBOARD"

if rg -n 'mimalloc-ptr-atomic-load-proof' lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: app-specific pointer atomic load matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

if rg -n 'hako_atomic_ptr_cas_ordered|HakoAtomicPtrCasOrdered|extern\\.hako_atomic\\.ptr_cas_ordered|ptr_fetch_add' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".inactive_pointer_rows 2>&1; then
  echo "[$TAG] ERROR: pointer atomic CAS/fetch_add rows must stay inactive in M39" >&2
  cat /tmp/"$TAG".inactive_pointer_rows >&2
  rm -f /tmp/"$TAG".inactive_pointer_rows
  exit 1
fi
rm -f /tmp/"$TAG".inactive_pointer_rows

cargo test -q refresh_function_extern_call_routes_records_hako_atomic_ptr_load_ordered_route -- --nocapture
cargo test -q refresh_function_extern_call_routes_records_hako_atomic_ptr_store_ordered_route -- --nocapture
cargo test -q -p nyash_kernel atomic -- --nocapture
pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m39_ptr_atomic_load.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m39.mir.json"
exe_out="$tmp_dir/m39.exe"
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
main = functions.get("main")
if main is None:
    raise SystemExit("missing main")

routes = main.get("metadata", {}).get("extern_call_routes", [])
plans = main.get("metadata", {}).get("lowering_plan", [])

expected_routes = {
    "hako_mem_alloc": (
        "extern.hako_mem.alloc",
        "HakoMemAlloc",
        1,
        "native_ptr_nullable",
        "native_ptr_nullable",
        ["hako.mem.alloc"],
    ),
    "hako_atomic_ptr_store_ordered": (
        "extern.hako_atomic.ptr_store_ordered",
        "HakoAtomicPtrStoreOrdered",
        3,
        "scalar_i64",
        "native_ptr_nullable",
        ["hako.atomic.ptr_store"],
    ),
    "hako_atomic_ptr_load_ordered": (
        "extern.hako_atomic.ptr_load_ordered",
        "HakoAtomicPtrLoadOrdered",
        2,
        "native_ptr_nullable",
        "native_ptr_nullable",
        ["hako.atomic.ptr_load"],
    ),
    "hako_mem_free": (
        "extern.hako_mem.free",
        "HakoMemFree",
        1,
        "void_sentinel_i64_zero",
        "scalar_i64",
        ["hako.mem.free"],
    ),
}

for symbol, (route_id, core_op, arity, ret, demand, effects) in expected_routes.items():
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
        raise SystemExit(f"missing extern route for {symbol}: {routes}")

    for plan in plans:
        if (
            plan.get("source") == "extern_call_routes"
            and plan.get("source_route_id") == route_id
            and plan.get("arity") == arity
            and plan.get("symbol") == symbol
        ):
            break
    else:
        raise SystemExit(f"missing lowering plan for {symbol}: {plans}")

print("[m39-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_store_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_load_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-ptr-atomic-load-proof' "$run_log"
rg -F -q 'store=0' "$run_log"
rg -F -q 'loaded_matches=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '| `M39 native pointer atomic load route proof` | `live-narrow` |' "$TASKBOARD"
rg -F -q 'M39 Native Ptr Atomic Load Route Proof' "$CARD"
rg -F -q 'hako_atomic_ptr_load_ordered' "$APP_README"
rg -F -q 'k2_wide_mimalloc_ptr_atomic_load_exe_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"

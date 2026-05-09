#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-rawarray-slot-store-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/rawarray-slot-store-exe-proof/main.hako"
APP_README="apps/rawarray-slot-store-exe-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-071-M19-RAWARRAY-SLOT-STORE-GENERIC-I64-ROUTE.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M19 RawArray slot_store_i64 EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$CARD" "$TASKBOARD"

cargo test -q generic_i64_body_accepts_array_slot_store_extern_route -- --nocapture
pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m19_rawarray.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m19.mir.json"
exe_out="$tmp_dir/m19.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {f.get("name"): f for f in data.get("functions", [])}

def require_function(name):
    fn = functions.get(name)
    if fn is None:
        raise SystemExit(f"missing function: {name}")
    return fn

def require_global_route(function_name, symbol):
    fn = require_function(function_name)
    routes = fn.get("metadata", {}).get("global_call_routes", [])
    for route in routes:
        if route.get("symbol") != symbol:
            continue
        if (
            route.get("target_shape") == "generic_i64_body"
            and route.get("proof") == "typed_global_call_generic_i64"
            and route.get("return_shape") == "ScalarI64"
        ):
            return route
    raise SystemExit(f"missing generic-i64 route {function_name} -> {symbol}: {routes}")

require_global_route("main", "RawArrayCoreBox.slot_store_i64/3")
require_global_route("main", "RawArrayCoreBox.slot_load_i64/2")
require_global_route("RawArrayCoreBox.slot_store_i64/3", "OwnershipCoreBox.ensure_handle_writable_i64/1")
require_global_route("RawArrayCoreBox.slot_store_i64/3", "BoundsCoreBox.ensure_index_i64/2")
require_global_route("RawArrayCoreBox.slot_store_i64/3", "PtrCoreBox.slot_store_i64/3")

ptr_store = require_function("PtrCoreBox.slot_store_i64/3")
ptr_routes = ptr_store.get("metadata", {}).get("extern_call_routes", [])
if not any(
    route.get("route_id") == "extern.array.slot_store_i64"
    and route.get("symbol") == "nyash.array.slot_store_hii"
    and route.get("return_shape") == "scalar_i64"
    for route in ptr_routes
):
    raise SystemExit(f"missing ArraySlotStoreI64 extern route: {ptr_routes}")

print("[m19-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_array_slot_store_i64_emit' "$build_log"
rg -F -q 'mir_call_array_slot_load_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'rawarray-slot-store-exe-proof' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '`M19 RawArray slot_store_i64 generic-i64 route`' "$TASKBOARD"
rg -F -q 'M19 RawArray slot_store_i64 generic-i64 route' "$CARD"
rg -F -q 'No RawArray store-handle/string parity' "$APP_README"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-raw-page-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-raw-page-proof/main.hako"
APP_README="apps/mimalloc-raw-page-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-072-M20-MIMALLOC-RAW-PAGE-EXE-PARITY-GUARD.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M20 mimalloc raw-page EXE parity guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$CARD" "$TASKBOARD"

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m20_raw_page.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/raw_page.mir.json"
exe_out="$tmp_dir/raw_page.exe"
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

require_global_route("MiRawPageProof.acquireBlock/1", "RawArrayCoreBox.slot_load_i64/2")
require_global_route("MiRawPageProof.releaseBlock/1", "RawArrayCoreBox.slot_store_i64/3")
require_global_route("MiRawPageProof.birth/1", "RawArrayCoreBox.slot_append_any/2")

print("[m20-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"
rg -F -q 'mir_call_array_slot_append_any_emit' "$build_log"
rg -F -q 'mir_call_array_slot_len_i64_emit' "$build_log"
rg -F -q 'mir_call_array_slot_load_i64_emit' "$build_log"
rg -F -q 'mir_call_array_slot_store_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-raw-page-proof' "$run_log"
rg -F -q 'allocs=6 frees=2 reused=2 peak=4 free=0' "$run_log"
rg -F -q 'rejects=2' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '`M20 mimalloc raw-page EXE parity guard`' "$TASKBOARD"
rg -F -q 'M20 mimalloc raw-page EXE parity guard' "$CARD"
rg -F -q 'M12 proof fixture' "$APP_README"

echo "[$TAG] ok"

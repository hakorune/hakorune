#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-two-class-page-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-two-class-page-proof/main.hako"
APP_README="apps/mimalloc-two-class-page-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-074-M22-MIMALLOC-TWO-CLASS-PAGE-EXE-PROOF.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M22 mimalloc two-class page EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$CARD" "$TASKBOARD"

if rg -n 'MI_SIZE_CLASS|MI_CLASS_CAP|MiTwoClassPageProof|mimalloc-two-class-page' lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: app-specific two-class page matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

cargo test -q static_const_table_load
pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m22_two_class.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m22.mir.json"
exe_out="$tmp_dir/m22.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

plans = {
    plan.get("source_name"): plan
    for plan in data.get("static_data_plans", [])
}
expected = {
    "MI_SIZE_CLASS": [32, 64],
    "MI_CLASS_CAP": [4, 2],
}
for name, values in expected.items():
    plan = plans.get(name)
    if plan is None:
        raise SystemExit(f"missing static data plan: {name}")
    if plan.get("element") != "u16" or plan.get("values") != values:
        raise SystemExit(f"unexpected static data plan {name}: {plan}")

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
main = functions.get("main")
if main is None:
    raise SystemExit("missing main")

static_loads = []
for block in main.get("blocks", []):
    for inst in block.get("instructions", []):
        if inst.get("op") == "static_data_load":
            static_loads.append(inst.get("source_name"))

for name in expected:
    if name not in static_loads:
        raise SystemExit(f"missing static_data_load for {name}: {static_loads}")

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

require_global_route("MiTwoClassPageProof.birth/2", "RawBufCoreBox.alloc_bytes_i64/1")
require_global_route("MiTwoClassPageProof.birth/2", "RawArrayCoreBox.slot_append_any/2")
require_global_route("MiTwoClassPageProof.acquireBlock/1", "RawArrayCoreBox.slot_load_i64/2")
require_global_route("MiTwoClassPageProof.releaseBlock/1", "RawArrayCoreBox.slot_store_i64/3")
require_global_route("MiTwoClassPageProof.destroy/0", "RawBufCoreBox.free_bytes_i64/1")

print("[m22-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"
rg -F -q 'mir_call_array_slot_append_any_emit' "$build_log"
rg -F -q 'mir_call_array_slot_load_i64_emit' "$build_log"
rg -F -q 'mir_call_array_slot_store_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-two-class-page-proof' "$run_log"
rg -F -q 'table=32/4,64/2' "$run_log"
rg -F -q 'small=6/2/2 peak=4 free=0' "$run_log"
rg -F -q 'medium=3/1/1 peak=2 free=0' "$run_log"
rg -F -q 'rejects=3' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '`M22 mimalloc two-class page EXE proof`' "$TASKBOARD"
rg -F -q 'M22 mimalloc two-class page EXE proof' "$CARD"
rg -F -q 'two raw pages from MIR-owned static size-class metadata' "$APP_README"
rg -F -q 'k2_wide_mimalloc_two_class_page_exe_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"

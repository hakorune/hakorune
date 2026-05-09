#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-mem-extern-pure-first"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-mem-extern-exe-proof/main.hako"
APP_README="apps/hako-mem-extern-exe-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-066-M14-HAKO-MEM-EXTERN-PURE-FIRST-ROUTE.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
RETURN_PROOF="docs/development/current/main/design/return-proof-vocabulary-ssot.md"

echo "[$TAG] running M14 hako.mem extern pure-first guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$CARD" "$TASKBOARD" "$RETURN_PROOF"

cargo test -q refresh_function_extern_call_routes_records_hako_mem_alloc_route -- --nocapture
cargo test -q refresh_function_extern_call_routes_records_hako_mem_free_route -- --nocapture
cargo test -q generic_i64_body_accepts_hako_mem_alloc_free_extern_routes -- --nocapture
pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m14_hako_mem.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m14.mir.json"
exe_out="$tmp_dir/m14.exe"
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
main = functions.get("main")
if main is None:
    raise SystemExit("missing main")

routes = main.get("metadata", {}).get("extern_call_routes", [])
seen = {
    (route.get("route_id"), route.get("symbol"), route.get("return_shape"), route.get("value_demand"))
    for route in routes
}
want_alloc = (
    "extern.hako_mem.alloc",
    "hako_mem_alloc",
    "native_ptr_nullable",
    "native_ptr_nullable",
)
want_free = (
    "extern.hako_mem.free",
    "hako_mem_free",
    "void_sentinel_i64_zero",
    "scalar_i64",
)
if want_alloc not in seen:
    raise SystemExit("missing hako_mem_alloc extern route")
if want_free not in seen:
    raise SystemExit("missing hako_mem_free extern route")

print("[m14-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"
rg -F -q 'hako-mem-extern-exe-proof' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '`M14 hako.mem extern pure-first route`' "$TASKBOARD"
rg -F -q 'M14 hako.mem extern pure-first route' "$CARD"
rg -F -q 'hako_mem_alloc -> native_ptr_nullable' "$RETURN_PROOF"
rg -F -q 'hako_mem_free(native_ptr_nullable) -> void' "$RETURN_PROOF"

echo "[$TAG] ok"

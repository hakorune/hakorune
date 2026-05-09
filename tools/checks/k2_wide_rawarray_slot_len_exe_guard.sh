#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-rawarray-slot-len-exe"
cd "$ROOT_DIR"

APP="apps/rawarray-slot-len-exe-proof/main.hako"
APP_README="apps/rawarray-slot-len-exe-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-069-M17-RAWARRAY-SLOT-LEN-GENERIC-I64-ROUTE.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M17 RawArray slot_len_i64 EXE guard"

for file in "$APP" "$APP_README" "$CARD" "$TASKBOARD"; do
  if [ ! -f "$file" ]; then
    echo "[$TAG] missing file: $file" >&2
    exit 1
  fi
done

cargo test -q generic_i64_body_accepts_array_slot_len_extern_route -- --nocapture
cargo build -q --bin hakorune
cargo build --release -q -p nyash_kernel
cargo build --release -q -p nyash-llvm-compiler --bin ny-llvmc
bash tools/build_hako_llvmc_ffi.sh >/dev/null

tmp_dir="$(mktemp -d /tmp/hakorune_m17_rawarray.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m17.mir.json"
exe_out="$tmp_dir/m17.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
"$ROOT_DIR/target/debug/hakorune" --emit-mir-json "$mir_json" "$APP" >/dev/null

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

require_global_route("main", "RawArrayCoreBox.slot_len_i64/1")
require_global_route("main", "BoundsCoreBox.ensure_index_i64/2")
require_global_route("main", "InitializedRangeCoreBox.ensure_initialized_index_i64/2")
require_global_route("RawArrayCoreBox.slot_len_i64/1", "PtrCoreBox.slot_len_i64/1")
require_global_route("BufCoreBox.len_i64/1", "PtrCoreBox.slot_len_i64/1")
require_global_route("BoundsCoreBox.ensure_index_i64/2", "BufCoreBox.len_i64/1")
require_global_route(
    "InitializedRangeCoreBox.ensure_initialized_index_i64/2",
    "BufCoreBox.len_i64/1",
)

ptr_len = require_function("PtrCoreBox.slot_len_i64/1")
ptr_routes = ptr_len.get("metadata", {}).get("extern_call_routes", [])
if not any(
    route.get("route_id") == "extern.array.slot_len_i64"
    and route.get("symbol") == "nyash.array.slot_len_h"
    and route.get("return_shape") == "scalar_i64"
    for route in ptr_routes
):
    raise SystemExit(f"missing ArraySlotLenI64 extern route: {ptr_routes}")

print("[m17-mir-json] ok")
PY

if ! NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
  NYASH_FEATURES=rune \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_LLVM_ROUTE_TRACE=1 \
  HAKO_BACKEND_COMPILE_RECIPE=pure-first \
  HAKO_BACKEND_COMPAT_REPLAY=none \
  timeout 120 tools/selfhost/selfhost_build.sh \
    --in "$APP" \
    --mir "$mir_json" \
    --exe "$exe_out" >"$build_log" 2>&1; then
  echo "[$TAG] ERROR: pure-first build failed" >&2
  sed -n '1,220p' "$build_log" >&2
  exit 1
fi

if rg -F -q 'unsupported_pure_shape' "$build_log"; then
  echo "[$TAG] ERROR: pure-first reported unsupported shape" >&2
  sed -n '1,180p' "$build_log" >&2
  exit 1
fi

if rg -F -q 'compat_replay=harness' "$build_log"; then
  echo "[$TAG] ERROR: compat replay must stay disabled for M17 proof" >&2
  sed -n '1,180p' "$build_log" >&2
  exit 1
fi

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_array_slot_len_i64_emit' "$build_log"

if ! "$exe_out" >"$run_log" 2>&1; then
  echo "[$TAG] ERROR: EXE run failed" >&2
  sed -n '1,120p' "$run_log" >&2
  exit 1
fi

rg -F -q 'rawarray-slot-len-exe-proof' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '`M17 RawArray slot_len_i64 generic-i64 route`' "$TASKBOARD"
rg -F -q 'M17 RawArray slot_len_i64 generic-i64 route' "$CARD"
rg -F -q 'No PtrCoreBox slot_load/slot_store widening' "$APP_README"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-rawarray-slot-append-exe"
cd "$ROOT_DIR"

APP="apps/rawarray-slot-append-exe-proof/main.hako"
APP_README="apps/rawarray-slot-append-exe-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-068-M16-RAWARRAY-SLOT-APPEND-GENERIC-I64-ROUTE.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M16 RawArray slot_append_any EXE guard"

for file in "$APP" "$APP_README" "$CARD" "$TASKBOARD"; do
  if [ ! -f "$file" ]; then
    echo "[$TAG] missing file: $file" >&2
    exit 1
  fi
done

cargo test -q generic_i64_body_accepts_any_handle_live_and_array_slot_append_extern_routes -- --nocapture
cargo build -q --bin hakorune
cargo build --release -q -p nyash_kernel
cargo build --release -q -p nyash-llvm-compiler --bin ny-llvmc
bash tools/build_hako_llvmc_ffi.sh >/dev/null

tmp_dir="$(mktemp -d /tmp/hakorune_m16_rawarray.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m16.mir.json"
exe_out="$tmp_dir/m16.exe"
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

main = require_function("main")
routes = main.get("metadata", {}).get("global_call_routes", [])
seen = {
    (route.get("symbol"), route.get("target_shape"), route.get("proof"), route.get("return_shape"))
    for route in routes
}
want = (
    "RawArrayCoreBox.slot_append_any/2",
    "generic_i64_body",
    "typed_global_call_generic_i64",
    "ScalarI64",
)
if want not in seen:
    raise SystemExit(f"missing RawArray slot_append route: {want}; seen={sorted(seen)}")

live = require_function("OwnershipCoreBox._handle_live_i64/1")
live_routes = live.get("metadata", {}).get("extern_call_routes", [])
if not any(
    route.get("route_id") == "extern.any.handle_live"
    and route.get("symbol") == "nyash.any.handle_live_h"
    and route.get("return_shape") == "scalar_i64"
    for route in live_routes
):
    raise SystemExit("missing AnyHandleLive extern route")

append = require_function("PtrCoreBox.slot_append_any/2")
append_routes = append.get("metadata", {}).get("extern_call_routes", [])
if not any(
    route.get("route_id") == "extern.array.slot_append_any"
    and route.get("symbol") == "nyash.array.slot_append_hh"
    and route.get("return_shape") == "scalar_i64"
    for route in append_routes
):
    raise SystemExit("missing ArraySlotAppendAny extern route")

print("[m16-mir-json] ok")
PY

NYASH_BIN="$ROOT_DIR/target/debug/hakorune" \
NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
NYASH_LLVM_ROUTE_TRACE=1 \
HAKO_BACKEND_COMPILE_RECIPE=pure-first \
HAKO_BACKEND_COMPAT_REPLAY=none \
timeout 120 tools/selfhost/selfhost_build.sh \
  --in "$APP" \
  --mir "$mir_json" \
  --exe "$exe_out" >"$build_log" 2>&1

if rg -F -q 'unsupported_pure_shape' "$build_log"; then
  echo "[$TAG] ERROR: pure-first reported unsupported shape" >&2
  sed -n '1,180p' "$build_log" >&2
  exit 1
fi

if rg -F -q 'compat_replay=harness' "$build_log"; then
  echo "[$TAG] ERROR: compat replay must stay disabled for M16 proof" >&2
  sed -n '1,180p' "$build_log" >&2
  exit 1
fi

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_any_handle_live_emit' "$build_log"
rg -F -q 'mir_call_array_slot_append_any_emit' "$build_log"

"$exe_out" >"$run_log" 2>&1

rg -F -q 'rawarray-slot-append-exe-proof' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '`M16 RawArray slot_append_any generic-i64 route`' "$TASKBOARD"
rg -F -q 'M16 RawArray slot_append_any generic-i64 route' "$CARD"
rg -F -q 'No RawArray slot load/store parity' "$APP_README"

echo "[$TAG] ok"

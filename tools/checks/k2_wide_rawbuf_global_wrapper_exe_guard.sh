#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-rawbuf-global-wrapper-exe"
cd "$ROOT_DIR"

APP="apps/rawbuf-global-wrapper-exe-proof/main.hako"
APP_README="apps/rawbuf-global-wrapper-exe-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-067-M15-RAWBUF-GLOBAL-WRAPPER-GENERIC-I64-ROUTE.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M15 RawBuf global wrapper EXE guard"

for file in "$APP" "$APP_README" "$CARD" "$TASKBOARD"; do
  if [ ! -f "$file" ]; then
    echo "[$TAG] missing file: $file" >&2
    exit 1
  fi
done

cargo test -q generic_i64_body_accepts_void_sentinel_global_side_call -- --nocapture
cargo build -q --bin hakorune
cargo build --release -q -p nyash_kernel
cargo build --release -q -p nyash-llvm-compiler --bin ny-llvmc
bash tools/build_hako_llvmc_ffi.sh >/dev/null

tmp_dir="$(mktemp -d /tmp/hakorune_m15_rawbuf.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m15.mir.json"
exe_out="$tmp_dir/m15.exe"
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
main = functions.get("main")
if main is None:
    raise SystemExit("missing main")

routes = main.get("metadata", {}).get("global_call_routes", [])
seen = {
    (route.get("symbol"), route.get("target_shape"), route.get("proof"), route.get("return_shape"))
    for route in routes
}
want = {
    (
        "RawBufCoreBox.alloc_bytes_i64/1",
        "generic_i64_body",
        "typed_global_call_generic_i64",
        "ScalarI64",
    ),
    (
        "RawBufCoreBox.free_bytes_i64/1",
        "generic_i64_body",
        "typed_global_call_generic_i64",
        "ScalarI64",
    ),
}
missing = want - seen
if missing:
    raise SystemExit(f"missing RawBuf generic-i64 routes: {sorted(missing)}")

print("[m15-mir-json] ok")
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
  echo "[$TAG] ERROR: compat replay must stay disabled for M15 proof" >&2
  sed -n '1,180p' "$build_log" >&2
  exit 1
fi

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"

"$exe_out" >"$run_log" 2>&1

rg -F -q 'rawbuf-global-wrapper-exe-proof' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '`M15 RawBuf global wrapper generic-i64 route`' "$TASKBOARD"
rg -F -q 'M15 RawBuf global wrapper generic-i64 route' "$CARD"
rg -F -q 'No RawArrayCoreBox slot parity' "$APP_README"

echo "[$TAG] ok"

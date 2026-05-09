#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-mem-extern-pure-first"
cd "$ROOT_DIR"

APP="apps/hako-mem-extern-exe-proof/main.hako"
APP_README="apps/hako-mem-extern-exe-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-066-M14-HAKO-MEM-EXTERN-PURE-FIRST-ROUTE.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
RETURN_PROOF="docs/development/current/main/design/return-proof-vocabulary-ssot.md"

echo "[$TAG] running M14 hako.mem extern pure-first guard"

for file in "$APP" "$APP_README" "$CARD" "$TASKBOARD" "$RETURN_PROOF"; do
  if [ ! -f "$file" ]; then
    echo "[$TAG] missing file: $file" >&2
    exit 1
  fi
done

cargo test -q refresh_function_extern_call_routes_records_hako_mem_alloc_route -- --nocapture
cargo test -q refresh_function_extern_call_routes_records_hako_mem_free_route -- --nocapture
cargo test -q generic_i64_body_accepts_hako_mem_alloc_free_extern_routes -- --nocapture
cargo build -q --bin hakorune
cargo build --release -q -p nyash_kernel
cargo build --release -q -p nyash-llvm-compiler --bin ny-llvmc
bash tools/build_hako_llvmc_ffi.sh >/dev/null

tmp_dir="$(mktemp -d /tmp/hakorune_m14_hako_mem.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m14.mir.json"
exe_out="$tmp_dir/m14.exe"
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
  echo "[$TAG] ERROR: compat replay must stay disabled for M14 proof" >&2
  sed -n '1,180p' "$build_log" >&2
  exit 1
fi

rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"

"$exe_out" >"$run_log" 2>&1

rg -F -q 'hako-mem-extern-exe-proof' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '`M14 hako.mem extern pure-first route`' "$TASKBOARD"
rg -F -q 'M14 hako.mem extern pure-first route' "$CARD"
rg -F -q 'hako_mem_alloc -> native_ptr_nullable' "$RETURN_PROOF"
rg -F -q 'hako_mem_free(native_ptr_nullable) -> void' "$RETURN_PROOF"

echo "[$TAG] ok"

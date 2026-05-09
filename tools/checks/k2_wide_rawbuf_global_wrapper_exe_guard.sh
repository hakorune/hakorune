#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-rawbuf-global-wrapper-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

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
pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m15_rawbuf.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m15.mir.json"
exe_out="$tmp_dir/m15.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
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

pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"
rg -F -q 'rawbuf-global-wrapper-exe-proof' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '`M15 RawBuf global wrapper generic-i64 route`' "$TASKBOARD"
rg -F -q 'M15 RawBuf global wrapper generic-i64 route' "$CARD"
rg -F -q 'No RawArrayCoreBox slot parity' "$APP_README"

echo "[$TAG] ok"

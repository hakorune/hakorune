#!/bin/bash
# Phase 291x: MapBox stable surface catalog + VM route smoke.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase291x_mapbox_surface_catalog_vm"

run_catalog_unit_lock() {
  local out
  set +e
  out=$(cargo test map_surface_catalog --lib 2>&1)
  local rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    echo "$out" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: catalog unit lock failed"
    exit 1
  fi

  set +e
  out=$(cargo test invoke_surface_routes_current_map_rows --lib 2>&1)
  rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    echo "$out" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: invoke_surface unit lock failed"
    exit 1
  fi
}

run_vm_surface_route_lock() {
  local code
  code=$(cat <<'HCODE'
static box Main {
  main() {
    local m = new MapBox()
    print(m.size())
    print(m.length())
    print(m.len())
    m.set("a", 1)
    m.set("b", 2)
    print(m.has("a"))
    print(m.get("a"))
    print(m.size())
    print(m.len())
    print(m.has("b"))
    print("OK: map-surface")
    return 0
  }
}
HCODE
)

  local tmpfile out_file out
  tmpfile="$(mktemp /tmp/phase291x_mapbox_surface.XXXXXX.hako)"
  out_file="$(mktemp /tmp/phase291x_mapbox_surface.XXXXXX.out)"
  printf '%s\n' "$code" >"$tmpfile"

  set +e
  # This smoke pins the Rust CoreBox catalog + MIR router surface. Use the
  # direct Rust VM path so vm-hako subset BoxCall debt does not own the result.
  env \
    -u NYASH_USING_AST \
    -u NYASH_ROOT \
    -u HAKO_JOINIR_STRICT \
    -u NYASH_JOINIR_DEV \
    -u HAKO_SILENT_TAGS \
    -u HAKO_TRACE_EXECUTION \
    -u HAKO_VERIFY_SHOW_LOGS \
    -u NYASH_DEBUG_FUEL \
    -u NYASH_LOAD_NY_PLUGINS \
    -u NYASH_CLI_VERBOSE \
    NYASH_FEATURES=stage3 NYASH_MIR_UNIFIED_CALL=1 \
    "$NYASH_BIN" --backend vm --dev "$tmpfile" 2>&1 | filter_noise >"$out_file"
  local rc=${PIPESTATUS[0]}
  set -e
  out="$(<"$out_file")"
  rm -f "$tmpfile" "$out_file"
  if [ "$rc" -ne 0 ]; then
    echo "$out" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: VM surface route failed rc=$rc"
    exit 1
  fi

  if echo "$out" | rg -q '\[vm/method/stub:(size|length|len|has|get|set)\]'; then
    echo "$out" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: stable MapBox method hit VM stub"
    exit 1
  fi

  local actual expected
  actual=$(printf '%s\n' "$out" | awk '/^(0|1|2|true|false|OK: map-surface)$/ { print }')
  expected=$(cat <<'EXPECT'
0
0
0
true
1
2
2
true
OK: map-surface
EXPECT
)
  if [ "$actual" != "$expected" ]; then
    echo "$out" | tail -n 120 >&2 || true
    echo "[FAIL] expected:" >&2
    printf '%s\n' "$expected" >&2
    echo "[FAIL] actual:" >&2
    printf '%s\n' "$actual" >&2
    test_fail "$SMOKE_NAME: VM surface output mismatch"
    exit 1
  fi
}

run_catalog_unit_lock
run_vm_surface_route_lock

test_pass "$SMOKE_NAME: PASS"

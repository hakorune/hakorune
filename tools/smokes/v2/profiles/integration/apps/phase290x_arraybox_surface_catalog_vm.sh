#!/bin/bash
# Phase 290x: ArrayBox stable surface catalog + VM route smoke.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase290x_arraybox_surface_catalog_vm"
INPUT="${1:-$ROOT/apps/tests/phase290x_arraybox_surface_catalog_vm.hako}"

run_catalog_unit_lock() {
  local out
  set +e
  out=$(cargo test array_surface_catalog --lib 2>&1)
  local rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    echo "$out" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: catalog unit lock failed"
    exit 1
  fi

  set +e
  out=$(cargo test invoke_surface_routes_insert_remove_clear_contains_and_length_alias --lib 2>&1)
  rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    echo "$out" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: invoke_surface unit lock failed"
    exit 1
  fi
}

run_vm_surface_route_lock() {
  local out
  local timeout_secs="${RUN_TIMEOUT_SECS:-30}"
  set +e
  out=$(timeout "$timeout_secs" env -i \
    PATH="$PATH" \
    HOME="$HOME" \
    "$NYASH_BIN" --backend vm "$INPUT" --dev 2>&1)
  local rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    echo "$out" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: VM surface route failed rc=$rc"
    exit 1
  fi

  if echo "$out" | rg -q '\[vm/method/stub:(length|size|len|get|set|push|pop|clear|contains|slice|remove|insert)\]'; then
    echo "$out" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: stable ArrayBox method hit VM stub"
    exit 1
  fi

  local actual expected
  actual=$(printf '%s\n' "$out" | awk '/^(-?[0-9]+|true|false|OK: array-surface)$/ { print }')
  expected=$(cat <<'EXPECT'
2
2
2
25
3
15
true
15
2
25
1
0
OK: array-surface
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

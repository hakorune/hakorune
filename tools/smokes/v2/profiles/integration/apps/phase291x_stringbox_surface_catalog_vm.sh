#!/bin/bash
# Phase 291x: StringBox stable surface catalog + VM route smoke.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi
source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase291x_stringbox_surface_catalog_vm"

run_catalog_unit_lock() {
  local out
  set +e
  out=$(cargo test string_surface_catalog --lib 2>&1)
  local rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    echo "$out" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: catalog unit lock failed"
    exit 1
  fi

  set +e
  out=$(cargo test invoke_surface_routes_string_aliases_and_values --lib 2>&1)
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
    local s = "banana"
    print(s.length())
    print(s.len())
    print(s.size())
    print(s.substring(1, 4))
    print(s.substr(1, 4))
    print(s.indexOf("na"))
    print(s.indexOf("na", 3))
    print(s.find("na", 3))
    print(s.concat("!"))
    print(s.replace("na", "NA"))
    print(s.trim())
    print(s.lastIndexOf("na"))
    print(s.lastIndexOf("na", 3))
    print(s.lastIndexOf("na", 1))
    print(s.contains("nan"))
    print("OK: string-surface")
    return 0
  }
}
HCODE
)

  local out
  set +e
  out=$(run_nyash_vm -c "$code" --dev 2>&1)
  local rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    echo "$out" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: VM surface route failed rc=$rc"
    exit 1
  fi

  if echo "$out" | rg -q '\[vm/method/stub:(length|len|size|substring|substr|concat|indexOf|find|replace|trim|lastIndexOf|contains)\]'; then
    echo "$out" | tail -n 120 >&2 || true
    test_fail "$SMOKE_NAME: stable StringBox method hit VM stub"
    exit 1
  fi

  local actual expected
  actual=$(printf '%s\n' "$out" | awk '/^(6|ana|2|4|-1|banana!|baNANA|banana|true|OK: string-surface)$/ { print }')
  expected=$(cat <<'EXPECT'
6
6
6
ana
ana
2
4
4
banana!
baNANA
banana
4
2
-1
true
OK: string-surface
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

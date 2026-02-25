#!/usr/bin/env bash
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true
source "$ROOT_DIR/tools/smokes/v2/lib/result_checker.sh" || true

require_env || { echo "[SKIP] env not ready"; exit 0; }

test_scope_loop_body_local_vm() {
  local code='static box Main { method main(args) { local i = 0 local sum = 0 loop(i < 10) { local y = 7 sum = sum + y if i == 1 { break } i = i + 1 } return y } }'
  local tmp
  tmp=$(mktemp --suffix .hako)
  printf '%s' "$code" > "$tmp"

  # Spec: loop body is a block; `local y` must not leak outside the loop.
  local out
  out=$(NYASH_FEATURES=stage3 "$NYASH_BIN" --backend vm "$tmp" 2>&1)
  local rc=$?
  if [[ "$rc" -eq 0 ]]; then
    echo "[FAIL] scope_loop_body_local_vm: expected failure, got rc=0"
    rm -f "$tmp" 2>/dev/null || true
    return 1
  fi
  if ! printf '%s' "$out" | grep -q "Undefined variable: y"; then
    echo "[FAIL] scope_loop_body_local_vm: missing 'Undefined variable: y' in output"
    echo "$out"
    rm -f "$tmp" 2>/dev/null || true
    return 1
  fi

  echo "[PASS] scope_loop_body_local_vm"
  rm -f "$tmp" 2>/dev/null || true
  return 0
}

run_test "scope_loop_body_local_vm" test_scope_loop_body_local_vm

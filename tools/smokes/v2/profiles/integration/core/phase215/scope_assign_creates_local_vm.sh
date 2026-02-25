#!/usr/bin/env bash
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true
source "$ROOT_DIR/tools/smokes/v2/lib/result_checker.sh" || true

require_env || { echo "[SKIP] env not ready"; exit 0; }

test_scope_assign_creates_local_vm() {
  local code='static box Main { method main(args) { { if (1==1) { y = 42 } } return y } }'
  local tmp
  tmp=$(mktemp --suffix .hako)
  printf '%s' "$code" > "$tmp"

  # Spec (LANGUAGE_REFERENCE_2025 / syntax-cheatsheet):
  # - Undeclared assignment is an error (must use `local y`).
  local out
  out=$(NYASH_FEATURES=stage3 "$NYASH_BIN" --backend vm "$tmp" 2>&1)
  local rc=$?
  if [[ "$rc" -eq 0 ]]; then
    echo "[FAIL] scope_assign_creates_local_vm: expected failure, got rc=0"
    rm -f "$tmp" 2>/dev/null || true
    return 1
  fi
  if ! printf '%s' "$out" | grep -q "Undefined variable: y"; then
    echo "[FAIL] scope_assign_creates_local_vm: missing 'Undefined variable: y' in output"
    echo "$out"
    rm -f "$tmp" 2>/dev/null || true
    return 1
  fi

  echo "[PASS] scope_assign_creates_local_vm"
  rm -f "$tmp" 2>/dev/null || true
  return 0
}

run_test "scope_assign_creates_local_vm" test_scope_assign_creates_local_vm

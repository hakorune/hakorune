#!/usr/bin/env bash
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true
source "$ROOT_DIR/tools/smokes/v2/lib/result_checker.sh" || true

require_env || { echo "[SKIP] env not ready"; exit 0; }

test_scope_shadow_vm() {
  local code='static box Main { method main(args) { local x = 1 { local x = 2 } return x } }'
  local tmp
  tmp=$(mktemp --suffix .hako)
  printf '%s' "$code" > "$tmp"

  # Spec (LANGUAGE_REFERENCE_2025 / syntax-cheatsheet):
  # `local` is block-scoped and supports shadowing.
  # The inner `local x` must not overwrite the outer binding.
  local rc
  NYASH_FEATURES=stage3 "$NYASH_BIN" --backend vm "$tmp" >/dev/null 2>&1
  rc=$?
  if [[ "$rc" -ne 1 ]]; then
    echo "[FAIL] scope_shadow_vm: vm rc=$rc (expected 1)"
    rm -f "$tmp" 2>/dev/null || true
    return 1
  fi

  echo "[PASS] scope_shadow_vm"
  rm -f "$tmp" 2>/dev/null || true
  return 0
}

run_test "scope_shadow_vm" test_scope_shadow_vm

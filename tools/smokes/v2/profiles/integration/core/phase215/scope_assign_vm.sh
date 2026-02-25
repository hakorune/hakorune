#!/usr/bin/env bash
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true
source "$ROOT_DIR/tools/smokes/v2/lib/result_checker.sh" || true

require_env || { echo "[SKIP] env not ready"; exit 0; }

test_scope_assign_vm() {
  local code='static box Main { method main(args) { local x = 0 { if (1==1) { x = 42 } } return x } }'
  local tmp
  tmp=$(mktemp --suffix .hako)
  printf '%s' "$code" > "$tmp"
  # Quick: VM should reflect nested assignment
  local rc
  NYASH_FEATURES=stage3 "$NYASH_BIN" --backend vm "$tmp" >/dev/null 2>&1
  rc=$?
  if [[ "$rc" -ne 42 ]]; then
    echo "[FAIL] scope_assign_vm: vm rc=$rc (expected 42)"
    return 1
  fi
  echo "[PASS] scope_assign_vm"
  rm -f "$tmp" 2>/dev/null || true
  return 0
}

run_test "scope_assign_vm" test_scope_assign_vm

#!/usr/bin/env bash
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../.." && pwd)"
source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh" || true
source "$ROOT_DIR/tools/smokes/v2/lib/result_checker.sh" || true

require_env || { echo "[SKIP] env not ready"; exit 0; }

test_vm_llvm_scope_assign() {
  local code='static box Main { method main(args) { local x = 0 { if (1==1) { x = 42 } } return x } }'
  # Stage-3 parse is required for `local`
  NYASH_FEATURES=stage3 check_parity -c "$code" "vm_llvm_scope_assign"
}

run_test "vm_llvm_scope_assign" test_vm_llvm_scope_assign

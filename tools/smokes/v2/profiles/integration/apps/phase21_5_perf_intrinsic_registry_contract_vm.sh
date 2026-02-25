#!/bin/bash
# phase21_5_perf_intrinsic_registry_contract_vm.sh
#
# Contract pin (LLVM-HOT-20 / HINT-CONTRACT-01):
# - MIR call intrinsic registry is table-driven (single declaration source).
# - Registry consistency validation helper is present.
# - Registry + autospecialize unit tests pass under perf gate context.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_intrinsic_registry_contract_vm"
REGISTRY_FILE="$NYASH_ROOT/src/llvm_py/instructions/mir_call/intrinsic_registry.py"
TEST_REGISTRY="$NYASH_ROOT/src/llvm_py/tests/test_mir_call_intrinsic_registry.py"
TEST_AUTOSPEC="$NYASH_ROOT/src/llvm_py/tests/test_mir_call_auto_specialize.py"

for f in "$REGISTRY_FILE" "$TEST_REGISTRY" "$TEST_AUTOSPEC"; do
  if [ ! -f "$f" ]; then
    test_fail "$SMOKE_NAME: missing file: $f"
    exit 2
  fi
done

if ! grep -q "_INTRINSIC_SPECS" "$REGISTRY_FILE"; then
  test_fail "$SMOKE_NAME: declarative intrinsic table not found (_INTRINSIC_SPECS)"
  exit 1
fi

if ! grep -q "validate_intrinsic_specs" "$REGISTRY_FILE"; then
  test_fail "$SMOKE_NAME: consistency validator not found (validate_intrinsic_specs)"
  exit 1
fi

if ! grep -q "TAG_INTRINSIC_CANDIDATE" "$REGISTRY_FILE"; then
  test_fail "$SMOKE_NAME: intrinsic-candidate tag contract not found"
  exit 1
fi

if ! command -v python3 >/dev/null 2>&1; then
  test_fail "$SMOKE_NAME: python3 is required"
  exit 2
fi

set +e
test_out="$(
  cd "$NYASH_ROOT" && \
  PYTHONPATH=src/llvm_py:. python3 -m unittest \
    src/llvm_py/tests/test_mir_call_intrinsic_registry.py \
    src/llvm_py/tests/test_mir_call_auto_specialize.py 2>&1
)"
test_rc=$?
set -e

if [ "$test_rc" -ne 0 ]; then
  printf '%s\n' "$test_out"
  test_fail "$SMOKE_NAME: python unittest failed rc=$test_rc"
  exit 1
fi

test_pass "$SMOKE_NAME"

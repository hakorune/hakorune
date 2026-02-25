#!/bin/bash
# phase21_5_perf_aot_auto_safepoint_env_contract_vm.sh
#
# Contract pin:
# - PERF_AOT_AUTO_SAFEPOINT must be 0|1 (invalid value -> fail-fast rc=2).
# - When PERF_AOT_AUTO_SAFEPOINT is unset, NYASH_LLVM_AUTO_SAFEPOINT is accepted as fallback.
# - PERF_SKIP_VM_PREFLIGHT must be 0|1 (invalid value -> fail-fast rc=2).

set -euo pipefail

SMOKE_NAME="phase21_5_perf_aot_auto_safepoint_env_contract_vm"
KEY="chip8_kernel_small"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SCRIPT="$NYASH_ROOT/tools/perf/bench_compare_c_py_vs_hako.sh"
if [ ! -f "$SCRIPT" ]; then
  test_fail "$SMOKE_NAME: missing script: $SCRIPT"
  exit 2
fi

# Case 1: invalid PERF_AOT_AUTO_SAFEPOINT must fail-fast.
set +e
bad_out="$(
  PERF_AOT_AUTO_SAFEPOINT=2 \
  PERF_AOT=1 \
  PERF_SKIP_VM_PREFLIGHT=1 \
  NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  bash "$SCRIPT" "$KEY" 1 1 2>&1
)"
bad_rc=$?
set -e
if [ "$bad_rc" -eq 0 ]; then
  printf '%s\n' "$bad_out"
  test_fail "$SMOKE_NAME: invalid PERF_AOT_AUTO_SAFEPOINT unexpectedly succeeded"
  exit 1
fi
if ! printf '%s\n' "$bad_out" | grep -q 'PERF_AOT_AUTO_SAFEPOINT must be 0|1'; then
  printf '%s\n' "$bad_out"
  test_fail "$SMOKE_NAME: missing fail-fast error for invalid PERF_AOT_AUTO_SAFEPOINT"
  exit 1
fi

# Case 2: fallback via NYASH_LLVM_AUTO_SAFEPOINT works when PERF_* is unset.
set +e
ok_out="$(
  env -u PERF_AOT_AUTO_SAFEPOINT \
    NYASH_LLVM_AUTO_SAFEPOINT=1 \
    PERF_AOT=1 \
    PERF_SKIP_VM_PREFLIGHT=1 \
    NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
    bash "$SCRIPT" "$KEY" 1 1 2>&1
)"
ok_rc=$?
set -e
if [ "$ok_rc" -ne 0 ]; then
  printf '%s\n' "$ok_out"
  test_fail "$SMOKE_NAME: NYASH_LLVM_AUTO_SAFEPOINT fallback case failed rc=$ok_rc"
  exit 1
fi
if ! printf '%s\n' "$ok_out" | grep -q "\\[bench4\\] name=${KEY} "; then
  printf '%s\n' "$ok_out"
  test_fail "$SMOKE_NAME: fallback case missing bench4 output"
  exit 1
fi
if ! printf '%s\n' "$ok_out" | grep -q 'aot_status=ok'; then
  printf '%s\n' "$ok_out"
  test_fail "$SMOKE_NAME: fallback case AOT status not ok"
  exit 1
fi

# Case 3: invalid PERF_SKIP_VM_PREFLIGHT must fail-fast.
set +e
bad_preflight_out="$(
  PERF_SKIP_VM_PREFLIGHT=2 \
  PERF_AOT=1 \
  NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  bash "$SCRIPT" "$KEY" 1 1 2>&1
)"
bad_preflight_rc=$?
set -e
if [ "$bad_preflight_rc" -eq 0 ]; then
  printf '%s\n' "$bad_preflight_out"
  test_fail "$SMOKE_NAME: invalid PERF_SKIP_VM_PREFLIGHT unexpectedly succeeded"
  exit 1
fi
if ! printf '%s\n' "$bad_preflight_out" | grep -q 'PERF_SKIP_VM_PREFLIGHT must be 0|1'; then
  printf '%s\n' "$bad_preflight_out"
  test_fail "$SMOKE_NAME: missing fail-fast error for invalid PERF_SKIP_VM_PREFLIGHT"
  exit 1
fi

test_pass "$SMOKE_NAME"

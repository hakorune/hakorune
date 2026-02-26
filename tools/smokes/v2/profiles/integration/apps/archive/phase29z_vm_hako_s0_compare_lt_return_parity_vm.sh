#!/bin/bash
# Phase29z-S0k: vm-hako S0 parity smoke (compare lt + return)
#
# Contract:
# - S0(compare-lt) fixture executes on both `--backend vm` and `--backend vm-hako`.
# - Exit codes must match and be 1.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

INPUT="${1:-$NYASH_ROOT/apps/tests/phase29z_vm_hako_s0_compare_lt_return_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -f "$INPUT" ]; then
    test_fail "phase29z_vm_hako_s0_compare_lt_return_parity_vm: fixture missing: $INPUT"
    exit 1
fi

run_backend() {
    local backend="$1"
    local output
    local rc
    set +e
    output=$(
        env \
            NYASH_USE_NY_COMPILER=0 \
            NYASH_DISABLE_NY_COMPILER=1 \
            HAKO_DISABLE_NY_COMPILER=1 \
            timeout "$RUN_TIMEOUT_SECS" \
            "$NYASH_BIN" --backend "$backend" "$INPUT" 2>&1
    )
    rc=$?
    set -e
    echo "$output"
    return "$rc"
}

set +e
VM_OUTPUT=$(run_backend vm)
VM_RC=$?
set -e

if [ "$VM_RC" -eq 124 ]; then
    test_fail "phase29z_vm_hako_s0_compare_lt_return_parity_vm: vm timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

set +e
HAKO_OUTPUT=$(run_backend vm-hako)
HAKO_RC=$?
set -e

if [ "$HAKO_RC" -eq 124 ]; then
    test_fail "phase29z_vm_hako_s0_compare_lt_return_parity_vm: vm-hako timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$VM_RC" -ne 1 ]; then
    echo "[FAIL] expected vm rc=1, got rc=$VM_RC"
    echo "$VM_OUTPUT" | tail -n 80 || true
    test_fail "phase29z_vm_hako_s0_compare_lt_return_parity_vm: vm rc mismatch"
    exit 1
fi

if [ "$HAKO_RC" -ne 1 ]; then
    echo "[FAIL] expected vm-hako rc=1, got rc=$HAKO_RC"
    echo "$HAKO_OUTPUT" | tail -n 80 || true
    test_fail "phase29z_vm_hako_s0_compare_lt_return_parity_vm: vm-hako rc mismatch"
    exit 1
fi

if [ "$VM_RC" -ne "$HAKO_RC" ]; then
    echo "[FAIL] parity mismatch: vm=$VM_RC vm-hako=$HAKO_RC"
    test_fail "phase29z_vm_hako_s0_compare_lt_return_parity_vm: parity mismatch"
    exit 1
fi

if echo "$HAKO_OUTPUT" | rg -q '^\[vm-hako/unimplemented\]'; then
    echo "[FAIL] vm-hako reported unimplemented on S0(compare-lt) fixture"
    echo "$HAKO_OUTPUT" | tail -n 80 || true
    test_fail "phase29z_vm_hako_s0_compare_lt_return_parity_vm: unexpected unimplemented tag"
    exit 1
fi

test_pass "phase29z_vm_hako_s0_compare_lt_return_parity_vm: PASS (vm=$VM_RC vm-hako=$HAKO_RC)"

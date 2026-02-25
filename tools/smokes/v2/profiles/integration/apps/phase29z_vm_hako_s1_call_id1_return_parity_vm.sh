#!/bin/bash
# Phase29z-S1f: vm-hako S1 parity smoke (call id(1-int) + return, optimize-on)
#
# Contract:
# - S1(call id(1-int)) fixture executes on both `--backend vm` and `--backend vm-hako`.
# - MIR(JSON v0, unified-call=0) contains `call(args=1)` in `main`.
# - `main` does not rely on dynamic method-call bridge (`call(args=2,func=4294967295)`).
# - Exit codes must match and be 9.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

INPUT="${1:-$NYASH_ROOT/apps/tests/phase29z_vm_hako_s1_call_id1_return_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -f "$INPUT" ]; then
    test_fail "phase29z_vm_hako_s1_call_id1_return_parity_vm: fixture missing: $INPUT"
    exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
    test_fail "phase29z_vm_hako_s1_call_id1_return_parity_vm: jq not found"
    exit 1
fi

TMP_MIR_JSON="${TMPDIR:-/tmp}/phase29z_vm_hako_s1f_call_id1_$$.json"
cleanup() {
    rm -f "$TMP_MIR_JSON"
}
trap cleanup EXIT

check_call1_shape() {
    local output
    local rc
    set +e
    output=$(
        env \
            NYASH_USE_NY_COMPILER=0 \
            NYASH_DISABLE_NY_COMPILER=1 \
            HAKO_DISABLE_NY_COMPILER=1 \
            NYASH_MIR_UNIFIED_CALL=0 \
            NYASH_JSON_SCHEMA_V1=0 \
            timeout "$RUN_TIMEOUT_SECS" \
            "$NYASH_BIN" --emit-mir-json "$TMP_MIR_JSON" "$INPUT" 2>&1
    )
    rc=$?
    set -e

    if [ "$rc" -eq 124 ]; then
        test_fail "phase29z_vm_hako_s1_call_id1_return_parity_vm: emit-mir timed out (>${RUN_TIMEOUT_SECS}s)"
        exit 1
    fi
    if [ "$rc" -ne 0 ]; then
        echo "[FAIL] emit-mir-json failed rc=$rc"
        echo "$output" | tail -n 80 || true
        test_fail "phase29z_vm_hako_s1_call_id1_return_parity_vm: emit-mir failed"
        exit 1
    fi

    local call1_count
    local dynamic_count
    call1_count=$(jq '[.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="call" and (.args|length)==1)] | length' "$TMP_MIR_JSON")
    dynamic_count=$(jq '[.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="call" and (.args|length)==2 and (.func==4294967295))] | length' "$TMP_MIR_JSON")

    if [ "$call1_count" -lt 1 ]; then
        echo "[FAIL] expected call(args=1) in main, found=$call1_count"
        jq '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | {op,args,func,dst}' "$TMP_MIR_JSON" | tail -n 120 || true
        test_fail "phase29z_vm_hako_s1_call_id1_return_parity_vm: call1 shape missing"
        exit 1
    fi
    if [ "$dynamic_count" -ne 0 ]; then
        echo "[FAIL] expected no dynamic method-call bridge in main, found=$dynamic_count"
        jq '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="call") | {op,args,func,dst}' "$TMP_MIR_JSON" | tail -n 120 || true
        test_fail "phase29z_vm_hako_s1_call_id1_return_parity_vm: unexpected dynamic bridge shape"
        exit 1
    fi
}

check_call1_shape

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
    test_fail "phase29z_vm_hako_s1_call_id1_return_parity_vm: vm timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

set +e
HAKO_OUTPUT=$(run_backend vm-hako)
HAKO_RC=$?
set -e

if [ "$HAKO_RC" -eq 124 ]; then
    test_fail "phase29z_vm_hako_s1_call_id1_return_parity_vm: vm-hako timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$VM_RC" -ne 9 ]; then
    echo "[FAIL] expected vm rc=9, got rc=$VM_RC"
    echo "$VM_OUTPUT" | tail -n 80 || true
    test_fail "phase29z_vm_hako_s1_call_id1_return_parity_vm: vm rc mismatch"
    exit 1
fi

if [ "$HAKO_RC" -ne 9 ]; then
    echo "[FAIL] expected vm-hako rc=9, got rc=$HAKO_RC"
    echo "$HAKO_OUTPUT" | tail -n 120 || true
    test_fail "phase29z_vm_hako_s1_call_id1_return_parity_vm: vm-hako rc mismatch"
    exit 1
fi

if [ "$VM_RC" -ne "$HAKO_RC" ]; then
    echo "[FAIL] parity mismatch: vm=$VM_RC vm-hako=$HAKO_RC"
    test_fail "phase29z_vm_hako_s1_call_id1_return_parity_vm: parity mismatch"
    exit 1
fi

if echo "$HAKO_OUTPUT" | rg -q '^\[vm-hako/unimplemented\]'; then
    echo "[FAIL] vm-hako reported unimplemented on S1(call id(1-int)) fixture"
    echo "$HAKO_OUTPUT" | tail -n 120 || true
    test_fail "phase29z_vm_hako_s1_call_id1_return_parity_vm: unexpected unimplemented tag"
    exit 1
fi

test_pass "phase29z_vm_hako_s1_call_id1_return_parity_vm: PASS (vm=$VM_RC vm-hako=$HAKO_RC)"

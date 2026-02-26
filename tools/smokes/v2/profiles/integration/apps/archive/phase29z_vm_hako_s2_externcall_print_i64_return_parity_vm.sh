#!/bin/bash
# Phase29z-S2a: vm-hako S2 parity smoke (externcall print 1-int)
#
# Contract:
# - S2(externcall print 1-int) fixture executes on both `--backend vm` and `--backend vm-hako`.
# - MIR(JSON v0, unified-call=0) contains `externcall(func=nyash.console.log,args=1)` in `main`.
# - First integer line in stdout must be `9`; exit codes must match and be `7`.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

INPUT="${1:-$NYASH_ROOT/apps/tests/phase29z_vm_hako_s2_externcall_print_i64_return_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -f "$INPUT" ]; then
    test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: fixture missing: $INPUT"
    exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
    test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: jq not found"
    exit 1
fi

TMP_MIR_JSON="${TMPDIR:-/tmp}/phase29z_vm_hako_s2a_externcall_print_$$.json"
cleanup() {
    rm -f "$TMP_MIR_JSON"
}
trap cleanup EXIT

check_externcall_shape() {
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
        test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: emit-mir timed out (>${RUN_TIMEOUT_SECS}s)"
        exit 1
    fi
    if [ "$rc" -ne 0 ]; then
        echo "[FAIL] emit-mir-json failed rc=$rc"
        echo "$output" | tail -n 80 || true
        test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: emit-mir failed"
        exit 1
    fi

    local extern_count
    local call_count
    local mir_call_count
    extern_count=$(jq '[.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="externcall" and .func=="nyash.console.log" and (.args|length)==1)] | length' "$TMP_MIR_JSON")
    call_count=$(jq '[.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="call")] | length' "$TMP_MIR_JSON")
    mir_call_count=$(jq '[.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="mir_call")] | length' "$TMP_MIR_JSON")

    if [ "$extern_count" -lt 1 ]; then
        echo "[FAIL] expected externcall(nyash.console.log,args=1) in main, found=$extern_count"
        jq '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | {op,func,args,dst,mir_call}' "$TMP_MIR_JSON" | tail -n 120 || true
        test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: externcall shape missing"
        exit 1
    fi
    if [ "$call_count" -ne 0 ] || [ "$mir_call_count" -ne 0 ]; then
        echo "[FAIL] expected no call/mir_call in main for S2a externcall fixture (call=$call_count mir_call=$mir_call_count)"
        jq '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | {op,func,args,dst,mir_call}' "$TMP_MIR_JSON" | tail -n 120 || true
        test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: unexpected call shape"
        exit 1
    fi
}

check_externcall_shape

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

extract_first_int_line() {
    local text="$1"
    printf "%s\n" "$text" | rg '^-?[0-9]+$' | head -n 1
}

set +e
VM_OUTPUT=$(run_backend vm)
VM_RC=$?
set -e

if [ "$VM_RC" -eq 124 ]; then
    test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: vm timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

set +e
HAKO_OUTPUT=$(run_backend vm-hako)
HAKO_RC=$?
set -e

if [ "$HAKO_RC" -eq 124 ]; then
    test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: vm-hako timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$VM_RC" -ne 7 ]; then
    echo "[FAIL] expected vm rc=7, got rc=$VM_RC"
    echo "$VM_OUTPUT" | tail -n 80 || true
    test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: vm rc mismatch"
    exit 1
fi

if [ "$HAKO_RC" -ne 7 ]; then
    echo "[FAIL] expected vm-hako rc=7, got rc=$HAKO_RC"
    echo "$HAKO_OUTPUT" | tail -n 120 || true
    test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: vm-hako rc mismatch"
    exit 1
fi

VM_FIRST_INT=$(extract_first_int_line "$VM_OUTPUT")
HAKO_FIRST_INT=$(extract_first_int_line "$HAKO_OUTPUT")

if [ "$VM_FIRST_INT" != "9" ]; then
    echo "[FAIL] expected vm first integer line=9, got '$VM_FIRST_INT'"
    echo "$VM_OUTPUT" | tail -n 120 || true
    test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: vm print mismatch"
    exit 1
fi

if [ "$HAKO_FIRST_INT" != "9" ]; then
    echo "[FAIL] expected vm-hako first integer line=9, got '$HAKO_FIRST_INT'"
    echo "$HAKO_OUTPUT" | tail -n 120 || true
    test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: vm-hako print mismatch"
    exit 1
fi

if [ "$VM_FIRST_INT" != "$HAKO_FIRST_INT" ]; then
    echo "[FAIL] first integer line mismatch: vm='$VM_FIRST_INT' vm-hako='$HAKO_FIRST_INT'"
    test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: stdout parity mismatch"
    exit 1
fi

if [ "$VM_RC" -ne "$HAKO_RC" ]; then
    echo "[FAIL] parity mismatch: vm=$VM_RC vm-hako=$HAKO_RC"
    test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: parity mismatch"
    exit 1
fi

if echo "$HAKO_OUTPUT" | rg -q '^\[vm-hako/unimplemented\]'; then
    echo "[FAIL] vm-hako reported unimplemented on S2(externcall print 1-int) fixture"
    echo "$HAKO_OUTPUT" | tail -n 120 || true
    test_fail "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: unexpected unimplemented tag"
    exit 1
fi

test_pass "phase29z_vm_hako_s2_externcall_print_i64_return_parity_vm: PASS (vm=$VM_RC vm-hako=$HAKO_RC print=$VM_FIRST_INT)"

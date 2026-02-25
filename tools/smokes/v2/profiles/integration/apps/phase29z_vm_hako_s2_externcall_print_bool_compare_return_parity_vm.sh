#!/bin/bash
# Phase29z-S2b: vm-hako S2 parity smoke (externcall print 1-reg from compare)
#
# Contract:
# - S2(compare->externcall print) fixture executes on both `--backend vm` and `--backend vm-hako`.
# - MIR(JSON v0, unified-call=0) contains `compare` + `externcall(func=nyash.console.log,args=1)` in `main`.
# - `main` contains no `call`/`mir_call` print path.
# - Normalized print lines (`true/false` => `1/0`) must be `1 0`; exit codes must match and be `7`.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

INPUT="${1:-$NYASH_ROOT/apps/tests/phase29z_vm_hako_s2_externcall_print_bool_compare_return_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -f "$INPUT" ]; then
    test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: fixture missing: $INPUT"
    exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
    test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: jq not found"
    exit 1
fi

TMP_MIR_JSON="${TMPDIR:-/tmp}/phase29z_vm_hako_s2b_externcall_print_bool_$$.json"
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
        test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: emit-mir timed out (>${RUN_TIMEOUT_SECS}s)"
        exit 1
    fi
    if [ "$rc" -ne 0 ]; then
        echo "[FAIL] emit-mir-json failed rc=$rc"
        echo "$output" | tail -n 80 || true
        test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: emit-mir failed"
        exit 1
    fi

    local extern_count
    local compare_count
    local linked_count
    local call_count
    local mir_call_count
    extern_count=$(jq '[.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="externcall" and .func=="nyash.console.log" and (.args|length)==1)] | length' "$TMP_MIR_JSON")
    compare_count=$(jq '[.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="compare")] | length' "$TMP_MIR_JSON")
    linked_count=$(jq '[.functions[] | select(.name=="main") as $main | ($main.blocks[].instructions[] | select(.op=="compare") | .dst) as $dst | $main.blocks[].instructions[] | select(.op=="externcall" and .func=="nyash.console.log" and (.args|length)==1 and .args[0]==$dst)] | length' "$TMP_MIR_JSON")
    call_count=$(jq '[.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="call")] | length' "$TMP_MIR_JSON")
    mir_call_count=$(jq '[.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="mir_call")] | length' "$TMP_MIR_JSON")

    if [ "$extern_count" -lt 2 ] || [ "$compare_count" -lt 2 ] || [ "$linked_count" -lt 2 ]; then
        echo "[FAIL] expected compare-origin externcall print shape (extern=$extern_count compare=$compare_count linked=$linked_count)"
        jq '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | {op,dst,lhs,rhs,func,args,mir_call}' "$TMP_MIR_JSON" | tail -n 200 || true
        test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: compare->externcall shape missing"
        exit 1
    fi
    if [ "$call_count" -ne 0 ] || [ "$mir_call_count" -ne 0 ]; then
        echo "[FAIL] expected no call/mir_call in main for S2b compare fixture (call=$call_count mir_call=$mir_call_count)"
        jq '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | {op,func,args,dst,mir_call}' "$TMP_MIR_JSON" | tail -n 120 || true
        test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: unexpected call shape"
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

extract_bool01_lines() {
    local text="$1"
    printf "%s\n" "$text" | awk '
      /^-?[0-9]+$/ { print $0; next }
      /^true$/ { print "1"; next }
      /^false$/ { print "0"; next }
    '
}

first_two_bool01() {
    local text="$1"
    extract_bool01_lines "$text" | head -n 2 | tr '\n' ' ' | sed 's/ $//'
}

set +e
VM_OUTPUT=$(run_backend vm)
VM_RC=$?
set -e

if [ "$VM_RC" -eq 124 ]; then
    test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: vm timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

set +e
HAKO_OUTPUT=$(run_backend vm-hako)
HAKO_RC=$?
set -e

if [ "$HAKO_RC" -eq 124 ]; then
    test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: vm-hako timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$VM_RC" -ne 7 ]; then
    echo "[FAIL] expected vm rc=7, got rc=$VM_RC"
    echo "$VM_OUTPUT" | tail -n 120 || true
    test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: vm rc mismatch"
    exit 1
fi

if [ "$HAKO_RC" -ne 7 ]; then
    echo "[FAIL] expected vm-hako rc=7, got rc=$HAKO_RC"
    echo "$HAKO_OUTPUT" | tail -n 120 || true
    test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: vm-hako rc mismatch"
    exit 1
fi

VM_BOOL01=$(first_two_bool01 "$VM_OUTPUT")
HAKO_BOOL01=$(first_two_bool01 "$HAKO_OUTPUT")

if [ "$VM_BOOL01" != "1 0" ]; then
    echo "[FAIL] expected vm normalized bool lines '1 0', got '$VM_BOOL01'"
    echo "$VM_OUTPUT" | tail -n 160 || true
    test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: vm bool print mismatch"
    exit 1
fi

if [ "$HAKO_BOOL01" != "1 0" ]; then
    echo "[FAIL] expected vm-hako normalized bool lines '1 0', got '$HAKO_BOOL01'"
    echo "$HAKO_OUTPUT" | tail -n 160 || true
    test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: vm-hako bool print mismatch"
    exit 1
fi

if [ "$VM_BOOL01" != "$HAKO_BOOL01" ]; then
    echo "[FAIL] normalized bool print mismatch: vm='$VM_BOOL01' vm-hako='$HAKO_BOOL01'"
    test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: stdout parity mismatch"
    exit 1
fi

if [ "$VM_RC" -ne "$HAKO_RC" ]; then
    echo "[FAIL] parity mismatch: vm=$VM_RC vm-hako=$HAKO_RC"
    test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: parity mismatch"
    exit 1
fi

if echo "$HAKO_OUTPUT" | rg -q '^\[vm-hako/unimplemented\]'; then
    echo "[FAIL] vm-hako reported unimplemented on S2(compare->externcall print) fixture"
    echo "$HAKO_OUTPUT" | tail -n 120 || true
    test_fail "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: unexpected unimplemented tag"
    exit 1
fi

test_pass "phase29z_vm_hako_s2_externcall_print_bool_compare_return_parity_vm: PASS (vm=$VM_RC vm-hako=$HAKO_RC bool01='$VM_BOOL01')"

#!/bin/bash
# RC/GC alignment G-RC-1: lifecycle parity gate (VM/LLVM)
#
# Contract pin:
# - Representative lifecycle fixtures must keep VM/LLVM exit-code parity.
# - Expected exits are inventory-driven and fixed by SSOT.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/vm_route_pin.sh"
require_env || exit 2

CASES_FILE="$NYASH_ROOT/tools/checks/rc_gc_alignment_g1_lifecycle_cases.txt"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-45}"

llvm_harness_enabled() {
    "$NYASH_BIN" --version 2>/dev/null | grep -q "features.*llvm"
}

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "rc_gc_alignment_g1_lifecycle_parity_vm_llvm: step failed: $cmd"
        exit 1
    fi
}

run_backend_case() {
    local backend="$1"
    local fixture="$2"
    local output
    local rc
    local -a cmd=("$NYASH_BIN" --backend "$backend" "$fixture")

    local -a env_prefix=()
    if [[ "$backend" = "vm" ]]; then
        env_prefix=(run_with_vm_route_pin env NYASH_DISABLE_PLUGINS=1)
    else
        env_prefix=(env NYASH_LLVM_USE_HARNESS=1 NYASH_DISABLE_PLUGINS=1)
    fi

    set +e
    output=$(
        "${env_prefix[@]}" timeout "$RUN_TIMEOUT_SECS" "${cmd[@]}" 2>&1
    )
    rc=$?
    set -e

    printf '%s' "$output"
    return "$rc"
}

if [ ! -f "$CASES_FILE" ]; then
    test_fail "rc_gc_alignment_g1_lifecycle_parity_vm_llvm: cases file missing: $CASES_FILE"
    exit 1
fi

run_step "tools/checks/rc_gc_alignment_g1_guard.sh"

if ! llvm_harness_enabled; then
    test_skip "rc_gc_alignment_g1_lifecycle_parity_vm_llvm: hakorune built without --features llvm (harness lane unavailable)"
    exit 0
fi

while IFS= read -r row || [ -n "$row" ]; do
    [ -z "$row" ] && continue
    IFS='|' read -r case_id fixture_rel expected_exit <<<"$row"
    fixture="$NYASH_ROOT/$fixture_rel"
    if [ ! -f "$fixture" ]; then
        test_fail "rc_gc_alignment_g1_lifecycle_parity_vm_llvm: fixture missing: $fixture"
        exit 1
    fi

    set +e
    out_vm="$(run_backend_case vm "$fixture")"
    rc_vm=$?
    set -e
    if [ "$rc_vm" -eq 124 ]; then
        test_fail "rc_gc_alignment_g1_lifecycle_parity_vm_llvm: VM timed out (>${RUN_TIMEOUT_SECS}s, case=$case_id)"
        exit 1
    fi

    set +e
    out_llvm="$(run_backend_case llvm "$fixture")"
    rc_llvm=$?
    set -e
    if [ "$rc_llvm" -eq 124 ]; then
        test_fail "rc_gc_alignment_g1_lifecycle_parity_vm_llvm: LLVM timed out (>${RUN_TIMEOUT_SECS}s, case=$case_id)"
        exit 1
    fi

    if [ "$rc_vm" -ne "$expected_exit" ]; then
        echo "[INFO] vm output (case=$case_id):"
        echo "$out_vm" | tail -n 120 || true
        test_fail "rc_gc_alignment_g1_lifecycle_parity_vm_llvm: VM exit mismatch (case=$case_id expected=$expected_exit got=$rc_vm)"
        exit 1
    fi
    if [ "$rc_llvm" -ne "$expected_exit" ]; then
        echo "[INFO] llvm output (case=$case_id):"
        echo "$out_llvm" | tail -n 120 || true
        test_fail "rc_gc_alignment_g1_lifecycle_parity_vm_llvm: LLVM exit mismatch (case=$case_id expected=$expected_exit got=$rc_llvm)"
        exit 1
    fi
    if [ "$rc_vm" -ne "$rc_llvm" ]; then
        echo "[INFO] vm output (case=$case_id):"
        echo "$out_vm" | tail -n 120 || true
        echo "[INFO] llvm output (case=$case_id):"
        echo "$out_llvm" | tail -n 120 || true
        test_fail "rc_gc_alignment_g1_lifecycle_parity_vm_llvm: parity mismatch (case=$case_id vm=$rc_vm llvm=$rc_llvm)"
        exit 1
    fi
done < <(grep -v '^[[:space:]]*#' "$CASES_FILE" | sed '/^[[:space:]]*$/d')

test_pass "rc_gc_alignment_g1_lifecycle_parity_vm_llvm: PASS (G-RC-1 lifecycle parity locked)"

#!/bin/bash
# Phase 29x X59: ABI borrowed/owned conformance extension gate
#
# Contract pin:
# - Replay X51 Core C ABI delegation guard before borrowed/owned checks.
# - Execute borrowed/owned matrix tests as single-entry evidence.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

CASES_FILE="$NYASH_ROOT/tools/checks/phase29x_abi_borrowed_owned_matrix_cases.txt"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-180}"

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_abi_borrowed_owned_conformance_vm: step failed: $cmd"
        exit 1
    fi
}

run_step "tools/checks/phase29x_abi_borrowed_owned_matrix_guard.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_core_cabi_delegation_guard_vm.sh"

set +e
OUTPUT=$(cd "$NYASH_ROOT" && timeout "$RUN_TIMEOUT_SECS" cargo test -p nyash_kernel handle_abi_borrowed_owned_ -- --nocapture --test-threads=1 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29x_abi_borrowed_owned_conformance_vm: timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
    echo "[FAIL] borrowed/owned matrix cargo test failed"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output tail:"
    echo "$OUTPUT" | tail -n 120 || true
    test_fail "phase29x_abi_borrowed_owned_conformance_vm: cargo test failed"
    exit 1
fi

while IFS='|' read -r case_id test_name; do
    if [[ -z "${case_id}" || "${case_id}" =~ ^# ]]; then
        continue
    fi
    if ! echo "$OUTPUT" | rg -q "\b${test_name}\b"; then
        echo "[FAIL] borrowed/owned matrix output missing test: $test_name (case=$case_id)"
        echo "[INFO] Output tail:"
        echo "$OUTPUT" | tail -n 120 || true
        test_fail "phase29x_abi_borrowed_owned_conformance_vm: matrix coverage drift"
        exit 1
    fi
done <"$CASES_FILE"

test_pass "phase29x_abi_borrowed_owned_conformance_vm: PASS (X59 borrowed/owned matrix locked)"

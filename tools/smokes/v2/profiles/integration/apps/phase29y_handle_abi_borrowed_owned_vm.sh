#!/bin/bash
# Phase 29y.1 Task 1b: ABI borrowed/owned conformance smoke
#
# Contract pin (10-ABI-SSOT.md §3):
# - args borrowed
# - return owned
# - caller can release borrowed path while owned return remains alive

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-120}"

set +e
OUTPUT=$(cd "$NYASH_ROOT" && timeout "$RUN_TIMEOUT_SECS" cargo test -p nyash_kernel handle_abi_borrowed_owned_conformance -- --nocapture 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29y_handle_abi_borrowed_owned_vm: timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
    echo "[FAIL] borrowed/owned conformance test failed"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output tail:"
    echo "$OUTPUT" | tail -n 80 || true
    test_fail "phase29y_handle_abi_borrowed_owned_vm: cargo test failed"
    exit 1
fi

test_pass "phase29y_handle_abi_borrowed_owned_vm: PASS"

#!/bin/bash
# Phase 29y.1 Task 1: Handle ABI shim smoke test (LLVM harness)
#
# Purpose:
# - Ensure the LLVM backend can execute the fixture end-to-end.
# - Use exit-code SSOT (stdout may include harness/runtime logs).
#
# SSOT: docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

if ! can_run_llvm; then
    test_skip "phase29y_handle_abi_llvm: LLVM backend not available"
    exit 0
fi

INPUT="$NYASH_ROOT/apps/tests/phase29y_handle_abi.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-60}

set +e
OUTPUT=$(NYASH_LLVM_USE_HARNESS=1 NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend llvm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29y_handle_abi_llvm: timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -eq 0 ]; then
    test_pass "phase29y_handle_abi_llvm: PASS (exit 0)"
    exit 0
fi

echo "[FAIL] Expected exit 0"
echo "[INFO] Exit code: $EXIT_CODE"
echo "[INFO] Output:"
echo "$OUTPUT" | head -n 60 || true
test_fail "phase29y_handle_abi_llvm: exit code mismatch"
exit 1

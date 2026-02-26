#!/bin/bash
# Phase 29y.1 Task 1: Handle ABI shim smoke test (VM)
#
# Purpose:
# - Establish a stable, exit-code-based smoke for handle lifecycle behavior.
# - Keep this in integration (not quick): Phase 29y is docs-first and this is a pilot.
#
# SSOT: docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase29y_handle_abi.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-30}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29y_handle_abi_vm: timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

# Exit code SSOT (stdout may include logs in some environments)
if [ "$EXIT_CODE" -eq 0 ]; then
    test_pass "phase29y_handle_abi_vm: PASS (exit 0)"
    exit 0
fi

echo "[FAIL] Expected exit 0"
echo "[INFO] Exit code: $EXIT_CODE"
echo "[INFO] Output:"
echo "$OUTPUT" | head -n 40 || true
test_fail "phase29y_handle_abi_vm: exit code mismatch"
exit 1


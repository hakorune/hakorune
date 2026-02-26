#!/bin/bash
# Phase 284 P2: "return in loop" VM smoke test
# Purpose: Verify return-in-loop works correctly in VM backend
# Uses existing Phase 286 Pattern5 fixture for reuse
#
# Expected: exit code 7 or "RC: 7" output

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase286_pattern5_return_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase284_p2_return_in_loop_vm: timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

# Expected: exit code 7 or "RC: 7" in output
if [ "$EXIT_CODE" -eq 7 ] || echo "$OUTPUT" | grep -qE "(^7$|RC: 7$)"; then
    test_pass "phase284_p2_return_in_loop_vm: return-in-loop succeeded (exit/output: 7)"
    exit 0
else
    echo "[FAIL] Unexpected result (expected: 7)"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 20 || true
    test_fail "phase284_p2_return_in_loop_vm: Unexpected result"
    exit 1
fi

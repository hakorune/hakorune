#!/bin/bash
# Phase 286 P3.2: Pattern5 "infinite loop with break" test
# Tests: loop(true) with if (i == 3) { break } and carrier update
#
# Expected: Output "3" (sum = 0 + 1 + 1 + 1 = 3 for i = 0, 1, 2)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase286_pattern5_break_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase286_pattern5_break_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

# Expected output: "3" (may be "RC: 3" or "3" format)
if echo "$OUTPUT" | grep -qE "(^3$|RC: 3$)"; then
    test_pass "phase286_pattern5_break_vm: Pattern5 break succeeded (output: 3)"
    exit 0
else
    echo "[FAIL] Unexpected output (expected: 3)"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 20 || true
    test_fail "phase286_pattern5_break_vm: Unexpected output"
    exit 1
fi

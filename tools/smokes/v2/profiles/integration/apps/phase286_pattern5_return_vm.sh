#!/bin/bash
# Phase 286 P3.2: Pattern5 "infinite loop with early return" test
# Tests: loop(true) with if (i == 3) { return 7 }
#
# Expected: Output "7"

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase286_pattern5_return_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase286_pattern5_return_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

# Expected output: "7" (may be "RC: 7" or "7" format)
if echo "$OUTPUT" | grep -qE "(^7$|RC: 7$)"; then
    test_pass "phase286_pattern5_return_vm: Pattern5 early-return succeeded (output: 7)"
    exit 0
else
    echo "[FAIL] Unexpected output (expected: 7)"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 20 || true
    test_fail "phase286_pattern5_return_vm: Unexpected output"
    exit 1
fi

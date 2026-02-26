#!/bin/bash
# Phase 286 P3.1: Pattern2 "break without update" test
# Tests: Pattern2 where break path has NO carrier update
#
# Key insight (after_bb PHI):
#   - break path: carrier_break = carrier_current (no update)
#   - after_bb PHI: carrier_out = PHI(header: carrier_current, break_then: carrier_break)
#
# Expected: Output "11" (sum=10 init, sum+1 at i==0, break at i==1)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase286_pattern2_break_no_update_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase286_pattern2_break_no_update_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

# Expected output: "11" (may be "RC: 11" or "11" format)
# Exit code may be 11 (return value) or 0
if echo "$OUTPUT" | grep -qE "(^11$|RC: 11$)"; then
    test_pass "phase286_pattern2_break_no_update_vm: Pattern2 break-no-update succeeded (output: 11)"
    exit 0
else
    echo "[FAIL] Unexpected output (expected: 11)"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 20 || true
    test_fail "phase286_pattern2_break_no_update_vm: Unexpected output"
    exit 1
fi

#!/bin/bash
# Phase 29ac P3: Pattern7 split-scan near-miss OK minimal (renamed)
# Tests: split("a,b,c", ",") -> length 3

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase29ab_pattern7_splitscan_nearmiss_ok_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29ab_pattern7_splitscan_nearmiss_ok_min_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -eq 3 ]; then
    test_pass "phase29ab_pattern7_splitscan_nearmiss_ok_min_vm: RC=3 (expected)"
    exit 0
fi

echo "[FAIL] Expected exit 3, got $EXIT_CODE"
echo "$OUTPUT" | tail -n 40 || true
test_fail "phase29ab_pattern7_splitscan_nearmiss_ok_min_vm: Unexpected RC"
exit 1

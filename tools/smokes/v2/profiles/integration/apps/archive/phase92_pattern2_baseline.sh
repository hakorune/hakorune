#!/bin/bash
# Phase 92: Pattern2 baseline regression test
# Tests: Verifies Pattern2 break functionality doesn't regress with Phase 92 changes
#
# Phase 92 scope:
# - Body-local variable support in condition lowering
# - Variable resolution priority: ConditionEnv → LoopBodyLocalEnv
# - Break condition lowering reordered (after body-local init)
#
# This test uses existing Pattern2Break tests as baseline (Level 1 in P4-E2E-PLAN.md)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

# Create tmp directory
mkdir -p "$NYASH_ROOT/tmp"

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

# ===== Case A: Simple while loop with break (Pattern2 baseline) =====
echo "[INFO] Case A: loop_min_while.hako (Pattern2 baseline)"

INPUT_A="$NYASH_ROOT/apps/tests/loop_min_while.hako"

set +e
OUTPUT_A=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$INPUT_A" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    echo "[FAIL] Case A: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 0 ]; then
    # Expected output: "0\n1\n2\n" (prints 0, 1, 2 then breaks at i==3)
    if echo "$OUTPUT_A" | grep -q "^0$" && echo "$OUTPUT_A" | grep -q "^1$" && echo "$OUTPUT_A" | grep -q "^2$"; then
        echo "[PASS] Case A: Pattern2 break baseline verified (output: 0, 1, 2)"
        PASS_COUNT=$((PASS_COUNT + 1))
    else
        echo "[FAIL] Case A: Unexpected output (expected lines: 0, 1, 2)"
        echo "[INFO] Case A output:"
        echo "$OUTPUT_A" | head -n 20 || true
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
else
    echo "[FAIL] Case A: hakorune failed with exit code $EXIT_CODE"
    echo "[INFO] Case A output (tail):"
    echo "$OUTPUT_A" | tail -n 20 || true
    FAIL_COUNT=$((FAIL_COUNT + 1))
fi

# ===== Case B: Conditional increment (Phase 92 minimal test) =====
echo "[INFO] Case B: phase92_conditional_step_minimal.hako"

INPUT_B="$NYASH_ROOT/apps/tests/phase92_conditional_step_minimal.hako"

set +e
OUTPUT_B=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$INPUT_B" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    echo "[FAIL] Case B: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 0 ]; then
    # Expected output: "3" (i increments: 0→1→2→3, then breaks)
    if echo "$OUTPUT_B" | grep -q "^3$"; then
        echo "[PASS] Case B: Conditional increment baseline verified (output: 3)"
        PASS_COUNT=$((PASS_COUNT + 1))
    else
        echo "[FAIL] Case B: Unexpected output (expected: 3)"
        echo "[INFO] Case B output:"
        echo "$OUTPUT_B" | head -n 20 || true
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
else
    echo "[FAIL] Case B: hakorune failed with exit code $EXIT_CODE"
    echo "[INFO] Case B output (tail):"
    echo "$OUTPUT_B" | tail -n 20 || true
    FAIL_COUNT=$((FAIL_COUNT + 1))
fi

# ===== Summary =====
echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
    test_pass "phase92_pattern2_baseline: All tests passed"
    exit 0
else
    test_fail "phase92_pattern2_baseline: $FAIL_COUNT test(s) failed"
    exit 1
fi

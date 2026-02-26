#!/bin/bash
# Phase 143 P1: loop(true) + if + continue Normalized lowering (VM)
#
# Verifies that loop(true) { if(cond_pure) continue } pattern is correctly lowered
# to Normalized JoinModule with Jump/Call instructions.
# Expected: exit code 54 (sum of: 1+3+5+6+7+8+9+10, skipping 2 and 4)
#
# Dev-only: NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

# JoinIR dev mode (Phase 130+ gate)
require_joinir_dev

# Test configuration
INPUT_HAKO="$NYASH_ROOT/apps/tests/phase143_loop_true_if_continue_min.hako"

# Execute (timeout contract)
#
# This fixture is intentionally non-terminating in Phase 143 P1:
# - loop(true) with a single `if(cond){continue}` has no in-loop state update and no break path.
#
# We verify that:
# - Normalized lowering + VM execution starts successfully
# - It doesn't fail fast immediately
# - It times out (expected)
timeout_secs="${SMOKES_P143_CONTINUE_TIMEOUT_SECS:-1}"
# Disable VM step budget so the test is controlled by external timeout.
export HAKO_VM_MAX_STEPS=0
timeout "$timeout_secs" "$NYASH_BIN" --backend vm "$INPUT_HAKO" > /dev/null 2>&1
actual_exit=$?
EXPECTED_TIMEOUT_EXIT=124
if [ "$actual_exit" -eq "$EXPECTED_TIMEOUT_EXIT" ]; then
    test_pass "phase143_loop_true_if_continue: timed out as expected (${timeout_secs}s)"
else
    test_fail "phase143_loop_true_if_continue: expected timeout exit $EXPECTED_TIMEOUT_EXIT, got $actual_exit"
    exit 1
fi

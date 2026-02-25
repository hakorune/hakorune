#!/bin/bash
# Phase 143 P0: loop(true) + if + break Normalized lowering (VM)
#
# Verifies that loop(true) { if(cond_pure) break } pattern is correctly lowered
# to Normalized JoinModule with Jump/Return instructions.
# Expected: exit code 7 (condition true, immediate break, return 7)
#
# Dev-only: NYASH_JOINIR_DEV=1 HAKO_JOINIR_STRICT=1

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

# JoinIR dev mode (Phase 130+ gate)
require_joinir_dev

# Test configuration
INPUT_HAKO="$NYASH_ROOT/apps/tests/phase143_loop_true_if_break_min.hako"

# Execute (exit code contract)
EXPECTED_EXIT_CODE=7
"$NYASH_BIN" --backend vm "$INPUT_HAKO" > /dev/null 2>&1
actual_exit=$?
if [ "$actual_exit" -eq "$EXPECTED_EXIT_CODE" ]; then
    test_pass "phase143_loop_true_if_break: exit code $EXPECTED_EXIT_CODE matches"
else
    test_fail "phase143_loop_true_if_break: expected exit code $EXPECTED_EXIT_CODE, got $actual_exit"
    exit 1
fi

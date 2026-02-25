#!/bin/bash
# Phase 146 P0: If condition unified lowering (VM)
#
# Expected: exit code 7

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT_HAKO="$NYASH_ROOT/apps/tests/phase146_p0_if_cond_unified_min.hako"

EXPECTED_EXIT_CODE=7
"$NYASH_BIN" --backend vm "$INPUT_HAKO" >/dev/null 2>&1
actual_exit=$?

if [ "$actual_exit" -eq "$EXPECTED_EXIT_CODE" ]; then
  test_pass "phase146_p0_if_cond_unified_vm: exit code matches (7)"
else
  test_fail "phase146_p0_if_cond_unified_vm: expected 7, got $actual_exit"
  exit 1
fi


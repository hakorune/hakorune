#!/bin/bash
# Phase 29y.1 Task 2: RC insertion pass entry smoke (VM + selfcheck)
#
# Contract pin:
# 1) Default runtime path (without rc-insertion-minimal feature) remains behavior-stable.
# 2) Feature path selfcheck (`rc_insertion_selfcheck`) passes when rc-insertion-minimal is enabled.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase29y_rc_insertion_entry_noop_min.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
SELFCHECK_TIMEOUT_SECS="${SMOKES_RC_INSERTION_SELFCHECK_TIMEOUT_SECS:-120}"

if [ ! -f "$INPUT" ]; then
    test_fail "phase29y_rc_insertion_entry_vm: fixture missing: $INPUT"
    exit 1
fi

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29y_rc_insertion_entry_vm: fixture timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
    echo "[FAIL] Expected fixture exit 0"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 60 || true
    test_fail "phase29y_rc_insertion_entry_vm: fixture execution failed"
    exit 1
fi

set +e
SELFCHECK_OUT=$(cd "$NYASH_ROOT" && timeout "$SELFCHECK_TIMEOUT_SECS" cargo run -q --bin rc_insertion_selfcheck --features rc-insertion-minimal 2>&1)
SELFCHECK_RC=$?
set -e

if [ "$SELFCHECK_RC" -eq 124 ]; then
    test_fail "phase29y_rc_insertion_entry_vm: rc_insertion_selfcheck timed out (>${SELFCHECK_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$SELFCHECK_RC" -ne 0 ]; then
    echo "[FAIL] rc_insertion_selfcheck failed"
    echo "[INFO] Exit code: $SELFCHECK_RC"
    echo "[INFO] Output:"
    echo "$SELFCHECK_OUT" | tail -n 80 || true
    test_fail "phase29y_rc_insertion_entry_vm: selfcheck failed"
    exit 1
fi

test_pass "phase29y_rc_insertion_entry_vm: PASS (fixture rc=0 + selfcheck rc=0)"

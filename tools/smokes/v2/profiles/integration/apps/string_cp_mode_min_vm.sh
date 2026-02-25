#!/bin/bash
# String CP/byte mode parity for primitive String and StringBox.

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/string_cp_mode_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

run_case() {
    local mode="$1"
    local expect_len="$2"

    set +e
    OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_STR_CP="$mode" "$NYASH_BIN" "$INPUT" 2>&1)
    EXIT_CODE=$?
    set -e

    if [ "$EXIT_CODE" -eq 124 ]; then
        test_fail "string_cp_mode_min_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
        exit 1
    fi

    OUTPUT_CLEAN="$OUTPUT"

    expect_line "$((1000 + expect_len))"
    expect_line "$((2000 + expect_len))"
}

expect_line() {
    local line="$1"
    if ! echo "$OUTPUT_CLEAN" | grep -Fxq "$line"; then
        echo "[FAIL] Missing expected line: $line"
        echo "[INFO] Output (clean):"
        echo "$OUTPUT_CLEAN" | tail -n 20 || true
        test_fail "string_cp_mode_min_vm: output mismatch"
        exit 1
    fi
}

run_case 0 7
run_case 1 3

test_pass "string_cp_mode_min_vm: NYASH_STR_CP byte/cp modes match for String and StringBox"

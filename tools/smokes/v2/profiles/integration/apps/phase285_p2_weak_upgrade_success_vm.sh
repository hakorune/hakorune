#!/bin/bash
# Phase 285 P2: "weak upgrade success" VM smoke test
# Purpose: Verify weak_to_strong() succeeds when strong ref is alive (VM/LLVM parity)
# Uses existing phase285_weak_basic.hako (modified: exit 0 → exit 2)
#
# Expected: exit code 2 (non-zero success code, distinguishes from "nothing happened")

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase285_weak_basic.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-30}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase285_p2_weak_upgrade_success_vm: timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

# Expected: exit code 2 (non-zero success code)
if [ "$EXIT_CODE" -eq 2 ]; then
    test_pass "phase285_p2_weak_upgrade_success_vm: weak_to_strong succeeded (exit: 2)"
    exit 0
else
    echo "[FAIL] Unexpected result (expected: exit 2)"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 30 || true
    test_fail "phase285_p2_weak_upgrade_success_vm: exit code mismatch"
    exit 1
fi

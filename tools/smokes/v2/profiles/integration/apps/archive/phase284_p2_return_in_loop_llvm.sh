#!/bin/bash
# Phase 284 P2: "return in loop" LLVM harness smoke test
# Purpose: Verify return-in-loop works correctly in LLVM backend (VM/LLVM parity)
# Uses existing Phase 286 Pattern5 fixture for reuse
#
# Expected: exit code 7 or "RC: 7" output (same as VM)
# SKIP: If LLVM backend not available in build

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

# LLVM feature check (v2 smoke SKIP convention)
if ! can_run_llvm; then
    test_skip "phase284_p2_return_in_loop_llvm: LLVM backend not available in this build"
    exit 0
fi

INPUT="$NYASH_ROOT/apps/tests/phase286_pattern5_return_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-30}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_LLVM_USE_HARNESS=1 NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" --backend llvm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase284_p2_return_in_loop_llvm: timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

# Expected: exit code 7 (same as VM) - exit code SSOT
if [ "$EXIT_CODE" -eq 7 ]; then
    test_pass "phase284_p2_return_in_loop_llvm: return-in-loop succeeded (exit: 7, VM/LLVM parity)"
    exit 0
else
    echo "[FAIL] Unexpected result (expected: exit 7, same as VM)"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output (for debugging):"
    echo "$OUTPUT" | head -n 30 || true
    test_fail "phase284_p2_return_in_loop_llvm: VM/LLVM parity failed"
    exit 1
fi

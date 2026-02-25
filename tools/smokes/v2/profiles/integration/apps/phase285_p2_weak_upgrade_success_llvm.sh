#!/bin/bash
# Phase 285 P2: "weak upgrade success" LLVM harness smoke test
# Purpose: Verify weak_to_strong() succeeds when strong ref is alive (VM/LLVM parity)
# Uses existing phase285_weak_basic.hako (modified: exit 0 → exit 2)
#
# Expected: exit code 2 (same as VM) or SKIP if LLVM backend not available
# SKIP: If LLVM backend/harness not available in build

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

# LLVM feature check (v2 smoke SKIP convention)
if ! can_run_llvm; then
    test_skip "phase285_p2_weak_upgrade_success_llvm: LLVM backend not available in this build"
    exit 0
fi

INPUT="$NYASH_ROOT/apps/tests/phase285_weak_basic.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-30}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_LLVM_USE_HARNESS=1 NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" --backend llvm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase285_p2_weak_upgrade_success_llvm: timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

# Expected: exit code 2 (same as VM)
if [ "$EXIT_CODE" -eq 2 ]; then
    test_pass "phase285_p2_weak_upgrade_success_llvm: weak_to_strong succeeded (exit: 2, VM/LLVM parity)"
    exit 0
else
    echo "[FAIL] Unexpected result (expected: exit 2, same as VM)"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 30 || true
    test_fail "phase285_p2_weak_upgrade_success_llvm: VM/LLVM parity failed"
    exit 1
fi

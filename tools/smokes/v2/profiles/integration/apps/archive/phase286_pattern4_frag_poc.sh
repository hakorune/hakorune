#!/bin/bash
# Phase 286 P2: Pattern4 → Frag PoC test
# Tests: Pattern4 (Loop with Continue) using Plan/Frag SSOT
#
# PoC Goal:
#   Pattern4 → DomainPlan → CorePlan → Frag → emit_frag()
#   (Skip: JoinIR → bridge → merge)
#
# Expected: Output "6" (sum of 1+2+3, loop skips i==0)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase286_pattern4_frag_poc.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase286_pattern4_frag_poc: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
elif [ "$EXIT_CODE" -eq 0 ]; then
    # Expected output: "6"
    if echo "$OUTPUT" | grep -q "^6$"; then
        test_pass "phase286_pattern4_frag_poc: Pattern4 Frag PoC succeeded (output: 6)"
        exit 0
    else
        echo "[FAIL] Unexpected output (expected: 6)"
        echo "[INFO] Output:"
        echo "$OUTPUT" | head -n 20 || true
        test_fail "phase286_pattern4_frag_poc: Unexpected output"
        exit 1
    fi
else
    echo "[FAIL] hakorune failed with exit code $EXIT_CODE"
    echo "[INFO] Output (tail):"
    echo "$OUTPUT" | tail -n 20 || true
    test_fail "phase286_pattern4_frag_poc: hakorune failed"
    exit 1
fi

#!/bin/bash
# Phase 286 P2.1: Pattern1 → Frag PoC test
# Tests: Pattern1 (SimpleWhile) using Plan/Frag SSOT
#
# PoC Goal:
#   Pattern1 → DomainPlan → CorePlan → Frag → emit_frag()
#   (Skip: JoinIR → bridge → merge)
#
# Expected: return 3 (loop 3 times: i=0,1,2 → i=3 で終了)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase286_pattern1_frag_poc.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase286_pattern1_frag_poc: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
elif [ "$EXIT_CODE" -eq 3 ]; then
    # Expected: return 3 (exit code)
    test_pass "phase286_pattern1_frag_poc: Pattern1 Frag PoC succeeded (return: 3)"
    exit 0
else
    echo "[FAIL] Unexpected exit code (expected: 3, got: $EXIT_CODE)"
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 20 || true
    test_fail "phase286_pattern1_frag_poc: Unexpected exit code"
    exit 1
fi

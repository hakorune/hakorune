#!/bin/bash
# Phase 135: MIR verify regression for Trim(A-3) + Pattern2 lowering
# Purpose: prevent ValueId/SSA corruption introduced by allocator mismatch or duplicated boundary copies.
#
# P0: ConditionLoweringBox uses SSOT allocator (ConditionContext.alloc_value)
# P1: contract_checks Fail-Fast:
#     - verify_condition_bindings_consistent (alias allowed, conflict fails)
#     - verify_header_phi_dsts_not_redefined (PHI dst overwrite detection)
#
# Expected: `--verify` PASS for the Phase 133 Trim-derived fixture.

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase133_json_skip_whitespace_min.hako"

echo "[INFO] Phase 135: --verify (phase133_json_skip_whitespace_min.hako)"
OUT="$(timeout "${RUN_TIMEOUT_SECS:-60}" "$NYASH_BIN" --verify "$INPUT" 2>&1)"
RC=$?

if [ "$RC" -ne 0 ]; then
    echo "[FAIL] verify: hakorune --verify failed (rc=$RC)"
    echo "[INFO] output (tail):"
    echo "$OUT" | tail -n 120 || true
    exit 1
fi

if echo "$OUT" | grep -q "MIR verification failed"; then
    echo "[FAIL] verify: MIR verification failed"
    echo "[INFO] output (tail):"
    echo "$OUT" | tail -n 120 || true
    exit 1
fi

echo "[PASS] verify: MIR is valid (SSA/ValueId OK)"
exit 0


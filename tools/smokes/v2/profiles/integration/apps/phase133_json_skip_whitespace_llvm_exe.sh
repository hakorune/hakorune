#!/bin/bash
# Phase 133 P1: Promoted carrier join_id regression (compile-only)
# Tests: Pattern2 + Trim(A-3) promotion no longer triggers:
#   [phase229] promoted carrier '<name>' has no join_id
#
# Note:
# - This smoke is intentionally compile-only (`--dump-mir`).
# - Runtime/LLVM EXE execution depends on Box providers/plugins and is out of scope here.

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

# ===== Compile-only: ensure JoinIR lowering does not panic =====
echo "[INFO] Phase 133 P1: compile-only (phase133_json_skip_whitespace_min.hako)"
INPUT="$NYASH_ROOT/apps/tests/phase133_json_skip_whitespace_min.hako"
OUT="$(timeout "${RUN_TIMEOUT_SECS:-30}" "$NYASH_BIN" --dump-mir "$INPUT" 2>&1)"
RC=$?

if [ "$RC" -ne 0 ]; then
    echo "[FAIL] compile: hakorune --dump-mir failed (rc=$RC)"
    echo "[INFO] output (tail):"
    echo "$OUT" | tail -n 80 || true
    exit 1
fi

if echo "$OUT" | grep -q "phase229.*promoted carrier.*join_id"; then
    echo "[FAIL] compile: promoted carrier join_id error still present"
    echo "[INFO] output (match):"
    echo "$OUT" | grep -n "phase229" | tail -n 20 || true
    exit 1
fi

echo "[PASS] compile: no promoted carrier join_id error"
exit 0

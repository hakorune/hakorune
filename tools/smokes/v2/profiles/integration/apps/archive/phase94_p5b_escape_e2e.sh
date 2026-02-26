#!/bin/bash
# Phase 94: P5b escape handling complete E2E (VM)
#
# Verifies:
# - Pattern2Break JoinIR lowering supports:
#   - body-local `ch` conditional override (derived Select)
#   - loop counter skip on escape (i += 2 when ch == "\\")
# - Fixture prints: hello" world
#
# Notes:
# - We keep this in `integration` (not `quick`) to avoid adding more output-heavy cases
#   to the fastest profile.

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

INPUT="$NYASH_ROOT/tools/selfhost/test_pattern5b_escape_minimal.hako"

echo "[INFO] Phase 94: P5b escape E2E (VM) - $INPUT"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  HAKO_JOINIR_STRICT=1 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "[FAIL] hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 0 ]; then
  if echo "$OUTPUT" | grep -q '^hello" world$'; then
    echo "[PASS] Output verified: hello\" world"
    PASS_COUNT=$((PASS_COUNT + 1))
  else
    echo "[FAIL] Unexpected output (expected line: hello\" world)"
    echo "[INFO] output (tail):"
    echo "$OUTPUT" | tail -n 50 || true
    FAIL_COUNT=$((FAIL_COUNT + 1))
  fi
else
  echo "[FAIL] hakorune failed with exit code $EXIT_CODE"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 50 || true
  FAIL_COUNT=$((FAIL_COUNT + 1))
fi

echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
  test_pass "phase94_p5b_escape_e2e: All tests passed"
  exit 0
else
  test_fail "phase94_p5b_escape_e2e: $FAIL_COUNT test(s) failed"
  exit 1
fi


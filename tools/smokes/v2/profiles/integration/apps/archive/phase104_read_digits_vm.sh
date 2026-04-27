#!/bin/bash
# Phase 104: read_digits loop(true) + break-only.
#
# Historical filename note:
# - This archived smoke used to force `--backend vm`.
# - `--backend vm` is now a raw legacy compat/proof ingress and currently
#   times out on this fixture.
# - Keep the filename stable, but verify behavior on the mainline MIR route.

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

INPUT="$NYASH_ROOT/apps/tests/phase104_read_digits_loop_true_min.hako"

echo "[INFO] Phase 104: read_digits loop(true) break-only (mainline MIR; historical _vm filename) - $INPUT"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=0 \
  HAKO_JOINIR_STRICT=1 \
  "$NYASH_BIN" --backend mir "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "[FAIL] hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 0 ]; then
  EXPECTED=$'2\n1'
  CLEAN=$(printf "%s\n" "$OUTPUT" | grep -E '^-?[0-9]+$' | head -n 2 | paste -sd '\n' - | tr -d '\r')
  if [ "$CLEAN" = "$EXPECTED" ]; then
    echo "[PASS] Output verified: 2 then 1"
    PASS_COUNT=$((PASS_COUNT + 1))
  else
    echo "[FAIL] Unexpected output (expected lines: 2 then 1)"
    echo "[INFO] output (tail):"
    echo "$OUTPUT" | tail -n 60 || true
    FAIL_COUNT=$((FAIL_COUNT + 1))
  fi
else
  echo "[FAIL] hakorune failed with exit code $EXIT_CODE"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 60 || true
  FAIL_COUNT=$((FAIL_COUNT + 1))
fi

echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
  test_pass "phase104_read_digits_vm: All tests passed"
  exit 0
else
  test_fail "phase104_read_digits_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

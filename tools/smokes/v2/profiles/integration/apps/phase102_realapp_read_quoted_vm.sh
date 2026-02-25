#!/bin/bash
# Phase 102: real-app read_quoted loop (VM)
#
# Fixture extracted from MiniJsonLoader.read_quoted_from:
# - out = out + ch (string accumulator)
# - escape branch consumes 2 chars (\X)
# - quote terminator breaks

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

INPUT="$NYASH_ROOT/apps/tests/phase102_realapp_read_quoted_min.hako"

echo "[INFO] Phase 102: real-app read_quoted loop (VM) - $INPUT"

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
  EXPECTED="4"
  CLEAN=$(printf "%s\n" "$OUTPUT" | grep -E '^-?[0-9]+$' | head -n 1 | tr -d '\r')
  if [ "$CLEAN" = "$EXPECTED" ]; then
    echo "[PASS] Output verified: 4"
    PASS_COUNT=$((PASS_COUNT + 1))
  else
    echo "[FAIL] Unexpected output (expected: 4)"
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
  test_pass "phase102_realapp_read_quoted_vm: All tests passed"
  exit 0
else
  test_fail "phase102_realapp_read_quoted_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi


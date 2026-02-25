#!/bin/bash
# Phase 145 P1: ANF String.length() hoist (VM)
#
# Verifies ANF transformation: x + s.length() → t = s.length(); result = x + t
# Expected exit code: 12 (5 + 3 + 4)

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

INPUT="$NYASH_ROOT/apps/tests/phase145_p1_anf_length_min.hako"

echo "[INFO] Phase 145 P1: ANF length() hoist (VM) - $INPUT"

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  HAKO_ANF_DEV=1 \
  NYASH_DISABLE_PLUGINS=1 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "[FAIL] hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
  FAIL_COUNT=$((FAIL_COUNT + 1))
elif [ "$EXIT_CODE" -eq 12 ]; then
  echo "[PASS] Exit code verified: 12 (5 + 3 + 4)"
  PASS_COUNT=$((PASS_COUNT + 1))
else
  echo "[FAIL] Expected exit code 12, got $EXIT_CODE"
  echo "[INFO] output (tail):"
  echo "$OUTPUT" | tail -n 50 || true
  FAIL_COUNT=$((FAIL_COUNT + 1))
fi

echo "[INFO] PASS: $PASS_COUNT, FAIL: $FAIL_COUNT"

if [ "$FAIL_COUNT" -eq 0 ]; then
  test_pass "phase145_p1_anf_length_vm: ANF transformation verified (exit 12)"
  exit 0
else
  test_fail "phase145_p1_anf_length_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

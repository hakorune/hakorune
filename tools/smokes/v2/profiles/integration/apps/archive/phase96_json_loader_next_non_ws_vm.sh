#!/bin/bash
# Phase 96: json_loader next_non_ws loop (VM)
#
# Verifies Trim-style loop (next non-whitespace) extracted from MiniJsonLoader.

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

PASS_COUNT=0
FAIL_COUNT=0
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

INPUT="$NYASH_ROOT/apps/tests/phase96_json_loader_next_non_ws_min.hako"

echo "[INFO] Phase 96: json_loader next_non_ws loop (VM) - $INPUT"

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
  EXPECTED=$'2\n-1\n3'
  CLEAN=$(printf "%s\n" "$OUTPUT" | grep -E '^-?[0-9]+$' | head -n 3 | paste -sd '\n' - | tr -d '\r')
  if [ "$CLEAN" = "$EXPECTED" ]; then
    echo "[PASS] Output verified: 2, -1, then 3"
    PASS_COUNT=$((PASS_COUNT + 1))
  else
    echo "[FAIL] Unexpected output (expected lines: 2, -1, then 3)"
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
  test_pass "phase96_json_loader_next_non_ws_vm: All tests passed"
  exit 0
else
  test_fail "phase96_json_loader_next_non_ws_vm: $FAIL_COUNT test(s) failed"
  exit 1
fi

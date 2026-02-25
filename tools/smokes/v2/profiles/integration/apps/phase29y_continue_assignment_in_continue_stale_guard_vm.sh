#!/bin/bash
# phase29y_continue_assignment_in_continue_stale_guard_vm.sh
# Contract pin (stale-guard, non-gating):
# - assignment inside continue branch must be preserved.
# - expected output is FINAL=7.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

INPUT="${1:-$NYASH_ROOT/apps/tests/phase29y_continue_assignment_in_continue_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -f "$INPUT" ]; then
  test_fail "phase29y_continue_assignment_in_continue_stale_guard_vm: fixture missing: $INPUT"
  exit 2
fi

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_JOINIR_DEV=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_STRICT=0 \
  "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  test_fail "phase29y_continue_assignment_in_continue_stale_guard_vm: timed out (>${RUN_TIMEOUT_SECS}s)"
  exit 1
fi
if [ "$EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT" | tail -n 80 || true
  test_fail "phase29y_continue_assignment_in_continue_stale_guard_vm: run failed (rc=$EXIT_CODE)"
  exit 1
fi

OUTPUT_CLEAN=$(printf '%s\n' "$OUTPUT" | filter_noise)

if ! printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^FINAL=7$'; then
  echo "$OUTPUT_CLEAN" | tail -n 80 || true
  test_fail "phase29y_continue_assignment_in_continue_stale_guard_vm: contract drifted (expected FINAL=7)"
  exit 1
fi

test_pass "phase29y_continue_assignment_in_continue_stale_guard_vm: PASS (continue-branch assignment preserved)"

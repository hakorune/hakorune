#!/bin/bash
# phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh
# Contract pin (blocked, non-gating):
# - `--hako-emit-mir-json` on continue-assignment fixture still hits stage1 timeout path.
# - timeout must be raised inside stage1 bridge (fail-fast), not by outer shell timeout.
# - MIR output file must not be produced.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

INPUT="${1:-$NYASH_ROOT/apps/tests/phase29y_continue_assignment_in_continue_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-20}"
TMP_MIR="$(mktemp /tmp/phase29y_hako_emit_mir_timeout.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

if [ ! -f "$INPUT" ]; then
  test_fail "phase29y_hako_emit_mir_continue_assignment_timeout_block_vm: fixture missing: $INPUT"
  exit 2
fi

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_USE_FALLBACK=0 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_JOINIR_DEV=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_STRICT=0 \
  HAKO_JOINIR_PLANNER_REQUIRED=0 \
  "$NYASH_BIN" --hako-emit-mir-json "$TMP_MIR" "$INPUT" 2>&1)
RC=$?
set -e

if [ "$RC" -eq 124 ]; then
  echo "$OUTPUT" | tail -n 80 || true
  test_fail "phase29y_hako_emit_mir_continue_assignment_timeout_block_vm: outer timeout triggered (expected internal fail-fast timeout)"
  exit 1
fi

if [ "$RC" -eq 0 ]; then
  echo "$OUTPUT" | tail -n 80 || true
  test_fail "phase29y_hako_emit_mir_continue_assignment_timeout_block_vm: unexpected success (blocked pin must remain non-zero)"
  exit 1
fi

if ! printf '%s\n' "$OUTPUT" | rg -q '\[stage1-cli\] emit-mir: stage1 stub timed out after [0-9]+ ms'; then
  echo "$OUTPUT" | tail -n 80 || true
  test_fail "phase29y_hako_emit_mir_continue_assignment_timeout_block_vm: missing internal stage1 timeout marker"
  exit 1
fi

if [ -s "$TMP_MIR" ]; then
  test_fail "phase29y_hako_emit_mir_continue_assignment_timeout_block_vm: unexpected MIR output exists"
  exit 1
fi

test_pass "phase29y_hako_emit_mir_continue_assignment_timeout_block_vm: PASS (blocked internal timeout pinned)"

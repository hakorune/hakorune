#!/bin/bash
# phase29y_hako_emit_mir_binary_only_ported_vm.sh
# Contract pin (ported, non-gating):
# - repo-outside `./hakorune --hako-emit-mir-json` succeeds in binary-only mode.
# - stage1 entry path must not depend on repo checkout files.
# - MIR output file must be produced and contain functions payload.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/lib/phase29y_binary_only_common.sh"
require_env || exit 2

SMOKE_NAME="phase29y_hako_emit_mir_binary_only_ported_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/phase29y_continue_assignment_in_continue_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-20}"

cleanup() {
  phase29y_binary_only_cleanup_workdir
}
trap cleanup EXIT

phase29y_binary_only_require_input_and_bin "$SMOKE_NAME" "$INPUT" || exit 2
phase29y_binary_only_prepare_workdir "$INPUT" "phase29y_binary_only_ported"

phase29y_binary_only_run_in_workdir "$RUN_TIMEOUT_SECS" 0 --hako-emit-mir-json ./out.mir ./input.hako
OUTPUT="$PHASE29Y_BINARY_ONLY_OUTPUT"
RC="$PHASE29Y_BINARY_ONLY_RC"

if [ "$RC" -eq 124 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: timeout triggered"
  exit 1
fi

if [ "$RC" -ne 0 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: command failed rc=$RC"
  exit 1
fi

if printf '%s\n' "$OUTPUT" | rg -q '\[stage1-cli\] entry not found:.*stage1_cli\.hako'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: stale entry-path blocker reappeared"
  exit 1
fi

if printf '%s\n' "$OUTPUT" | rg -q '\[stage1-cli\] emit-mir: stage1 stub timed out after [0-9]+ ms'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: stale stage1 timeout marker reappeared"
  exit 1
fi

if printf '%s\n' "$OUTPUT" | rg -q "using: failed to read"; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: stale repo file dependency remained"
  exit 1
fi

if [ ! -s "$PHASE29Y_BINARY_ONLY_WORKDIR/out.mir" ]; then
  test_fail "$SMOKE_NAME: MIR output missing"
  exit 1
fi

if ! rg -q '"functions"' "$PHASE29Y_BINARY_ONLY_WORKDIR/out.mir"; then
  tail -n 80 "$PHASE29Y_BINARY_ONLY_WORKDIR/out.mir" || true
  test_fail "$SMOKE_NAME: MIR output missing functions key"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (binary-only ported contract fixed)"

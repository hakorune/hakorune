#!/bin/bash
# phase29y_hako_run_binary_only_backend_mismatch_block_vm.sh
# Contract pin (blocked, non-gating):
# - repo-outside `./hakorune --backend llvm --hako-run` is rejected by binary-only direct run route.
# - fail-fast marker must be explicit (unsupported backend), not stale lang/src read/entry/timeout blockers.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/lib/phase29y_binary_only_common.sh"
require_env || exit 2

SMOKE_NAME="phase29y_hako_run_binary_only_backend_mismatch_block_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/phase29y_hako_run_binary_only_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-20}"

cleanup() {
  phase29y_binary_only_cleanup_workdir
}
trap cleanup EXIT

phase29y_binary_only_require_input_and_bin "$SMOKE_NAME" "$INPUT" || exit 2
phase29y_binary_only_prepare_workdir "$INPUT" "phase29y_hako_run_binary_only_backend_mismatch_block"

phase29y_binary_only_run_in_workdir "$RUN_TIMEOUT_SECS" 1 --backend llvm --hako-run ./input.hako
OUTPUT="$PHASE29Y_BINARY_ONLY_OUTPUT"
RC="$PHASE29Y_BINARY_ONLY_RC"

if [ "$RC" -eq 124 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: timeout triggered"
  exit 1
fi

if [ "$RC" -eq 0 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: unexpected success (backend mismatch must fail-fast)"
  exit 1
fi

if ! printf '%s\n' "$OUTPUT" | rg -q '\[stage1-cli\] run\(binary-only\): unsupported backend for run binary-only direct route: llvm'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: missing backend mismatch fail-fast marker"
  exit 1
fi

if printf '%s\n' "$OUTPUT" | rg -q '\[stage1-cli\] entry not found:'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: stale entry-path blocker reappeared"
  exit 1
fi

if printf '%s\n' "$OUTPUT" | rg -q '\[stage1-cli\] .*timed out after [0-9]+ ms'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: stale timeout marker reappeared"
  exit 1
fi

if printf '%s\n' "$OUTPUT" | rg -q "using: failed to read 'lang/src/"; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: stale lang/src read blocker reappeared"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (backend mismatch fail-fast contract fixed)"

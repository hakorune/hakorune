#!/bin/bash
# phase29y_hako_run_binary_only_ported_vm.sh
# Contract pin (ported, non-gating):
# - repo-outside `./hakorune --backend vm --hako-run` succeeds in binary-only mode.
# - stage1 run route must not depend on repo checkout files.
# - stale blockers (`entry not found`, internal timeout marker, lang/src read fail-fast) must not reappear.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/lib/phase29y_binary_only_common.sh"
require_env || exit 2

SMOKE_NAME="phase29y_hako_run_binary_only_ported_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/phase29y_hako_run_binary_only_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-20}"

cleanup() {
  phase29y_binary_only_cleanup_workdir
}
trap cleanup EXIT

phase29y_binary_only_require_input_and_bin "$SMOKE_NAME" "$INPUT" || exit 2
phase29y_binary_only_prepare_workdir "$INPUT" "phase29y_hako_run_binary_only_ported"

phase29y_binary_only_run_in_workdir "$RUN_TIMEOUT_SECS" 1 --backend vm --hako-run ./input.hako
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

if printf '%s\n' "$OUTPUT" | rg -q '\[stage1-cli\] entry not found:'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: stale entry-path blocker reappeared"
  exit 1
fi

if printf '%s\n' "$OUTPUT" | rg -q '\[stage1-cli\] .*timed out after [0-9]+ ms'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: stale internal timeout marker reappeared"
  exit 1
fi

if printf '%s\n' "$OUTPUT" | rg -q "using: failed to read 'lang/src/"; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: stale lang/src read blocker reappeared"
  exit 1
fi

if printf '%s\n' "$OUTPUT" | rg -q '^\[vm-hako/unimplemented\]'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: unexpected vm-hako unimplemented tag"
  exit 1
fi

if printf '%s\n' "$OUTPUT" | rg -q '^\[vm-hako/contract'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: unexpected vm-hako contract tag"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (binary-only run ported contract fixed)"

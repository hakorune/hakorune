#!/bin/bash
# RVP-C11: vm-hako blocker pin for boxcall(args>1) on APP-1 cutover
#
# Contract:
# 1) MIR preflight must contain non-open boxcall with args length > 1 in APP-1.
# 2) vm-hako route must fail-fast with subset-check blocker tag op=boxcall(args>1).
# 3) timeout is forbidden.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_boxcall_args_gt1_block_vm"
APP="${1:-$NYASH_ROOT/apps/tools/gate_log_summarizer/main.hako}"
FIXTURE="${GATE_LOG_FIXTURE:-$NYASH_ROOT/apps/tests/gate_log_summarizer/sample_mixed.log}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c11.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$APP" || exit 1
vm_hako_caps_require_fixture "$SMOKE_NAME" "$FIXTURE" || exit 1

vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$APP" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="boxcall" and .method != "open" and (.args|length) > 1)' \
  "MIR missing non-open boxcall(args>1) shape" || exit 1

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  GATE_LOG_FILE="$FIXTURE" \
  "$NYASH_BIN" --backend vm-hako "$APP" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  test_fail "vm_hako_caps_boxcall_args_gt1_block_vm: timed out (>${RUN_TIMEOUT_SECS}s)"
  exit 1
fi
if [ "$EXIT_CODE" -eq 0 ]; then
  echo "$OUTPUT" | tail -n 120 || true
  test_fail "vm_hako_caps_boxcall_args_gt1_block_vm: expected boxcall(args>1) blocker (non-zero exit)"
  exit 1
fi

OUTPUT_CLEAN=$(printf '%s\n' "$OUTPUT" | filter_noise)

if ! printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/unimplemented\].*route=subset-check.*op=boxcall\(args>1\)'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_boxcall_args_gt1_block_vm: expected subset blocker tag for boxcall(args>1)"
  exit 1
fi

if printf '%s\n' "$OUTPUT_CLEAN" | rg -q 'op=compare\(>=\)'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_boxcall_args_gt1_block_vm: stale compare(>=) blocker reappeared"
  exit 1
fi

test_pass "vm_hako_caps_boxcall_args_gt1_block_vm: PASS (RVP-C11 blocked pin fixed at boxcall(args>1))"

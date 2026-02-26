#!/bin/bash
# RVP-C12: vm-hako blocker pin for FileBox.open path-handle propagation across phi/copy
#
# Contract:
# 1) MIR preflight must contain phi + boxcall(open) shape in APP-1.
# 2) vm-hako route must fail-fast with runtime contract tag boxcall-open-handle-missing.
# 3) timeout is forbidden.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_open_handle_phi_block_vm"
APP="${1:-$NYASH_ROOT/apps/tools/gate_log_summarizer/main.hako}"
FIXTURE="${GATE_LOG_FIXTURE:-$NYASH_ROOT/apps/tests/gate_log_summarizer/sample_mixed.log}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c12.XXXXXX.json)"
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
  '.functions[] | select(.name=="main") | .blocks[] | select(any(.instructions[]; .op=="phi") and any(.instructions[]; .op=="boxcall" and .method=="open" and ((.args|length)==2 or (.args|length)==3)))' \
  "MIR missing phi + boxcall(open) shape" || exit 1

vm_hako_caps_run_vm_hako_with_fixture_or_fail_timeout \
  "$SMOKE_NAME" \
  "$RUN_TIMEOUT_SECS" \
  "$APP" \
  "$FIXTURE" || exit 1

OUTPUT="$VM_HAKO_CAPS_OUTPUT"
OUTPUT_CLEAN="$VM_HAKO_CAPS_OUTPUT_CLEAN"
EXIT_CODE="$VM_HAKO_CAPS_EXIT_CODE"
if [ "$EXIT_CODE" -eq 0 ]; then
  echo "$OUTPUT" | tail -n 120 || true
  test_fail "vm_hako_caps_open_handle_phi_block_vm: expected boxcall-open-handle-missing blocker (non-zero exit)"
  exit 1
fi

if ! printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/contract\]\[boxcall-open-handle-missing\]'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_open_handle_phi_block_vm: expected runtime contract tag boxcall-open-handle-missing"
  exit 1
fi

if printf '%s\n' "$OUTPUT_CLEAN" | rg -q 'op=boxcall\(args>1\)'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_open_handle_phi_block_vm: stale boxcall(args>1) blocker reappeared"
  exit 1
fi

test_pass "vm_hako_caps_open_handle_phi_block_vm: PASS (RVP-C12 blocked pin fixed at boxcall-open-handle-missing)"

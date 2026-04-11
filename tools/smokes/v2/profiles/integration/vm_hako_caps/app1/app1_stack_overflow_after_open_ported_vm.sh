#!/bin/bash
# RVP-C13: vm-hako capability smoke for APP-1 run completion after open-path handling (ported pin)
#
# Contract:
# 1) MIR preflight must contain phi + mir_call(open) shape in APP-1.
# 2) stale C12 blocker boxcall-open-handle-missing must not appear.
# 3) stale stack-overflow signature must not appear.
# 4) vm-hako execution must complete with RC=0 (run completion pin).
#
# Note:
# - This contract pins run completion only.
# - Full APP-1 summary-content parity is tracked by next blocker task.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_app1_stack_overflow_after_open_ported_vm"
APP="${1:-$NYASH_ROOT/apps/tools/gate_log_summarizer/main.hako}"
FIXTURE="${GATE_LOG_FIXTURE:-$NYASH_ROOT/apps/tests/vm_hako_caps/app1_summary_contract_min.txt}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-60}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c13_ported.XXXXXX.json)"
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
  '.functions[] | select(.name=="main") | .blocks[] | select(any(.instructions[]; .op=="phi") and any(.instructions[]; .op=="mir_call" and .mir_call.callee.type=="Method" and .mir_call.callee.box_name=="FileBox" and .mir_call.callee.name=="open" and ((.mir_call.args|length)==2 or (.mir_call.args|length)==3)))' \
  "MIR missing phi + mir_call(open) shape" || exit 1

vm_hako_caps_run_vm_hako_with_fixture \
  "$SMOKE_NAME" \
  "$RUN_TIMEOUT_SECS" \
  "$APP" \
  "$FIXTURE" || exit 1

OUTPUT_CLEAN="$VM_HAKO_CAPS_OUTPUT_CLEAN"
EXIT_CODE="$VM_HAKO_CAPS_EXIT_CODE"

if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/contract\]\[boxcall-open-handle-missing\]'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_app1_stack_overflow_after_open_ported_vm: stale C12 blocker reappeared"
  exit 1
fi

if printf '%s\n' "$OUTPUT_CLEAN" | rg -q 'stack overflow'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_app1_stack_overflow_after_open_ported_vm: stale stack-overflow blocker reappeared"
  exit 1
fi

vm_hako_caps_assert_no_unimplemented "$SMOKE_NAME" "$OUTPUT_CLEAN" || exit 1
vm_hako_caps_assert_no_contract "$SMOKE_NAME" "$OUTPUT_CLEAN" || exit 1

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_app1_stack_overflow_after_open_ported_vm: timed out"
  exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_app1_stack_overflow_after_open_ported_vm: expected RC=0 (got rc=$EXIT_CODE)"
  exit 1
fi

test_pass "vm_hako_caps_app1_stack_overflow_after_open_ported_vm: PASS (RVP-C13 ported)"

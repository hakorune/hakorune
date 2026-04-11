#!/bin/bash
# RVP-C12: vm-hako capability smoke for FileBox.open path-handle propagation across phi/copy (ported pin)
#
# Contract:
# 1) MIR preflight must contain phi + mir_call(open) shape in APP-1.
# 2) vm-hako route must no longer fail with boxcall-open-handle-missing.
# 3) stale subset blocker op=boxcall(args>1) must not reappear.
# 4) execution must be non-timeout and exit RC=0.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_open_handle_phi_ported_vm"
APP="${1:-$NYASH_ROOT/apps/tools/gate_log_summarizer/main.hako}"
FIXTURE="${GATE_LOG_FIXTURE:-$NYASH_ROOT/apps/tests/vm_hako_caps/app1_summary_contract_min.txt}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-60}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c12_ported.XXXXXX.json)"
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
  test_fail "vm_hako_caps_open_handle_phi_ported_vm: stale blocker boxcall-open-handle-missing remained"
  exit 1
fi

if printf '%s\n' "$OUTPUT_CLEAN" | rg -q 'op=boxcall\(args>1\)'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_open_handle_phi_ported_vm: stale subset blocker boxcall(args>1) reappeared"
  exit 1
fi

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_open_handle_phi_ported_vm: timed out"
  exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_open_handle_phi_ported_vm: unexpected non-zero exit (rc=$EXIT_CODE)"
  exit 1
fi

test_pass "vm_hako_caps_open_handle_phi_ported_vm: PASS (RVP-C12 ported)"

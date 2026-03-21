#!/bin/bash
# RVP-C08: vm-hako capability smoke for FileBox.close (ported contract)
#
# Contract:
# 1) MIR preflight must contain FileBox.close method-call shape.
# 2) vm-hako runtime must execute close path without unimplemented/contract errors.
# 3) execution must finish with RC: 0 marker.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_file_close_ported_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/vm_hako_caps/file_close_block_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c08.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select((.op=="mir_call" and .mir_call.callee.type=="Method" and .mir_call.callee.box_name=="FileBox" and .mir_call.callee.name=="close") or (.op=="boxcall" and .method=="close"))' \
  "MIR missing FileBox.close method-call shape" || exit 1

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$VM_HAKO_CAPS_OUTPUT" | tail -n 80 || true
  test_fail "vm_hako_caps_file_close_ported_vm: expected rc=0 for FileBox.close path"
  exit 1
fi

vm_hako_caps_assert_no_unimplemented "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1
vm_hako_caps_assert_no_contract "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1
vm_hako_caps_assert_rc_marker "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" "0" || exit 1

test_pass "vm_hako_caps_file_close_ported_vm: PASS (RVP-C08 ported contract pinned)"

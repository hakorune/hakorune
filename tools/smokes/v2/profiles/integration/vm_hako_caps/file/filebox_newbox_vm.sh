#!/bin/bash
# RVP-C01: vm-hako capability smoke for newbox(FileBox)
#
# Contract:
# 1) vm-hako route must accept newbox(FileBox) and finish execution.
# 2) exit must be zero and never timeout.
# 3) MIR preflight must include newbox(FileBox) shape.
# 4) output must not contain [vm-hako/unimplemented].

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_filebox_newbox_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/vm_hako_caps/filebox_newbox_block_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c01.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

# Preflight: pin MIR shape so this smoke cannot pass when FileBox newbox is optimized out.
vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[]?.blocks[]?.instructions[]? | select(.op=="newbox" and .type=="FileBox")' \
  "MIR missing newbox(FileBox) shape" || exit 1

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$VM_HAKO_CAPS_OUTPUT" | tail -n 80 || true
  test_fail "vm_hako_caps_filebox_newbox_vm: expected success exit (rc=$VM_HAKO_CAPS_EXIT_CODE)"
  exit 1
fi

vm_hako_caps_assert_no_unimplemented "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1

test_pass "vm_hako_caps_filebox_newbox_vm: PASS (RVP-C01 ported contract pinned)"

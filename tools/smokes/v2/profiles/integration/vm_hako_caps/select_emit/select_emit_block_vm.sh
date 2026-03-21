#!/bin/bash
# RVP-C04: vm-hako capability smoke for Select emit contract (ported pin)
#
# Contract:
# 1) MIR preflight must contain `select` shape.
# 2) vm-hako route must execute `select` fixture without emit/subset/runtime contract errors.
# 3) execution must reach RC marker (panic/abort禁止), and never timeout.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_select_emit_block_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/vm_hako_caps/select_emit_block_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c04.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="select")' \
  "MIR missing select shape" || exit 1

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

if [ "$VM_HAKO_CAPS_EXIT_CODE" -eq 134 ]; then
  echo "$VM_HAKO_CAPS_OUTPUT" | tail -n 80 || true
  test_fail "vm_hako_caps_select_emit_block_vm: process aborted (rc=134)"
  exit 1
fi

vm_hako_caps_assert_no_unimplemented "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1
vm_hako_caps_assert_no_contract "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1
if echo "$VM_HAKO_CAPS_OUTPUT" | rg -q "^\[vm-hako/emit-error\]"; then
  echo "$VM_HAKO_CAPS_OUTPUT" | tail -n 80 || true
  test_fail "vm_hako_caps_select_emit_block_vm: unexpected emit-error tag"
  exit 1
fi

if ! echo "$VM_HAKO_CAPS_OUTPUT" | rg -q "^RC: [0-9]+$"; then
  echo "$VM_HAKO_CAPS_OUTPUT" | tail -n 80 || true
  test_fail "vm_hako_caps_select_emit_block_vm: missing RC marker"
  exit 1
fi

test_pass "vm_hako_caps_select_emit_block_vm: PASS (RVP-C04 ported contract pinned)"

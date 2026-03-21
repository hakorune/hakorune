#!/bin/bash
# RVP-C05: vm-hako capability smoke for env.get externcall (ported pin)
#
# Contract:
# 1) MIR preflight must contain extern call shape `env.get/1`.
# 2) vm-hako route must execute env.get path without emit/subset/runtime contract errors.
# 3) exit code must be zero with stable RC marker.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_env_get_ported_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/vm_hako_caps/env_get_externcall_block_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c05.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select((.op=="mir_call" and .mir_call.callee.type=="Extern" and .mir_call.callee.name=="env.get/1") or (.op=="call" and .callee.type=="Global" and .callee.name=="env.get/1"))' \
  "MIR missing extern env.get/1 shape" || exit 1

RVP_C05_ENV_KEY="phase29y-c05-env-value" \
vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$VM_HAKO_CAPS_OUTPUT" | tail -n 80 || true
  test_fail "vm_hako_caps_env_get_ported_vm: expected success exit (rc=0), got rc=$VM_HAKO_CAPS_EXIT_CODE"
  exit 1
fi

vm_hako_caps_assert_no_unimplemented "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1
vm_hako_caps_assert_no_contract "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1
if echo "$VM_HAKO_CAPS_OUTPUT" | rg -q "^\[vm-hako/emit-error\]"; then
  echo "$VM_HAKO_CAPS_OUTPUT" | tail -n 80 || true
  test_fail "vm_hako_caps_env_get_ported_vm: unexpected emit-error tag"
  exit 1
fi
vm_hako_caps_assert_rc_marker "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" "0" || exit 1

test_pass "vm_hako_caps_env_get_ported_vm: PASS (RVP-C05 ported contract pinned)"

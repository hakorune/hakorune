#!/bin/bash
# RVP-C19: vm-hako capability smoke for TlsCoreBox.last_error_text_h()
#
# Contract:
# 1) MIR preflight must contain `TlsCoreBox.last_error_text_h()` call shape.
# 2) vm-hako route must print `OK` and exit with rc=0.
# 3) timeout / unimplemented / contract tags are forbidden.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_tls_last_error_ported_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/vm_hako_caps/tls_last_error_ported_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c19.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[]?.blocks[]?.instructions[]? | select(.op=="call" and .callee.type=="Global" and .callee.name=="TlsCoreBox.last_error_text_h/0")' \
  "MIR missing TlsCoreBox.last_error_text_h() call shape" || exit 1

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

OUTPUT_CLEAN=$(printf '%s\n' "$VM_HAKO_CAPS_OUTPUT" | filter_noise || true)
if ! printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^OK$'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_tls_last_error_ported_vm: expected printed OK"
  exit 1
fi
if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_tls_last_error_ported_vm: expected RC=0 (got rc=$VM_HAKO_CAPS_EXIT_CODE)"
  exit 1
fi

vm_hako_caps_assert_no_unimplemented "$SMOKE_NAME" "$OUTPUT_CLEAN" || exit 1
vm_hako_caps_assert_no_contract "$SMOKE_NAME" "$OUTPUT_CLEAN" || exit 1
test_pass "vm_hako_caps_tls_last_error_ported_vm: PASS (RVP-C19 ported contract pinned)"

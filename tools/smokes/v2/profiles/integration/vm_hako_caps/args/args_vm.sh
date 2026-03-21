#!/bin/bash
# RVP-C02: vm-hako capability smoke for args-based routing
#
# Contract:
# 1) MIR preflight must include args bootstrap shape (birth + push + length).
# 2) vm-hako route must execute this path without subset/contract errors.
# 3) exit code must equal args.length() (=1 for "-- hello"), and never timeout.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_args_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/vm_hako_caps/args_route_block_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c02.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

# Preflight: pin main(args) bootstrap shape before runtime check.
vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" -- hello || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="boxcall" and .method=="birth")' \
  "MIR missing main(args) birth bootstrap shape" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="boxcall" and .method=="length")' \
  "MIR missing args.length boxcall shape" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="boxcall" and .method=="push" and .dst==null)' \
  "MIR missing args.push bootstrap shape" || exit 1

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" -- hello || exit 1

if [ "$VM_HAKO_CAPS_EXIT_CODE" -eq 0 ]; then
  echo "$VM_HAKO_CAPS_OUTPUT" | tail -n 80 || true
  test_fail "vm_hako_caps_args_vm: expected args.length exit code (non-zero)"
  exit 1
fi

if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 1 ]; then
  echo "$VM_HAKO_CAPS_OUTPUT" | tail -n 80 || true
  test_fail "vm_hako_caps_args_vm: expected exit=1 (args.length), got rc=$VM_HAKO_CAPS_EXIT_CODE"
  exit 1
fi

vm_hako_caps_assert_no_unimplemented "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1
vm_hako_caps_assert_no_contract "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1
vm_hako_caps_assert_rc_marker "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" "1" || exit 1

test_pass "vm_hako_caps_args_vm: PASS (RVP-C02 ported contract pinned)"

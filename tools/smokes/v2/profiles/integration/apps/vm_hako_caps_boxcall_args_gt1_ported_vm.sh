#!/bin/bash
# RVP-C11: vm-hako capability smoke for non-open boxcall(args>1) route (ported pin)
#
# Contract:
# 1) MIR preflight must contain non-open boxcall(args>1) shape in APP-1.
# 2) vm-hako route must no longer fail at subset-check op=boxcall(args>1).
# 3) timeout is forbidden.
# 4) stale post-C11 blocker boxcall-open-handle-missing must not reappear.
# 5) execution must complete with RC=0.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_boxcall_args_gt1_ported_vm"
APP="${1:-$NYASH_ROOT/apps/tools/gate_log_summarizer/main.hako}"
FIXTURE="${GATE_LOG_FIXTURE:-$NYASH_ROOT/apps/tests/vm_hako_caps/app1_summary_contract_min.txt}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-60}"
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

vm_hako_caps_run_vm_hako_with_fixture_or_fail_timeout \
  "$SMOKE_NAME" \
  "$RUN_TIMEOUT_SECS" \
  "$APP" \
  "$FIXTURE" || exit 1

OUTPUT_CLEAN="$VM_HAKO_CAPS_OUTPUT_CLEAN"
EXIT_CODE="$VM_HAKO_CAPS_EXIT_CODE"
if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/unimplemented\].*route=subset-check.*op=boxcall\(args>1\)'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_boxcall_args_gt1_ported_vm: stale subset blocker boxcall(args>1) remained"
  exit 1
fi

if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/contract\]\[boxcall-open-handle-missing\]'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_boxcall_args_gt1_ported_vm: stale blocker boxcall-open-handle-missing reappeared"
  exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_boxcall_args_gt1_ported_vm: expected RC=0 (got rc=$EXIT_CODE)"
  exit 1
fi

test_pass "vm_hako_caps_boxcall_args_gt1_ported_vm: PASS (RVP-C11 ported; subset blocker cleared)"

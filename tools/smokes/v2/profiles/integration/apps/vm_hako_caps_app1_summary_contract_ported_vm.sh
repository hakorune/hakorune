#!/bin/bash
# RVP-C15: vm-hako capability smoke for APP-1 full-fixture summary contract parity (ported pin)
#
# Contract:
# 1) MIR preflight must contain phi + boxcall(open) shape in APP-1.
# 2) Rust VM baseline output must match APP-1 full fixture summary contract.
# 3) vm-hako output must match Rust baseline exactly (SUMMARY / FAIL_LINES / FAIL order).
# 4) timeout and stale vm-hako unimplemented/contract tags are forbidden.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_app1_summary_contract_ported_vm"
APP="${1:-$NYASH_ROOT/apps/tools/gate_log_summarizer/main.hako}"
FIXTURE="${GATE_LOG_FIXTURE:-$NYASH_ROOT/apps/tests/gate_log_summarizer/sample_mixed.log}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-150}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c14_ported.XXXXXX.json)"
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

RUST_OUTPUT=$(NYASH_VM_HAKO_PREFER_STRICT_DEV=0 NYASH_VM_USE_FALLBACK=0 \
              NYASH_JOINIR_DEV=0 NYASH_JOINIR_STRICT=0 \
              GATE_LOG_FILE="$FIXTURE" \
              run_nyash_vm "$APP")

RUST_EXPECTED=$(cat << 'TXT'
SUMMARY pass=7 fail=2 skip=1
FAIL_LINES 2
[FAIL] phase29y_handle_abi_borrowed_owned_vm: rc=1
[FAIL] phase29y_lane_gate_vm: contract mismatch
TXT
)

compare_outputs "$RUST_EXPECTED" "$RUST_OUTPUT" "$SMOKE_NAME: rust baseline drift" || exit 1

vm_hako_caps_run_vm_hako_with_fixture \
  "$SMOKE_NAME" \
  "$RUN_TIMEOUT_SECS" \
  "$APP" \
  "$FIXTURE" || exit 1

OUTPUT_CLEAN="$VM_HAKO_CAPS_OUTPUT_CLEAN"
EXIT_CODE="$VM_HAKO_CAPS_EXIT_CODE"

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: timed out"
  exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected RC=0 (got rc=$EXIT_CODE)"
  exit 1
fi

vm_hako_caps_assert_no_unimplemented "$SMOKE_NAME" "$OUTPUT_CLEAN" || exit 1
vm_hako_caps_assert_no_contract "$SMOKE_NAME" "$OUTPUT_CLEAN" || exit 1

compare_outputs "$RUST_OUTPUT" "$OUTPUT_CLEAN" "$SMOKE_NAME: parity mismatch" || exit 1

test_pass "$SMOKE_NAME: PASS (RVP-C15 ported full-fixture parity pin)"

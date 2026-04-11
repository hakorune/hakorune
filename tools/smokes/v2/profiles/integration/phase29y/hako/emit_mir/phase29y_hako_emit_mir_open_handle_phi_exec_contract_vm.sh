#!/bin/bash
# phase29y_hako_emit_mir_open_handle_phi_exec_contract_vm.sh
# Contract pin:
# - selfhost emit must preserve phi + `mir_call(FileBox.open,args=2/3)` in MIR.
# - emitted MIR must execute through `--mir-json-file` without vm_hako helpers.
# - visible result must stay fixed at `READ_OK` with rc=0.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../apps/lib/phase29y_binary_only_common.sh"
require_env || exit 2

SMOKE_NAME="phase29y_hako_emit_mir_open_handle_phi_exec_contract_vm"
INPUT_FIXTURE="${1:-$NYASH_ROOT/apps/tests/phase29y_hako_emit_mir_open_handle_phi_exec_min.hako}"
SAMPLE_FIXTURE="$NYASH_ROOT/apps/tests/gate_log_summarizer/sample_mixed.log"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-20}"

phase29y_binary_only_require_input_and_bin "$SMOKE_NAME" "$INPUT_FIXTURE" || exit 2
phase29y_binary_only_require_input_and_bin "$SMOKE_NAME" "$SAMPLE_FIXTURE" || exit 2
phase29y_binary_only_prepare_workdir "$INPUT_FIXTURE" "phase29y_hako_emit_open_handle_phi_exec"
cp "$SAMPLE_FIXTURE" "$PHASE29Y_BINARY_ONLY_WORKDIR/sample_mixed.log"
cleanup() {
  phase29y_binary_only_cleanup_workdir
}
trap cleanup EXIT

phase29y_binary_only_run_in_workdir "$RUN_TIMEOUT_SECS" 1 --hako-emit-mir-json ./out.mir ./input.hako
EMIT_RC="$PHASE29Y_BINARY_ONLY_RC"

if [ "$EMIT_RC" -eq 124 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: emit outer-timeout triggered"
  exit 1
fi
if [ "$EMIT_RC" -ne 0 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: emit failed rc=$EMIT_RC"
  exit 1
fi

if [ ! -s "$PHASE29Y_BINARY_ONLY_WORKDIR/out.mir" ]; then
  test_fail "$SMOKE_NAME: emitted MIR missing"
  exit 1
fi

if ! jq -e '.functions[] | select(.name=="main") | .blocks[] | select(any(.instructions[]; .op=="phi") and any(.instructions[]; .op=="mir_call" and .mir_call.callee.type=="Method" and .mir_call.callee.box_name=="FileBox" and .mir_call.callee.name=="open" and ((.mir_call.args|length)==2 or (.mir_call.args|length)==3)))' \
  "$PHASE29Y_BINARY_ONLY_WORKDIR/out.mir" >/dev/null 2>&1; then
  tail -n 80 "$PHASE29Y_BINARY_ONLY_WORKDIR/out.mir" || true
  test_fail "$SMOKE_NAME: emitted MIR missing phi + FileBox.open shape"
  exit 1
fi

phase29y_binary_only_run_in_workdir "$RUN_TIMEOUT_SECS" 1 --mir-json-file ./out.mir
RUN_OUTPUT="$PHASE29Y_BINARY_ONLY_OUTPUT"
RUN_RC="$PHASE29Y_BINARY_ONLY_RC"
OUTPUT_CLEAN="$(printf '%s\n' "$RUN_OUTPUT" | filter_noise || true)"

if [ "$RUN_RC" -ne 0 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: --mir-json-file rc=$RUN_RC (expect 0)"
  exit 1
fi

if printf '%s\n' "$RUN_OUTPUT" | rg -q 'boxcall-open-handle-missing|Invalid value|undefined value|vm step budget exceeded'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: runtime blocker marker detected"
  exit 1
fi

if ! compare_outputs "READ_OK" "$OUTPUT_CLEAN" "$SMOKE_NAME"; then
  phase29y_binary_only_tail_output
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (.hako emit preserves FileBox.open phi/copy seam and rc=0)"

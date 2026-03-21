#!/bin/bash
# phase29y_hako_emit_mir_jsonfile_exec_contract_vm.sh
# Contract pin:
# - `--hako-emit-mir-json` output must execute via `--mir-json-file` in binary-only mode.
# - Route must stay fail-fast (no timeout marker / no invalid-value runtime crash).

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../apps/lib/phase29y_binary_only_common.sh"
require_env || exit 2

SMOKE_NAME="phase29y_hako_emit_mir_jsonfile_exec_contract_vm"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-20}"

TMP_SRC="$(mktemp /tmp/phase29y_hako_emit_exec_contract.XXXXXX.hako)"
cleanup() {
  rm -f "$TMP_SRC"
  phase29y_binary_only_cleanup_workdir
}
trap cleanup EXIT

cat >"$TMP_SRC" <<'HAKO'
static box Main {
  method main() {
    return 42
  }
}
HAKO

phase29y_binary_only_require_input_and_bin "$SMOKE_NAME" "$TMP_SRC" || exit 2
phase29y_binary_only_prepare_workdir "$TMP_SRC" "phase29y_hako_emit_exec_contract"

phase29y_binary_only_run_in_workdir "$RUN_TIMEOUT_SECS" 1 --hako-emit-mir-json ./out.mir ./input.hako
EMIT_OUTPUT="$PHASE29Y_BINARY_ONLY_OUTPUT"
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

if printf '%s\n' "$EMIT_OUTPUT" | rg -q '\[stage1-cli\] emit-mir: stage1 stub timed out after [0-9]+ ms'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: stale stage1 timeout marker detected"
  exit 1
fi

if [ ! -s "$PHASE29Y_BINARY_ONLY_WORKDIR/out.mir" ]; then
  test_fail "$SMOKE_NAME: emitted MIR missing"
  exit 1
fi

if ! rg -q '"functions"' "$PHASE29Y_BINARY_ONLY_WORKDIR/out.mir"; then
  tail -n 80 "$PHASE29Y_BINARY_ONLY_WORKDIR/out.mir" || true
  test_fail "$SMOKE_NAME: emitted MIR missing functions key"
  exit 1
fi

phase29y_binary_only_run_in_workdir "$RUN_TIMEOUT_SECS" 1 --mir-json-file ./out.mir
RUN_OUTPUT="$PHASE29Y_BINARY_ONLY_OUTPUT"
RUN_RC="$PHASE29Y_BINARY_ONLY_RC"

if [ "$RUN_RC" -ne 42 ]; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: --mir-json-file rc=$RUN_RC (expect 42)"
  exit 1
fi

if printf '%s\n' "$RUN_OUTPUT" | rg -q 'Invalid value|undefined value|vm step budget exceeded'; then
  phase29y_binary_only_tail_output
  test_fail "$SMOKE_NAME: runtime blocker marker detected"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (.hako emit -> mir-json-file execution contract)"

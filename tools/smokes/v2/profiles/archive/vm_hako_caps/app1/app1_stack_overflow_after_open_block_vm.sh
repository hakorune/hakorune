#!/bin/bash
# RVP-C13: vm-hako blocker pin after C12 port (APP-1 stack overflow after open-path handling)
#
# Contract:
# 1) MIR preflight must contain phi + boxcall(open) shape in APP-1.
# 2) stale blocker boxcall-open-handle-missing must not appear.
# 3) execution must currently fail with stack overflow (known post-C12 blocker pin).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT_DIR="$ROOT_GIT"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi

source "$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT_DIR/tools/smokes/v2/profiles/integration/vm_hako_caps/lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_app1_stack_overflow_after_open_block_vm"
APP="${1:-$NYASH_ROOT/apps/tools/gate_log_summarizer/main.hako}"
FIXTURE="${GATE_LOG_FIXTURE:-$NYASH_ROOT/apps/tests/gate_log_summarizer/sample_mixed.log}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-20}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c13_block.XXXXXX.json)"
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

vm_hako_caps_run_vm_hako_with_fixture \
  "$SMOKE_NAME" \
  "$RUN_TIMEOUT_SECS" \
  "$APP" \
  "$FIXTURE" || exit 1

OUTPUT_CLEAN="$VM_HAKO_CAPS_OUTPUT_CLEAN"
EXIT_CODE="$VM_HAKO_CAPS_EXIT_CODE"

if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/contract\]\[boxcall-open-handle-missing\]'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_app1_stack_overflow_after_open_block_vm: stale C12 blocker reappeared"
  exit 1
fi

if [ "$EXIT_CODE" -eq 124 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_app1_stack_overflow_after_open_block_vm: timed out (expected stack overflow signal)"
  exit 1
fi

if [ "$EXIT_CODE" -eq 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_app1_stack_overflow_after_open_block_vm: expected non-zero exit with stack overflow blocker"
  exit 1
fi

if ! printf '%s\n' "$OUTPUT_CLEAN" | rg -q 'stack overflow'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_app1_stack_overflow_after_open_block_vm: expected stack overflow blocker signature"
  exit 1
fi

test_pass "vm_hako_caps_app1_stack_overflow_after_open_block_vm: PASS (RVP-C13 blocked pin fixed at stack-overflow-after-open)"

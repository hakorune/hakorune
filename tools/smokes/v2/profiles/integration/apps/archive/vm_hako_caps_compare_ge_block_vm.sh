#!/bin/bash
# RVP-C10: vm-hako blocker pin for compare(>=) on APP-1 cutover
#
# Contract:
# 1) MIR preflight must contain compare operation ">=" in APP-1.
# 2) vm-hako route must fail-fast with subset-check blocker tag op=compare(>=).
# 3) timeout is forbidden.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_compare_ge_block_vm"
APP="${1:-$NYASH_ROOT/apps/tools/gate_log_summarizer/main.hako}"
FIXTURE="${GATE_LOG_FIXTURE:-$NYASH_ROOT/apps/tests/gate_log_summarizer/sample_mixed.log}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c10.XXXXXX.json)"
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
  '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="compare" and (.operation==">=" or .op_kind=="Ge"))' \
  "MIR missing compare(>=) shape" || exit 1

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  GATE_LOG_FILE="$FIXTURE" \
  "$NYASH_BIN" --backend vm-hako "$APP" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
  test_fail "vm_hako_caps_compare_ge_block_vm: timed out (>${RUN_TIMEOUT_SECS}s)"
  exit 1
fi
if [ "$EXIT_CODE" -eq 0 ]; then
  echo "$OUTPUT" | tail -n 120 || true
  test_fail "vm_hako_caps_compare_ge_block_vm: expected compare(>=) blocker (non-zero exit)"
  exit 1
fi

OUTPUT_CLEAN=$(printf '%s\n' "$OUTPUT" | filter_noise)

if ! printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/unimplemented\].*route=subset-check.*op=compare\(>=\)'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_compare_ge_block_vm: expected subset blocker tag for compare(>=)"
  exit 1
fi

if printf '%s\n' "$OUTPUT_CLEAN" | rg -q 'const\(non-i64-bool-handle:void\)|const\(non-i64-bool-void-handle:void\)'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_compare_ge_block_vm: stale const(void) blocker reappeared"
  exit 1
fi

test_pass "vm_hako_caps_compare_ge_block_vm: PASS (RVP-C10 blocked pin fixed at compare(>=))"

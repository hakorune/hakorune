#!/bin/bash
# RVP-C17: blocked pin for MapBox.set(key, value) multi-arg boxcall
#
# Contract:
# 1) MIR preflight must contain `boxcall(method=set,args=2)`.
# 2) vm-hako route currently fails at subset-check with `op=boxcall(set:args>1)`.
# 3) timeout is forbidden.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_mapbox_set_block_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/vm_hako_caps/mapbox_set_block_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c17.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[]?.blocks[]?.instructions[]? | select(.op=="boxcall" and .method=="set" and (.args|length)==2)' \
  "MIR missing boxcall(set,args=2) shape" || exit 1

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

OUTPUT_CLEAN=$(printf '%s\n' "$VM_HAKO_CAPS_OUTPUT" | filter_noise)
if ! printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/unimplemented\].*route=subset-check.*op=boxcall\(set:args>1\)'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_mapbox_set_block_vm: expected subset blocker boxcall(set:args>1)"
  exit 1
fi

test_pass "vm_hako_caps_mapbox_set_block_vm: PASS (RVP-C17 blocked pin)"

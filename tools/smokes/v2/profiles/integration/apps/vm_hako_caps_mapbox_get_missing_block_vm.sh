#!/bin/bash
# RVP-C24: blocked pin for MapBox.get(missing-key) visible contract after MapBox.clear port
#
# Contract:
# 1) MIR preflight must contain `boxcall(method=get,args=1)`.
# 2) vm-hako route currently prints stale scalar `0`.
# 3) timeout is forbidden.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_mapbox_get_missing_block_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/vm_hako_caps/mapbox_get_missing_block_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c24.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[]?.blocks[]?.instructions[]? | select(.op=="boxcall" and .method=="get" and (.args|length)==1)' \
  "MIR missing boxcall(get,args=1) shape" || exit 1

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

OUTPUT_CLEAN=$(printf '%s\n' "$VM_HAKO_CAPS_OUTPUT" | filter_noise || true)
if ! printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^0$'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_mapbox_get_missing_block_vm: expected stale scalar 0 for missing-key get"
  exit 1
fi
if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_mapbox_get_missing_block_vm: expected RC=0 while stale contract remains"
  exit 1
fi

test_pass "vm_hako_caps_mapbox_get_missing_block_vm: PASS (RVP-C24 blocked pin)"

#!/bin/bash
# RVP-C21: vm-hako capability smoke for MapBox.delete(key)
#
# Contract:
# 1) MIR preflight must contain `boxcall(method=delete,args=1)`.
# 2) vm-hako route must print `false` then `1`, and finish with RC=0.
# 3) stale `op=boxcall1 method=delete` blocker must not reappear.
# 4) timeout is forbidden.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_mapbox_delete_ported_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/vm_hako_caps/mapbox_delete_ported_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c21.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[]?.blocks[]?.instructions[]? | select(.op=="boxcall" and .method=="delete" and (.args|length)==1)' \
  "MIR missing boxcall(delete,args=1) shape" || exit 1

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

OUTPUT_CLEAN=$(printf '%s\n' "$VM_HAKO_CAPS_OUTPUT" | filter_noise || true)
if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/unimplemented op=boxcall1 method=delete\]$'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_mapbox_delete_ported_vm: stale runtime blocker op=boxcall1 method=delete remained"
  exit 1
fi
if ! printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^false$'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_mapbox_delete_ported_vm: expected printed deleted-key result false"
  exit 1
fi
if ! printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^1$'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_mapbox_delete_ported_vm: expected printed size result 1"
  exit 1
fi
if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_mapbox_delete_ported_vm: expected RC=0 (got rc=$VM_HAKO_CAPS_EXIT_CODE)"
  exit 1
fi

vm_hako_caps_assert_no_unimplemented "$SMOKE_NAME" "$OUTPUT_CLEAN" || exit 1
test_pass "vm_hako_caps_mapbox_delete_ported_vm: PASS (RVP-C21 ported contract pinned)"

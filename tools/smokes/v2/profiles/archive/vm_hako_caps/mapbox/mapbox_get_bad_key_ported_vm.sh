#!/bin/bash
# RVP-C25: vm-hako capability smoke for MapBox.get(non-string key)
#
# Contract:
# 1) MIR preflight must contain `mir_call(MapBox.get,args=1)`.
# 2) vm-hako route prints stable `[map/bad-key] key must be string`.
# 3) stale scalar `0` must not reappear.
# 4) timeout is forbidden.

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

SMOKE_NAME="vm_hako_caps_mapbox_get_bad_key_ported_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/vm_hako_caps/mapbox_get_bad_key_ported_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c25.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[]?.blocks[]?.instructions[]? | select(.op=="mir_call" and .mir_call.callee.type=="Method" and .mir_call.callee.box_name=="MapBox" and .mir_call.callee.name=="get" and (.mir_call.args|length)==1)' \
  "MIR missing mir_call(MapBox.get,args=1) shape" || exit 1

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

OUTPUT_CLEAN=$(printf '%s\n' "$VM_HAKO_CAPS_OUTPUT" | filter_noise || true)
if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^0$'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_mapbox_get_bad_key_ported_vm: stale scalar 0 remained for bad-key get"
  exit 1
fi
if ! printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[map/bad-key\] key must be string$'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_mapbox_get_bad_key_ported_vm: expected stable bad-key text"
  exit 1
fi
if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "vm_hako_caps_mapbox_get_bad_key_ported_vm: expected RC=0 (got rc=$VM_HAKO_CAPS_EXIT_CODE)"
  exit 1
fi

vm_hako_caps_assert_no_unimplemented "$SMOKE_NAME" "$OUTPUT_CLEAN" || exit 1
test_pass "vm_hako_caps_mapbox_get_bad_key_ported_vm: PASS (RVP-C25 ported contract pinned)"

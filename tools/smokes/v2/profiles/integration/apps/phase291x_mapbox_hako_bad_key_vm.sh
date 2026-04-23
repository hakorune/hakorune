#!/bin/bash
# Phase 291x: vm-hako MapBox bad-key source-route smoke.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../../.." && pwd)"
fi

source "$ROOT/tools/smokes/v2/lib/test_runner.sh"
source "$ROOT/tools/smokes/v2/profiles/integration/vm_hako_caps/lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="phase291x_mapbox_hako_bad_key_vm"
INPUT="${1:-$ROOT/apps/tests/phase291x_mapbox_hako_bad_key_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/phase291x_mapbox_bad_key.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" || exit 1
for method in set get has delete remove; do
  vm_hako_caps_assert_mir_jq \
    "$SMOKE_NAME" \
    "$TMP_MIR" \
    ".functions[]?.blocks[]?.instructions[]? | select(.op==\"mir_call\" and .mir_call.callee.type==\"Method\" and .mir_call.callee.box_name==\"MapBox\" and .mir_call.callee.name==\"${method}\")" \
    "MIR missing mir_call(MapBox.${method}) shape" || exit 1
done

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

OUTPUT_CLEAN=$(printf '%s\n' "$VM_HAKO_CAPS_OUTPUT" | filter_noise || true)
if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm/method/stub:(set|get|has|delete|remove)\]$'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: unexpected runtime stub marker"
  exit 1
fi
if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/(unimplemented|contract)'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: unexpected vm-hako contract/unimplemented tag"
  exit 1
fi
bad_key_count=$(printf '%s\n' "$OUTPUT_CLEAN" | rg -c '^\[map/bad-key\] key must be string$' || true)
if [ "$bad_key_count" -ne 5 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected five stable bad-key lines, got $bad_key_count"
  exit 1
fi
if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected RC=0 (got rc=$VM_HAKO_CAPS_EXIT_CODE)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (MapBox bad-key normalization pinned)"

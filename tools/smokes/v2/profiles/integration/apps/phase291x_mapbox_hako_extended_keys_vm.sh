#!/bin/bash
# Phase 291x: vm-hako MapBox.keys() source-route smoke.
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

SMOKE_NAME="phase291x_mapbox_hako_extended_keys_vm"
INPUT="${1:-$ROOT/apps/tests/phase291x_mapbox_hako_keys_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/phase291x_mapbox_keys.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

set +e
EMIT_OUT=$(vm_hako_caps_timeout_profile "$RUN_TIMEOUT_SECS" \
  env NYASH_MIR_UNIFIED_CALL=1 "$NYASH_BIN" --emit-mir-json "$TMP_MIR" "$INPUT" 2>&1)
EMIT_RC=$?
set -e
if [ "$EMIT_RC" -ne 0 ]; then
  echo "$EMIT_OUT" | tail -n 120 >&2 || true
  test_fail "$SMOKE_NAME: unified emit failed rc=$EMIT_RC"
  exit 1
fi
if ! jq -e '.functions[]?.blocks[]?.instructions[]? | select(.op=="mir_call" and .mir_call.callee.type=="Method" and .mir_call.callee.box_name=="MapBox" and .mir_call.callee.name=="keys" and (.mir_call.args|length)==1)' \
  "$TMP_MIR" >/dev/null 2>&1; then
  jq '.functions[]?.blocks[]?.instructions[]? | select(.op=="mir_call")' "$TMP_MIR" >&2 || true
  test_fail "$SMOKE_NAME: MIR missing MapBox.keys receiver-mirror shape"
  exit 1
fi

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

OUTPUT_CLEAN=$(printf '%s\n' "$VM_HAKO_CAPS_OUTPUT" | filter_noise || true)
if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/unimplemented'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: unexpected vm-hako unimplemented tag"
  exit 1
fi
if printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^\[vm-hako/contract'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: unexpected vm-hako contract tag"
  exit 1
fi
if ! printf '%s\n' "$OUTPUT_CLEAN" | rg -q '^2$'; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected printed keys size 2"
  exit 1
fi
if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$OUTPUT_CLEAN" | tail -n 120 || true
  test_fail "$SMOKE_NAME: expected RC=0 (got rc=$VM_HAKO_CAPS_EXIT_CODE)"
  exit 1
fi

test_pass "$SMOKE_NAME: PASS (MapBox.keys vm-hako shape pinned)"

#!/bin/bash
# RVP-C09: retired vm-hako const(void) evidence after phase-96x demotion
#
# Contract:
# 1) MIR preflight must contain const(type:void) shape.
# 2) vm-hako route must execute const(void) path without subset/contract errors.
# 3) execution must finish with RC: 0 marker.

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

SMOKE_NAME="vm_hako_caps_const_void_ported_archive_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/vm_hako_caps/const_void_block_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-60}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c09_archive.XXXXXX.json)"
cleanup() {
  rm -f "$TMP_MIR"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1

vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="const" and .value.type=="void")' \
  "MIR missing const(type:void) shape" || exit 1

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$VM_HAKO_CAPS_OUTPUT" | tail -n 80 || true
  test_fail "$SMOKE_NAME: expected rc=0 for const(void) path"
  exit 1
fi

vm_hako_caps_assert_no_unimplemented "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1
vm_hako_caps_assert_no_contract "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1
vm_hako_caps_assert_rc_marker "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" "0" || exit 1

test_pass "$SMOKE_NAME: PASS (retired vm-hako const(void) evidence pinned)"

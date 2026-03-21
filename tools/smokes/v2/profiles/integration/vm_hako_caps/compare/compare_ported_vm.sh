#!/bin/bash
# RVP-C06: vm-hako capability smoke for compare(!=) route (ported pin)
#
# Contract:
# 1) MIR preflight must contain compare operation `!=`.
# 2) vm-hako route must execute compare(!=) without emit/subset/runtime contract errors.
# 3) exit code must be zero with stable RC marker.
# 4) runtime alias path (`op_kind=Ne`) must remain executable via MiniVmS0Entry.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../lib/vm_hako_caps_common.sh"
require_env || exit 2

SMOKE_NAME="vm_hako_caps_compare_ported_vm"
INPUT="${1:-$NYASH_ROOT/apps/tests/vm_hako_caps/compare_block_min.hako}"
NE_ALIAS_INPUT="${NE_ALIAS_INPUT:-$NYASH_ROOT/apps/tests/vm_hako_caps/compare_op_kind_ne_mir_v0.json}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
TMP_MIR="$(mktemp /tmp/vm_hako_caps_c06.XXXXXX.json)"
TMP_DRIVER="$(mktemp /tmp/vm_hako_caps_c06_driver.XXXXXX.hako)"
cleanup() {
  rm -f "$TMP_MIR" "$TMP_DRIVER"
}
trap cleanup EXIT

vm_hako_caps_require_fixture "$SMOKE_NAME" "$INPUT" || exit 1
vm_hako_caps_require_fixture "$SMOKE_NAME" "$NE_ALIAS_INPUT" || exit 1

vm_hako_caps_emit_mir_or_fail "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$TMP_MIR" "$INPUT" || exit 1
vm_hako_caps_assert_mir_jq \
  "$SMOKE_NAME" \
  "$TMP_MIR" \
  '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="compare" and .operation=="!=")' \
  "MIR missing compare(!=) shape" || exit 1

vm_hako_caps_run_vm_hako_or_fail_timeout "$SMOKE_NAME" "$RUN_TIMEOUT_SECS" "$INPUT" || exit 1

if [ "$VM_HAKO_CAPS_EXIT_CODE" -ne 0 ]; then
  echo "$VM_HAKO_CAPS_OUTPUT" | tail -n 80 || true
  test_fail "vm_hako_caps_compare_ported_vm: expected success exit (rc=0), got rc=$VM_HAKO_CAPS_EXIT_CODE"
  exit 1
fi

vm_hako_caps_assert_no_unimplemented "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1
vm_hako_caps_assert_no_contract "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" || exit 1
if echo "$VM_HAKO_CAPS_OUTPUT" | rg -q "^\[vm-hako/emit-error\]"; then
  echo "$VM_HAKO_CAPS_OUTPUT" | tail -n 80 || true
  test_fail "vm_hako_caps_compare_ported_vm: unexpected emit-error tag"
  exit 1
fi
vm_hako_caps_assert_rc_marker "$SMOKE_NAME" "$VM_HAKO_CAPS_OUTPUT" "0" || exit 1

# runtime alias pin: run MiniVmS0Entry directly with op_kind=Ne payload
NE_ALIAS_JSON_PAYLOAD="$(tr -d '\n\r' < "$NE_ALIAS_INPUT")"
cat >"$TMP_DRIVER" <<'HKO'
using selfhost.vm.entry_s0 as MiniVmS0EntryBox
static box Main {
  main(args) {
    local j = env.get("NYASH_VERIFY_JSON")
    if j == null || j == "" {
      print("[vm-hako/contract][missing-json]")
      return 1
    }
    return MiniVmS0EntryBox.run_min(j)
  }
}
HKO

set +e
NE_ALIAS_OUTPUT=$(
  env -u HAKO_VERIFY_PRIMARY -u HAKO_ROUTE_HAKOVM \
    NYASH_VERIFY_JSON="$NE_ALIAS_JSON_PAYLOAD" \
    NYASH_PREINCLUDE=1 \
    NYASH_USING_AST=1 \
    NYASH_RESOLVE_FIX_BRACES=1 \
    NYASH_FEATURES=stage3 \
    NYASH_PARSER_ALLOW_SEMICOLON=1 \
    NYASH_PARSER_SEAM_TOLERANT=1 \
    NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
    NYASH_ENABLE_USING=1 \
    HAKO_ENABLE_USING=1 \
    NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
    NYASH_DISABLE_NY_COMPILER=1 \
    HAKO_DISABLE_NY_COMPILER=1 \
    NYASH_USE_NY_COMPILER=0 \
    HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
    timeout "$RUN_TIMEOUT_SECS" \
    "$NYASH_BIN" --backend vm "$TMP_DRIVER" 2>&1
)
NE_ALIAS_RC=$?
set -e

if [ "$NE_ALIAS_RC" -eq 124 ]; then
  test_fail "vm_hako_caps_compare_ported_vm: op_kind=Ne runtime alias probe timed out"
  exit 1
fi
if [ "$NE_ALIAS_RC" -ne 0 ]; then
  echo "$NE_ALIAS_OUTPUT" | tail -n 100 || true
  test_fail "vm_hako_caps_compare_ported_vm: op_kind=Ne runtime alias probe failed (rc=$NE_ALIAS_RC)"
  exit 1
fi
if echo "$NE_ALIAS_OUTPUT" | rg -q "^\[vm-hako/unimplemented\]"; then
  echo "$NE_ALIAS_OUTPUT" | tail -n 100 || true
  test_fail "vm_hako_caps_compare_ported_vm: op_kind=Ne emitted vm-hako unimplemented tag"
  exit 1
fi
if echo "$NE_ALIAS_OUTPUT" | rg -q "^\[vm-hako/contract"; then
  echo "$NE_ALIAS_OUTPUT" | tail -n 100 || true
  test_fail "vm_hako_caps_compare_ported_vm: op_kind=Ne emitted vm-hako contract tag"
  exit 1
fi

test_pass "vm_hako_caps_compare_ported_vm: PASS (RVP-C06 ported contract + op_kind=Ne runtime alias pinned)"

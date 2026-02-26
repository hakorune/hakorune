#!/bin/bash
# Phase29z-S4-clean-2: deep-payload scanner depth guard smoke
#
# Contract:
# - rust-vm core route can execute deep JSON payload (baseline behavior)
# - hako-runner route must fail-fast with stable scan-depth contract tag
#   instead of crashing with stack overflow.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
PAD_LEN="${VM_HAKO_SCANNER_PAD_LEN:-10000}"

TMP_JSON="${TMPDIR:-/tmp}/phase29z_vm_hako_s4_scanner_depth_$$.json"
TMP_DRIVER="${TMPDIR:-/tmp}/phase29z_vm_hako_s4_scanner_depth_driver_$$.hako"
cleanup() {
    rm -f "$TMP_JSON" "$TMP_DRIVER"
}
trap cleanup EXIT

PAD="$(printf '%*s' "$PAD_LEN" '')"
cat >"$TMP_JSON" <<JSON
{
  "functions": [
    {
      "name": "main",
      "entry_block": 0,
      "blocks": [
        {
          "id": 0,
          "instructions": [${PAD}
            { "op": "const", "dst": 1, "value": { "type": "i64", "value": 40 } },
            { "op": "const", "dst": 2, "value": { "type": "i64", "value": 2 } },
            { "op": "binop", "operation": "+", "lhs": 1, "rhs": 2, "dst": 3 },
            { "op": "ret", "value": 3 }
          ]
        }
      ]
    }
  ]
}
JSON

JSON_PAYLOAD="$(tr -d '\n\r' < "$TMP_JSON")"

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
RUST_OUTPUT=$(
    env \
        HAKO_VERIFY_PRIMARY=hakovm \
        NYASH_VERIFY_JSON="$JSON_PAYLOAD" \
        timeout "$RUN_TIMEOUT_SECS" \
        "$NYASH_BIN" --backend vm "$NYASH_ROOT/basic_test.hako" 2>&1
)
RUST_RC=$?
set -e

if [ "$RUST_RC" -eq 124 ]; then
    test_fail "phase29z_vm_hako_s4_scanner_depth_guard_parity_vm: rust-vm route timed out"
    exit 1
fi
if [ "$RUST_RC" -ne 42 ]; then
    echo "$RUST_OUTPUT" | tail -n 80 || true
    test_fail "phase29z_vm_hako_s4_scanner_depth_guard_parity_vm: rust-vm rc mismatch"
    exit 1
fi

set +e
HAKO_OUTPUT=$(
    env -u HAKO_VERIFY_PRIMARY -u HAKO_ROUTE_HAKOVM \
        NYASH_VERIFY_JSON="$JSON_PAYLOAD" \
        NYASH_PREINCLUDE=1 \
        NYASH_USING_AST=1 \
        NYASH_RESOLVE_FIX_BRACES=1 \
        NYASH_FEATURES=stage3 \
        NYASH_PARSER_ALLOW_SEMICOLON=1 \
        NYASH_PARSER_SEAM_TOLERANT=1 \
        NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
        NYASH_ENABLE_USING=1 \
        HAKO_ENABLE_USING=1 \
        NYASH_DISABLE_NY_COMPILER=1 \
        HAKO_DISABLE_NY_COMPILER=1 \
        NYASH_USE_NY_COMPILER=0 \
        HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
        timeout "$RUN_TIMEOUT_SECS" \
        "$NYASH_BIN" --backend vm "$TMP_DRIVER" 2>&1
)
HAKO_RC=$?
set -e

if [ "$HAKO_RC" -eq 124 ]; then
    test_fail "phase29z_vm_hako_s4_scanner_depth_guard_parity_vm: hako-runner route timed out"
    exit 1
fi
if [ "$HAKO_RC" -eq 0 ]; then
    echo "$HAKO_OUTPUT" | tail -n 80 || true
    test_fail "phase29z_vm_hako_s4_scanner_depth_guard_parity_vm: expected non-zero on depth guard"
    exit 1
fi
if ! echo "$HAKO_OUTPUT" | rg -q '^\[vm-hako/contract\]\[scan-depth-exceeded kind='; then
    echo "$HAKO_OUTPUT" | tail -n 120 || true
    test_fail "phase29z_vm_hako_s4_scanner_depth_guard_parity_vm: missing depth guard contract tag"
    exit 1
fi

test_pass "phase29z_vm_hako_s4_scanner_depth_guard_parity_vm: PASS (rust-vm=$RUST_RC hako-runner=$HAKO_RC)"

#!/bin/bash
# vm_hako_json_parity_common.sh — shared parity runner for vm-hako JSON-route smokes
#
# Required environment variables:
# - VM_HAKO_PARITY_NAME: smoke test id (for diagnostics)
# - VM_HAKO_PARITY_OPCODE: opcode that must exist in the fixture
# - VM_HAKO_PARITY_FIXTURE_REL: fixture path relative to $NYASH_ROOT
# Optional:
# - VM_HAKO_PARITY_EXPECTED_RC (default: 42)
# - VM_HAKO_PARITY_DENY_OPS (comma-separated op list that must not appear in fixture)
#
# Route contract:
# - Both rust-vm and hako-runner subprocesses are executed via vm route pin helper
#   so strict/dev default changes do not drift parity probes.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/vm_route_pin.sh"
require_env || exit 2

SMOKE_NAME="${VM_HAKO_PARITY_NAME:-}"
OPCODE="${VM_HAKO_PARITY_OPCODE:-}"
FIXTURE_REL="${VM_HAKO_PARITY_FIXTURE_REL:-}"
EXPECTED_RC="${VM_HAKO_PARITY_EXPECTED_RC:-42}"
DENY_OPS_RAW="${VM_HAKO_PARITY_DENY_OPS:-}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ -z "$SMOKE_NAME" ] || [ -z "$OPCODE" ] || [ -z "$FIXTURE_REL" ]; then
    test_fail "vm_hako_json_parity_common: missing required env (name/opcode/fixture_rel)"
    exit 1
fi

INPUT_JSON="${1:-$NYASH_ROOT/$FIXTURE_REL}"
if [ ! -f "$INPUT_JSON" ]; then
    test_fail "$SMOKE_NAME: fixture missing: $INPUT_JSON"
    exit 1
fi

if ! rg -q "\"op\"\\s*:\\s*\"$OPCODE\"" "$INPUT_JSON"; then
    test_fail "$SMOKE_NAME: $OPCODE opcode not found in fixture"
    exit 1
fi

if [ -n "$DENY_OPS_RAW" ]; then
    IFS=',' read -r -a DENY_OPS <<<"$DENY_OPS_RAW"
    for deny in "${DENY_OPS[@]}"; do
        deny="${deny// /}"
        if [ -z "$deny" ]; then
            continue
        fi
        if rg -q "\"op\"\\s*:\\s*\"$deny\"" "$INPUT_JSON"; then
            test_fail "$SMOKE_NAME: fixture includes denied op '$deny'"
            exit 1
        fi
    done
fi

JSON_PAYLOAD=$(tr -d '\n\r' < "$INPUT_JSON")
TMP_DRIVER="${TMPDIR:-/tmp}/${SMOKE_NAME}_driver_$$.hako"
cleanup() {
    rm -f "$TMP_DRIVER"
}
trap cleanup EXIT

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

run_rust_vm_json() {
    local output
    local rc
    set +e
    output=$(
        run_with_vm_route_pin env \
            HAKO_VERIFY_PRIMARY=hakovm \
            NYASH_VERIFY_JSON="$JSON_PAYLOAD" \
            timeout "$RUN_TIMEOUT_SECS" \
            "$NYASH_BIN" --backend vm "$NYASH_ROOT/basic_test.hako" 2>&1
    )
    rc=$?
    set -e
    echo "$output"
    return "$rc"
}

run_hako_vm_runner() {
    local output
    local rc
    set +e
    output=$(
        run_with_vm_route_pin env -u HAKO_VERIFY_PRIMARY -u HAKO_ROUTE_HAKOVM \
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
    rc=$?
    set -e
    echo "$output"
    return "$rc"
}

set +e
RUST_OUTPUT=$(run_rust_vm_json)
RUST_RC=$?
set -e

if [ "$RUST_RC" -eq 124 ]; then
    test_fail "$SMOKE_NAME: rust-vm route timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

set +e
HAKO_OUTPUT=$(run_hako_vm_runner)
HAKO_RC=$?
set -e

if [ "$HAKO_RC" -eq 124 ]; then
    test_fail "$SMOKE_NAME: hako-runner route timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$RUST_RC" -ne "$EXPECTED_RC" ]; then
    echo "[FAIL] expected rust-vm route rc=$EXPECTED_RC, got rc=$RUST_RC"
    echo "$RUST_OUTPUT" | tail -n 80 || true
    test_fail "$SMOKE_NAME: rust-vm rc mismatch"
    exit 1
fi

if [ "$HAKO_RC" -ne "$EXPECTED_RC" ]; then
    echo "[FAIL] expected hako-runner route rc=$EXPECTED_RC, got rc=$HAKO_RC"
    echo "$HAKO_OUTPUT" | tail -n 120 || true
    test_fail "$SMOKE_NAME: hako-runner rc mismatch"
    exit 1
fi

if [ "$RUST_RC" -ne "$HAKO_RC" ]; then
    echo "[FAIL] parity mismatch: rust-vm=$RUST_RC hako-runner=$HAKO_RC"
    test_fail "$SMOKE_NAME: parity mismatch"
    exit 1
fi

if echo "$HAKO_OUTPUT" | rg -q '^\[vm-hako/unimplemented\]'; then
    echo "[FAIL] hako-runner reported unimplemented on $OPCODE fixture"
    echo "$HAKO_OUTPUT" | tail -n 120 || true
    test_fail "$SMOKE_NAME: unexpected unimplemented tag"
    exit 1
fi

if echo "$HAKO_OUTPUT" | rg -q '^\[vm-hako/contract\]'; then
    echo "[FAIL] hako-runner reported contract failure on $OPCODE fixture"
    echo "$HAKO_OUTPUT" | tail -n 120 || true
    test_fail "$SMOKE_NAME: unexpected contract tag"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (rust-vm=$RUST_RC hako-runner=$HAKO_RC)"

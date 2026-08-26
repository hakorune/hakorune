#!/bin/bash
# Phase29z-S5i-clean: await non-future precondition reject smoke
#
# Contract:
# - vm core route must fail on await(non-future)
# - hako-runner route must fail-fast with [await-non-future]

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/vm_route_pin.sh"
require_env || exit 2

RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-60}"
INPUT_JSON="${1:-$NYASH_ROOT/apps/tests/phase29z_vm_hako_s5_await_non_future_reject_mir_v0.json}"
DRIVER="$NYASH_ROOT/src/runner/reference/vm_hako/driver_main.hako"

if [ ! -f "$INPUT_JSON" ]; then
    test_fail "phase29z_vm_hako_s5_await_non_future_reject_vm: fixture missing: $INPUT_JSON"
    exit 1
fi
if [ ! -f "$DRIVER" ]; then
    test_fail "phase29z_vm_hako_s5_await_non_future_reject_vm: driver missing: $DRIVER"
    exit 1
fi

JSON_PAYLOAD="$(tr -d '\n\r' < "$INPUT_JSON")"

set +e
RUST_OUTPUT=$(
    run_with_vm_route_pin env \
        HAKO_VERIFY_PRIMARY=hakovm \
        NYASH_VERIFY_JSON="$JSON_PAYLOAD" \
        timeout "$RUN_TIMEOUT_SECS" \
        "$NYASH_BIN" --backend vm "$NYASH_ROOT/basic_test.hako" 2>&1
)
RUST_RC=$?
set -e

if [ "$RUST_RC" -eq 124 ]; then
    test_fail "phase29z_vm_hako_s5_await_non_future_reject_vm: vm route timed out"
    exit 1
fi
if [ "$RUST_RC" -eq 0 ]; then
    echo "$RUST_OUTPUT" | tail -n 80 || true
    test_fail "phase29z_vm_hako_s5_await_non_future_reject_vm: vm expected non-zero"
    exit 1
fi

set +e
HAKO_OUTPUT=$(
    run_with_vm_route_pin env \
        -u NYASH_VERIFY_JSON \
        -u HAKO_VERIFY_PRIMARY \
        -u HAKO_ROUTE_HAKOVM \
        -u HAKO_VERIFY_V1_FORCE_HAKOVM \
        -u NYASH_USE_STAGE1_CLI \
        -u HAKO_STAGE1_ENABLE \
        -u HAKO_EMIT_PROGRAM_JSON \
        -u HAKO_EMIT_MIR_JSON \
        -u NYASH_STAGE1_CLI_CHILD \
        -u HAKO_PROGRAM_JSON \
        -u HAKO_PROGRAM_JSON_FILE \
        -u HAKO_STAGE1_PROGRAM_JSON \
        -u NYASH_STAGE1_PROGRAM_JSON \
        -u NYASH_STAGE1_MODE \
        -u HAKO_STAGE1_MODE \
        -u NYASH_STAGE1_INPUT \
        -u HAKO_STAGE1_INPUT \
        -u STAGE1_INPUT \
        -u NYASH_STAGE1_BACKEND \
        -u HAKO_STAGE1_BACKEND \
        -u STAGE1_BACKEND \
        -u NYASH_EMIT_MIR_TRACE \
        -u HAKO_VM_HAKO_DRIVER_PAYLOAD_JSON \
        -u HAKO_VM_HAKO_DRIVER_PAYLOAD_FILE \
        HAKO_VM_HAKO_DRIVER_PAYLOAD_JSON="$JSON_PAYLOAD" \
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
        NYASH_SKIP_TOML_ENV=1 \
        NYASH_VM_USE_FALLBACK=0 \
        NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
        HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
        timeout "$RUN_TIMEOUT_SECS" \
        "$NYASH_BIN" --backend vm "$DRIVER" 2>&1
)
HAKO_RC=$?
set -e

if [ "$HAKO_RC" -eq 124 ]; then
    test_fail "phase29z_vm_hako_s5_await_non_future_reject_vm: hako-runner route timed out"
    exit 1
fi
if [ "$HAKO_RC" -eq 0 ]; then
    echo "$HAKO_OUTPUT" | tail -n 80 || true
    test_fail "phase29z_vm_hako_s5_await_non_future_reject_vm: hako-runner expected non-zero"
    exit 1
fi
if ! echo "$HAKO_OUTPUT" | rg -q '^\[vm-hako/contract\]\[await-non-future\]'; then
    echo "$HAKO_OUTPUT" | tail -n 120 || true
    test_fail "phase29z_vm_hako_s5_await_non_future_reject_vm: missing await-non-future tag"
    exit 1
fi

test_pass "phase29z_vm_hako_s5_await_non_future_reject_vm: PASS (vm=$RUST_RC hako-runner=$HAKO_RC)"

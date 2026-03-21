#!/bin/bash
# Phase 29x X33: de-rust verifier path smoke
#
# Contract pin:
# 1) `.hako` verifier skeleton fails fast when Rust/Hako verifier result mismatches.
# 2) mismatch emits stable freeze tag `[freeze:contract][derust-verifier/mismatch]`.
# 3) matched result emits stable check tag `[derust-verifier/check]`.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../lib/vm_route_pin.sh"
require_env || exit 2

INPUT="$NYASH_ROOT/lang/src/vm/verifier_gate_skeleton.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -f "$INPUT" ]; then
    test_fail "phase29x_derust_verifier_vm: fixture missing: $INPUT"
    exit 1
fi

run_case() {
    local name="$1"
    local expect_rc="$2"
    local route="$3"
    local function_name="$4"
    local rust_errors="$5"
    local hako_errors="$6"

    local out=""
    local rc=0

    set +e
    out=$(
      run_with_vm_route_pin env \
      NYASH_DISABLE_PLUGINS=1 \
      HAKO_DERUST_VERIFY_ROUTE="$route" \
      HAKO_DERUST_VERIFY_FUNCTION="$function_name" \
      HAKO_DERUST_VERIFY_RUST_ERRORS="$rust_errors" \
      HAKO_DERUST_VERIFY_HAKO_ERRORS="$hako_errors" \
      timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$INPUT" 2>&1
    )
    rc=$?
    set -e

    if [ "$rc" -eq 124 ]; then
        test_fail "phase29x_derust_verifier_vm: run timed out (case=$name, >${RUN_TIMEOUT_SECS}s)"
        exit 1
    fi

    if [ "$rc" -ne "$expect_rc" ]; then
        echo "[INFO] output (case=$name):"
        echo "$out" | head -n 120 || true
        test_fail "phase29x_derust_verifier_vm: unexpected rc (case=$name expect=$expect_rc got=$rc)"
        exit 1
    fi

    if [ "$expect_rc" -ne 0 ]; then
        if ! echo "$out" | rg -q "^\[freeze:contract\]\[derust-verifier/mismatch\] route=${route} function=${function_name} rust_errors=${rust_errors} hako_errors=${hako_errors}$"; then
            echo "[INFO] output (case=$name):"
            echo "$out" | head -n 120 || true
            test_fail "phase29x_derust_verifier_vm: missing mismatch freeze tag (case=$name)"
            exit 1
        fi
    else
        if ! echo "$out" | rg -q "^\[derust-verifier/check\] route=${route} function=${function_name} errors=${rust_errors} source=hako-skeleton$"; then
            echo "[INFO] output (case=$name):"
            echo "$out" | head -n 120 || true
            test_fail "phase29x_derust_verifier_vm: missing verifier check tag (case=$name)"
            exit 1
        fi
    fi

    echo "[INFO] case=$name rc=$rc route=$route function=$function_name rust_errors=$rust_errors hako_errors=$hako_errors"
}

run_case "mismatch-failfast" 1 "vm" "main" "2" "1"
run_case "match-pass" 0 "vm-hako" "main" "3" "3"

test_pass "phase29x_derust_verifier_vm: PASS (mismatch fail-fast + match pass)"

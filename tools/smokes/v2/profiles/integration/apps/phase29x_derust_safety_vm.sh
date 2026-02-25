#!/bin/bash
# Phase 29x X34: de-rust safety path smoke
#
# Contract pin:
# 1) `.hako` safety skeleton fails fast on lifecycle contract violation.
# 2) lifecycle violation emits stable freeze tag `[freeze:contract][derust-safety/lifecycle]`.
# 3) safety-clean case emits stable check tag `[derust-safety/check]`.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/vm_route_pin.sh"
require_env || exit 2

INPUT="$NYASH_ROOT/lang/src/vm/safety_gate_skeleton.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -f "$INPUT" ]; then
    test_fail "phase29x_derust_safety_vm: fixture missing: $INPUT"
    exit 1
fi

run_case() {
    local name="$1"
    local expect_rc="$2"
    local route="$3"
    local hako_source="$4"
    local function_name="$5"
    local bb="$6"
    local inst_idx="$7"
    local reason="$8"

    local out=""
    local rc=0

    set +e
    out=$(
      run_with_vm_route_pin env \
      NYASH_DISABLE_PLUGINS=1 \
      HAKO_DERUST_SAFETY_ROUTE="$route" \
      HAKO_DERUST_SAFETY_HAKO_SOURCE="$hako_source" \
      HAKO_DERUST_SAFETY_FUNCTION="$function_name" \
      HAKO_DERUST_SAFETY_BB="$bb" \
      HAKO_DERUST_SAFETY_INST_IDX="$inst_idx" \
      HAKO_DERUST_SAFETY_REASON="$reason" \
      timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$INPUT" 2>&1
    )
    rc=$?
    set -e

    if [ "$rc" -eq 124 ]; then
        test_fail "phase29x_derust_safety_vm: run timed out (case=$name, >${RUN_TIMEOUT_SECS}s)"
        exit 1
    fi

    if [ "$rc" -ne "$expect_rc" ]; then
        echo "[INFO] output (case=$name):"
        echo "$out" | head -n 120 || true
        test_fail "phase29x_derust_safety_vm: unexpected rc (case=$name expect=$expect_rc got=$rc)"
        exit 1
    fi

    if [ "$expect_rc" -ne 0 ]; then
        if ! echo "$out" | rg -F -q "[freeze:contract][derust-safety/lifecycle] route=${route} fn=${function_name} bb=${bb} inst_idx=${inst_idx} reason=${reason}"; then
            echo "[INFO] output (case=$name):"
            echo "$out" | head -n 120 || true
            test_fail "phase29x_derust_safety_vm: missing lifecycle freeze tag (case=$name)"
            exit 1
        fi
    else
        if ! echo "$out" | rg -F -q "[derust-safety/check] route=${route} status=ok source=hako-skeleton"; then
            echo "[INFO] output (case=$name):"
            echo "$out" | head -n 120 || true
            test_fail "phase29x_derust_safety_vm: missing safety check tag (case=$name)"
            exit 1
        fi
    fi

    echo "[INFO] case=$name rc=$rc route=$route reason=$reason"
}

run_case "lifecycle-failfast" 1 "vm-fallback" "0" "main" "BasicBlockId(1)" "2" "release_strong-empty-values"
run_case "safety-clean-pass" 0 "vm-hako" "0" "main" "BasicBlockId(0)" "0" "ok"

test_pass "phase29x_derust_safety_vm: PASS (lifecycle fail-fast + clean pass)"

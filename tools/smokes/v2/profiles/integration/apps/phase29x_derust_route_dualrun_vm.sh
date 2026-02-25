#!/bin/bash
# Phase 29x X32: de-rust route dual-run smoke
#
# Contract pin:
# 1) Rust route orchestrator (`[vm-route/select]`) and `.hako` skeleton
#    (`[derust-route/select]`) choose the same lane for canonical cases.
# 2) `.hako` skeleton only mirrors selection policy; it does not execute backend lanes.
# 3) Precedence is fixed: explicit fallback > prefer vm-hako > vm default.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/vm_route_pin.sh"
require_env || exit 2

RUST_VM_INPUT="$NYASH_ROOT/apps/tests/phase285_leak_report.hako"
RUST_VM_HAKO_INPUT="$NYASH_ROOT/apps/tests/phase29z_vm_hako_s0_reject_compare_ne_min.hako"
HAKO_SKELETON_INPUT="$NYASH_ROOT/lang/src/vm/route_orchestrator_skeleton.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

for f in "$RUST_VM_INPUT" "$RUST_VM_HAKO_INPUT" "$HAKO_SKELETON_INPUT"; do
    if [ ! -f "$f" ]; then
        test_fail "phase29x_derust_route_dualrun_vm: fixture missing: $f"
        exit 1
    fi
done

extract_rust_lane() {
    local out="$1"
    # Pick the first route decision for the target input.
    # Some fixtures trigger nested dispatch traces later in the same run.
    echo "$out" \
        | rg '^\[vm-route/select\] backend=' \
        | head -n 1 \
        | sed -E 's/^.* lane=([^ ]+) reason=.*$/\1/'
}

extract_hako_lane() {
    local out="$1"
    echo "$out" \
        | rg '^\[derust-route/select\] backend=' \
        | tail -n 1 \
        | sed -E 's/^.* lane=([^ ]+) source=.*$/\1/'
}

run_case() {
    local name="$1"
    local backend="$2"
    local force_fallback="$3"
    local prefer_vm_hako="$4"
    local rust_input="$5"

    local rust_out=""
    local rust_rc=0
    local rust_lane=""
    local hako_out=""
    local hako_rc=0
    local hako_lane=""

    set +e
    rust_out=$(
      NYASH_VM_ROUTE_TRACE=1 \
      NYASH_VM_USE_FALLBACK="$force_fallback" \
      NYASH_VM_HAKO_PREFER_STRICT_DEV="$prefer_vm_hako" \
      NYASH_DISABLE_PLUGINS=1 \
      timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend "$backend" "$rust_input" 2>&1
    )
    rust_rc=$?
    set -e

    if [ "$rust_rc" -eq 124 ]; then
        test_fail "phase29x_derust_route_dualrun_vm: rust route run timed out (case=$name, >${RUN_TIMEOUT_SECS}s)"
        exit 1
    fi

    rust_lane="$(extract_rust_lane "$rust_out")"
    if [ -z "$rust_lane" ]; then
        echo "[INFO] rust output (case=$name):"
        echo "$rust_out" | head -n 120 || true
        test_fail "phase29x_derust_route_dualrun_vm: missing rust route select tag (case=$name)"
        exit 1
    fi

    set +e
    hako_out=$(
      run_with_vm_route_pin env \
      NYASH_VM_USE_FALLBACK=0 \
      NYASH_DISABLE_PLUGINS=1 \
      HAKO_DERUST_ROUTE_BACKEND="$backend" \
      HAKO_DERUST_ROUTE_FORCE_FALLBACK="$force_fallback" \
      HAKO_DERUST_ROUTE_PREFER_VM_HAKO="$prefer_vm_hako" \
      timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$HAKO_SKELETON_INPUT" 2>&1
    )
    hako_rc=$?
    set -e

    if [ "$hako_rc" -eq 124 ]; then
        test_fail "phase29x_derust_route_dualrun_vm: hako skeleton run timed out (case=$name, >${RUN_TIMEOUT_SECS}s)"
        exit 1
    fi
    if [ "$hako_rc" -ne 0 ]; then
        echo "[INFO] hako output (case=$name):"
        echo "$hako_out" | head -n 120 || true
        test_fail "phase29x_derust_route_dualrun_vm: hako skeleton exited non-zero (case=$name rc=$hako_rc)"
        exit 1
    fi

    hako_lane="$(extract_hako_lane "$hako_out")"
    if [ -z "$hako_lane" ]; then
        echo "[INFO] hako output (case=$name):"
        echo "$hako_out" | head -n 120 || true
        test_fail "phase29x_derust_route_dualrun_vm: missing hako route select tag (case=$name)"
        exit 1
    fi

    if [ "$rust_lane" != "$hako_lane" ]; then
        echo "[INFO] rust output (case=$name):"
        echo "$rust_out" | head -n 120 || true
        echo "[INFO] hako output (case=$name):"
        echo "$hako_out" | head -n 120 || true
        test_fail "phase29x_derust_route_dualrun_vm: lane mismatch (case=$name rust=$rust_lane hako=$hako_lane)"
        exit 1
    fi

    echo "[INFO] case=$name backend=$backend force_fallback=$force_fallback prefer_vm_hako=$prefer_vm_hako rust_rc=$rust_rc hako_rc=$hako_rc lane=$rust_lane"
}

run_case "vm-default" "vm" "0" "0" "$RUST_VM_INPUT"
run_case "vm-prefer-hako" "vm" "0" "1" "$RUST_VM_HAKO_INPUT"
run_case "vm-fallback-priority" "vm" "1" "1" "$RUST_VM_INPUT"
run_case "vm-hako-explicit" "vm-hako" "0" "0" "$RUST_VM_HAKO_INPUT"

test_pass "phase29x_derust_route_dualrun_vm: PASS (rust/hako route lane parity across 4 cases)"

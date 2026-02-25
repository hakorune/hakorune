#!/bin/bash
# Phase29z-S5x: vm-hako subset reject smoke (phase auto-resolved)
#
# Contract:
# - `--backend vm-hako` is accepted by the dispatcher.
# - S0 subset outside inputs must fail-fast with a stable unimplemented tag.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2
source "$(dirname "$0")/lib/vm_hako_phase.sh"

VM_HAKO_PHASE_EXPECTED="${VM_HAKO_PHASE_EXPECTED:-$(resolve_vm_hako_phase || true)}"
if [ -z "$VM_HAKO_PHASE_EXPECTED" ]; then
    test_fail "phase29z_vm_hako_backend_frame_vm: failed to resolve VM_HAKO_PHASE"
    exit 1
fi

INPUT="${1:-$NYASH_ROOT/apps/tests/phase29z_vm_hako_s0_reject_compare_ne_min.hako}"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"

if [ ! -f "$INPUT" ]; then
    test_fail "phase29z_vm_hako_backend_frame_vm: fixture missing: $INPUT"
    exit 1
fi

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm-hako "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29z_vm_hako_backend_frame_vm: timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -eq 0 ]; then
    echo "[FAIL] vm-hako subset reject expected non-zero (got rc=0)"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29z_vm_hako_backend_frame_vm: expected non-zero exit"
    exit 1
fi

if ! echo "$OUTPUT" | rg -q "^\[vm-hako/unimplemented\] phase=${VM_HAKO_PHASE_EXPECTED} route=subset-check file=.* func=.* bb=[0-9]+ op=.*$"; then
    echo "[FAIL] missing vm-hako fail-fast contract tag"
    echo "[INFO] Output:"
    echo "$OUTPUT" | tail -n 80 || true
    test_fail "phase29z_vm_hako_backend_frame_vm: contract tag not found"
    exit 1
fi

test_pass "phase29z_vm_hako_backend_frame_vm: PASS (rc=$EXIT_CODE)"

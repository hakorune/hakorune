#!/bin/bash
# Phase 29x X65: optimization gate integration + rollback lock
#
# Contract pin:
# - Replay X63 and X64 optimization contracts via one command.
# - Keep rollback switch (`--no-optimize`) alive via explicit probe fixture.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/vm_route_pin.sh"
require_env || exit 2

RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
ROLLBACK_FIXTURE="$NYASH_ROOT/apps/tests/phase29x_optimization_parity_const_fold_min.hako"

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_optimization_gate_vm: step failed: $cmd"
        exit 1
    fi
}

if [ ! -f "$ROLLBACK_FIXTURE" ]; then
    test_fail "phase29x_optimization_gate_vm: rollback fixture missing: $ROLLBACK_FIXTURE"
    exit 1
fi

run_step "tools/checks/phase29x_optimization_gate_guard.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_optimization_allowlist_lock_vm.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_optimization_parity_fixtures_vm.sh"

set +e
ROLLBACK_OUT=$(
    run_with_vm_route_pin env \
        NYASH_DISABLE_PLUGINS=1 \
        timeout "$RUN_TIMEOUT_SECS" \
        "$NYASH_BIN" --backend vm --no-optimize "$ROLLBACK_FIXTURE" 2>&1
)
ROLLBACK_RC=$?
set -e

if [ "$ROLLBACK_RC" -eq 124 ]; then
    test_fail "phase29x_optimization_gate_vm: rollback probe timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$ROLLBACK_RC" -ne 6 ]; then
    echo "[INFO] rollback probe output:"
    echo "$ROLLBACK_OUT" | tail -n 120 || true
    test_fail "phase29x_optimization_gate_vm: rollback probe rc mismatch (expected=6 got=$ROLLBACK_RC)"
    exit 1
fi

ROLLBACK_NORM="$(printf '%s\n' "$ROLLBACK_OUT" | filter_noise | sed '/^[[:space:]]*$/d' || true)"
if [ "$ROLLBACK_NORM" != "6" ]; then
    echo "[INFO] rollback probe normalized output:"
    echo "$ROLLBACK_NORM"
    test_fail "phase29x_optimization_gate_vm: rollback probe stdout mismatch (expected='6')"
    exit 1
fi

test_pass "phase29x_optimization_gate_vm: PASS (X65 optimization gate + rollback lock)"

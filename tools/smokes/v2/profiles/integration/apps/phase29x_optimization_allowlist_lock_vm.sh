#!/bin/bash
# Phase 29x X63: optimization allowlist lock gate
#
# Contract pin:
# - Replay X62 runtime-core gate as precondition.
# - Verify optimization safe-set vocabulary lock via targeted cargo test.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

ALLOWLIST_FILE="$NYASH_ROOT/tools/checks/phase29x_optimization_allowlist.txt"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-180}"
TMP_LOG="$(mktemp "${TMPDIR:-/tmp}/phase29x_opt_allowlist_XXXX.log")"
trap 'rm -f "$TMP_LOG"' EXIT

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_optimization_allowlist_lock_vm: step failed: $cmd"
        exit 1
    fi
}

run_step "tools/checks/phase29x_optimization_allowlist_guard.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_runtime_core_gate_vm.sh"

set +e
(
    cd "$NYASH_ROOT" &&
    timeout "$RUN_TIMEOUT_SECS" cargo test -q mir_optimizer_phase29x_allowlist_lock -- --nocapture >"$TMP_LOG" 2>&1
)
TEST_RC=$?
set -e

if [ "$TEST_RC" -eq 124 ]; then
    test_fail "phase29x_optimization_allowlist_lock_vm: cargo test timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$TEST_RC" -ne 0 ]; then
    echo "[FAIL] optimization allowlist cargo test failed"
    echo "[INFO] Exit code: $TEST_RC"
    echo "[INFO] Log tail:"
    tail -n 120 "$TMP_LOG" || true
    test_fail "phase29x_optimization_allowlist_lock_vm: allowlist cargo test failed"
    exit 1
fi

if [ ! -f "$ALLOWLIST_FILE" ]; then
    test_fail "phase29x_optimization_allowlist_lock_vm: allowlist missing: $ALLOWLIST_FILE"
    exit 1
fi

test_pass "phase29x_optimization_allowlist_lock_vm: PASS (X63 optimization allowlist locked)"

#!/bin/bash
# Phase 29x X60: RC insertion phase2 queue lock gate
#
# Contract pin:
# - Replay X59 ABI borrowed/owned gate as precondition.
# - Execute rc_insertion_selfcheck with rc-insertion-minimal and assert
#   phase2 queue markers (loop/call/early-exit) from cases inventory.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

CASES_FILE="$NYASH_ROOT/tools/checks/phase29x_rc_phase2_queue_cases.txt"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-240}"
TMP_LOG="$(mktemp "${TMPDIR:-/tmp}/phase29x_rc_phase2_queue_XXXX.log")"
trap 'rm -f "$TMP_LOG"' EXIT

run_step() {
    local cmd="$1"
    if ! bash "$NYASH_ROOT/$cmd"; then
        test_fail "phase29x_rc_phase2_queue_lock_vm: step failed: $cmd"
        exit 1
    fi
}

run_step "tools/checks/phase29x_rc_phase2_queue_guard.sh"
run_step "tools/smokes/v2/profiles/integration/apps/phase29x_abi_borrowed_owned_conformance_vm.sh"

set +e
(
    cd "$NYASH_ROOT" &&
    timeout "$RUN_TIMEOUT_SECS" cargo run -q --bin rc_insertion_selfcheck --features rc-insertion-minimal >"$TMP_LOG" 2>&1
)
SELFCHECK_RC=$?
set -e

if [ "$SELFCHECK_RC" -eq 124 ]; then
    test_fail "phase29x_rc_phase2_queue_lock_vm: rc_insertion_selfcheck timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$SELFCHECK_RC" -ne 0 ]; then
    echo "[FAIL] rc_insertion_selfcheck failed"
    echo "[INFO] Exit code: $SELFCHECK_RC"
    echo "[INFO] Log tail:"
    tail -n 120 "$TMP_LOG" || true
    test_fail "phase29x_rc_phase2_queue_lock_vm: selfcheck failed"
    exit 1
fi

while IFS='|' read -r case_id marker; do
    if [[ -z "${case_id}" || "${case_id}" =~ ^# ]]; then
        continue
    fi
    if ! rg -Fq "$marker" "$TMP_LOG"; then
        echo "[FAIL] rc phase2 queue marker missing: case=$case_id marker=$marker"
        echo "[INFO] Log tail:"
        tail -n 120 "$TMP_LOG" || true
        test_fail "phase29x_rc_phase2_queue_lock_vm: phase2 queue marker drift"
        exit 1
    fi
done <"$CASES_FILE"

if ! rg -Fq "[rc_phase2_queue] loop=ok call=ok early_exit=ok" "$TMP_LOG"; then
    echo "[FAIL] rc phase2 queue summary marker not found"
    echo "[INFO] Log tail:"
    tail -n 120 "$TMP_LOG" || true
    test_fail "phase29x_rc_phase2_queue_lock_vm: missing summary marker"
    exit 1
fi
if ! rg -q "^\[PASS\] rc_insertion_selfcheck$" "$TMP_LOG"; then
    echo "[FAIL] rc_insertion_selfcheck did not report PASS marker"
    echo "[INFO] Log tail:"
    tail -n 120 "$TMP_LOG" || true
    test_fail "phase29x_rc_phase2_queue_lock_vm: missing selfcheck PASS marker"
    exit 1
fi

test_pass "phase29x_rc_phase2_queue_lock_vm: PASS (X60 phase2 queue locked)"

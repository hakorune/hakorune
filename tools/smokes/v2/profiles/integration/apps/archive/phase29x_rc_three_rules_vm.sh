#!/bin/bash
# Phase 29x X12: RC three-rules comprehensive smoke
#
# Contract pin:
# - overwrite / explicit-drop / scope-end cleanup are all exercised by
#   `rc_insertion_selfcheck` and reported via a stable marker line.
# - rc-insertion-minimal path must stay green.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SELFCHECK_TIMEOUT_SECS="${SMOKES_RC_INSERTION_SELFCHECK_TIMEOUT_SECS:-180}"
TMP_LOG="$(mktemp "${TMPDIR:-/tmp}/phase29x_rc_three_rules_XXXX.log")"
trap 'rm -f "$TMP_LOG"' EXIT

set +e
(
    cd "$NYASH_ROOT" &&
    timeout "$SELFCHECK_TIMEOUT_SECS" cargo run -q --bin rc_insertion_selfcheck --features rc-insertion-minimal >"$TMP_LOG" 2>&1
)
SELFCHECK_RC=$?
set -e

if [ "$SELFCHECK_RC" -eq 124 ]; then
    test_fail "phase29x_rc_three_rules_vm: rc_insertion_selfcheck timed out (>${SELFCHECK_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$SELFCHECK_RC" -ne 0 ]; then
    echo "[FAIL] rc_insertion_selfcheck failed"
    echo "[INFO] Exit code: $SELFCHECK_RC"
    echo "[INFO] Log tail:"
    tail -n 80 "$TMP_LOG" || true
    test_fail "phase29x_rc_three_rules_vm: selfcheck failed"
    exit 1
fi

if ! rg -q "^\[rc_three_rules\] overwrite=ok explicit_drop=ok scope_end=ok$" "$TMP_LOG"; then
    echo "[FAIL] three-rules marker not found"
    echo "[INFO] Log tail:"
    tail -n 120 "$TMP_LOG" || true
    test_fail "phase29x_rc_three_rules_vm: missing three-rules marker"
    exit 1
fi

if ! rg -q "^\[PASS\] rc_insertion_selfcheck$" "$TMP_LOG"; then
    echo "[FAIL] rc_insertion_selfcheck did not report PASS marker"
    echo "[INFO] Log tail:"
    tail -n 120 "$TMP_LOG" || true
    test_fail "phase29x_rc_three_rules_vm: missing selfcheck PASS marker"
    exit 1
fi

test_pass "phase29x_rc_three_rules_vm: PASS (overwrite/explicit/scope-end)"

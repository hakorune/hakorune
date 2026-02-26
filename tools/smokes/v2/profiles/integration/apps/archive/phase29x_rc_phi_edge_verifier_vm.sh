#!/bin/bash
# Phase 29x X11: PHI/edge verifier smoke (VM + selfcheck)
#
# Contract pin:
# 1) Baseline runtime path stays behavior-stable (exit 0).
# 2) Feature path (`rc-insertion-minimal`) exposes PHI/edge contradiction as
#    fail-fast tag `[freeze:contract][rc_insertion/phi_edge_mismatch]`.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase29x_rc_continue_cleanup_min.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
SELFCHECK_TIMEOUT_SECS="${SMOKES_RC_INSERTION_SELFCHECK_TIMEOUT_SECS:-180}"
TMP_LOG="$(mktemp "${TMPDIR:-/tmp}/phase29x_rc_phi_edge_verifier_XXXX.log")"
trap 'rm -f "$TMP_LOG"' EXIT

if [ ! -f "$INPUT" ]; then
    test_fail "phase29x_rc_phi_edge_verifier_vm: fixture missing: $INPUT"
    exit 1
fi

set +e
BASE_OUT=$(timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
BASE_RC=$?
set -e

if [ "$BASE_RC" -eq 124 ]; then
    test_fail "phase29x_rc_phi_edge_verifier_vm: baseline run timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$BASE_RC" -ne 0 ]; then
    echo "[FAIL] baseline run failed"
    echo "[INFO] Exit code: $BASE_RC"
    echo "[INFO] Output:"
    echo "$BASE_OUT" | head -n 80 || true
    test_fail "phase29x_rc_phi_edge_verifier_vm: baseline execution failed"
    exit 1
fi

set +e
(
    cd "$NYASH_ROOT" &&
    timeout "$SELFCHECK_TIMEOUT_SECS" cargo run -q --bin rc_insertion_selfcheck --features rc-insertion-minimal >"$TMP_LOG" 2>&1
)
SELFCHECK_RC=$?
set -e

if [ "$SELFCHECK_RC" -eq 124 ]; then
    test_fail "phase29x_rc_phi_edge_verifier_vm: rc_insertion_selfcheck timed out (>${SELFCHECK_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$SELFCHECK_RC" -ne 0 ]; then
    echo "[FAIL] rc_insertion_selfcheck failed"
    echo "[INFO] Exit code: $SELFCHECK_RC"
    echo "[INFO] Log tail:"
    tail -n 80 "$TMP_LOG" || true
    test_fail "phase29x_rc_phi_edge_verifier_vm: selfcheck failed"
    exit 1
fi

if ! rg -q "^\[PASS\] rc_insertion_selfcheck$" "$TMP_LOG"; then
    echo "[FAIL] rc_insertion_selfcheck did not report PASS marker"
    echo "[INFO] Log tail:"
    tail -n 80 "$TMP_LOG" || true
    test_fail "phase29x_rc_phi_edge_verifier_vm: missing selfcheck PASS marker"
    exit 1
fi

if ! rg -Fq "[freeze:contract][rc_insertion/phi_edge_mismatch]" "$TMP_LOG"; then
    echo "[FAIL] fail-fast tag not observed in selfcheck output"
    echo "[INFO] Log tail:"
    tail -n 120 "$TMP_LOG" || true
    test_fail "phase29x_rc_phi_edge_verifier_vm: missing fail-fast tag"
    exit 1
fi

test_pass "phase29x_rc_phi_edge_verifier_vm: PASS (baseline rc=0 + fail-fast tag observed)"

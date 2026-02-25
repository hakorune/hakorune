#!/bin/bash
# Phase 29y.1 Task 2b: overwrite release smoke (VM + MIR emit)
#
# Contract pin:
# 1) Baseline runtime path (default build) stays behavior-stable (exit 0).
# 2) Feature path (`rc-insertion-minimal`) inserts exactly one `release_strong`
#    for the overwrite shape in `main`.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase29y_rc_insertion_overwrite_release_min.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
EMIT_TIMEOUT_SECS="${SMOKES_RC_INSERTION_EMIT_TIMEOUT_SECS:-180}"
TMP_JSON="$(mktemp "${TMPDIR:-/tmp}/phase29y_rc_overwrite_XXXX.json")"
TMP_LOG="$(mktemp "${TMPDIR:-/tmp}/phase29y_rc_overwrite_XXXX.log")"
trap 'rm -f "$TMP_JSON" "$TMP_LOG"' EXIT

if [ ! -f "$INPUT" ]; then
    test_fail "phase29y_rc_insertion_overwrite_release_vm: fixture missing: $INPUT"
    exit 1
fi

set +e
BASE_OUT=$(timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$INPUT" 2>&1)
BASE_RC=$?
set -e

if [ "$BASE_RC" -eq 124 ]; then
    test_fail "phase29y_rc_insertion_overwrite_release_vm: baseline run timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$BASE_RC" -ne 0 ]; then
    echo "[FAIL] baseline run failed"
    echo "[INFO] Exit code: $BASE_RC"
    echo "[INFO] Output:"
    echo "$BASE_OUT" | head -n 80 || true
    test_fail "phase29y_rc_insertion_overwrite_release_vm: baseline execution failed"
    exit 1
fi

set +e
(
    cd "$NYASH_ROOT" &&
    # Keep this gate focused on RC insertion contract only.
    # JoinIR dev strict mode can recurse into unrelated FlowBox paths and flake with stack overflow.
    timeout "$EMIT_TIMEOUT_SECS" env \
        NYASH_JOINIR_DEV=0 \
        HAKO_JOINIR_STRICT=0 \
        NYASH_USE_NY_COMPILER=0 \
        cargo run -q --features rc-insertion-minimal --bin hakorune -- \
        --emit-mir-json "$TMP_JSON" "$INPUT" >"$TMP_LOG" 2>&1
)
EMIT_RC=$?
set -e

if [ "$EMIT_RC" -eq 124 ]; then
    test_fail "phase29y_rc_insertion_overwrite_release_vm: emit timed out (>${EMIT_TIMEOUT_SECS}s)"
    exit 1
fi
if [ "$EMIT_RC" -ne 0 ]; then
    echo "[FAIL] emit with rc-insertion-minimal failed"
    echo "[INFO] Exit code: $EMIT_RC"
    echo "[INFO] Log tail:"
    tail -n 80 "$TMP_LOG" || true
    test_fail "phase29y_rc_insertion_overwrite_release_vm: emit failed"
    exit 1
fi

if [ ! -f "$TMP_JSON" ]; then
    test_fail "phase29y_rc_insertion_overwrite_release_vm: MIR JSON not produced"
    exit 1
fi

MAIN_RELEASE_COUNT=$(jq '[.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="release_strong")] | length' "$TMP_JSON")
TOTAL_RELEASE_COUNT=$(jq '[.functions[] | .blocks[] | .instructions[] | select(.op=="release_strong")] | length' "$TMP_JSON")

if [ "$MAIN_RELEASE_COUNT" -ne 1 ]; then
    echo "[FAIL] expected exactly 1 release_strong in main, got $MAIN_RELEASE_COUNT"
    echo "[INFO] release_strong entries in main:"
    jq '.functions[] | select(.name=="main") | .blocks[] | .instructions[] | select(.op=="release_strong")' "$TMP_JSON" || true
    test_fail "phase29y_rc_insertion_overwrite_release_vm: unexpected main release count"
    exit 1
fi

if [ "$TOTAL_RELEASE_COUNT" -ne 1 ]; then
    echo "[FAIL] expected exactly 1 release_strong in module, got $TOTAL_RELEASE_COUNT"
    echo "[INFO] all release_strong entries:"
    jq '.functions[] | {name, releases: [.blocks[] | .instructions[] | select(.op=="release_strong")] } | select((.releases|length) > 0)' "$TMP_JSON" || true
    test_fail "phase29y_rc_insertion_overwrite_release_vm: unexpected total release count"
    exit 1
fi

test_pass "phase29y_rc_insertion_overwrite_release_vm: PASS (main release_strong=1)"

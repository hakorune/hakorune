#!/bin/bash
# Phase 29ck numeric MatI64.mul_naive narrow pilot smoke
#
# Contract pin:
# - `lang.runtime.kernel.numeric.matrix_i64` resolves to a narrow `.hako` kernel module.
# - `nyash.core.numeric.matrix_i64.MatI64.mul_naive` stays a thin ring1 wrapper.
# - The kernel module owns the mul_naive loop/body for the fixture.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29ck_numeric_mat_i64_mul_naive_min"
APP="$NYASH_ROOT/apps/tests/phase29ck_numeric_mat_i64_mul_naive_min.hako"
TMP_LOG="${TMPDIR:-/tmp}/${SMOKE_NAME}_$$.log"

cleanup() {
    rm -f "$TMP_LOG"
}
trap cleanup EXIT

if [ ! -f "$APP" ]; then
    test_fail "$SMOKE_NAME: fixture missing: $APP"
    exit 1
fi

set +e
run_joinir_vm_release "$APP" >"$TMP_LOG" 2>&1
RUN_RC=$?
set -e

if [ "$RUN_RC" -ne 0 ]; then
    tail -n 120 "$TMP_LOG" || true
    test_fail "$SMOKE_NAME: VM run failed rc=$RUN_RC"
    exit 1
fi

if ! grep -Fq "OK: numeric mat_i64 mul_naive" "$TMP_LOG"; then
    tail -n 120 "$TMP_LOG" || true
    test_fail "$SMOKE_NAME: expected success marker missing"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (numeric MatI64.mul_naive narrow pilot)"

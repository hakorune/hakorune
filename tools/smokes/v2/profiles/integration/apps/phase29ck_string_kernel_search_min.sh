#!/bin/bash
# Phase 29ck string kernel search narrow pilot smoke
#
# Contract pin:
# - `lang.runtime.kernel.string.search` resolves to a narrow `.hako` kernel module.
# - The kernel module provides `find_index` / `contains` for the smoke fixture.
# - VM execution must stay on the direct `.hako` path.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase29ck_string_kernel_search_min"
APP="$NYASH_ROOT/apps/tests/string_kernel_search_min.hako"
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

if ! grep -Fq "OK: string kernel" "$TMP_LOG"; then
    tail -n 120 "$TMP_LOG" || true
    test_fail "$SMOKE_NAME: expected success marker missing"
    exit 1
fi

test_pass "$SMOKE_NAME: PASS (string search kernel narrow pilot)"

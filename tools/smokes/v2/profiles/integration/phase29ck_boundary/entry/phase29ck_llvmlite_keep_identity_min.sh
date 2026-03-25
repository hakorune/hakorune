#!/bin/bash
# Phase 29ck llvmlite keep-lane identity smoke
#
# Contract pin:
# 1) tools/llvmlite_harness.py self-identifies as a keep lane, not a daily owner.
# 2) keep-lane help/output wording stays separate from boundary/mainline acceptance.

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

if ! command -v python3 >/dev/null 2>&1; then
    test_skip "phase29ck_llvmlite_keep_identity_min: python3 not found"
    exit 0
fi

if ! python3 -c "import llvmlite" >/dev/null 2>&1; then
    test_skip "phase29ck_llvmlite_keep_identity_min: llvmlite not found"
    exit 0
fi

HARNESS="$NYASH_ROOT/tools/llvmlite_harness.py"
OUT_OBJ="${TMPDIR:-/tmp}/phase29ck_llvmlite_keep_identity_$$.o"
HELP_LOG="${TMPDIR:-/tmp}/phase29ck_llvmlite_keep_identity_help_$$.log"
RUN_LOG="${TMPDIR:-/tmp}/phase29ck_llvmlite_keep_identity_run_$$.log"

cleanup() {
    rm -f "$OUT_OBJ" "$HELP_LOG" "$RUN_LOG"
}
trap cleanup EXIT

python3 "$HARNESS" --help >"$HELP_LOG"
if ! grep -Fq 'compat/probe keep harness' "$HELP_LOG" || \
   ! grep -Fq 'daily mainline' "$HELP_LOG"; then
    test_fail "phase29ck_llvmlite_keep_identity_min: help no longer marks keep lane"
    exit 1
fi

python3 "$HARNESS" --out "$OUT_OBJ" >"$RUN_LOG"
if ! grep -Fq '[llvmlite-keep]' "$RUN_LOG"; then
    test_fail "phase29ck_llvmlite_keep_identity_min: keep tag missing from harness output"
    exit 1
fi

if [ ! -f "$OUT_OBJ" ]; then
    test_fail "phase29ck_llvmlite_keep_identity_min: object missing: $OUT_OBJ"
    exit 1
fi

test_pass "phase29ck_llvmlite_keep_identity_min: PASS (llvmlite harness stays explicit keep lane)"

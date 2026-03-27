#!/bin/bash
# Phase 29x: backend-owner daily substring-concat-loop owner flip smoke

set -euo pipefail

source "$(dirname "$0")/../../../../lib/test_runner.sh"
require_env || exit 2

if ! command -v llc >/dev/null 2>&1 && ! command -v llc-18 >/dev/null 2>&1; then
    test_skip "phase29x_backend_owner_daily_substring_concat_loop_min: llc not found"
    exit 0
fi

SMOKE_NAME="phase29x_backend_owner_daily_substring_concat_loop_min"
HAKORUNE_BIN="$NYASH_ROOT/target/release/hakorune"
APP="$NYASH_ROOT/apps/tests/phase29x_backend_owner_daily_min.hako"
FIXTURE="$NYASH_ROOT/apps/tests/mir_shape_guard/substring_concat_loop_pure_min_v1.mir.json"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-90}"
RAW_LOG="$(mktemp "/tmp/${SMOKE_NAME}.XXXXXX.log")"

cleanup() {
    if [ -f "$RAW_LOG" ]; then
        local out_obj
        out_obj="$(sed -n 's/.*\[backend-owner-daily\] object=//p' "$RAW_LOG" | tail -n 1)"
        if [ -n "$out_obj" ]; then rm -f "$out_obj"; fi
        rm -f "$RAW_LOG"
    fi
}
trap cleanup EXIT

require_smoke_path "$SMOKE_NAME" "hakorune" "$HAKORUNE_BIN" executable || exit 1
require_smoke_path "$SMOKE_NAME" "daily app" "$APP" || exit 1
require_smoke_path "$SMOKE_NAME" "fixture" "$FIXTURE" || exit 1

set +e
timeout "$RUN_TIMEOUT_SECS" env -i \
    PATH="$PATH" \
    HOME="${HOME:-/tmp}" \
    TMPDIR="${TMPDIR:-/tmp}" \
    "$HAKORUNE_BIN" "$APP" -- "$FIXTURE" >"$RAW_LOG" 2>&1
rc=$?
set -e
out="$(cat "$RAW_LOG")"

if [ "$rc" -eq 124 ]; then
    echo "[INFO] daily output:"
    echo "$out" | head -n 120 || true
    test_fail "$SMOKE_NAME: daily owner timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$rc" -ne 0 ]; then
    echo "[INFO] daily output:"
    echo "$out" | head -n 120 || true
    test_fail "$SMOKE_NAME: daily owner failed (rc=$rc)"
    exit 1
fi

if ! echo "$out" | grep -Fq "[hako-ll/daily] chosen_owner=hako-ll-min-v0 accepted=min-v0 first_blocker=none acceptance_case=substring-concat-loop-v1 legacy_daily_allowed=no"; then
    echo "[INFO] daily output:"
    echo "$out" | head -n 120 || true
    test_fail "$SMOKE_NAME: missing daily ownership evidence"
    exit 1
fi

if echo "$out" | grep -Fq "[hako-ll/compare]"; then
    echo "[INFO] daily output:"
    echo "$out" | head -n 120 || true
    test_fail "$SMOKE_NAME: compare lane leaked into daily route"
    exit 1
fi

OUT_OBJ="$(echo "$out" | sed -n 's/.*\[backend-owner-daily\] object=//p' | tail -n 1)"
if [ -z "$OUT_OBJ" ]; then
    echo "[INFO] daily output:"
    echo "$out" | head -n 120 || true
    test_fail "$SMOKE_NAME: missing object handoff tag"
    exit 1
fi

require_smoke_path "$SMOKE_NAME" "object" "$OUT_OBJ" || exit 1
test_pass "$SMOKE_NAME: PASS (substring-concat-loop uses .hako ll emitter as daily owner)"

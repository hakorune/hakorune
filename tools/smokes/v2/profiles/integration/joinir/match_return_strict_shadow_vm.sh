#!/bin/bash
# match_return_strict_shadow_vm.sh - match return-only strict/dev shadow adopt

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29at_match_return_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    log_error "match_return_strict_shadow_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 20 ]; then
    log_error "match_return_strict_shadow_vm: expected exit code 20, got $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Avoid `echo ... | grep -q` under `set -o pipefail` (SIGPIPE false negatives).
if ! grep -qF "[flowbox/adopt box_kind=Seq" <<<"$OUTPUT" \
    || ! grep -qF "features=return" <<<"$OUTPUT" \
    || ! grep -qF "via=shadow" <<<"$OUTPUT"; then
    echo "[FAIL] Missing FlowBox tag (box_kind=Seq features=return via=shadow)"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "match_return_strict_shadow_vm: Missing FlowBox tag"
    exit 1
fi

if grep -qF "[plan/fallback:" <<<"$OUTPUT"; then
    echo "[FAIL] Unexpected fallback tag"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "match_return_strict_shadow_vm: Fallback tag present"
    exit 1
fi

log_success "match_return_strict_shadow_vm: PASS (exit=20)"
exit 0

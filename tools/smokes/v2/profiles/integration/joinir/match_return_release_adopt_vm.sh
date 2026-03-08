#!/bin/bash
# match_return_release_adopt_vm.sh - match return-only release adopt
#
# Expected:
# - Exit code 20
# - No shadow adopt tag
# - No fallback tag
# - No output

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29at_match_return_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  -u HAKO_JOINIR_STRICT \
  -u NYASH_JOINIR_STRICT \
  -u HAKO_JOINIR_DEBUG \
  -u NYASH_JOINIR_DEBUG \
  -u HAKO_JOINIR_DEV \
  -u NYASH_JOINIR_DEV \
  NYASH_DISABLE_PLUGINS=1 \
  "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    log_error "match_return_release_adopt_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 20 ]; then
    log_error "match_return_release_adopt_vm: expected exit code 20, got $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

if grep -qF "[flowbox/" <<<"$OUTPUT"; then
    echo "[FAIL] Unexpected FlowBox tag in release output"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "match_return_release_adopt_vm: Unexpected tag"
    exit 1
fi

if grep -qF "[plan/fallback:" <<<"$OUTPUT"; then
    echo "[FAIL] Unexpected fallback tag"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "match_return_release_adopt_vm: Fallback tag present"
    exit 1
fi

OUTPUT_CLEAN=$(
    echo "$OUTPUT" | filter_noise | grep -v '^\[plugins\]' \
        | grep -v '^\[WARN\] \[plugin/init\]' || true
)
if [ -n "$OUTPUT_CLEAN" ]; then
    echo "[FAIL] Expected no output"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "match_return_release_adopt_vm: Unexpected output"
    exit 1
fi

log_success "match_return_release_adopt_vm: PASS (exit=20)"
exit 0

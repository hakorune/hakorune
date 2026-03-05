#!/bin/bash
# phase29ar_string_is_integer_min_vm.sh - StringUtils.is_integer minimal (strict/dev fail-fast reject)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29ar_string_is_integer_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
export NYASH_ALLOW_USING_FILE=1

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    log_error "phase29ar_string_is_integer_min_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 1 ]; then
    log_error "phase29ar_string_is_integer_min_vm: expected exit code 1, got $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

if ! grep -qF "[vm-hako/unimplemented]" <<<"$OUTPUT" \
    || ! grep -qF "newbox(StringUtils)" <<<"$OUTPUT"; then
    echo "[FAIL] Missing strict fail-fast marker for StringUtils"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29ar_string_is_integer_min_vm: Missing fail-fast marker"
    exit 1
fi

if grep -qF "[flowbox/" <<<"$OUTPUT"; then
    echo "[FAIL] Unexpected FlowBox tag in strict reject output"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29ar_string_is_integer_min_vm: Unexpected FlowBox tag"
    exit 1
fi

log_success "phase29ar_string_is_integer_min_vm: PASS (exit=1 fail-fast reject)"
exit 0

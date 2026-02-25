#!/bin/bash
# phase29ar_string_is_integer_min_vm.sh - StringUtils.is_integer minimal (strict/dev shadow adopt)

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

if [ "$EXIT_CODE" -ne 0 ]; then
    log_error "phase29ar_string_is_integer_min_vm: expected exit code 0, got $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

OUTPUT_CLEAN=$(echo "$OUTPUT" | filter_noise | grep -v '^\[plugins\]' | grep -v '^\[WARN\] \[plugin/init\]')
expected=$(cat << 'TXT'
1
0
TXT
)

compare_outputs "$expected" "$OUTPUT_CLEAN" "phase29ar_string_is_integer_min_vm" || exit 1

# Avoid `echo ... | grep -q` under `set -o pipefail` (SIGPIPE false negatives).
if ! grep -qF "[flowbox/adopt box_kind=Loop" <<<"$OUTPUT" \
    || ! grep -qF "features=return" <<<"$OUTPUT" \
    || ! grep -qF "via=shadow" <<<"$OUTPUT"; then
    echo "[FAIL] Missing FlowBox tag (box_kind=Loop features=return via=shadow)"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29ar_string_is_integer_min_vm: Missing FlowBox tag"
    exit 1
fi

log_success "phase29ar_string_is_integer_min_vm: PASS (exit=0)"
exit 0

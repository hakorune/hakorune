#!/bin/bash
# string_is_integer_release_adopt_vm.sh - StringUtils.is_integer minimal (release adopt)
#
# Expected:
# - Exit code 0
# - Output matches expected (1, 0)
# - No shadow adopt tag

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/string_is_integer_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}
export NYASH_ALLOW_USING_FILE=1

set +e
OUTPUT=$(NYASH_DISABLE_PLUGINS=1 run_joinir_vm_release "$FIXTURE")
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    log_error "string_is_integer_release_adopt_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
    log_error "string_is_integer_release_adopt_vm: expected exit code 0, got $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

if grep -qF "[flowbox/" <<<"$OUTPUT"; then
    echo "[FAIL] Unexpected FlowBox tag in release output"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "string_is_integer_release_adopt_vm: Unexpected tag"
    exit 1
fi

OUTPUT_CLEAN=$(echo "$OUTPUT" | filter_noise | grep -v '^\[plugins\]' | grep -v '^\[WARN\] \[plugin/init\]')
expected=$(cat << 'TXT'
1
0
TXT
)

compare_outputs "$expected" "$OUTPUT_CLEAN" "string_is_integer_release_adopt_vm" || exit 1

log_success "string_is_integer_release_adopt_vm: PASS (exit=0)"
exit 0

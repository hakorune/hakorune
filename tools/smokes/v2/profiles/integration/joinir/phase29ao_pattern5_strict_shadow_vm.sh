#!/bin/bash
# phase29ao_pattern5_strict_shadow_vm.sh - Pattern5 strict shadow adopt gate (VM)
#
# Expected:
# - Output "3" or "RC: 3"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase286_pattern5_break_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29ao_pattern5_strict_shadow_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if grep -qE "(^3$|RC: 3$)" <<<"$OUTPUT"; then
    # Avoid `echo ... | grep -q` under `set -o pipefail` (SIGPIPE false negatives).
    if ! grep -qF "[flowbox/adopt box_kind=Loop" <<<"$OUTPUT" \
        || ! grep -qF "features=break" <<<"$OUTPUT" \
        || ! grep -qF "via=shadow" <<<"$OUTPUT"; then
        echo "[FAIL] Missing FlowBox tag (box_kind=Loop features=break via=shadow)"
        echo "$OUTPUT" | tail -n 40 || true
        test_fail "phase29ao_pattern5_strict_shadow_vm: Missing FlowBox tag"
        exit 1
    fi
    test_pass "phase29ao_pattern5_strict_shadow_vm: PASS (output: 3)"
    exit 0
fi

echo "[FAIL] Unexpected output (expected: 3)"
echo "[INFO] Exit code: $EXIT_CODE"
echo "[INFO] Output:"
echo "$OUTPUT" | head -n 20 || true
test_fail "phase29ao_pattern5_strict_shadow_vm: Unexpected output"
exit 1

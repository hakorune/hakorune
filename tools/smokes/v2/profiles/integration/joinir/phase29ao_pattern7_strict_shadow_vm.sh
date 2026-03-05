#!/bin/bash
# phase29ao_pattern7_strict_shadow_vm.sh - Pattern7 strict shadow adopt tag gate (VM)
#
# Expected:
# - Exit code 1
# - FlowBox: box_kind=Loop via=shadow

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase29ab_pattern7_splitscan_ok_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29ao_pattern7_strict_shadow_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 1 ]; then
    echo "[FAIL] Expected exit 1, got $EXIT_CODE"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29ao_pattern7_strict_shadow_vm: Unexpected RC"
    exit 1
fi

# Avoid `echo ... | grep -q` under `set -o pipefail` (SIGPIPE false negatives).
if ! grep -qF "[flowbox/adopt box_kind=Loop" <<<"$OUTPUT" \
    || ! grep -qF "via=shadow" <<<"$OUTPUT"; then
    echo "[FAIL] Missing FlowBox tag (box_kind=Loop via=shadow)"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29ao_pattern7_strict_shadow_vm: Missing FlowBox tag"
    exit 1
fi

test_pass "phase29ao_pattern7_strict_shadow_vm: PASS (exit=1, flowbox tag)"
exit 0

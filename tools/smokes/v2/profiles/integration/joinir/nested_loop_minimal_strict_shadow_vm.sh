#!/bin/bash
# nested_loop_minimal_strict_shadow_vm.sh - nested_loop_minimal strict shadow gate (VM)
# current semantic wrapper; canonical entry for nested_loop_minimal_strict_shadow_vm
#
# Expected:
# - Exit code 9
# - FlowBox: box_kind=Loop features=nested_loop via=shadow

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

LEGACY_STEM="${LEGACY_STEM_OVERRIDE:-}"
SEMANTIC_STEM="nested_loop_minimal_strict_shadow_vm"
LABEL_PREFIX="$SEMANTIC_STEM"
if [ -n "$LEGACY_STEM" ]; then
    LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"
fi

FIXTURE="$NYASH_ROOT/apps/tests/phase1883_nested_minimal.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env HAKO_JOINIR_STRICT=1 "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "${LABEL_PREFIX}: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 9 ]; then
    echo "[FAIL] Expected exit 9, got $EXIT_CODE"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "${LABEL_PREFIX}: Unexpected RC"
    exit 1
fi

# Avoid `echo ... | grep -q` under `set -o pipefail` (SIGPIPE false negatives).
if ! grep -qF "[flowbox/adopt box_kind=Loop" <<<"$OUTPUT" \
    || ! grep -qF "features=nested_loop" <<<"$OUTPUT" \
    || ! grep -qF "via=shadow" <<<"$OUTPUT"; then
    echo "[FAIL] Missing FlowBox tag (box_kind=Loop features=nested_loop via=shadow)"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "${LABEL_PREFIX}: Missing FlowBox tag"
    exit 1
fi

if grep -qF '[plan/freeze:unstructured]' <<<"$OUTPUT"; then
    echo "[FAIL] Unexpected freeze tag in strict shadow adopt"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "${LABEL_PREFIX}: Unexpected freeze tag"
    exit 1
fi

test_pass "${LABEL_PREFIX}: PASS (exit=9, tag)"
exit 0

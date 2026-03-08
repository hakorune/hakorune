#!/bin/bash
# current semantic wrapper; canonical entry for loop_break plan-subset smoke

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

SEMANTIC_STEM="loop_break_plan_subset_vm"
LEGACY_STEM="${LEGACY_STEM_OVERRIDE:-}"
LABEL_PREFIX="$SEMANTIC_STEM"
if [ -n "$LEGACY_STEM" ]; then
    LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"
fi

INPUT="$NYASH_ROOT/apps/tests/loop_break_plan_subset_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "${LABEL_PREFIX}: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -eq 15 ]; then
    if ! echo "$OUTPUT" | grep -qF "[flowbox/adopt box_kind=Loop" \
        || ! echo "$OUTPUT" | grep -qF "features=break" \
        || ! echo "$OUTPUT" | grep -qF "via=shadow"; then
        echo "[FAIL] Missing FlowBox tag (box_kind=Loop features=break via=shadow)"
        echo "$OUTPUT" | tail -n 40 || true
        test_fail "${LABEL_PREFIX}: Missing FlowBox tag"
        exit 1
    fi
    test_pass "${LABEL_PREFIX}: RC=15 (expected)"
    exit 0
fi

echo "[FAIL] Expected exit 15, got $EXIT_CODE"
echo "$OUTPUT" | tail -n 40 || true
test_fail "${LABEL_PREFIX}: Unexpected RC"
exit 1

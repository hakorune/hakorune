#!/bin/bash
# current semantic wrapper; canonical entry for loop_break body-local-seg smoke

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

SEMANTIC_STEM="loop_break_body_local_seg_vm"
LEGACY_STEM="${LEGACY_STEM_OVERRIDE:-}"
LABEL_PREFIX="$SEMANTIC_STEM"
if [ -n "$LEGACY_STEM" ]; then
    LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"
fi

INPUT="$NYASH_ROOT/apps/tests/loop_break_body_local_seg_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "${LABEL_PREFIX}: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

OUTPUT_CLEAN=$(echo "$OUTPUT" | filter_noise)
EXPECTED_TAG_OLD="[plan/pattern2/promotion_hint:TrimSeg]"
EXPECTED_TAG_NEW="[plan/loop_break/promotion_hint:TrimSeg]"

if ! echo "$OUTPUT" | grep -qF "[flowbox/adopt box_kind=Loop" \
    || ! echo "$OUTPUT" | grep -qF "features=break" \
    || ! echo "$OUTPUT" | grep -qF "via=shadow"; then
    test_fail "${LABEL_PREFIX}: missing FlowBox tag"
    exit 1
fi

if echo "$OUTPUT_CLEAN" | grep -q "^2$" || echo "$OUTPUT" | grep -q "^RC: 2$"; then
    if echo "$OUTPUT_CLEAN" | grep -qF "$EXPECTED_TAG_NEW" || echo "$OUTPUT_CLEAN" | grep -qF "$EXPECTED_TAG_OLD"; then
        test_pass "${LABEL_PREFIX}: body-local seg promotion succeeded (output: 2)"
        exit 0
    fi
    echo "[FAIL] Missing promotion hint tag (expected: $EXPECTED_TAG_NEW or $EXPECTED_TAG_OLD)"
    echo "[INFO] Output (clean):"
    echo "$OUTPUT_CLEAN" | tail -n 20 || true
    test_fail "${LABEL_PREFIX}: Missing promotion hint tag"
    exit 1
fi

echo "[FAIL] Unexpected output (expected: 2)"
echo "[INFO] Exit code: $EXIT_CODE"
echo "[INFO] Output (clean):"
echo "$OUTPUT_CLEAN" | tail -n 20 || true
test_fail "${LABEL_PREFIX}: Unexpected output"
exit 1

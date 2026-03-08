#!/bin/bash
# current semantic wrapper; canonical entry for loop_true_early_exit release-adopt smoke

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SEMANTIC_STEM="loop_true_early_exit_release_adopt_vm"
LEGACY_STEM="${LEGACY_STEM_OVERRIDE:-}"
LABEL_PREFIX="$SEMANTIC_STEM"
if [ -n "$LEGACY_STEM" ]; then
    LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"
fi

FIXTURE="$NYASH_ROOT/apps/tests/loop_true_early_exit_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env \
  -u HAKO_JOINIR_STRICT \
  -u NYASH_JOINIR_STRICT \
  -u HAKO_JOINIR_DEBUG \
  -u NYASH_JOINIR_DEBUG \
  -u NYASH_JOINIR_DEV \
  NYASH_DISABLE_PLUGINS=1 \
  "$NYASH_BIN" "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "${LABEL_PREFIX}: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if grep -qF "[flowbox/" <<<"$OUTPUT"; then
    echo "[FAIL] Release adopt must not print FlowBox tags"
    echo "$OUTPUT" | tail -n 60 || true
    test_fail "${LABEL_PREFIX}: Unexpected tag"
    exit 1
fi

if grep -qE "(^3$|RC: 3$)" <<<"$OUTPUT"; then
    test_pass "${LABEL_PREFIX}: PASS (output: 3)"
    exit 0
fi

echo "[FAIL] Unexpected output (expected: 3)"
echo "[INFO] Exit code: $EXIT_CODE"
echo "[INFO] Output:"
echo "$OUTPUT" | head -n 40 || true
test_fail "${LABEL_PREFIX}: Unexpected output"
exit 1

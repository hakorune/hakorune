#!/bin/bash
# current semantic wrapper; canonical entry for loop_true_early_exit route smoke

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

SEMANTIC_STEM="loop_true_early_exit_vm"
LEGACY_STEM="${LEGACY_STEM_OVERRIDE:-}"
LABEL_PREFIX="$SEMANTIC_STEM"
if [ -n "$LEGACY_STEM" ]; then
    LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"
fi

INPUT="$NYASH_ROOT/apps/tests/loop_true_early_exit_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "${LABEL_PREFIX}: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if echo "$OUTPUT" | grep -qE "(^3$|RC: 3$)"; then
    test_pass "${LABEL_PREFIX}: succeeded (output: 3)"
    exit 0
fi

echo "[FAIL] Unexpected output (expected: 3)"
echo "[INFO] Exit code: $EXIT_CODE"
echo "[INFO] Output:"
echo "$OUTPUT" | head -n 20 || true
test_fail "${LABEL_PREFIX}: Unexpected output"
exit 1

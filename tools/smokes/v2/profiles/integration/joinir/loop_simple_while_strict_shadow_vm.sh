#!/bin/bash
# loop_simple_while_strict_shadow_vm.sh - loop_simple_while strict shadow adopt gate (VM)
# current semantic wrapper; canonical entry for loop_simple_while_strict_shadow_vm
#
# Purpose:
# - Exercise Phase 29ao P17/P18 strict/dev shadow adopt path:
#   loop_simple_while route selected -> Facts→CorePlan(skeleton) adopted in router
#
# Expected:
# - Exit code 3 (same as existing loop_simple_while PoC expectations)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

LEGACY_STEM="${LEGACY_STEM_OVERRIDE:-}"
SEMANTIC_STEM="loop_simple_while_strict_shadow_vm"
LABEL_PREFIX="$SEMANTIC_STEM"
if [ -n "$LEGACY_STEM" ]; then
    LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"
fi

FIXTURE="$NYASH_ROOT/apps/tests/phase286_pattern1_frag_poc.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    log_error "${LABEL_PREFIX}: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 3 ]; then
    log_error "${LABEL_PREFIX}: expected exit code 3, got $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Avoid `echo ... | grep -q` under `set -o pipefail` (SIGPIPE false negatives).
if ! grep -qF "[flowbox/adopt box_kind=Loop" <<<"$OUTPUT" \
    || ! grep -qF "via=shadow" <<<"$OUTPUT"; then
    echo "[FAIL] Missing FlowBox tag (box_kind=Loop via=shadow)"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "${LABEL_PREFIX}: Missing FlowBox tag"
    exit 1
fi

log_success "${LABEL_PREFIX}: PASS (exit=3)"
exit 0

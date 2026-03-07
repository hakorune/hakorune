#!/bin/bash
# phase29ao_pattern1_subset_reject_extra_stmt_vm.sh - loop_simple_while subset reject extra stmt (VM)
# legacy compat stem; current semantic entry = loop_simple_while_subset_reject_extra_stmt_vm.sh
#
# Purpose:
# - Ensure loop_simple_while subset enforces "step-only body" under strict/dev shadow adopt.
#
# Expected:
# - Exit code 3 (sum increments to 3). If body is dropped, exit would be 0.

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

LEGACY_STEM="phase29ao_pattern1_subset_reject_extra_stmt_vm"
SEMANTIC_STEM="loop_simple_while_subset_reject_extra_stmt_vm"
LABEL_PREFIX="${SEMANTIC_STEM} (legacy stem ${LEGACY_STEM})"

FIXTURE="$NYASH_ROOT/apps/tests/phase29ao_pattern1_subset_reject_extra_stmt.hako"
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

if grep -Eq "\\[flowbox/adopt box_kind=Loop features=(break|continue)" <<<"$OUTPUT"; then
    log_error "${LABEL_PREFIX}: loop break/continue adopt tag must not appear"
    echo "$OUTPUT"
    exit 1
fi

log_success "${LABEL_PREFIX}: PASS (exit=3)"
exit 0

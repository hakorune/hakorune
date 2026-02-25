#!/bin/bash
# Phase 29ad P0: Pattern6 reverse scan contract violation
# Tests: reverse step mismatch must fail-fast

source "$(dirname "$0")/../../../lib/test_runner.sh"
export SMOKES_USE_PYVM=0
require_env || exit 2

INPUT="$NYASH_ROOT/apps/tests/phase29ab_pattern6_reverse_contract_min.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" "$INPUT" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29ab_pattern6_reverse_contract_min_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if expect_joinir_contract_freeze "phase29ab_pattern6_reverse_contract_min_vm" "$OUTPUT" "$EXIT_CODE" "[joinir/phase29ab/pattern6/contract]"; then
    exit 0
fi
exit 1

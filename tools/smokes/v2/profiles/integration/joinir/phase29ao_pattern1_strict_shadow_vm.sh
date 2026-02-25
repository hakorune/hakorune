#!/bin/bash
# phase29ao_pattern1_strict_shadow_vm.sh - Pattern1 strict shadow adopt gate (VM)
#
# Purpose:
# - Exercise Phase 29ao P17/P18 strict/dev shadow adopt path:
#   DomainPlan Pattern1 selected -> Facts→CorePlan(skeleton) adopted in router
#
# Expected:
# - Exit code 3 (same as existing Pattern1 PoC expectations)

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase286_pattern1_frag_poc.hako"
RUN_TIMEOUT_SECS=${RUN_TIMEOUT_SECS:-10}

set +e
OUTPUT=$(timeout "$RUN_TIMEOUT_SECS" env NYASH_DISABLE_PLUGINS=1 HAKO_JOINIR_STRICT=1 "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    log_error "phase29ao_pattern1_strict_shadow_vm: hakorune timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 3 ]; then
    log_error "phase29ao_pattern1_strict_shadow_vm: expected exit code 3, got $EXIT_CODE"
    echo "$OUTPUT"
    exit 1
fi

# Avoid `echo ... | grep -q` under `set -o pipefail` (SIGPIPE false negatives).
if ! grep -qF "[flowbox/adopt box_kind=Loop" <<<"$OUTPUT" \
    || ! grep -qF "via=shadow" <<<"$OUTPUT"; then
    echo "[FAIL] Missing FlowBox tag (box_kind=Loop via=shadow)"
    echo "$OUTPUT" | tail -n 40 || true
    test_fail "phase29ao_pattern1_strict_shadow_vm: Missing FlowBox tag"
    exit 1
fi

log_success "phase29ao_pattern1_strict_shadow_vm: PASS (exit=3)"
exit 0

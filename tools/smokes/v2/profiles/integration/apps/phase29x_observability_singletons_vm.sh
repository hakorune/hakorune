#!/bin/bash
# Phase 29x X16: observability `singletons` smoke (VM)
#
# Contract pin:
# 1) root categories include `singletons`.
# 2) VM lane reports measured `singletons` (>0 for fixture).
# 3) legacy limitation aliases are not emitted.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase285_leak_report.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
VM_HAKO_PREFER_STRICT_DEV="${NYASH_VM_HAKO_PREFER_STRICT_DEV:-0}"

if [ ! -f "$FIXTURE" ]; then
    test_fail "phase29x_observability_singletons_vm: fixture missing: $FIXTURE"
    exit 1
fi

set +e
OUTPUT=$(env NYASH_VM_HAKO_PREFER_STRICT_DEV="$VM_HAKO_PREFER_STRICT_DEV" NYASH_LEAK_LOG=1 NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29x_observability_singletons_vm: fixture timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
    echo "[FAIL] Expected fixture exit 0"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 120 || true
    test_fail "phase29x_observability_singletons_vm: fixture execution failed"
    exit 1
fi

SINGLETONS_COUNT="$(echo "$OUTPUT" | sed -n 's/.*\[lifecycle\/leak\]   singletons: \([0-9][0-9]*\).*/\1/p' | head -n 1)"
if [ -z "$SINGLETONS_COUNT" ]; then
    test_fail "phase29x_observability_singletons_vm: missing singletons category"
    exit 1
fi
if [ "$SINGLETONS_COUNT" -le 0 ]; then
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 120 || true
    test_fail "phase29x_observability_singletons_vm: expected singletons > 0, got $SINGLETONS_COUNT"
    exit 1
fi

if echo "$OUTPUT" | grep -q "singletons=0"; then
    test_fail "phase29x_observability_singletons_vm: stale limitation line still present"
    exit 1
fi

if echo "$OUTPUT" | grep -q "heap_fields/singletons=0"; then
    test_fail "phase29x_observability_singletons_vm: legacy limitation alias still present"
    exit 1
fi

test_pass "phase29x_observability_singletons_vm: PASS (singletons=$SINGLETONS_COUNT)"

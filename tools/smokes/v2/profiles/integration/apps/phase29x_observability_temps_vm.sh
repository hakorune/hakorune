#!/bin/bash
# Phase 29x X14: observability `temps` smoke (VM)
#
# Contract pin:
# 1) root categories include `temps`.
# 2) VM lane reports measured `temps` (>0 for fixture).
# 3) X16 以降は `singletons` category へ移行し、旧 limitation 依存を残さない。

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase285_leak_report.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
VM_HAKO_PREFER_STRICT_DEV="${NYASH_VM_HAKO_PREFER_STRICT_DEV:-0}"

if [ ! -f "$FIXTURE" ]; then
    test_fail "phase29x_observability_temps_vm: fixture missing: $FIXTURE"
    exit 1
fi

set +e
OUTPUT=$(env NYASH_VM_HAKO_PREFER_STRICT_DEV="$VM_HAKO_PREFER_STRICT_DEV" NYASH_LEAK_LOG=1 NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29x_observability_temps_vm: fixture timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
    echo "[FAIL] Expected fixture exit 0"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 120 || true
    test_fail "phase29x_observability_temps_vm: fixture execution failed"
    exit 1
fi

if ! echo "$OUTPUT" | grep -q "\[lifecycle/leak\] Root categories:"; then
    test_fail "phase29x_observability_temps_vm: missing root categories header"
    exit 1
fi

if ! echo "$OUTPUT" | grep -q "\[lifecycle/leak\]   handles:"; then
    test_fail "phase29x_observability_temps_vm: missing handles category"
    exit 1
fi

if ! echo "$OUTPUT" | grep -q "\[lifecycle/leak\]   locals:"; then
    test_fail "phase29x_observability_temps_vm: missing locals category"
    exit 1
fi

TEMPS_COUNT="$(echo "$OUTPUT" | sed -n 's/.*\[lifecycle\/leak\]   temps: \([0-9][0-9]*\).*/\1/p' | head -n 1)"
if [ -z "$TEMPS_COUNT" ]; then
    test_fail "phase29x_observability_temps_vm: missing temps category"
    exit 1
fi
if [ "$TEMPS_COUNT" -le 0 ]; then
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 120 || true
    test_fail "phase29x_observability_temps_vm: expected temps > 0, got $TEMPS_COUNT"
    exit 1
fi

if ! echo "$OUTPUT" | grep -q "\[lifecycle/leak\]   singletons:"; then
    if ! echo "$OUTPUT" | grep -q "\[lifecycle/leak\]   (Phase 1 limitation: .*singletons=0)"; then
        test_fail "phase29x_observability_temps_vm: singletons evidence missing"
        exit 1
    fi
fi

if echo "$OUTPUT" | grep -q "temps/heap_fields/singletons=0"; then
    test_fail "phase29x_observability_temps_vm: legacy limitation line still present"
    exit 1
fi

test_pass "phase29x_observability_temps_vm: PASS (temps=$TEMPS_COUNT)"

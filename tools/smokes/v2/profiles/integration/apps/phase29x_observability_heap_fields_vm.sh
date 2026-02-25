#!/bin/bash
# Phase 29x X15: observability `heap_fields` smoke (VM)
#
# Contract pin:
# 1) root categories include `heap_fields`.
# 2) VM lane reports measured `heap_fields` (>0 for fixture).
# 3) `singletons` category is present（X16 以降は limitation 行が消えても許容）。

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase285_leak_report.hako"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
VM_HAKO_PREFER_STRICT_DEV="${NYASH_VM_HAKO_PREFER_STRICT_DEV:-0}"

if [ ! -f "$FIXTURE" ]; then
    test_fail "phase29x_observability_heap_fields_vm: fixture missing: $FIXTURE"
    exit 1
fi

set +e
OUTPUT=$(env NYASH_VM_HAKO_PREFER_STRICT_DEV="$VM_HAKO_PREFER_STRICT_DEV" NYASH_LEAK_LOG=1 NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29x_observability_heap_fields_vm: fixture timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
    echo "[FAIL] Expected fixture exit 0"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 120 || true
    test_fail "phase29x_observability_heap_fields_vm: fixture execution failed"
    exit 1
fi

if ! echo "$OUTPUT" | grep -q "\[lifecycle/leak\] Root categories:"; then
    test_fail "phase29x_observability_heap_fields_vm: missing root categories header"
    exit 1
fi

if ! echo "$OUTPUT" | grep -q "\[lifecycle/leak\]   temps:"; then
    test_fail "phase29x_observability_heap_fields_vm: missing temps category"
    exit 1
fi

HEAP_FIELDS_COUNT="$(echo "$OUTPUT" | sed -n 's/.*\[lifecycle\/leak\]   heap_fields: \([0-9][0-9]*\).*/\1/p' | head -n 1)"
if [ -z "$HEAP_FIELDS_COUNT" ]; then
    test_fail "phase29x_observability_heap_fields_vm: missing heap_fields category"
    exit 1
fi
if [ "$HEAP_FIELDS_COUNT" -le 0 ]; then
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 120 || true
    test_fail "phase29x_observability_heap_fields_vm: expected heap_fields > 0, got $HEAP_FIELDS_COUNT"
    exit 1
fi

if ! echo "$OUTPUT" | grep -q "\[lifecycle/leak\]   singletons:"; then
    test_fail "phase29x_observability_heap_fields_vm: missing singletons category"
    exit 1
fi

if echo "$OUTPUT" | grep -q "heap_fields/singletons=0"; then
    test_fail "phase29x_observability_heap_fields_vm: legacy limitation line still present"
    exit 1
fi

test_pass "phase29x_observability_heap_fields_vm: PASS (heap_fields=$HEAP_FIELDS_COUNT)"

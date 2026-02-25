#!/bin/bash
# Phase 29x X17: observability summary contract smoke (VM)
#
# Contract pin:
# 1) `Root categories` emits fixed 5-category vocabulary exactly once.
# 2) Category order is stable: handles -> locals -> temps -> heap_fields -> singletons.
# 3) VM lane emits no Phase-1 limitation line once all 5 categories are observed.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

FIXTURE="$NYASH_ROOT/apps/tests/phase285_leak_report.hako"
CATEGORIES_FILE="$NYASH_ROOT/tools/checks/phase29x_observability_categories.txt"
RUN_TIMEOUT_SECS="${RUN_TIMEOUT_SECS:-30}"
VM_HAKO_PREFER_STRICT_DEV="${NYASH_VM_HAKO_PREFER_STRICT_DEV:-0}"

if [ ! -f "$FIXTURE" ]; then
    test_fail "phase29x_observability_summary_vm: fixture missing: $FIXTURE"
    exit 1
fi
if [ ! -f "$CATEGORIES_FILE" ]; then
    test_fail "phase29x_observability_summary_vm: category inventory missing: $CATEGORIES_FILE"
    exit 1
fi

set +e
OUTPUT=$(env NYASH_VM_HAKO_PREFER_STRICT_DEV="$VM_HAKO_PREFER_STRICT_DEV" NYASH_LEAK_LOG=1 NYASH_DISABLE_PLUGINS=1 timeout "$RUN_TIMEOUT_SECS" "$NYASH_BIN" --backend vm "$FIXTURE" 2>&1)
EXIT_CODE=$?
set -e

if [ "$EXIT_CODE" -eq 124 ]; then
    test_fail "phase29x_observability_summary_vm: fixture timed out (>${RUN_TIMEOUT_SECS}s)"
    exit 1
fi

if [ "$EXIT_CODE" -ne 0 ]; then
    echo "[FAIL] Expected fixture exit 0"
    echo "[INFO] Exit code: $EXIT_CODE"
    echo "[INFO] Output:"
    echo "$OUTPUT" | head -n 120 || true
    test_fail "phase29x_observability_summary_vm: fixture execution failed"
    exit 1
fi

if ! echo "$OUTPUT" | grep -q "\[lifecycle/leak\] Root categories:"; then
    test_fail "phase29x_observability_summary_vm: missing root categories header"
    exit 1
fi

PREV_LINE=0
CATEGORY_COUNT=0
while IFS= read -r CAT; do
    if [[ -z "$CAT" || "$CAT" =~ ^# ]]; then
        continue
    fi
    CATEGORY_COUNT=$((CATEGORY_COUNT + 1))
    COUNT="$(echo "$OUTPUT" | grep -c "\[lifecycle/leak\]   $CAT:")"
    if [ "$COUNT" -ne 1 ]; then
        test_fail "phase29x_observability_summary_vm: category '$CAT' count expected=1 got=$COUNT"
        exit 1
    fi

    LINE_NO="$(echo "$OUTPUT" | sed -n "/\\[lifecycle\\/leak\\]   $CAT:/=" | head -n 1)"
    if [ -z "$LINE_NO" ]; then
        test_fail "phase29x_observability_summary_vm: category '$CAT' line not found"
        exit 1
    fi
    if [ "$LINE_NO" -le "$PREV_LINE" ]; then
        test_fail "phase29x_observability_summary_vm: category order violation at '$CAT'"
        exit 1
    fi
    PREV_LINE="$LINE_NO"
done < "$CATEGORIES_FILE"

if [ "$CATEGORY_COUNT" -ne 5 ]; then
    test_fail "phase29x_observability_summary_vm: category inventory drift (expected=5 got=$CATEGORY_COUNT)"
    exit 1
fi

if echo "$OUTPUT" | grep -q "\[lifecycle/leak\]   (Phase 1 limitation:"; then
    test_fail "phase29x_observability_summary_vm: unexpected limitation line in VM lane"
    exit 1
fi

test_pass "phase29x_observability_summary_vm: PASS (5-category contract fixed)"

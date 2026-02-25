#!/usr/bin/env bash
# Phase 286A: Deprecated environment variable warning behavior
# Goal: Verify deprecated env vars only warn when explicitly set

set -euo pipefail

# Resolve PROJECT_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../../../.." && pwd)"
HAKORUNE="$PROJECT_ROOT/target/release/hakorune"
TEST_FILE="$PROJECT_ROOT/apps/tests/phase285_weak_basic.hako"

# Check hakorune binary exists
if [[ ! -f "$HAKORUNE" ]]; then
    echo "❌ FAIL: hakorune binary not found at $HAKORUNE"
    exit 1
fi

# Check test file exists
if [[ ! -f "$TEST_FILE" ]]; then
    echo "❌ FAIL: Test file not found at $TEST_FILE"
    exit 1
fi

# ========================================
# Case 1: Unset env - No warnings expected
# ========================================
set +e
# Use env -u to truly unset variables (empty string counts as "set")
OUTPUT_UNSET=$(env -u NYASH_MACRO_TOPLEVEL_ALLOW -u NYASH_MACRO_BOX_CHILD_RUNNER "$HAKORUNE" "$TEST_FILE" 2>&1)
EXIT_CODE_UNSET=$?
set -e

# Should succeed (exit code 2, Phase 285 P2: non-zero success code)
if [[ $EXIT_CODE_UNSET -ne 2 ]]; then
    echo "❌ FAIL: Test file should run successfully (got exit code $EXIT_CODE_UNSET, expected 2)"
    echo "Output: $OUTPUT_UNSET"
    exit 1
fi

# Should NOT contain deprecated warnings
if echo "$OUTPUT_UNSET" | grep -q "\[macro\]\[compat\]"; then
    echo "❌ FAIL: Unset env should not produce [macro][compat] warnings"
    echo "Got: $OUTPUT_UNSET"
    exit 1
fi

echo "✅ Case 1 PASS: Unset env produces no deprecated warnings"

# ========================================
# Case 2: Set env - Warnings expected
# ========================================
set +e
OUTPUT_SET=$(NYASH_MACRO_TOPLEVEL_ALLOW=1 "$HAKORUNE" "$TEST_FILE" 2>&1)
EXIT_CODE_SET=$?
set -e

# Should succeed (exit code 2, Phase 285 P2: non-zero success code)
if [[ $EXIT_CODE_SET -ne 2 ]]; then
    echo "❌ FAIL: Test file should run successfully with deprecated env (got exit code $EXIT_CODE_SET, expected 2)"
    echo "Output: $OUTPUT_SET"
    exit 1
fi

# Should contain specific deprecated warning
if ! echo "$OUTPUT_SET" | grep -q "\[macro\]\[compat\] NYASH_MACRO_TOPLEVEL_ALLOW"; then
    echo "❌ FAIL: Set env should produce [macro][compat] NYASH_MACRO_TOPLEVEL_ALLOW warning"
    echo "Got: $OUTPUT_SET"
    exit 1
fi

echo "✅ Case 2 PASS: Set env produces correct deprecated warning"

# ========================================
# Case 3: Second deprecated env test
# ========================================
set +e
OUTPUT_SET2=$(NYASH_MACRO_BOX_CHILD_RUNNER=1 "$HAKORUNE" "$TEST_FILE" 2>&1)
EXIT_CODE_SET2=$?
set -e

# Should succeed (exit code 2, Phase 285 P2: non-zero success code)
if [[ $EXIT_CODE_SET2 -ne 2 ]]; then
    echo "❌ FAIL: Test file should run successfully with deprecated env (got exit code $EXIT_CODE_SET2, expected 2)"
    echo "Output: $OUTPUT_SET2"
    exit 1
fi

# Should contain specific deprecated warning
if ! echo "$OUTPUT_SET2" | grep -q "\[macro\]\[compat\] NYASH_MACRO_BOX_CHILD_RUNNER"; then
    echo "❌ FAIL: Set env should produce [macro][compat] NYASH_MACRO_BOX_CHILD_RUNNER warning"
    echo "Got: $OUTPUT_SET2"
    exit 1
fi

echo "✅ Case 3 PASS: Second deprecated env produces correct warning"

echo "✅ PASS: All deprecated env warning tests passed"

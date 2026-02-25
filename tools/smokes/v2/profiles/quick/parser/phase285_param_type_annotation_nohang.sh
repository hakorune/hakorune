#!/usr/bin/env bash
# Phase 285A1.5: Parser hang fix - Parameter type annotations
# Goal: Verify parser does not hang on unsupported param: Type syntax

set -euo pipefail

# Resolve PROJECT_ROOT
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../../../../../.." && pwd)"
HAKORUNE="$PROJECT_ROOT/target/release/hakorune"
TEST_FILE="$PROJECT_ROOT/apps/tests/phase285_parser_param_type_annot_should_not_hang.hako"

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

# Run with timeout (3 seconds)
set +e
OUTPUT=$(timeout 3s "$HAKORUNE" "$TEST_FILE" 2>&1)
EXIT_CODE=$?
set -e

# Check 1: No timeout (exit code 124 means timeout)
if [[ $EXIT_CODE -eq 124 ]]; then
    echo "❌ FAIL: Parser hung (timeout after 3s)"
    exit 1
fi

# Check 2: Parse error (non-zero exit code expected)
if [[ $EXIT_CODE -eq 0 ]]; then
    echo "❌ FAIL: Parser should error on unsupported syntax (got exit code 0)"
    exit 1
fi

# Check 3: Error message mentions the COLON token issue
if ! echo "$OUTPUT" | grep -q "Unexpected token COLON"; then
    echo "❌ FAIL: Wrong error message (should mention 'Unexpected token COLON')"
    echo "Got: $OUTPUT"
    exit 1
fi

# Check 4: Error message mentions parameter type annotations
if ! echo "$OUTPUT" | grep -q "Parameter type annotations"; then
    echo "❌ FAIL: Error message should mention 'Parameter type annotations'"
    echo "Got: $OUTPUT"
    exit 1
fi

echo "✅ PASS: Parser correctly rejects param type annotations without hanging"

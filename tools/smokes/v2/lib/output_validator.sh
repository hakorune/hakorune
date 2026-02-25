#!/bin/bash
# Output Validator SSOT
# Single Source of Truth for smoke test output validation
#
# Box-First principle: Separation of concerns
# - Extract: Pattern-based extraction (numeric lines, etc.)
# - Assert: Comparison and failure reporting
#
# Fail-Fast principle: Explicit errors with clear messages

set -e

# Extract numeric lines from output
# Usage: extract_numeric_lines N < output.txt
# Returns: First N lines that match numeric pattern (including negative numbers)
extract_numeric_lines() {
  local limit="$1"
  if [ -z "$limit" ]; then
    echo "[ERROR] extract_numeric_lines: limit parameter required" >&2
    return 1
  fi

  # Pattern: optional minus, followed by digits
  # Use head to limit output
  grep -E '^-?[0-9]+$' | head -n "$limit" | tr -d '\r'
}

# Assert multiline string equality
# Usage: assert_equals_multiline EXPECTED ACTUAL
# Returns: 0 if equal, 1 if different (with error message)
assert_equals_multiline() {
  local expected="$1"
  local actual="$2"

  if [ -z "$expected" ]; then
    echo "[ERROR] assert_equals_multiline: expected parameter required" >&2
    return 1
  fi

  if [ "$actual" = "$expected" ]; then
    return 0
  else
    echo "[FAIL] Output mismatch" >&2
    echo "[EXPECTED]:" >&2
    echo "$expected" >&2
    echo "[ACTUAL]:" >&2
    echo "$actual" >&2
    return 1
  fi
}

# Validate numeric output (extract + assert in one step)
# Usage: validate_numeric_output EXPECTED_LINES EXPECTED_VALUE OUTPUT
# Returns: 0 if valid, 1 if invalid
validate_numeric_output() {
  local expected_lines="$1"
  local expected_value="$2"
  local output="$3"

  if [ -z "$expected_lines" ] || [ -z "$expected_value" ]; then
    echo "[ERROR] validate_numeric_output: expected_lines and expected_value required" >&2
    return 1
  fi

  local clean
  clean=$(printf "%s\n" "$output" | extract_numeric_lines "$expected_lines")

  assert_equals_multiline "$expected_value" "$clean"
}

# Box-First design notes:
# 1. extract_numeric_lines: Single Responsibility - extraction only
# 2. assert_equals_multiline: Single Responsibility - comparison only
# 3. validate_numeric_output: Composition box - combines extract + assert
# 4. All functions follow Fail-Fast principle - explicit parameter checks
# 5. Clear separation of concerns - easy to extend with new patterns

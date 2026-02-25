#!/usr/bin/env bash
# mir_canary.sh — Common helpers for MIR(JSON) canary tests
# Note: library only; do not set -e here. test_runner controls shell flags.

# Extract MIR JSON content between [MIR_BEGIN] and [MIR_END] markers from stdin
extract_mir_from_output() {
  awk '/\[MIR_BEGIN\]/{f=1;next}/\[MIR_END\]/{f=0}f'
}

# Assert that all given tokens are present in the provided input (stdin)
# Usage: some_source | assert_has_tokens token1 token2 ...
assert_has_tokens() {
  local content
  content=$(cat)
  for tk in "$@"; do
    if ! grep -Fq -- "$tk" <<<"$content"; then
      echo "[FAIL] token missing: $tk" >&2
      return 1
    fi
  done
  return 0
}

# Assert that output contains a specific SKIP tag line (exact or prefix match)
# Usage: some_source | assert_skip_tag "[SKIP] loop_toggle"  (prefix ok)
assert_skip_tag() {
  local pattern="$1"
  local content
  content=$(cat)
  if ! grep -Fq -- "$pattern" <<<"$content"; then
    echo "[FAIL] skip tag not found: $pattern" >&2
    return 1
  fi
  return 0
}

# Assert token1 appears before token2 in the input (byte offset order)
# Usage: some_source | assert_order token1 token2
assert_order() {
  local t1="$1"; shift
  local t2="$1"; shift || true
  local content
  content=$(cat)
  local p1 p2
  p1=$(grep -b -o -- "$t1" <<<"$content" | head -n1 | cut -d: -f1)
  p2=$(grep -b -o -- "$t2" <<<"$content" | head -n1 | cut -d: -f1)
  if [[ -z "$p1" || -z "$p2" ]]; then
    echo "[FAIL] tokens not found for order check: '$t1' vs '$t2'" >&2
    return 1
  fi
  if (( p1 < p2 )); then return 0; fi
  echo "[FAIL] token order invalid: '$t1' not before '$t2' (p1=$p1 p2=$p2)" >&2
  return 1
}

# Assert exact occurrence count of a token in input
# Usage: some_source | assert_token_count token expected_count
assert_token_count() {
  local token="$1"; local expected="$2"
  local content
  content=$(cat)
  local cnt
  cnt=$(grep -o -- "$token" <<<"$content" | wc -l | tr -d ' \t\n')
  if [[ "$cnt" = "$expected" ]]; then return 0; fi
  echo "[FAIL] token count mismatch for '$token': got $cnt, want $expected" >&2
  return 1
}

#!/usr/bin/env bash
# canary.sh — Common helpers for canary scripts (JSON extract / token checks)
# Library only; safe to source from any test harness or standalone script.

# Extract content between [MIR_BEGIN] and [MIR_END] tags from stdin
extract_mir_between_tags() {
  awk '/\[MIR_BEGIN\]/{f=1;next}/\[MIR_END\]/{f=0}f'
}

# Require that all given tokens appear in stdin; prints a FAIL line and
# exits with non-zero status if any token is missing.
require_tokens() {
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


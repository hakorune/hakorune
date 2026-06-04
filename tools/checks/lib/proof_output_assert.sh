#!/usr/bin/env bash
# Shared fixed-output assertions for proof/guard logs.

proof_output_assert_fixed_lines() {
  local tag="$1"
  local file="$2"
  shift 2

  local expected
  for expected in "$@"; do
    if ! rg -F -q -- "$expected" "$file"; then
      local display_file="$file"
      if [[ "$display_file" == "$PWD/"* ]]; then
        display_file="${display_file#$PWD/}"
      fi
      echo "[$tag] ERROR: missing expected output line in $display_file: $expected" >&2
      sed -n '1,200p' "$file" >&2
      exit 1
    fi
  done
}

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-physical-result-discard"
TARGET="src/mir/builder"
cd "$ROOT_DIR"

command -v rg >/dev/null || {
  echo "[$TAG] ERROR: rg is required" >&2
  exit 1
}
[[ -d "$TARGET" ]] || {
  echo "[$TAG] ERROR: missing target: $TARGET" >&2
  exit 1
}

# These are deliberately lexical guardrails for the sole physical writer.
# They do not classify the unrelated `let _ =` census or enable a workspace
# lint.  A future intentional exception must first change this card and guard.
UNDERSCORE_ASSIGN='^[[:space:]]*let[[:space:]]+_[[:alnum:]_]*[[:space:]]*=[^;]*\bemit_instruction[[:space:]]*\('
OPTION_OK='\bemit_instruction[[:space:]]*\([^;]*\.ok[[:space:]]*\([[:space:]]*\)[[:space:]]*'
DROP_CALL='\bdrop[[:space:]]*\([^;]*\bemit_instruction[[:space:]]*\('

reject_source_pattern() {
  local label="$1"
  local pattern="$2"
  local matches
  if matches="$(rg -n -U --pcre2 --glob '*.rs' "$pattern" "$TARGET")"; then
    echo "[$TAG] ERROR: $label" >&2
    printf '%s\n' "$matches" >&2
    return 1
  else
    local status=$?
    if (( status != 1 )); then
      echo "[$TAG] ERROR: rg failed while checking $label (status=$status)" >&2
      return "$status"
    fi
  fi
}

assert_rejected() {
  local label="$1"
  local text="$2"
  local pattern="$3"
  if printf '%s' "$text" | rg -q -U --pcre2 "$pattern"; then
    return 0
  fi
  echo "[$TAG] ERROR: fixture was not rejected: $label" >&2
  exit 1
}

assert_allowed() {
  local label="$1"
  local text="$2"
  local pattern="$3"
  if printf '%s' "$text" | rg -q -U --pcre2 "$pattern"; then
    echo "[$TAG] ERROR: fixture was rejected: $label" >&2
    exit 1
  fi
}

reject_source_pattern "underscore-bound emit_instruction result" "$UNDERSCORE_ASSIGN"
reject_source_pattern "emit_instruction followed by .ok()" "$OPTION_OK"
reject_source_pattern "emit_instruction passed to drop()" "$DROP_CALL"

# Keep multiline matching executable and keep propagation explicitly allowed.
assert_rejected "multiline underscore binding" $'let _ignored =\n    self.emit_instruction(\n        instruction,\n    );' "$UNDERSCORE_ASSIGN"
assert_rejected "multiline .ok()" $'self.emit_instruction(\n    instruction,\n).ok();' "$OPTION_OK"
assert_rejected "multiline drop" $'drop(\n    self.emit_instruction(instruction),\n);' "$DROP_CALL"

assert_allowed "propagated Result" $'self.emit_instruction(instruction)?;' "$UNDERSCORE_ASSIGN"
assert_allowed "named used Result" $'let emission = self.emit_instruction(instruction)?;' "$OPTION_OK"
assert_allowed "ordinary drop without emit" $'drop(previous_value);' "$DROP_CALL"

echo "[$TAG] PASS: no ignored emit_instruction result in $TARGET"

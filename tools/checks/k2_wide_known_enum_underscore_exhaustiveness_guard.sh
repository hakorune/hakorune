#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-321-RESULT-002C-KNOWN-ENUM-EXHAUSTIVENESS-UNDERSCORE-RULES.md'
ssot='docs/development/current/main/design/known-enum-underscore-exhaustiveness-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[known-enum-underscore] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-321 RESULT-002C known-enum exhaustiveness underscore rules"
require_text "$ssot" "Known Enum Underscore Exhaustiveness SSOT"
require_text docs/reference/language/EBNF.md 'RESULT-002C tags known-enum `_` exhaustiveness diagnostics.'
require_text src/parser/expr/match_expr_impl.rs "[enum/exhaustiveness][underscore]"
require_text docs/tools/check-scripts-index.md "k2_wide_known_enum_underscore_exhaustiveness_guard.sh"

cargo test -q underscore_exhaustiveness --lib
cargo test -q prelude_result_missing_arm_diagnostic --lib

echo "[known-enum-underscore] OK"

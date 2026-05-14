#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-323-GUARDLET-001-GUARD-LET-PATTERN-SUGAR.md'
ssot='docs/development/current/main/design/guard-let-pattern-sugar-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[guard-let] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-323 GUARDLET-001 guard-let pattern sugar"
require_text "$ssot" "Guard-Let Pattern Sugar SSOT"
require_text docs/reference/language/EBNF.md "guard let Type::Variant(binding) = expr else block"
require_text src/parser/statements/control_flow.rs "parse_guard_let_after_guard"
require_text src/parser/statements/control_flow.rs "__ny_guard_let_subject_"
require_text docs/tools/check-scripts-index.md "k2_wide_guard_let_pattern_sugar_guard.sh"

cargo test -q guard_let_enum_variant --lib

echo "[guard-let] OK"

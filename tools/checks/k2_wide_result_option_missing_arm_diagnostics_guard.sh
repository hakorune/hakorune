#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-319-RESULT-002A-PRELUDE-ENUM-MISSING-ARM-DIAGNOSTICS.md'
ssot='docs/development/current/main/design/result-option-missing-arm-diagnostics-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[result-option-missing-arm] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-319 RESULT-002A prelude enum missing-arm diagnostics"
require_text "$ssot" "Result / Option Missing-Arm Diagnostics SSOT"
require_text docs/reference/language/option.md "[enum/missing-arm][prelude]"
require_text docs/reference/language/EBNF.md 'RESULT-002A adds tagged prelude missing-arm diagnostics.'
require_text src/parser/expr/match_expr_impl.rs "[enum/missing-arm][prelude]"
require_text src/parser/expr/match_expr_impl.rs "is_result_option_prelude_enum"
require_text docs/tools/check-scripts-index.md "k2_wide_result_option_missing_arm_diagnostics_guard.sh"

cargo test -q prelude_option_missing_arm_diagnostic --lib
cargo test -q prelude_result_missing_arm_diagnostic --lib

echo "[result-option-missing-arm] OK"

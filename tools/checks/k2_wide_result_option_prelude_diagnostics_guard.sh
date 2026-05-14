#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-314-RESULT-001-RESULT-OPTION-PRELUDE-DIAGNOSTICS.md'
ssot='docs/development/current/main/design/result-option-prelude-diagnostics-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[result-option-prelude] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-314 RESULT-001 Result/Option prelude diagnostics"
require_text "$ssot" "Result / Option Prelude Diagnostics SSOT"
require_text docs/reference/language/EBNF.md 'RESULT-001 implements `Option<T>` / `Result<T,E>` as built-in enum prelude surfaces.'
require_text src/semantics/result_option_prelude.rs "result_option_prelude_enum_decls"
require_text src/parser/mod.rs "result_option_prelude"
require_text src/stage1/program_json_v0/authority.rs "result_option_prelude"
require_text src/stage1/program_json_v0/lowering.rs "[enum/variant-surface]"
require_text docs/tools/check-scripts-index.md "k2_wide_result_option_prelude_diagnostics_guard.sh"

cargo test -q result_option_prelude --lib
cargo test -q dot_enum_variant_surface --lib
cargo test -q prelude_option_some_null_payload --lib

echo "[result-option-prelude] OK"

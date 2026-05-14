#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-322-RESULT-002D-GENERIC-ENUM-EXPECTED-TYPE-DIAGNOSTICS.md'
ssot='docs/development/current/main/design/result-option-expected-type-diagnostics-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[result-option-expected-type] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-322 RESULT-002D generic enum expected-type diagnostics"
require_text "$ssot" "Result / Option Expected-Type Diagnostics SSOT"
require_text docs/reference/language/option.md "[enum/expected-type][prelude]"
require_text docs/reference/language/EBNF.md 'RESULT-002D adds tagged prelude expected-type diagnostics.'
require_text src/stage1/program_json_v0/lowering.rs "[enum/expected-type][prelude]"
require_text src/stage1/program_json_v0/lowering.rs "is_prelude_result_option_enum"
require_text docs/tools/check-scripts-index.md "k2_wide_result_option_expected_type_diagnostics_guard.sh"

cargo test -q prelude_result_constructor_without_expected_type --lib
cargo test -q prelude_option_unit_constructor_without_expected_type --lib
cargo test -q prelude_enum_payload --lib
cargo test -q prelude_option_some_null_payload --lib
cargo test -q source_to_program_json_v0_emits_enum_inventory_and_ctor --lib
cargo test -q source_to_program_json_v0_emits_unit_enum_ctor --lib

echo "[result-option-expected-type] OK"

#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-320-RESULT-002B-PRELUDE-ENUM-PAYLOAD-DIAGNOSTICS.md'
ssot='docs/development/current/main/design/result-option-payload-diagnostics-ssot.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[result-option-payload] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-320 RESULT-002B prelude enum payload diagnostics"
require_text "$ssot" "Result / Option Payload Diagnostics SSOT"
require_text docs/reference/language/option.md "[enum/payload][prelude]"
require_text docs/reference/language/EBNF.md 'RESULT-002B adds tagged prelude payload arity diagnostics.'
require_text src/stage1/program_json_v0/lowering.rs "[enum/payload][prelude]"
require_text src/stage1/program_json_v0/lowering.rs "is_prelude_result_option_enum"
require_text docs/tools/check-scripts-index.md "k2_wide_result_option_payload_diagnostics_guard.sh"

cargo test -q prelude_enum_payload --lib
cargo test -q prelude_option_some_null_payload --lib

echo "[result-option-payload] OK"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[record-with-update-lowering] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/record-with-update-lowering-ssot.md "REC-003 Record With-Update Lowering SSOT"
require_text docs/development/current/main/phases/phase-293x/293x-281-REC-003-RECORD-WITH-UPDATE-LOWERING.md "Status: complete"
require_text docs/reference/language/EBNF.md "record_update := expr 'with' '{' record_update_field"
require_text docs/reference/language/low-level-capabilities.md "with-update lowering complete"
require_text src/ast/mod.rs "RecordUpdate"
require_text src/parser/expr/call.rs "parse_record_update"
require_text src/stage1/program_json_v0/lowering.rs '"type": "RecordUpdate"'
require_text src/tests/parser_record_literal_surface.rs "parser_record_update_surface_parses_explicit_named_updates"
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_lowers_record_with_update"
require_text docs/tools/check-scripts-index.md "k2_wide_record_with_update_lowering_guard.sh"

cargo test -q record_

echo "[record-with-update-lowering] OK"

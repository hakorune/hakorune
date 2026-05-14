#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[record-literal-parser-capsule] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/record-literal-parser-capsule-ssot.md "REC-001 Stage0 Record Literal Parser Capsule SSOT"
require_text docs/development/current/main/phases/phase-293x/293x-279-REC-001-STAGE0-RECORD-LITERAL-PARSER-CAPSULE.md "Status: complete"
require_text docs/reference/language/EBNF.md "record_literal := IDENT '{' record_literal_field"
require_text src/ast/mod.rs "RecordLiteral"
require_text src/parser/expr/primary.rs "parse_record_literal"
require_text src/macro/ast_json/joinir_compat.rs '"kind": "RecordLiteral"'
require_text src/stage1/program_json_v0/lowering.rs '"type": "RecordLiteral"'
require_text src/tests/parser_record_literal_surface.rs "parser_record_literal_surface_parses_explicit_named_fields"
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_emits_record_literal_shape_metadata"
require_text docs/tools/check-scripts-index.md "k2_wide_record_literal_parser_capsule_guard.sh"

cargo test -q record_literal

echo "[record-literal-parser-capsule] OK"

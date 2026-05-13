#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[type-alias-parser-capsule] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/type-alias-parser-capsule-ssot.md "TYPE-001 Stage0 Type Alias Parser Capsule SSOT"
require_text docs/development/current/main/phases/phase-293x/293x-278-TYPE-001-STAGE0-TYPE-ALIAS-PARSER-CAPSULE.md "Status: complete"
require_text docs/reference/language/EBNF.md "type_alias_decl := 'type' IDENT '=' TYPE_REF"
require_text src/ast/mod.rs "TypeAliasDeclaration"
require_text src/parser/declarations/type_alias_def.rs "parse_type_alias_declaration"
require_text src/macro/ast_json/joinir_compat.rs '"kind": "TypeAliasDeclaration"'
require_text src/stage1/program_json_v0/authority.rs '"type_alias_decls"'
require_text src/tests/parser_type_alias_surface.rs "parser_type_alias_surface_parses_metadata_only_alias"
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_emits_type_alias_decls_metadata_only"
require_text docs/tools/check-scripts-index.md "k2_wide_type_alias_parser_capsule_guard.sh"

cargo test -q type_alias

echo "[type-alias-parser-capsule] OK"

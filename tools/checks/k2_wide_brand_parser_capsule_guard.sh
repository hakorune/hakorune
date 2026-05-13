#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[brand-parser-capsule] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/brand-parser-capsule-ssot.md "BRAND-001 Stage0 Brand Parser Capsule SSOT"
require_text docs/development/current/main/phases/phase-293x/293x-275-BRAND-001-STAGE0-BRAND-PARSER-CAPSULE.md "Status: complete"
require_text docs/reference/language/EBNF.md "brand_decl := 'brand' IDENT ':' TYPE_REF"
require_text src/ast/mod.rs "BrandDeclaration"
require_text src/parser/declarations/brand_def.rs "parse_brand_declaration"
require_text src/macro/ast_json/joinir_compat.rs '"kind": "BrandDeclaration"'
require_text src/stage1/program_json_v0/authority.rs '"brand_decls"'
require_text src/tests/parser_brand_surface.rs "parser_brand_surface_parses_brand_declaration_metadata"
require_text docs/tools/check-scripts-index.md "k2_wide_brand_parser_capsule_guard.sh"

cargo test -q parser_brand_surface

echo "[brand-parser-capsule] OK"

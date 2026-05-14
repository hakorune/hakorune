#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[uses-metadata-capsule] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/uses-metadata-capsule-ssot.md "USES-001 Method-Level Uses Metadata Capsule SSOT"
require_text docs/development/current/main/phases/phase-293x/293x-284-USES-001-METHOD-LEVEL-USES-METADATA-CAPSULE.md "Status: complete"
require_text docs/reference/language/EBNF.md "uses_clause := 'uses' IDENT (',' IDENT)*"
require_text docs/reference/language/low-level-capabilities.md "method-level uses metadata capsule complete"
require_text src/ast/mod.rs "uses: Vec<String>"
require_text src/parser/contracts.rs "parse_signature_metadata_until_body"
require_text src/macro/ast_json/joinir_compat.rs '"uses"'
require_text src/stage1/program_json_v0/lowering.rs '"uses"'
require_text src/tests/parser_uses_surface.rs "parser_uses_surface_parses_method_uses_metadata_only"
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_transports_uses_metadata"
require_text docs/tools/check-scripts-index.md "k2_wide_uses_metadata_capsule_guard.sh"

cargo test -q parser_uses_surface
cargo test -q source_to_program_json_v0_transports_uses_metadata

echo "[uses-metadata-capsule] OK"

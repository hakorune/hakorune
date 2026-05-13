#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[delegate-parser-capsule] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/delegation-parser-capsule-ssot.md "DEL-002 Stage0 Delegate Syntax Metadata Capsule"
require_text docs/development/current/main/phases/phase-293x/293x-273-DEL-002-STAGE0-DELEGATE-PARSER-CAPSULE.md "Status: complete"
require_text src/ast/mod.rs "pub struct DelegateDecl"
require_text src/parser/declarations/box_def/members/delegates.rs "parse_delegate_decl"
require_text src/parser/declarations/box_def/mod.rs "box_try_delegate"
require_text src/macro/ast_json/joinir_compat.rs '"delegates"'
require_text src/stage1/program_json_v0/authority.rs '"delegates"'
require_text src/tests/parser_delegate_surface.rs "parser_delegate_surface_parses_explicit_exposes_list"

cargo test -q parser_delegate_surface

echo "[delegate-parser-capsule] OK"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[contract-syntax-metadata] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/contract-syntax-metadata-capsule-ssot.md "CONTRACT-002 Contract Syntax Metadata Capsule SSOT"
require_text docs/development/current/main/phases/phase-293x/293x-282-CONTRACT-002-CONTRACT-SYNTAX-METADATA-CAPSULE.md "Status: complete"
require_text docs/reference/language/EBNF.md "contract_clause := ('requires' | 'ensures') expr"
require_text docs/reference/language/EBNF.md "invariant_member := 'invariant' expr"
require_text docs/reference/language/low-level-capabilities.md "contract syntax metadata capsule complete"
require_text src/ast/mod.rs "pub struct ContractClause"
require_text src/parser/contracts.rs "parse_signature_metadata_until_body"
require_text src/macro/ast_json/joinir_compat.rs '"contracts"'
require_text src/stage1/program_json_v0/lowering.rs '"contracts"'
require_text src/stage1/program_json_v0/authority.rs '"invariants"'
require_text src/tests/parser_contract_surface.rs "parser_contract_surface_parses_method_requires_ensures_metadata_only"
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_transports_contract_metadata"
require_text docs/tools/check-scripts-index.md "k2_wide_contract_syntax_metadata_guard.sh"

cargo test -q contract_surface
cargo test -q source_to_program_json_v0_transports_contract_metadata
cargo test -q source_to_program_json_v0_transports_invariant_metadata

echo "[contract-syntax-metadata] OK"

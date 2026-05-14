#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$path"; then
    echo "[astclean-legacy-enum] missing '$needle' in $path" >&2
    exit 1
  fi
}

reject_text() {
  local path="$1"
  local needle="$2"
  if grep -Fq "$needle" "$path"; then
    echo "[astclean-legacy-enum] forbidden '$needle' in $path" >&2
    exit 1
  fi
}

require_text "docs/development/current/main/design/ast-cleanup-before-localtype-ssot.md" "ASTCLEAN-001"
require_text "docs/development/current/main/phases/phase-293x/293x-294-ASTCLEAN-001-LEGACY-AST-ENUM-REMOVAL.md" "Decision: accepted"
require_text "src/ast/utils/classify.rs" "pub fn is_expression"
require_text "docs/tools/check-scripts-index.md" "k2_wide_astclean_legacy_enum_guard.sh"

reject_text "src/ast/mod.rs" "pub enum ASTNodeType"
reject_text "src/ast/mod.rs" "pub enum StructureNode"
reject_text "src/ast/mod.rs" "pub enum ExpressionNode"
reject_text "src/ast/mod.rs" "pub enum StatementNode"
reject_text "src/ast/utils/classify.rs" "ASTNodeType"
reject_text "src/ast/utils/classify.rs" "pub fn classify"
reject_text "src/ast/utils/classify.rs" "pub fn is_structure"
reject_text "src/ast/utils/classify.rs" "pub fn is_statement"

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[astclean-legacy-enum] OK"

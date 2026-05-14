#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$path"; then
    echo "[loopclean-while-variant-quarantine] missing '$needle' in $path" >&2
    exit 1
  fi
}

reject_text() {
  local path="$1"
  local needle="$2"
  if grep -Fq "$needle" "$path"; then
    echo "[loopclean-while-variant-quarantine] forbidden '$needle' in $path" >&2
    exit 1
  fi
}

require_text "docs/development/current/main/phases/phase-293x/293x-291-LOOPCLEAN-003-WHILE-VARIANT-QUARANTINE.md" "Decision: accepted"
require_text "src/parser/statements/control_flow.rs" "Ok(ASTNode::Loop"
require_text "src/tests/parser_loop_cleanup_surface.rs" "parser output must not propagate While"
require_text "src/stage1/program_json_v0/lowering.rs" "program_json_v0_from_body_lowers_legacy_while_ast_to_loop_json"
require_text "docs/tools/check-scripts-index.md" "k2_wide_loopclean_while_variant_quarantine_guard.sh"

reject_text "src/tests/parser_loop_scan_range_shape.rs" "ASTNode::While { condition"
reject_text "src/tests/parser_loop_scan_range_shape.rs" "expected While"

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast
cargo test -q program_json_v0_from_body_lowers_legacy_while_ast_to_loop_json

echo "[loopclean-while-variant-quarantine] OK"

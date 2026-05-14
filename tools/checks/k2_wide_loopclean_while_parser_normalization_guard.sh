#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$path"; then
    echo "[loopclean-while-parser-normalization] missing '$needle' in $path" >&2
    exit 1
  fi
}

require_text "docs/development/current/main/design/loop-cleanup-before-packedarray-ssot.md" "LOOPCLEAN-002"
require_text "docs/development/current/main/phases/phase-293x/293x-290-LOOPCLEAN-002-WHILE-PARSER-NORMALIZATION.md" "Status: implemented"
require_text "src/parser/statements/control_flow.rs" "legacy while sugar normalizes to canonical Loop"
require_text "src/tests/parser_loop_cleanup_surface.rs" "parser_loopclean_while_stage3_normalizes_to_loop_ast"
require_text "docs/tools/check-scripts-index.md" "k2_wide_loopclean_while_parser_normalization_guard.sh"

cargo test -q parser_loopclean_while_stage3_normalizes_to_loop_ast

echo "[loopclean-while-parser-normalization] OK"

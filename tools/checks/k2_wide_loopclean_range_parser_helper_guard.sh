#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$path"; then
    echo "[loopclean-range-parser-helper] missing '$needle' in $path" >&2
    exit 1
  fi
}

reject_text() {
  local path="$1"
  local needle="$2"
  if grep -Fq "$needle" "$path"; then
    echo "[loopclean-range-parser-helper] forbidden '$needle' in $path" >&2
    exit 1
  fi
}

require_text "docs/development/current/main/phases/phase-293x/293x-292-LOOPCLEAN-004-RANGE-PARSER-HELPER-COMMONIZATION.md" "Decision: accepted"
require_text "src/parser/statements/control_flow.rs" "fn parse_range_header"
require_text "src/parser/statements/control_flow.rs" "parse_range_header(\"for range index identifier\")"
require_text "src/tests/parser_loop_scan_range_shape.rs" "parser_legacy_for_range_surface_uses_shared_for_range_shape"
require_text "docs/tools/check-scripts-index.md" "k2_wide_loopclean_range_parser_helper_guard.sh"

reject_text "src/parser/statements/control_flow.rs" "Stage-3: for-range parsing helper (currently unused)"

cargo test -q parser_loop_range_surface_parses_parenless_loop_header
cargo test -q parser_legacy_for_range_surface_uses_shared_for_range_shape

echo "[loopclean-range-parser-helper] OK"

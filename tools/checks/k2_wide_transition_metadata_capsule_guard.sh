#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[transition-metadata-capsule] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/transition-metadata-capsule-ssot.md "TRANS-001 Transition Metadata Capsule SSOT"
require_text docs/development/current/main/phases/phase-293x/293x-283-TRANS-001-TRANSITION-METADATA-CAPSULE.md "Status: complete"
require_text docs/reference/language/EBNF.md "transition_member := 'transition' TYPE_REF '.' IDENT '-' '>' TYPE_REF '.' IDENT 'by' IDENT"
require_text docs/reference/language/low-level-capabilities.md "transition metadata capsule complete"
require_text src/ast/mod.rs "pub struct TransitionDecl"
require_text src/parser/declarations/box_def/members/transitions.rs "try_parse_transition_decl"
require_text src/parser/declarations/box_def/mod.rs "box_try_transition"
require_text src/macro/ast_json/joinir_compat.rs '"transitions"'
require_text src/stage1/program_json_v0/authority.rs '"transitions"'
require_text src/tests/parser_transition_surface.rs "parser_transition_surface_parses_box_transition_metadata_only"
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_transports_transition_metadata"
require_text docs/tools/check-scripts-index.md "k2_wide_transition_metadata_capsule_guard.sh"

cargo test -q parser_transition_surface
cargo test -q source_to_program_json_v0_transports_transition_metadata

echo "[transition-metadata-capsule] OK"

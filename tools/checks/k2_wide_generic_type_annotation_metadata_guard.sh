#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[generic-type-annotation-metadata] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/generic-type-annotation-metadata-capsule-ssot.md "GEN-001 Generic Type Annotation Metadata Capsule SSOT"
require_text docs/development/current/main/phases/phase-293x/293x-285-GEN-001-GENERIC-TYPE-ANNOTATION-METADATA-CAPSULE.md "Status: complete"
require_text docs/reference/language/EBNF.md "TYPE_REF       := IDENT ('.' IDENT)* ('<' TYPE_REF (',' TYPE_REF)* '>')? ('[' ']')*"
require_text docs/reference/language/low-level-capabilities.md "generic type annotation metadata capsule complete"
require_text src/parser/common/type_refs.rs "parse_type_ref_text"
require_text src/parser/declarations/box_def/members/syntax.rs "parse_type_ref_text"
require_text src/tests/parser_generic_type_annotation_surface.rs "parser_generic_type_annotation_surface_parses_box_field_metadata"
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_transports_generic_type_metadata"
require_text docs/tools/check-scripts-index.md "k2_wide_generic_type_annotation_metadata_guard.sh"

cargo test -q parser_generic_type_annotation_surface
cargo test -q source_to_program_json_v0_transports_generic_type_metadata

echo "[generic-type-annotation-metadata] OK"

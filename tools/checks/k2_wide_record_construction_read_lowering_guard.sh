#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[record-construction-read-lowering] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/record-construction-read-lowering-ssot.md "REC-002 Stage1 Record Construction/Read Lowering SSOT"
require_text docs/development/current/main/phases/phase-293x/293x-280-REC-002-STAGE1-RECORD-CONSTRUCTION-READ-LOWERING.md "Status: complete"
require_text docs/reference/language/EBNF.md "Record literals must mention exactly the declared field set"
require_text docs/reference/language/low-level-capabilities.md "construction/read lowering complete"
require_text src/stage1/program_json_v0/authority.rs "collect_record_decl_index"
require_text src/stage1/program_json_v0/lowering.rs "validate_record_literal_fields"
require_text src/stage1/program_json_v0/lowering.rs '"type": "RecordField"'
require_text src/stage1/program_json_v0/tests/basics_and_enums.rs "source_to_program_json_v0_lowers_record_field_read"
require_text docs/tools/check-scripts-index.md "k2_wide_record_construction_read_lowering_guard.sh"

cargo test -q record_literal

echo "[record-construction-read-lowering] OK"

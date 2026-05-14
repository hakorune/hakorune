#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$path"; then
    echo "[packed-array-eligibility] missing '$needle' in $path" >&2
    exit 1
  fi
}

reject_text() {
  local path="$1"
  local needle="$2"
  if grep -Fq "$needle" "$path"; then
    echo "[packed-array-eligibility] forbidden '$needle' in $path" >&2
    exit 1
  fi
}

require_text "docs/development/current/main/phases/phase-293x/293x-293-PACKED-001-PACKEDARRAY-ELIGIBILITY-GATE.md" "Decision: accepted"
require_text "src/stage1/program_json_v0/packed_array_eligibility_checker.rs" "[packed/eligibility]"
require_text "src/stage1/program_json_v0/packed_array_eligibility_checker.rs" "ordinary-box-element"
require_text "src/stage1/program_json_v0/authority.rs" "check_packed_array_eligibility"
require_text "src/stage1/program_json_v0/type_ref.rs" "parse_type_ref_text"
require_text "src/stage1/program_json_v0/tests/basics_and_enums.rs" "source_to_program_json_v0_accepts_packed_array_integer_record_eligibility"
require_text "docs/tools/check-scripts-index.md" "k2_wide_packed_array_eligibility_guard.sh"

reject_text "src/stage1/program_json_v0/packed_array_eligibility_checker.rs" "ArrayStorage::InlineRecord"
reject_text "src/stage1/program_json_v0/packed_array_eligibility_checker.rs" "production_auto_use_enabled"

cargo test -q source_to_program_json_v0_accepts_packed_array_integer_record_eligibility
cargo test -q source_to_program_json_v0_rejects_packed_array_ordinary_box_element
cargo test -q source_to_program_json_v0_rejects_packed_array_handle_field
cargo test -q source_to_program_json_v0_rejects_packed_array_generic_record_instantiation
cargo test -q source_to_program_json_v0_rejects_builtin_generic_arity_mismatch

echo "[packed-array-eligibility] OK"

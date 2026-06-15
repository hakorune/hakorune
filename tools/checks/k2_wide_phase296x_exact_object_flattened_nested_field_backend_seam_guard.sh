#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-718-EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-SEAM-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-717-EXACT-OBJECT-FLATTENED-NESTED-FIELD-SHADOW-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-719-EXACT-OBJECT-PILOT-001S.md"
SEAM="src/llvm_py/instructions/flattened_nested_fields.py"
TOOL="tools/allocator/hako_exact_object_flattened_nested_field_backend_seam.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_flattened_nested_field_backend_seam_guard.sh"

[[ -f "$CARD" ]] || { echo "[flattened-nested-backend-seam] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[flattened-nested-backend-seam] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[flattened-nested-backend-seam] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$SEAM" ]] || { echo "[flattened-nested-backend-seam] missing seam: $SEAM" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[flattened-nested-backend-seam] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[flattened-nested-backend-seam] row718 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[flattened-nested-backend-seam] row717 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[flattened-nested-backend-seam] row719 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[flattened-nested-backend-seam] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[flattened-nested-backend-seam] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-flattened-nested-field-backend-seam-v0"
require_line_in_file "$CARD" "source_evidence=296x-717"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "representation_choice=flatten_nested_fields"
require_line_in_file "$CARD" "flattened_nested_field_count=4"
require_line_in_file "$CARD" "backend_flattened_nested_field_consumer=1"
require_line_in_file "$CARD" "backend_lowering_enabled=0"
require_line_in_file "$CARD" "object_storage_plan_execution_enabled=0"
require_line_in_file "$CARD" "pilot_exact_object_enabled=0"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "mirbuilder_special_case_count=0"
require_line_in_file "$CARD" "benchmark_name_branch_count=0"
require_line_in_file "$CARD" "helper_name_branch_count=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "fallback_to_generic_box_supported=1"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-PILOT-001S"
require_line_in_file "$CARD" "summary=ok"
require_line_in_file "$NEXT_CARD" "Task: EXACT-OBJECT-PILOT-001S"

grep -q 'BACKEND_FLATTENED_NESTED_FIELD_CONSUMER = True' "$SEAM" || { echo "[flattened-nested-backend-seam] seam consumer flag missing" >&2; exit 1; }
grep -q 'FLATTENED_NESTED_FIELD_LOWERING_ENABLED =' "$SEAM" || { echo "[flattened-nested-backend-seam] seam lowering flag missing" >&2; exit 1; }
grep -q 'validate_flattened_nested_field_plan' "$SEAM" || { echo "[flattened-nested-backend-seam] seam validator missing" >&2; exit 1; }
grep -q 'output_contract=hako-exact-object-flattened-nested-field-backend-seam-v0' "$TOOL" || { echo "[flattened-nested-backend-seam] tool missing output contract" >&2; exit 1; }

echo "[flattened-nested-backend-seam] ok"

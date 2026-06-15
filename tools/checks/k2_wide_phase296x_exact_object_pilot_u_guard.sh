#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-723-EXACT-OBJECT-PILOT-001U.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-722-EXACT-OBJECT-FLATTENED-NESTED-FIELD-ROUTE-WIRING-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-724-EXACT-OBJECT-PILOT-MEASUREMENT-001.md"
SEAM="src/llvm_py/instructions/flattened_nested_fields.py"
FIELD_ACCESS="src/llvm_py/instructions/field_access.py"
METHOD_CALL="src/llvm_py/instructions/mir_call/method_call.py"
TOOL="tools/allocator/hako_exact_object_pilot_u_enablement.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_pilot_u_guard.sh"

[[ -f "$CARD" ]] || { echo "[exact-object-pilot-u] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[exact-object-pilot-u] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[exact-object-pilot-u] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$SEAM" ]] || { echo "[exact-object-pilot-u] missing seam: $SEAM" >&2; exit 1; }
[[ -f "$FIELD_ACCESS" ]] || { echo "[exact-object-pilot-u] missing field access module: $FIELD_ACCESS" >&2; exit 1; }
[[ -f "$METHOD_CALL" ]] || { echo "[exact-object-pilot-u] missing method call module: $METHOD_CALL" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[exact-object-pilot-u] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[exact-object-pilot-u] row723 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[exact-object-pilot-u] row722 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[exact-object-pilot-u] row724 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[exact-object-pilot-u] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[exact-object-pilot-u] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-pilot-u-v0"
require_line_in_file "$CARD" "source_evidence=296x-722"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "nested_owner=HakoAllocObjectLifecycleFacade.alignment_result"
require_line_in_file "$CARD" "nested_object=HakoAllocObjectLifecycleAlignmentResult"
require_line_in_file "$CARD" "state_sharing_seam_defined=1"
require_line_in_file "$CARD" "route_wiring_ready=1"
require_line_in_file "$CARD" "field_access_flattened_nested_route_enabled=1"
require_line_in_file "$CARD" "method_call_flattened_nested_route_enabled=1"
require_line_in_file "$CARD" "backend_lowering_enabled=1"
require_line_in_file "$CARD" "object_storage_plan_execution_enabled=1"
require_line_in_file "$CARD" "pilot_exact_object_enabled=1"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "benchmark_name_branch_count=0"
require_line_in_file "$CARD" "helper_name_branch_count=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "fallback_to_generic_box_supported=1"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-PILOT-MEASUREMENT-001"
require_line_in_file "$CARD" "summary=ok"
require_line_in_file "$NEXT_CARD" "Task: EXACT-OBJECT-PILOT-MEASUREMENT-001"

grep -q 'FLATTENED_NESTED_FIELD_LOWERING_ENABLED = True' "$SEAM" || { echo "[exact-object-pilot-u] lowering flag must be enabled" >&2; exit 1; }
grep -q 'try_lower_owner_field_get' "$FIELD_ACCESS" || { echo "[exact-object-pilot-u] field get route missing" >&2; exit 1; }
grep -q 'try_lower_owner_field_set' "$FIELD_ACCESS" || { echo "[exact-object-pilot-u] field set route missing" >&2; exit 1; }
grep -q 'try_lower_nested_method_call' "$METHOD_CALL" || { echo "[exact-object-pilot-u] nested method route missing" >&2; exit 1; }
grep -q 'output_contract=hako-exact-object-pilot-u-v0' "$TOOL" || { echo "[exact-object-pilot-u] tool missing output contract" >&2; exit 1; }

echo "[exact-object-pilot-u] ok"

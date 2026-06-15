#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-722-EXACT-OBJECT-FLATTENED-NESTED-FIELD-ROUTE-WIRING-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-721-EXACT-OBJECT-PILOT-001T.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-723-EXACT-OBJECT-PILOT-001U.md"
FIELD_ACCESS="src/llvm_py/instructions/field_access.py"
METHOD_CALL="src/llvm_py/instructions/mir_call/method_call.py"
TOOL="tools/allocator/hako_exact_object_flattened_nested_field_route_wiring.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_flattened_nested_field_route_wiring_guard.sh"

[[ -f "$CARD" ]] || { echo "[flattened-nested-route-wiring] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[flattened-nested-route-wiring] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[flattened-nested-route-wiring] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$FIELD_ACCESS" ]] || { echo "[flattened-nested-route-wiring] missing field access module: $FIELD_ACCESS" >&2; exit 1; }
[[ -f "$METHOD_CALL" ]] || { echo "[flattened-nested-route-wiring] missing method call module: $METHOD_CALL" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[flattened-nested-route-wiring] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[flattened-nested-route-wiring] row722 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[flattened-nested-route-wiring] row721 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[flattened-nested-route-wiring] row723 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[flattened-nested-route-wiring] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[flattened-nested-route-wiring] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-flattened-nested-field-route-wiring-v0"
require_line_in_file "$CARD" "source_evidence=296x-721"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "nested_owner=HakoAllocObjectLifecycleFacade.alignment_result"
require_line_in_file "$CARD" "nested_object=HakoAllocObjectLifecycleAlignmentResult"
require_line_in_file "$CARD" "state_sharing_seam_defined=1"
require_line_in_file "$CARD" "field_access_flattened_nested_route_enabled=1"
require_line_in_file "$CARD" "method_call_flattened_nested_route_enabled=1"
require_line_in_file "$CARD" "route_wiring_ready=1"
require_line_in_file "$CARD" "backend_lowering_enabled=0"
require_line_in_file "$CARD" "object_storage_plan_execution_enabled=0"
require_line_in_file "$CARD" "pilot_exact_object_enabled=0"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "benchmark_name_branch_count=0"
require_line_in_file "$CARD" "helper_name_branch_count=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "fallback_to_generic_box_supported=1"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-PILOT-001U"
require_line_in_file "$CARD" "summary=ok"
require_line_in_file "$NEXT_CARD" "Task: EXACT-OBJECT-PILOT-001U"

grep -q '_flattened_nested_field_access_route_enabled' "$FIELD_ACCESS" || { echo "[flattened-nested-route-wiring] field access hook missing" >&2; exit 1; }
grep -q '_flattened_nested_method_call_route_enabled' "$METHOD_CALL" || { echo "[flattened-nested-route-wiring] method call hook missing" >&2; exit 1; }
grep -q 'FLATTENED_NESTED_FIELD_LOWERING_ENABLED =' src/llvm_py/instructions/flattened_nested_fields.py || { echo "[flattened-nested-route-wiring] lowering flag missing" >&2; exit 1; }
grep -q 'output_contract=hako-exact-object-flattened-nested-field-route-wiring-v0' "$TOOL" || { echo "[flattened-nested-route-wiring] tool missing output contract" >&2; exit 1; }

echo "[flattened-nested-route-wiring] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-721-EXACT-OBJECT-PILOT-001T.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-720-EXACT-OBJECT-FLATTENED-NESTED-FIELD-STATE-SEAM-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-722-EXACT-OBJECT-FLATTENED-NESTED-FIELD-ROUTE-WIRING-001.md"
TOOL="tools/allocator/hako_exact_object_pilot_t_preflight.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_pilot_t_guard.sh"

[[ -f "$CARD" ]] || { echo "[exact-object-pilot-t] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[exact-object-pilot-t] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[exact-object-pilot-t] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[exact-object-pilot-t] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[exact-object-pilot-t] row721 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[exact-object-pilot-t] row720 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[exact-object-pilot-t] row722 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[exact-object-pilot-t] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[exact-object-pilot-t] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-pilot-t-v0"
require_line_in_file "$CARD" "source_evidence=296x-720"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "nested_owner=HakoAllocObjectLifecycleFacade.alignment_result"
require_line_in_file "$CARD" "nested_object=HakoAllocObjectLifecycleAlignmentResult"
require_line_in_file "$CARD" "representation_choice=flatten_nested_fields"
require_line_in_file "$CARD" "state_sharing_seam_defined=1"
require_line_in_file "$CARD" "typed_newbox_preempts_local_aggregate=1"
require_line_in_file "$CARD" "field_access_flattened_nested_route_enabled=0"
require_line_in_file "$CARD" "method_call_flattened_nested_route_enabled=0"
require_line_in_file "$CARD" "route_wiring_ready=0"
require_line_in_file "$CARD" "backend_lowering_enabled=0"
require_line_in_file "$CARD" "object_storage_plan_execution_enabled=0"
require_line_in_file "$CARD" "pilot_exact_object_enabled=0"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "benchmark_name_branch_count=0"
require_line_in_file "$CARD" "helper_name_branch_count=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "fallback_to_generic_box_supported=1"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-ROUTE-WIRING-001"
require_line_in_file "$CARD" "summary=blocked"
require_line_in_file "$NEXT_CARD" "Task: EXACT-OBJECT-FLATTENED-NESTED-FIELD-ROUTE-WIRING-001"

grep -q 'output_contract=hako-exact-object-pilot-t-v0' "$TOOL" || { echo "[exact-object-pilot-t] tool missing output contract" >&2; exit 1; }
grep -q 'route_wiring_ready' "$TOOL" || { echo "[exact-object-pilot-t] tool missing route wiring evidence" >&2; exit 1; }
grep -q 'selected_next' "$TOOL" || { echo "[exact-object-pilot-t] tool missing selected next key" >&2; exit 1; }
grep -q 'EXACT-OBJECT-FLATTENED-NESTED-FIELD-ROUTE-WIRING-001' "$TOOL" || { echo "[exact-object-pilot-t] tool missing route wiring next task" >&2; exit 1; }

echo "[exact-object-pilot-t] ok"

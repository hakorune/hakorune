#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-710-OBJECT-BOUNDARY-INVENTORY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-709-MIMALLOC-POST-ARRAY-LEN-HELPER-OWNER-REFRESH-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-711-OBJECT-STORAGE-PLAN-SSOT-001.md"
SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
TOOL="tools/allocator/hako_object_boundary_inventory.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_object_boundary_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[object-boundary-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[object-boundary-inventory] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[object-boundary-inventory] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$SSOT" ]] || { echo "[object-boundary-inventory] missing SSOT: $SSOT" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[object-boundary-inventory] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[object-boundary-inventory] row710 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[object-boundary-inventory] row709 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[object-boundary-inventory] row711 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[object-boundary-inventory] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[object-boundary-inventory] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-object-boundary-inventory-v0"
require_line_in_file "$CARD" "source_evidence=296x-709"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "box_callable_registry_is_callable_truth=1"
require_line_in_file "$CARD" "routeplan_is_call_execution_truth=1"
require_line_in_file "$CARD" "object_storage_plan_is_representation_truth=1"
require_line_in_file "$CARD" "arc_dynbox_boundary_count=2"
require_line_in_file "$CARD" "host_handle_boundary_count=3"
require_line_in_file "$CARD" "runtime_helper_boundary_count=1"
require_line_in_file "$CARD" "dynamic_box_method_route_count=1"
require_line_in_file "$CARD" "box_callable_routeplan_dynamic_count=1"
require_line_in_file "$CARD" "closed_world_direct_method_candidate_count=31"
require_line_in_file "$CARD" "exact_stack_object_candidate_count=5"
require_line_in_file "$CARD" "exact_native_struct_candidate_count=9"
require_line_in_file "$CARD" "scalarized_object_candidate_count=5"
require_line_in_file "$CARD" "object_escape_count=4"
require_line_in_file "$CARD" "selected_object_boundary_owner=object_handle_boundary_inventory"
require_line_in_file "$CARD" "selected_owner_confidence=medium"
require_line_in_file "$CARD" "implementation_started=0"
require_line_in_file "$CARD" "summary=ok"

require_line_in_file "$SSOT" "object_storage_plan_is_representation_truth=1"
require_line_in_file "$NEXT_CARD" "Task: OBJECT-STORAGE-PLAN-SSOT-001"

echo "[object-boundary-inventory] ok"

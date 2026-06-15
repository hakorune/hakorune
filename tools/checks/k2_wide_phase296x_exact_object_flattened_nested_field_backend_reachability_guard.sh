#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-726-EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-REACHABILITY-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-725-EXACT-OBJECT-PILOT-EFFECT-ATTRIBUTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_flattened_nested_field_backend_reachability_guard.sh"

[[ -f "$CARD" ]] || { echo "[exact-object-backend-reachability] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[exact-object-backend-reachability] missing previous card: $PREV_CARD" >&2; exit 1; }

grep -q '^Status: Landed / Blocked$' "$CARD" || { echo "[exact-object-backend-reachability] row726 card must be Landed / Blocked" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[exact-object-backend-reachability] row725 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[exact-object-backend-reachability] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[exact-object-backend-reachability] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "python_llvmlite_route_updated=1"
require_line_in_file "$CARD" "measured_exact_exe_driver=ny_llvmc_boundary"
require_line_in_file "$CARD" "python_route_is_measurement_owner=0"
require_line_in_file "$CARD" "boundary_driver_flattened_nested_consumer=0"
require_line_in_file "$CARD" "selected_owner=ny_llvmc_boundary_driver_reachability"
require_line_in_file "$CARD" "mir_json_object_storage_plan_count=0"
require_line_in_file "$CARD" "mir_json_flattened_nested_plan_count=0"
require_line_in_file "$CARD" "boundary_driver_has_input_plan_for_flattened_nested_fields=0"
require_line_in_file "$CARD" "backend_reachability_fixed=0"
require_line_in_file "$CARD" "missing_owner=object_storage_plan_mir_json_export"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-PLAN-EXPORT-001"
require_line_in_file "$CARD" "summary=blocked"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "type_abi_execution_truth=0"
require_line_in_file "$CARD" "hako_check_execution_truth=0"
require_line_in_file "$CARD" "benchmark_name_branch_count=0"
require_line_in_file "$CARD" "helper_name_branch_count=0"
require_line_in_file "$CARD" "source_file_name_branch_count=0"

echo "[exact-object-backend-reachability] ok"

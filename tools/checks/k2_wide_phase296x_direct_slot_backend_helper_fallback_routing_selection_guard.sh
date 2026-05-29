#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-329-DIRECT-SLOT-BACKEND-HELPER-FALLBACK-ROUTING-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-328-DIRECT-SLOT-BACKEND-MATERIALIZATION-SNAPSHOT-PILOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row329-direct-slot-backend-helper-fallback-routing-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-backend-helper-fallback-routing-selection-v0"
require_line "$DOC" "input_contract=direct-slot-backend-materialization-snapshot-pilot-v0"
require_line "$DOC" "selected_owner=direct_slot_materialized_view_boundary_handle_policy"
require_line "$DOC" "existing_helper_route_to_direct_backend=0"
require_line "$DOC" "existing_helper_route_to_snapshot_per_call=0"
require_line "$DOC" "generic_helper_route_to_direct_backend=0"
require_line "$DOC" "exact_slot_helper_route_to_direct_backend=0"
require_line "$DOC" "snapshot_materialization_allowed=1"
require_line "$DOC" "snapshot_materialization_trigger=explicit_boundary_only"
require_line "$DOC" "materialized_view_handle_required_before_helper_routing=1"
require_line "$DOC" "direct_cell_primary_storage=1"
require_line "$DOC" "typed_slot_role=materialized_view_not_primary_storage"
require_line "$DOC" "dual_truth_allowed=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "selected_next=direct_slot_materialized_view_handle_policy_selection"
require_line "$DOC" "summary=ok"

echo "[row329-direct-slot-backend-helper-fallback-routing-selection] ok"

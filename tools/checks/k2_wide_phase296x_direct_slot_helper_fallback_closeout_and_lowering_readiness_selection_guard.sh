#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-332-DIRECT-SLOT-HELPER-FALLBACK-CLOSEOUT-AND-LOWERING-READINESS-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-331-DIRECT-SLOT-MATERIALIZED-VIEW-HANDLE-PILOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row332-direct-slot-helper-fallback-closeout-and-lowering-readiness-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-helper-fallback-closeout-and-lowering-readiness-selection-v0"
require_line "$DOC" "input_contract=direct-slot-materialized-view-handle-pilot-v0"
require_line "$DOC" "fallback_scaffolding_closeout=1"
require_line "$DOC" "direct_slot_object_layout_ready=1"
require_line "$DOC" "direct_slot_handle_resolution_ready=1"
require_line "$DOC" "direct_slot_cell_primary_storage_ready=1"
require_line "$DOC" "explicit_snapshot_materialization_ready=1"
require_line "$DOC" "materialized_view_handle_ready=1"
require_line "$DOC" "existing_helper_fallback_boundary_ready=1"
require_line "$DOC" "direct_handle_helper_route_closed=1"
require_line "$DOC" "per_helper_snapshot_routing_closed=1"
require_line "$DOC" "typed_slot_enum_layout_exposure=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "selected_owner=direct_slot_nativedirect_lowering_readiness_inventory"
require_line "$DOC" "selected_target_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_next=direct_slot_nativedirect_lowering_readiness_inventory"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "summary=ok"

echo "[row332-direct-slot-helper-fallback-closeout-and-lowering-readiness-selection] ok"

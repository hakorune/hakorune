#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-324-DIRECT-SLOT-MATERIALIZATION-FALLBACK-SYNC-SSOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-323-DIRECT-SLOT-OBJECT-LAYOUT-PILOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row324-direct-slot-materialization-fallback-sync-ssot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-materialization-fallback-sync-ssot-v0"
require_line "$DOC" "input_contract=direct-slot-object-layout-pilot-v0"
require_line "$DOC" "selected_owner=direct_slot_materialization_fallback_sync_policy"
require_line "$DOC" "direct_backend_primary_storage=DirectSlotCellV0"
require_line "$DOC" "direct_object_layout=DirectSlotObjectV0"
require_line "$DOC" "typed_slot_role=fallback_materialization_debug_view"
require_line "$DOC" "typed_slot_primary_storage_in_direct_backend=0"
require_line "$DOC" "direct_cell_cache_only_policy=0"
require_line "$DOC" "dual_truth_allowed=0"
require_line "$DOC" "sync_direction_for_current_pilot=direct_cell_to_typed_slot_on_explicit_materialization"
require_line "$DOC" "implicit_sync_on_every_direct_write=0"
require_line "$DOC" "materialization_required_before_public_observer=1"
require_line "$DOC" "materialization_required_before_existing_helper_fallback=1"
require_line "$DOC" "materialization_required_before_unknown_escape=1"
require_line "$DOC" "materialization_policy_implementation_open=0"
require_line "$DOC" "fallback_bridge_implementation_open=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "existing_helper_abi_unchanged=1"
require_line "$DOC" "raw_runtime_vec_pointer_exposure=0"
require_line "$DOC" "typed_slot_enum_layout_exposure=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "selected_next=direct_slot_object_backend_connection_selection"
require_line "$DOC" "summary=ok"

echo "[row324-direct-slot-materialization-fallback-sync-ssot] ok"

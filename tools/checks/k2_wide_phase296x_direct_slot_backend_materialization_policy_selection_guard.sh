#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-327-DIRECT-SLOT-BACKEND-MATERIALIZATION-POLICY-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-326-DIRECT-SLOT-OBJECT-BACKEND-CONNECTION-PILOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row327-direct-slot-backend-materialization-policy-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-backend-materialization-policy-selection-v0"
require_line "$DOC" "input_contract=direct-slot-object-backend-connection-pilot-v0"
require_line "$DOC" "selected_owner=direct_slot_object_to_typed_slot_snapshot_materialization"
require_line "$DOC" "selected_bridge=direct_slot_object_v0_to_typed_slot_object_snapshot"
require_line "$DOC" "sync_direction=direct_cell_to_typed_slot_snapshot"
require_line "$DOC" "direct_cell_primary_storage=1"
require_line "$DOC" "typed_slot_role=fallback_materialization_debug_view"
require_line "$DOC" "dual_truth_allowed=0"
require_line "$DOC" "implicit_sync_on_every_direct_write=0"
require_line "$DOC" "per_write_typed_slot_update=0"
require_line "$DOC" "materialization_trigger=explicit_only"
require_line "$DOC" "materialization_view_lifetime=snapshot"
require_line "$DOC" "unsupported_storage_tag_policy=fail_or_none_not_silent_fallback"
require_line "$DOC" "generic_helper_route_to_direct_backend=0"
require_line "$DOC" "exact_slot_helper_route_to_direct_backend=0"
require_line "$DOC" "helper_routing_implementation_open=0"
require_line "$DOC" "materialization_bridge_implementation_open=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "selected_next=direct_slot_backend_materialization_snapshot_pilot"
require_line "$DOC" "summary=ok"

echo "[row327-direct-slot-backend-materialization-policy-selection] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-325-DIRECT-SLOT-OBJECT-BACKEND-CONNECTION-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-324-DIRECT-SLOT-MATERIALIZATION-FALLBACK-SYNC-SSOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row325-direct-slot-object-backend-connection-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-object-backend-connection-selection-v0"
require_line "$DOC" "input_contract=direct-slot-materialization-fallback-sync-ssot-v0"
require_line "$DOC" "selected_owner=typed_object_store_direct_slot_backend_connection"
require_line "$DOC" "selected_backend_name=direct_slot_exact"
require_line "$DOC" "new_backend_allowed=1"
require_line "$DOC" "default_backend_unchanged=1"
require_line "$DOC" "pinned_arena_exact_backend_unchanged=1"
require_line "$DOC" "direct_slot_primary_storage=DirectSlotCellV0"
require_line "$DOC" "typed_slot_role=fallback_materialization_debug_view"
require_line "$DOC" "direct_handle_emission_default=0"
require_line "$DOC" "generic_helper_route_to_direct_backend=0"
require_line "$DOC" "exact_slot_helper_route_to_direct_backend=0"
require_line "$DOC" "materialization_bridge_required_before_helper_route=1"
require_line "$DOC" "new_c_abi_helper_symbols=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "selected_next=direct_slot_object_backend_connection_pilot"
require_line "$DOC" "summary=ok"

echo "[row325-direct-slot-object-backend-connection-selection] ok"

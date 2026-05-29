#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-330-DIRECT-SLOT-MATERIALIZED-VIEW-HANDLE-POLICY-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-329-DIRECT-SLOT-BACKEND-HELPER-FALLBACK-ROUTING-SELECTION.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row330-direct-slot-materialized-view-handle-policy-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-materialized-view-handle-policy-selection-v0"
require_line "$DOC" "input_contract=direct-slot-backend-helper-fallback-routing-selection-v0"
require_line "$DOC" "selected_owner=direct_slot_materialized_view_negative_handle_store"
require_line "$DOC" "direct_handle_kind=tagged_stable_direct_slot_object_pointer"
require_line "$DOC" "direct_handle_sign=positive_tagged"
require_line "$DOC" "materialized_view_handle_kind=typed_slot_view_handle"
require_line "$DOC" "materialized_view_handle_sign=negative_index_handle"
require_line "$DOC" "materialized_view_storage=separate_thread_local_typed_slot_object_vec"
require_line "$DOC" "materialized_view_source=explicit_direct_slot_snapshot"
require_line "$DOC" "existing_helper_route_to_direct_backend=0"
require_line "$DOC" "existing_helper_route_to_materialized_view_handle=1"
require_line "$DOC" "existing_helper_route_to_snapshot_per_call=0"
require_line "$DOC" "view_writeback_to_direct_slot=0"
require_line "$DOC" "dual_truth_allowed=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "selected_next=direct_slot_materialized_view_handle_pilot"
require_line "$DOC" "summary=ok"

echo "[row330-direct-slot-materialized-view-handle-policy-selection] ok"

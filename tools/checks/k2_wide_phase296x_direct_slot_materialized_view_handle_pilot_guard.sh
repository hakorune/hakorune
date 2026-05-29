#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-331-DIRECT-SLOT-MATERIALIZED-VIEW-HANDLE-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-330-DIRECT-SLOT-MATERIALIZED-VIEW-HANDLE-POLICY-SELECTION.md"
STORE="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_store.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row331-direct-slot-materialized-view-handle-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row331-direct-slot-materialized-view-handle-pilot] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-materialized-view-handle-pilot-v0"
require_line "$DOC" "input_contract=direct-slot-materialized-view-handle-policy-selection-v0"
require_line "$DOC" "implemented_api=materialize_direct_slot_view_handle"
require_line "$DOC" "materialized_view_storage=separate_thread_local_typed_slot_object_vec"
require_line "$DOC" "direct_handle_sign=positive_tagged"
require_line "$DOC" "materialized_view_handle_sign=negative_index_handle"
require_line "$DOC" "direct_handle_helper_route=closed"
require_line "$DOC" "materialized_view_helper_route=implemented"
require_line "$DOC" "view_writeback_to_direct_slot=0"
require_line "$DOC" "generic_helper_route_to_direct_backend=0"
require_line "$DOC" "exact_slot_helper_route_to_direct_backend=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "direct_materialized_view_handle_smoke=ok"
require_line "$DOC" "summary=ok"

require_pattern "$STORE" "DIRECT_SLOT_MATERIALIZED_VIEWS"
require_pattern "$STORE" "pub(crate) fn materialize_direct_slot_view_handle"
require_pattern "$STORE" "fn with_direct_slot_materialized_view"
require_pattern "$STORE" "fn direct_slot_exact_materialized_view_handle_routes_existing_helpers"

HAKO_TYPED_OBJECT_STORE=direct_slot_exact \
  cargo test -p nyash_kernel direct_slot_exact_materialized_view_handle_routes_existing_helpers -- --nocapture

echo "[row331-direct-slot-materialized-view-handle-pilot] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-357-DIRECT-ARRAY-I64-MATERIALIZED-VIEW-HANDLE-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-356-DIRECT-ARRAY-I64-MATERIALIZED-VIEW-HANDLE-POLICY-SELECTION.md"
SRC="$ROOT_DIR/crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row357-direct-array-i64-view-handle-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq -- "$pattern" "$file"; then
    echo "[row357-direct-array-i64-view-handle-pilot] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-materialized-view-handle-pilot-v0"
require_line "$DOC" "input_contract=direct-array-i64-materialized-view-handle-policy-selection-v0"
require_line "$DOC" "implemented_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"
require_line "$DOC" "implemented_api=materialize_public_arraybox_snapshot_handle"
require_line "$DOC" "materialized_view_storage=host_handle_arc_arraybox_snapshot"
require_line "$DOC" "direct_array_handle_public=0"
require_line "$DOC" "materialized_view_handle_sign=positive_host_handle"
require_line "$DOC" "materialized_view_source=explicit_direct_array_i64_snapshot"
require_line "$DOC" "materialized_view_lifetime=host_handle_lifetime"
require_line "$DOC" "direct_array_helper_route=closed"
require_line "$DOC" "materialized_view_helper_route=public_arraybox_existing_helpers"
require_line "$DOC" "view_writeback_to_direct_array=0"
require_line "$DOC" "direct_array_primary_storage=1"
require_line "$DOC" "public_arraybox_role=materialized_view_not_primary_storage"
require_line "$DOC" "dual_truth_allowed=0"
require_line "$DOC" "implicit_sync_on_every_direct_write=0"
require_line "$DOC" "generic_array_helper_route_to_direct_backend=0"
require_line "$DOC" "i64_slot_helper_route_to_direct_backend=0"
require_line "$DOC" "new_c_abi_helper_symbols=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "public_arraybox_host_handle_smoke=ok"
require_line "$DOC" "snapshot_len_preserved=1"
require_line "$DOC" "snapshot_i64_values_preserved=1"
require_line "$DOC" "selected_next=direct_array_i64_helper_fallback_closeout_and_lowering_readiness_selection"
require_line "$DOC" "summary=ok"

require_pattern "$SRC" "pub(crate) fn materialize_public_arraybox_snapshot_handle"
require_pattern "$SRC" "host_handles::to_handle_arc"
require_pattern "$SRC" "fn direct_array_i64_buffer_v0_materializes_public_arraybox_host_handle"

cargo test -p nyash_kernel direct_array_i64_buffer_v0_materializes_public_arraybox_host_handle --lib -- --nocapture

echo "[row357-direct-array-i64-view-handle-pilot] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-356-DIRECT-ARRAY-I64-MATERIALIZED-VIEW-HANDLE-POLICY-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-355-DIRECT-ARRAY-I64-MATERIALIZATION-SNAPSHOT-PILOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row356-direct-array-i64-view-handle-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq -- "$pattern" "$file"; then
    echo "[row356-direct-array-i64-view-handle-selection] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-materialized-view-handle-policy-selection-v0"
require_line "$DOC" "input_contract=direct-array-i64-materialization-snapshot-pilot-v0"
require_line "$DOC" "selected_owner=direct_array_i64_materialized_public_arraybox_handle"
require_line "$DOC" "direct_array_handle_kind=backend_internal_direct_array_buffer"
require_line "$DOC" "direct_array_handle_public=0"
require_line "$DOC" "materialized_view_handle_kind=public_arraybox_host_handle"
require_line "$DOC" "materialized_view_handle_sign=positive_host_handle"
require_line "$DOC" "materialized_view_storage=host_handle_arc_arraybox_snapshot"
require_line "$DOC" "materialized_view_source=explicit_direct_array_i64_snapshot"
require_line "$DOC" "materialized_view_lifetime=host_handle_lifetime"
require_line "$DOC" "existing_helper_route_to_direct_backend=0"
require_line "$DOC" "existing_helper_route_to_materialized_view_handle=1"
require_line "$DOC" "existing_helper_route_to_snapshot_per_call=0"
require_line "$DOC" "direct_array_primary_storage=1"
require_line "$DOC" "public_arraybox_role=materialized_view_not_primary_storage"
require_line "$DOC" "view_writeback_to_direct_array=0"
require_line "$DOC" "dual_truth_allowed=0"
require_line "$DOC" "implicit_sync_on_every_direct_write=0"
require_line "$DOC" "new_c_abi_helper_symbols=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "selected_next=direct_array_i64_materialized_view_handle_pilot"
require_line "$DOC" "summary=ok"

require_pattern "$DOC" "Materialized view:"
require_pattern "$DOC" "separate public ArrayBox host handle"
require_pattern "$DOC" "rejected=helper_reads_direct_array_buffer"

echo "[row356-direct-array-i64-view-handle-selection] ok"

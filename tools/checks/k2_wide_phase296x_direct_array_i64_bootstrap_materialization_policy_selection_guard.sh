#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-354-DIRECT-ARRAY-I64-BOOTSTRAP-MATERIALIZATION-POLICY-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-353-DIRECT-ARRAY-I64-BACKEND-CONNECTION-PILOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row354-direct-array-i64-materialization-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq -- "$pattern" "$file"; then
    echo "[row354-direct-array-i64-materialization-selection] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-bootstrap-materialization-policy-selection-v0"
require_line "$DOC" "input_contract=direct-array-i64-backend-connection-pilot-v0"
require_line "$DOC" "selected_owner=direct_array_i64_to_public_arraybox_snapshot_materialization"
require_line "$DOC" "selected_bridge=direct_array_i64_buffer_v0_to_public_arraybox_snapshot"
require_line "$DOC" "sync_direction=direct_array_i64_to_public_arraybox_snapshot"
require_line "$DOC" "bootstrap_direction=public_arraybox_to_direct_array_i64_deferred"
require_line "$DOC" "direct_array_primary_storage=1"
require_line "$DOC" "public_arraybox_role=fallback_materialization_debug_view"
require_line "$DOC" "public_arraybox_primary_storage_in_direct_backend=0"
require_line "$DOC" "dual_truth_allowed=0"
require_line "$DOC" "implicit_sync_on_every_direct_write=0"
require_line "$DOC" "per_write_public_arraybox_update=0"
require_line "$DOC" "materialization_trigger=explicit_only"
require_line "$DOC" "materialization_view_lifetime=snapshot"
require_line "$DOC" "materialization_requires_direct_array_backend=1"
require_line "$DOC" "materialization_requires_generation_validation=1"
require_line "$DOC" "materialization_requires_i64_element_tag=1"
require_line "$DOC" "materialization_requires_len_le_capacity=1"
require_line "$DOC" "unsupported_element_tag_policy=fail_or_none_not_silent_fallback"
require_line "$DOC" "generic_array_helper_route_to_direct_backend=0"
require_line "$DOC" "i64_slot_helper_route_to_direct_backend=0"
require_line "$DOC" "fallback_helper_reads_direct_array_before_bridge=0"
require_line "$DOC" "helper_routing_implementation_open=0"
require_line "$DOC" "materialization_bridge_implementation_open=0"
require_line "$DOC" "bootstrap_bridge_implementation_open=0"
require_line "$DOC" "new_c_abi_helper_symbols=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "selected_next=direct_array_i64_materialization_snapshot_pilot"
require_line "$DOC" "summary=ok"

require_pattern "$DOC" "DirectArrayI64BufferV0"
require_pattern "$DOC" "-> public ArrayBox snapshot"
require_pattern "$DOC" "This is not a permanent dual-storage policy."
require_pattern "$DOC" "rejected=route_existing_helpers_directly_to_direct_array"

echo "[row354-direct-array-i64-materialization-selection] ok"

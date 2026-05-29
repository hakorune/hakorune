#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-355-DIRECT-ARRAY-I64-MATERIALIZATION-SNAPSHOT-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-354-DIRECT-ARRAY-I64-BOOTSTRAP-MATERIALIZATION-POLICY-SELECTION.md"
SRC="$ROOT_DIR/crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row355-direct-array-i64-snapshot-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq -- "$pattern" "$file"; then
    echo "[row355-direct-array-i64-snapshot-pilot] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-materialization-snapshot-pilot-v0"
require_line "$DOC" "input_contract=direct-array-i64-bootstrap-materialization-policy-selection-v0"
require_line "$DOC" "implemented_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"
require_line "$DOC" "implemented_bridge=direct_array_i64_buffer_v0_to_public_arraybox_snapshot"
require_line "$DOC" "sync_direction=direct_array_i64_to_public_arraybox_snapshot"
require_line "$DOC" "materialization_trigger=explicit_only"
require_line "$DOC" "materialization_view_lifetime=snapshot"
require_line "$DOC" "direct_array_primary_storage=1"
require_line "$DOC" "public_arraybox_role=fallback_materialization_debug_view"
require_line "$DOC" "dual_truth_allowed=0"
require_line "$DOC" "implicit_sync_on_every_direct_write=0"
require_line "$DOC" "per_write_public_arraybox_update=0"
require_line "$DOC" "materialization_requires_generation_validation=1"
require_line "$DOC" "materialization_requires_i64_element_tag=1"
require_line "$DOC" "materialization_requires_len_le_capacity=1"
require_line "$DOC" "unsupported_element_tag_policy=none_not_silent_fallback"
require_line "$DOC" "public_arraybox_snapshot_smoke=ok"
require_line "$DOC" "snapshot_len_preserved=1"
require_line "$DOC" "snapshot_i64_values_preserved=1"
require_line "$DOC" "snapshot_oob_semantics_preserved=1"
require_line "$DOC" "generic_array_helper_route_to_direct_backend=0"
require_line "$DOC" "i64_slot_helper_route_to_direct_backend=0"
require_line "$DOC" "helper_routing_implementation_open=0"
require_line "$DOC" "bootstrap_bridge_implementation_open=0"
require_line "$DOC" "new_c_abi_helper_symbols=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "selected_next=direct_array_i64_materialized_view_handle_policy_selection"
require_line "$DOC" "summary=ok"

require_pattern "$SRC" "pub(crate) fn materialize_public_arraybox_snapshot"
require_pattern "$SRC" "fn header_is_supported"
require_pattern "$SRC" "ArrayBox::new()"
require_pattern "$SRC" "slot_store_i64_raw"
require_pattern "$SRC" "fn direct_array_i64_buffer_v0_materializes_public_arraybox_snapshot"

cargo test -p nyash_kernel direct_array_i64_buffer_v0_materializes_public_arraybox_snapshot --lib -- --nocapture

echo "[row355-direct-array-i64-snapshot-pilot] ok"

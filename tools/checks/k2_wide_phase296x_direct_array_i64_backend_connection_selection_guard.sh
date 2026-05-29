#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-352-DIRECT-ARRAY-I64-BACKEND-CONNECTION-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-351-DIRECT-ARRAY-I64-MATERIALIZATION-SYNC-SSOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row352-direct-array-i64-backend-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row352-direct-array-i64-backend-selection] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-backend-connection-selection-v0"
require_line "$DOC" "input_contract=direct-array-i64-materialization-sync-ssot-v0"
require_line "$DOC" "selected_owner=array_slot_backend_direct_array_i64_connection"
require_line "$DOC" "selected_backend_name=direct_array_i64_exact"
require_line "$DOC" "selected_reason=connect_stable_direct_array_i64_storage_before_lowering_retry"
require_line "$DOC" "new_backend_allowed=1"
require_line "$DOC" "default_backend_unchanged=1"
require_line "$DOC" "single_thread_exact_backend_unchanged=1"
require_line "$DOC" "direct_array_buffer_storage_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"
require_line "$DOC" "array_slot_backend_selector_owner=crates/nyash_kernel/src/plugin/array_slot_backend.rs"
require_line "$DOC" "direct_array_primary_storage=DirectArrayI64BufferV0"
require_line "$DOC" "public_arraybox_role=fallback_materialization_debug_view"
require_line "$DOC" "array_slot_cache_role=diagnostic_helper_floor_only"
require_line "$DOC" "direct_array_emission_default=0"
require_line "$DOC" "generic_array_helper_route_to_direct_backend=0"
require_line "$DOC" "i64_slot_helper_route_to_direct_backend=0"
require_line "$DOC" "materialization_bridge_required_before_helper_route=1"
require_line "$DOC" "bootstrap_bridge_required_before_helper_route=1"
require_line "$DOC" "generation_validation_required_before_handle_route=1"
require_line "$DOC" "new_c_abi_helper_symbols=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "selected_next=direct_array_i64_backend_connection_pilot"
require_line "$DOC" "summary=ok"

require_pattern "$DOC" 'Add a distinct backend selection point named `direct_array_i64_exact`.'
require_pattern "$DOC" '`single_thread_exact` is a diagnostic helper floor'
require_pattern "$DOC" '`direct_array_i64_exact` is the future primary-storage backend'
require_pattern "$DOC" "The next row must not silently make existing helper calls hit the DirectArray"

echo "[row352-direct-array-i64-backend-selection] ok"

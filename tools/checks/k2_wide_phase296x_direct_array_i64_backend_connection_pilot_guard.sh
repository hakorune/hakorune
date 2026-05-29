#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-353-DIRECT-ARRAY-I64-BACKEND-CONNECTION-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-352-DIRECT-ARRAY-I64-BACKEND-CONNECTION-SELECTION.md"
BACKEND="$ROOT_DIR/crates/nyash_kernel/src/plugin/array_slot_backend.rs"
STORAGE="$ROOT_DIR/crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row353-direct-array-i64-backend-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row353-direct-array-i64-backend-pilot] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-backend-connection-pilot-v0"
require_line "$DOC" "input_contract=direct-array-i64-backend-connection-selection-v0"
require_line "$DOC" "implemented_backend=direct_array_i64_exact"
require_line "$DOC" "implemented_owner=crates/nyash_kernel/src/plugin/array_slot_backend.rs"
require_line "$DOC" "storage_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"
require_line "$DOC" "direct_array_i64_buffer_allocation_smoke=ok"
require_line "$DOC" "direct_array_i64_buffer_store_load_smoke=ok"
require_line "$DOC" "helper_route_to_direct_backend=fail_fast_closed"
require_line "$DOC" "generic_array_helper_route_to_direct_backend=0"
require_line "$DOC" "i64_slot_helper_route_to_direct_backend=0"
require_line "$DOC" "materialization_bridge_implemented=0"
require_line "$DOC" "bootstrap_bridge_implemented=0"
require_line "$DOC" "existing_array_helper_abi_unchanged=1"
require_line "$DOC" "default_backend_unchanged=1"
require_line "$DOC" "single_thread_exact_backend_unchanged=1"
require_line "$DOC" "new_c_abi_helper_symbols=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "selected_next=direct_array_i64_bootstrap_materialization_policy_selection"
require_line "$DOC" "summary=ok"

require_pattern "$BACKEND" "DirectArrayI64Exact"
require_pattern "$BACKEND" 'Some("direct_array_i64_exact")'
require_pattern "$BACKEND" "DIRECT_ARRAY_I64_BUFFERS"
require_pattern "$BACKEND" "DirectArrayI64BufferV0Box::new"
require_pattern "$BACKEND" "fn direct_array_i64_helper_route_closed"
require_pattern "$BACKEND" "fn direct_array_i64_exact_backend_allocates_storage_without_helper_route"
require_pattern "$STORAGE" "pub(crate) struct DirectArrayI64BufferV0Box"

HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact \
  cargo test -p nyash_kernel direct_array_i64_exact_backend_allocates_storage_without_helper_route --lib -- --nocapture

echo "[row353-direct-array-i64-backend-pilot] ok"

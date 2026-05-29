#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-350-DIRECT-ARRAY-I64-BUFFER-V0-STORAGE-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-349-DIRECT-ARRAY-I64-BUFFER-V0-LAYOUT-SELECTION.md"
SRC="$ROOT_DIR/crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"
MOD="$ROOT_DIR/crates/nyash_kernel/src/plugin/mod.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row350-direct-array-i64-storage-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row350-direct-array-i64-storage-pilot] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-buffer-v0-storage-pilot-v0"
require_line "$DOC" "input_contract=direct-array-i64-buffer-v0-layout-selection-v0"
require_line "$DOC" "implemented_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"
require_line "$DOC" "module_registered=crates/nyash_kernel/src/plugin/mod.rs"
require_line "$DOC" "implemented_layout=DirectArrayI64BufferV0"
require_line "$DOC" "layout_repr=repr_c"
require_line "$DOC" "header_size_bytes=32"
require_line "$DOC" "header_alignment_bytes=8"
require_line "$DOC" "data0_offset_bytes=32"
require_line "$DOC" "element_layout=trailing_i64_slice"
require_line "$DOC" "element_size_bytes=8"
require_line "$DOC" "element_alignment_bytes=8"
require_line "$DOC" "element_tag=i64"
require_line "$DOC" "allocation_stable=1"
require_line "$DOC" "contiguous_i64_store_load_smoke=ok"
require_line "$DOC" "append_at_end_smoke=ok"
require_line "$DOC" "oob_preservation_smoke=ok"
require_line "$DOC" "zero_generation_rejected=1"
require_line "$DOC" "storage_only_dead_code_allowance=1"
require_line "$DOC" "public_arraybox_semantics_unchanged=1"
require_line "$DOC" "default_safe_rwlock_path_unchanged=1"
require_line "$DOC" "existing_array_helper_abi_unchanged=1"
require_line "$DOC" "backend_connection_open=0"
require_line "$DOC" "materialization_policy=deferred_required_before_lowering"
require_line "$DOC" "fallback_sync_policy=deferred_required_before_lowering"
require_line "$DOC" "arraybox_items_rwlock_exposure=0"
require_line "$DOC" "array_slot_cache_vec_exposure=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "selected_next=direct_array_i64_materialization_sync_ssot"
require_line "$DOC" "summary=ok"

require_pattern "$SRC" "pub(crate) struct DirectArrayI64BufferV0"
require_pattern "$SRC" "pub(crate) struct DirectArrayI64BufferV0Box"
require_pattern "$SRC" "pub(crate) fn direct_array_i64_buffer_data_offset"
require_pattern "$SRC" "fn direct_array_i64_buffer_v0_header_and_data_offsets_are_stable"
require_pattern "$SRC" "fn direct_array_i64_buffer_v0_stores_and_loads_contiguous_i64_data"
require_pattern "$SRC" "fn direct_array_i64_buffer_v0_preserves_append_and_oob_policy"
require_pattern "$SRC" "fn direct_array_i64_buffer_v0_rejects_zero_generation"
require_pattern "$SRC" "fn direct_array_i64_buffer_v0_data_pointer_uses_header_offset"
require_pattern "$SRC" "#![allow(dead_code)]"
require_pattern "$MOD" "mod array_direct_i64_buffer;"

cargo test -p nyash_kernel direct_array_i64_buffer --lib -- --nocapture

echo "[row350-direct-array-i64-storage-pilot] ok"

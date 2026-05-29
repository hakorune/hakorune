#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-349-DIRECT-ARRAY-I64-BUFFER-V0-LAYOUT-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-348-ARRAY-SLOT-NATIVEDIRECT-GUARD-SURFACE.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row349-direct-array-i64-layout-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row349-direct-array-i64-layout-selection] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-array-i64-buffer-v0-layout-selection-v0"
require_line "$DOC" "input_contract=array-slot-nativedirect-guard-surface-v0"
require_line "$DOC" "selected_layout=DirectArrayI64BufferV0"
require_line "$DOC" "selected_owner=array_slot_nativedirect_storage_layout"
require_line "$DOC" "layout_repr=repr_c"
require_line "$DOC" "header_kind=u32"
require_line "$DOC" "header_flags=u32"
require_line "$DOC" "header_generation=u32"
require_line "$DOC" "header_element_tag=u32"
require_line "$DOC" "header_len=u64"
require_line "$DOC" "header_capacity=u64"
require_line "$DOC" "header_size_bytes=32"
require_line "$DOC" "header_alignment_bytes=8"
require_line "$DOC" "element_layout=trailing_i64_slice"
require_line "$DOC" "element_size_bytes=8"
require_line "$DOC" "element_alignment_bytes=8"
require_line "$DOC" "data0_offset_bytes=32"
require_line "$DOC" "element_tag=i64"
require_line "$DOC" "mixed_storage_supported=0"
require_line "$DOC" "boxed_storage_supported=0"
require_line "$DOC" "string_storage_supported=0"
require_line "$DOC" "bool_f64_storage_supported=0"
require_line "$DOC" "per_element_tag_supported=0"
require_line "$DOC" "direct_slot_cell_reuse=0"
require_line "$DOC" "public_arraybox_semantics_unchanged=1"
require_line "$DOC" "default_safe_rwlock_path_unchanged=1"
require_line "$DOC" "arraybox_items_rwlock_exposure=0"
require_line "$DOC" "array_slot_cache_vec_exposure=0"
require_line "$DOC" "plugin_runtime_helper_boundary_owner=fallback_materialization_debug"
require_line "$DOC" "storage_pilot_open_next=1"
require_line "$DOC" "materialization_policy=deferred_required_before_lowering"
require_line "$DOC" "fallback_sync_policy=deferred_required_before_lowering"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "selected_next=direct_array_i64_buffer_v0_storage_pilot"
require_line "$DOC" "summary=ok"

require_pattern "$DOC" "DirectArrayI64BufferV0:"
require_pattern "$DOC" "data: trailing [i64; capacity]"
require_pattern "$DOC" 'This keeps the Array NativeDirect path close to a C-style contiguous `i64`'
require_pattern "$DOC" '`ArrayBox` remains the public runtime/plugin semantic owner.'

cat <<REPORT_TEXT
output_contract=direct-array-i64-buffer-v0-layout-selection-v0
input_contract=array-slot-nativedirect-guard-surface-v0
selected_layout=DirectArrayI64BufferV0
header_size_bytes=32
data0_offset_bytes=32
element_tag=i64
implementation_open=0
llvm_lowering_open=0
native_direct_open=0
direct_load_store_open=0
summary=ok
REPORT_TEXT

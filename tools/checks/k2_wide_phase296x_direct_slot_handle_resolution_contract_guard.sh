#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-322-DIRECT-SLOT-HANDLE-RESOLUTION-CONTRACT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-321-DIRECT-SLOT-CELL-STORAGE-PILOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row322-direct-slot-handle-resolution-contract] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-handle-resolution-contract-v0"
require_line "$DOC" "input_contract=direct-slot-cell-storage-pilot-v0"
require_line "$DOC" "selected_owner=direct_slot_handle_resolution_contract"
require_line "$DOC" "selected_handle_kind=tagged_stable_object_pointer"
require_line "$DOC" "object_layout=DirectSlotObjectV0"
require_line "$DOC" "object_layout_repr=repr_c"
require_line "$DOC" "object_header_type_id=i64"
require_line "$DOC" "object_header_generation=u32"
require_line "$DOC" "object_header_field_count=u32"
require_line "$DOC" "object_fields_layout=trailing_direct_slot_cell_v0_slice"
require_line "$DOC" "handle_payload=stable_object_pointer"
require_line "$DOC" "handle_tag_bits_required=1"
require_line "$DOC" "handle_alignment_required=8"
require_line "$DOC" "handle_resolution_in_llvm_allowed_after_layout_pilot=1"
require_line "$DOC" "handle_points_to_vec_storage=0"
require_line "$DOC" "handle_points_to_refcell_storage=0"
require_line "$DOC" "handle_points_to_rust_enum_typed_slot=0"
require_line "$DOC" "raw_runtime_vec_pointer_exposure=0"
require_line "$DOC" "c_abi_load_writeback_helper_count=0"
require_line "$DOC" "existing_helper_abi_unchanged=1"
require_line "$DOC" "default_backend_direct_handle_emission=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "selected_next=direct_slot_object_layout_pilot"
require_line "$DOC" "summary=ok"

echo "[row322-direct-slot-handle-resolution-contract] ok"

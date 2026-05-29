#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-320-DIRECT-SLOT-CELL-STORAGE-LAYOUT-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-319-DIRECT-SLOT-LEASE-ADDRESSABLE-SLOT-BRIDGE-SSOT.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row320-direct-slot-cell-storage-layout-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-cell-storage-layout-selection-v0"
require_line "$DOC" "input_contract=direct-slot-lease-addressable-slot-bridge-ssot-v0"
require_line "$DOC" "selected_owner=typed_object_direct_slot_cell_storage_layout"
require_line "$DOC" "selected_layout=DirectSlotCellV0"
require_line "$DOC" "cell_repr=repr_c"
require_line "$DOC" "cell_storage_tag_type=u32"
require_line "$DOC" "cell_flags_type=u32"
require_line "$DOC" "cell_payload_type=u64"
require_line "$DOC" "cell_size_bytes=16"
require_line "$DOC" "cell_alignment_bytes=8"
require_line "$DOC" "cell_payload_i64_encoding=two_complement_bits"
require_line "$DOC" "cell_payload_u64_encoding=raw_u64_bits"
require_line "$DOC" "cell_payload_handle_encoding=i64_bits"
require_line "$DOC" "storage_tag_i64=1"
require_line "$DOC" "storage_tag_u64=2"
require_line "$DOC" "storage_tag_handle=3"
require_line "$DOC" "unsupported_storage_policy=no_direct_cell_plan"
require_line "$DOC" "object_header_repr=repr_c"
require_line "$DOC" "fields_storage=pinned_boxed_slice"
require_line "$DOC" "handle_resolution_contract=separate_next_row"
require_line "$DOC" "llvm_consumable_slot_address_open=0"
require_line "$DOC" "runtime_typed_slot_enum_layout_exposure=0"
require_line "$DOC" "raw_runtime_vec_pointer_exposure=0"
require_line "$DOC" "thread_local_refcell_pointer_exposure=0"
require_line "$DOC" "lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "selected_next=direct_slot_cell_storage_pilot"
require_line "$DOC" "summary=ok"

echo "[row320-direct-slot-cell-storage-layout-selection] ok"

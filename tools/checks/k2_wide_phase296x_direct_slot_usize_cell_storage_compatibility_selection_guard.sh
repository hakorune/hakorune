#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-336-DIRECT-SLOT-USIZE-CELL-STORAGE-COMPATIBILITY-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-335-DIRECT-SLOT-NATIVEDIRECT-LOWERING-OWNER-SELECTION.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row336-direct-slot-usize-cell-storage-compatibility-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-usize-cell-storage-compatibility-selection-v0"
require_line "$DOC" "input_contract=direct-slot-nativedirect-lowering-owner-selection-v0"
require_line "$DOC" "selected_owner=direct_slot_cell_v0_usize_storage_tag"
require_line "$DOC" "selected_owner_file=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs"
require_line "$DOC" "selected_storage_tag=DirectSlotCellV0::USize"
require_line "$DOC" "selected_storage_tag_value=4"
require_line "$DOC" "cell_layout_size_bytes_unchanged=16"
require_line "$DOC" "cell_layout_alignment_bytes_unchanged=8"
require_line "$DOC" "target_pointer_width_required=64"
require_line "$DOC" "usize_payload_representation=u64_payload"
require_line "$DOC" "usize_materialization_storage=TypedSlotStorage::USize"
require_line "$DOC" "u64_lease_storage_accepts_usize=1"
require_line "$DOC" "typed_slot_enum_layout_exposure=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "selected_next=direct_slot_usize_cell_storage_compatibility_pilot"
require_line "$DOC" "summary=ok"

echo "[row336-direct-slot-usize-cell-storage-compatibility-selection] ok"

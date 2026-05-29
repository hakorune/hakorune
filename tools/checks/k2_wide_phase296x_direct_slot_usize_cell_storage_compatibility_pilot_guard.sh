#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-337-DIRECT-SLOT-USIZE-CELL-STORAGE-COMPATIBILITY-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-336-DIRECT-SLOT-USIZE-CELL-STORAGE-COMPATIBILITY-SELECTION.md"
SRC="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row337-direct-slot-usize-cell-storage-compatibility-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row337-direct-slot-usize-cell-storage-compatibility-pilot] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-usize-cell-storage-compatibility-pilot-v0"
require_line "$DOC" "input_contract=direct-slot-usize-cell-storage-compatibility-selection-v0"
require_line "$DOC" "implemented_owner=direct_slot_cell_v0_usize_storage_tag"
require_line "$DOC" "implemented_owner_file=crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs"
require_line "$DOC" "implemented_storage_tag=DirectSlotCellV0::USize"
require_line "$DOC" "implemented_storage_tag_value=4"
require_line "$DOC" "usize_materialization_storage=TypedSlotStorage::USize"
require_line "$DOC" "u64_lease_storage_accepts_usize=1"
require_line "$DOC" "direct_slot_lease_usize_read_write_smoke=ok"
require_line "$DOC" "direct_slot_cell_usize_tag_smoke=ok"
require_line "$DOC" "direct_slot_object_usize_snapshot_smoke=ok"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "selected_next=direct_slot_nativedirect_lowering_selected_method_pilot"
require_line "$DOC" "summary=ok"

require_pattern "$SRC" "const DIRECT_SLOT_TAG_USIZE: u32 = 4;"
require_pattern "$SRC" "(TypedSlotStorage::USize, DirectSlotLeaseStorage::U64)"
require_pattern "$SRC" "storage: TypedSlotStorage::USize"
require_pattern "$SRC" "fn direct_u64_storage_supported"
require_pattern "$SRC" "fn direct_slot_object_snapshot_preserves_usize_storage"

cargo test -p nyash_kernel direct_slot_lease_token_reads_and_writes_supported_storage -- --nocapture
cargo test -p nyash_kernel direct_slot_cells_preserve_tagged_payloads -- --nocapture
cargo test -p nyash_kernel direct_slot_object_snapshot_preserves_usize_storage -- --nocapture

echo "[row337-direct-slot-usize-cell-storage-compatibility-pilot] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-321-DIRECT-SLOT-CELL-STORAGE-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-320-DIRECT-SLOT-CELL-STORAGE-LAYOUT-SELECTION.md"
SRC="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row321-direct-slot-cell-storage-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row321-direct-slot-cell-storage-pilot] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-cell-storage-pilot-v0"
require_line "$DOC" "input_contract=direct-slot-cell-storage-layout-selection-v0"
require_line "$DOC" "implemented_layout=DirectSlotCellV0"
require_line "$DOC" "cell_size_bytes=16"
require_line "$DOC" "cell_alignment_bytes=8"
require_line "$DOC" "direct_cell_parallel_storage=1"
require_line "$DOC" "typed_slot_fallback_storage_preserved=1"
require_line "$DOC" "direct_slot_lease_token_reads_cells=1"
require_line "$DOC" "direct_slot_lease_token_updates_fallback_field_on_write=1"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "new_c_abi_helper_symbols=0"
require_line "$DOC" "summary=ok"

require_pattern "$SRC" "#[repr(C)]"
require_pattern "$SRC" "pub(crate) struct DirectSlotCellV0"
require_pattern "$SRC" "storage_tag: u32"
require_pattern "$SRC" "flags: u32"
require_pattern "$SRC" "payload: u64"
require_pattern "$SRC" "direct_cells: Box<[DirectSlotCellV0]>"
require_pattern "$SRC" "fn direct_slot_cell_v0_layout_is_stable"

cargo test -p nyash_kernel direct_slot_cell -- --nocapture
cargo test -p nyash_kernel direct_slot_lease_token_reads_and_writes_supported_storage -- --nocapture

echo "[row321-direct-slot-cell-storage-pilot] ok"

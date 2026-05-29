#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-323-DIRECT-SLOT-OBJECT-LAYOUT-PILOT.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-322-DIRECT-SLOT-HANDLE-RESOLUTION-CONTRACT.md"
SRC="$ROOT_DIR/crates/nyash_kernel/src/exports/typed_object_pinned_arena.rs"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row323-direct-slot-object-layout-pilot] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row323-direct-slot-object-layout-pilot] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-object-layout-pilot-v0"
require_line "$DOC" "input_contract=direct-slot-handle-resolution-contract-v0"
require_line "$DOC" "implemented_layout=DirectSlotObjectV0"
require_line "$DOC" "object_repr=repr_c"
require_line "$DOC" "object_header_size_bytes=24"
require_line "$DOC" "object_alignment_bytes=8"
require_line "$DOC" "fields_layout=trailing_direct_slot_cell_v0_slice"
require_line "$DOC" "cell_size_bytes=16"
require_line "$DOC" "field0_offset_bytes=24"
require_line "$DOC" "field_address_calculation_smoke=ok"
require_line "$DOC" "handle_payload=tagged_stable_object_pointer"
require_line "$DOC" "handle_roundtrip_smoke=ok"
require_line "$DOC" "direct_cell_primary_storage_policy=selected_for_direct_backend"
require_line "$DOC" "typed_slot_fallback_view_policy=preserved_for_current_helpers"
require_line "$DOC" "materialization_policy=deferred_required_before_lowering"
require_line "$DOC" "fallback_sync_policy=deferred_required_before_lowering"
require_line "$DOC" "typed_slot_enum_layout_exposure=0"
require_line "$DOC" "raw_runtime_vec_pointer_exposure=0"
require_line "$DOC" "thread_local_refcell_pointer_exposure=0"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "native_direct_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "selected_next=direct_slot_materialization_fallback_sync_ssot"
require_line "$DOC" "summary=ok"

require_pattern "$SRC" "pub(crate) struct DirectSlotObjectV0"
require_pattern "$SRC" "pub(crate) struct DirectSlotObjectV0Box"
require_pattern "$SRC" "fn direct_slot_object_field_offset"
require_pattern "$SRC" "fn encode_direct_slot_object_handle"
require_pattern "$SRC" "fn direct_slot_object_v0_header_and_field_offsets_are_stable"
require_pattern "$SRC" "fn direct_slot_object_handle_roundtrips_stable_pointer"

cargo test -p nyash_kernel direct_slot_object -- --nocapture

echo "[row323-direct-slot-object-layout-pilot] ok"

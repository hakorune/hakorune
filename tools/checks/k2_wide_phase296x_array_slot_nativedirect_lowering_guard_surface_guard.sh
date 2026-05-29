#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-360-ARRAY-SLOT-NATIVEDIRECT-LOWERING-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-359-ARRAY-SLOT-NATIVEDIRECT-LOWERING-READINESS-INVENTORY.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row360-array-slot-nativedirect-lowering-guard-surface] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=array-slot-nativedirect-lowering-guard-surface-v0"
require_line "$DOC" "input_contract=array-slot-nativedirect-lowering-readiness-inventory-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_backend=direct_array_i64_exact"
require_line "$DOC" "selected_representation=NativeDirect"
require_line "$DOC" "selected_storage_substrate=DirectArrayI64BufferV0"
require_line "$DOC" "selected_buffer_layout=repr_c_header_trailing_i64"
require_line "$DOC" "selected_lowering_owner=src/llvm_py/instructions/mir_call/collection_method_call.py"
require_line "$DOC" "runtime_layout_owner=crates/nyash_kernel/src/plugin/array_direct_i64_buffer.rs"
require_line "$DOC" "selected_method_only=1"
require_line "$DOC" "default_backend_emission=0"
require_line "$DOC" "direct_array_buffer_required=1"
require_line "$DOC" "receiver_array_exact_required=1"
require_line "$DOC" "index_i64_required=1"
require_line "$DOC" "element_storage_i64_required=1"
require_line "$DOC" "same_block_get_set_pair_required=1"
require_line "$DOC" "set_uses_get_result_required=1"
require_line "$DOC" "field_address_formula=buffer_base_plus_header_offset_plus_index_times_8"
require_line "$DOC" "fallback_boundary=explicit_public_arraybox_snapshot_handle"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "helper_load_writeback_substitution_allowed=0"
require_line "$DOC" "arraybox_items_rwlock_exposure=0"
require_line "$DOC" "array_slot_cache_vec_exposure=0"
require_line "$DOC" "direct_array_helper_route_reuse_allowed=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "planned_net_helper_delta=2"
require_line "$DOC" "planned_net_helper_delta_positive=1"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "selected_next=array_slot_nativedirect_lowering_owner_selection"
require_line "$DOC" "summary=ok"

echo "[row360-array-slot-nativedirect-lowering-guard-surface] ok"

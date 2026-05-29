#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-334-DIRECT-SLOT-NATIVEDIRECT-LOWERING-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-333-DIRECT-SLOT-NATIVEDIRECT-LOWERING-READINESS-INVENTORY.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row334-direct-slot-nativedirect-lowering-guard-surface] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-nativedirect-lowering-guard-surface-v0"
require_line "$DOC" "input_contract=direct-slot-nativedirect-lowering-readiness-inventory-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_backend=direct_slot_exact"
require_line "$DOC" "selected_representation=NativeDirect"
require_line "$DOC" "selected_storage_substrate=DirectSlotObjectV0"
require_line "$DOC" "selected_cell_layout=DirectSlotCellV0"
require_line "$DOC" "selected_lowering_owner=src/llvm_py/instructions/field_access.py"
require_line "$DOC" "selected_method_only=1"
require_line "$DOC" "default_backend_emission=0"
require_line "$DOC" "direct_handle_required=1"
require_line "$DOC" "slot_constant_required=1"
require_line "$DOC" "storage_tag_known_required=1"
require_line "$DOC" "field_address_formula=object_base_plus_header_offset_plus_slot_times_16"
require_line "$DOC" "fallback_boundary=explicit_materialized_view_handle"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "helper_load_writeback_substitution_allowed=0"
require_line "$DOC" "typed_slot_enum_layout_exposure=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "planned_net_helper_delta=21"
require_line "$DOC" "planned_net_helper_delta_positive=1"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "selected_next=direct_slot_nativedirect_lowering_owner_selection"
require_line "$DOC" "summary=ok"

echo "[row334-direct-slot-nativedirect-lowering-guard-surface] ok"

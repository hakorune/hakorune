#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-333-DIRECT-SLOT-NATIVEDIRECT-LOWERING-READINESS-INVENTORY.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-332-DIRECT-SLOT-HELPER-FALLBACK-CLOSEOUT-AND-LOWERING-READINESS-SELECTION.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row333-direct-slot-nativedirect-lowering-readiness-inventory] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=direct-slot-nativedirect-lowering-readiness-inventory-v0"
require_line "$DOC" "input_contract=direct-slot-helper-fallback-closeout-and-lowering-readiness-selection-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "candidate_representation=NativeDirect"
require_line "$DOC" "storage_substrate=DirectSlotObjectV0"
require_line "$DOC" "direct_cell_layout=DirectSlotCellV0"
require_line "$DOC" "fallback_boundary=explicit_materialized_view_handle"
require_line "$DOC" "prior_resident_scalar_net_helper_call_delta=0"
require_line "$DOC" "candidate_exact_slot_helper_count=21"
require_line "$DOC" "planned_net_helper_delta=21"
require_line "$DOC" "planned_net_helper_delta_positive=1"
require_line "$DOC" "direct_handle_available=1"
require_line "$DOC" "slot_address_calculation_available=1"
require_line "$DOC" "materialized_view_boundary_available=1"
require_line "$DOC" "helper_free_bridge_available=1"
require_line "$DOC" "fallback_materialization_boundary_known=1"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "selected_next=direct_slot_nativedirect_lowering_guard_surface"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "summary=ok"

echo "[row333-direct-slot-nativedirect-lowering-readiness-inventory] ok"

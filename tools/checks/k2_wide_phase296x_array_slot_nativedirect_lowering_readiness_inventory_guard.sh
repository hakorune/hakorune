#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-359-ARRAY-SLOT-NATIVEDIRECT-LOWERING-READINESS-INVENTORY.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-358-DIRECT-ARRAY-I64-HELPER-FALLBACK-CLOSEOUT-AND-LOWERING-READINESS-SELECTION.md"
TOOL="$ROOT_DIR/tools/allocator/array_slot_nativedirect_lowering_readiness_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row359_array_nativedirect_readiness.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row359-array-slot-nativedirect-readiness] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=array-slot-nativedirect-lowering-readiness-inventory-v0"
require_line "$DOC" "input_contract=direct-array-i64-helper-fallback-closeout-and-lowering-readiness-selection-v0"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "candidate_representation=NativeDirect"
require_line "$DOC" "storage_substrate=DirectArrayI64BufferV0"
require_line "$DOC" "fallback_boundary=explicit_public_arraybox_snapshot_handle"
require_line "$DOC" "candidate_array_helper_count=2"
require_line "$DOC" "planned_net_helper_delta=2"
require_line "$DOC" "planned_net_helper_delta_positive=1"
require_line "$DOC" "direct_array_buffer_available=1"
require_line "$DOC" "helper_free_bridge_available=1"
require_line "$DOC" "index_and_bounds_facts_available=1"
require_line "$DOC" "fallback_materialization_boundary_known=1"
require_line "$DOC" "silent_fallback_allowed=0"
require_line "$DOC" "selected_next=array_slot_nativedirect_lowering_guard_surface"
require_line "$DOC" "llvm_lowering_open=0"
require_line "$DOC" "direct_load_store_open=0"
require_line "$DOC" "summary=ok"

"$TOOL" --method "HakoAllocPageModel.acquire_usize/1" --out "$REPORT"

require_line "$REPORT" "output_contract=array-slot-nativedirect-lowering-readiness-inventory-v0"
require_line "$REPORT" "input_contract=direct-array-i64-helper-fallback-closeout-and-lowering-readiness-selection-v0"
require_line "$REPORT" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "candidate_representation=NativeDirect"
require_line "$REPORT" "storage_substrate=DirectArrayI64BufferV0"
require_line "$REPORT" "candidate_array_helper_count=2"
require_line "$REPORT" "same_block_get_set_pair=1"
require_line "$REPORT" "set_uses_get_result=1"
require_line "$REPORT" "planned_net_helper_delta=2"
require_line "$REPORT" "planned_net_helper_delta_positive=1"
require_line "$REPORT" "direct_array_buffer_available=1"
require_line "$REPORT" "contiguous_i64_data_available=1"
require_line "$REPORT" "materialized_view_boundary_available=1"
require_line "$REPORT" "helper_free_bridge_available=1"
require_line "$REPORT" "index_and_bounds_facts_available=1"
require_line "$REPORT" "fallback_materialization_boundary_known=1"
require_line "$REPORT" "silent_fallback_allowed=0"
require_line "$REPORT" "selected_next=array_slot_nativedirect_lowering_guard_surface"
require_line "$REPORT" "llvm_lowering_open=0"
require_line "$REPORT" "direct_load_store_open=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

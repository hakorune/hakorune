#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-209-MIR-ARRAY-SLOT-RESIDENCE-SELECTED-METHOD-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-208-MIR-ARRAY-SLOT-RESIDENCE-INVENTORY.md"
TOOL="$ROOT_DIR/tools/allocator/mir_array_slot_residence_selected_method_guard_surface.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row209_array_guard_surface.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row209-array-guard-surface] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Current"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "mir_array_slot_residence_selected_method_guard_surface=accepted"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "implementation_surface_supported=1"
require_line "$DOC" "implementation_owner=selected_method_same_block_array_get_set_direct_slot_op"
require_line "$DOC" "generic_array_residence_open=0"
require_line "$DOC" "by_name_hako_alloc_special_case=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

"$TOOL" --out "$REPORT"

require_line "$REPORT" "output_contract=mir-array-slot-residence-selected-method-guard-surface-v0"
require_line "$REPORT" "input_contract=mir-array-slot-residence-inventory-v0"
require_line "$REPORT" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "selected_reason=explicit_hot_context"
require_line "$REPORT" "array_get_call_count=1"
require_line "$REPORT" "array_set_call_count=1"
require_line "$REPORT" "same_block_get_set_pair=1"
require_line "$REPORT" "set_uses_get_result=1"
require_line "$REPORT" "planned_transform_kind=selected_method_same_block_array_get_set_direct_slot_op"
require_line "$REPORT" "planned_erased_get_set_helper_calls=2"
require_line "$REPORT" "planned_added_guard_helper_calls=1"
require_line "$REPORT" "planned_added_writeback_helper_calls=0"
require_line "$REPORT" "planned_net_helper_call_delta=1"
require_line "$REPORT" "implementation_surface_supported=1"
require_line "$REPORT" "generic_array_residence_open=0"
require_line "$REPORT" "by_name_hako_alloc_special_case=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

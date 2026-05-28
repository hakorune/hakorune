#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-208-MIR-ARRAY-SLOT-RESIDENCE-INVENTORY.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-207-MIR-ARRAY-SLOT-RESIDENCE-SSOT.md"
TOOL="$ROOT_DIR/tools/allocator/mir_array_slot_residence_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row208_array_residence.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row208-mir-array-slot-inventory] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Current"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "mir_array_slot_residence_inventory=accepted"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_method_reason=dynamic_hot_context_object_lifecycle_small_alloc"
require_line "$DOC" "positive_net_helper_call_delta=1"
require_line "$DOC" "transform_open=0"
require_line "$DOC" "array_helper_abi_fallback=1"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

"$TOOL" --method "HakoAllocPageModel.acquire_usize/1" --out "$REPORT"

require_line "$REPORT" "output_contract=mir-array-slot-residence-inventory-v0"
require_line "$REPORT" "input_kind=mir_json"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "selected_reason=explicit_hot_context"
require_line "$REPORT" "eligible_array_get_count=1"
require_line "$REPORT" "eligible_array_set_count=1"
require_line "$REPORT" "erased_get_set_helper_calls=2"
require_line "$REPORT" "added_guard_helper_calls=1"
require_line "$REPORT" "added_writeback_helper_calls=0"
require_line "$REPORT" "net_helper_call_delta=1"
require_line "$REPORT" "positive_net_helper_call_delta=1"
require_line "$REPORT" "transform_open=0"
require_line "$REPORT" "array_helper_abi_fallback=1"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

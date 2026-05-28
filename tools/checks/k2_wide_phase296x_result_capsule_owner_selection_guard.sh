#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-256-RESULT-CAPSULE-OWNER-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-255-RESULT-CAPSULE-IR-SHAPE-DIFF-INVENTORY.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row256-result-capsule-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=result-capsule-owner-selection-v0"
require_line "$DOC" "input_contract=result-capsule-ir-shape-diff-inventory-v0"
require_line "$DOC" "selected_owner=result_capsule_reset_field_batching_guard_surface"
require_line "$DOC" "selected_owner_kind=runtime_exact_slot_batch_helper"
require_line "$DOC" "selected_methods=HakoAllocObjectLifecycleAllocResult.reset/0,HakoAllocObjectLifecycleReleaseResult.reset/0"
require_line "$DOC" "alloc_reset_field_set_count=4"
require_line "$DOC" "release_reset_field_set_count=4"
require_line "$DOC" "planned_erased_exact_slot_set_count=8"
require_line "$DOC" "planned_added_batch_helper_count=2"
require_line "$DOC" "planned_net_helper_call_delta=6"
require_line "$DOC" "requires_new_runtime_symbols=1"
require_line "$DOC" "requires_c_abi_same_module_emit=1"
require_line "$DOC" "requires_hako_source_change=0"
require_line "$DOC" "rejected_owner=birth_batching"
require_line "$DOC" "rejected_owner_1=record_success_fusion"
require_line "$DOC" "rejected_owner_2=record_request_batching"
require_line "$DOC" "rejected_owner_3=capsule_flattening"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat <<REPORT
output_contract=result-capsule-owner-selection-v0
selected_owner=result_capsule_reset_field_batching_guard_surface
planned_net_helper_call_delta=6
next_row=result_capsule_reset_field_batching_guard_surface
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

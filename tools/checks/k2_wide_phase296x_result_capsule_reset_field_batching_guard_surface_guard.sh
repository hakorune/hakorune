#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-257-RESULT-CAPSULE-RESET-FIELD-BATCHING-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-256-RESULT-CAPSULE-OWNER-SELECTION.md"
SOURCE="$ROOT_DIR/lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row257-result-capsule-reset-guard] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_source_line() {
  local expected="$1"
  if ! grep -q "$expected" "$SOURCE"; then
    echo "[row257-result-capsule-reset-guard] missing source pattern: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=result-capsule-reset-field-batching-guard-surface-v0"
require_line "$DOC" "input_contract=result-capsule-owner-selection-v0"
require_line "$DOC" "selected_owner=result_capsule_reset_field_batching"
require_line "$DOC" "selected_owner_kind=runtime_exact_slot_batch_helper"
require_line "$DOC" "target_method_count=2"
require_line "$DOC" "target_method_0=HakoAllocObjectLifecycleAllocResult.reset/0"
require_line "$DOC" "target_method_1=HakoAllocObjectLifecycleReleaseResult.reset/0"
require_line "$DOC" "target_field_count_per_method=4"
require_line "$DOC" "target_slot_0=0"
require_line "$DOC" "target_value_0=-1"
require_line "$DOC" "target_slot_1=1"
require_line "$DOC" "target_value_1=-1"
require_line "$DOC" "target_slot_2=2"
require_line "$DOC" "target_value_2=0"
require_line "$DOC" "target_slot_3=3"
require_line "$DOC" "target_value_3=0"
require_line "$DOC" "new_runtime_helper_symbol=nyash.object.exact_slot_set4_i64_hiiiii"
require_line "$DOC" "new_runtime_helper_contract=handle_start_slot_value0_value1_value2_value3"
require_line "$DOC" "helper_sets_consecutive_i64_slots=1"
require_line "$DOC" "helper_start_slot=0"
require_line "$DOC" "planned_erased_exact_slot_set_count=8"
require_line "$DOC" "planned_added_batch_helper_count=2"
require_line "$DOC" "planned_net_helper_call_delta=6"
require_line "$DOC" "planned_net_helper_call_delta_positive=1"
require_line "$DOC" "requires_c_abi_same_module_emit=1"
require_line "$DOC" "requires_new_runtime_symbol=1"
require_line "$DOC" "requires_hako_source_change=0"
require_line "$DOC" "generic_typed_field_residence_open=0"
require_line "$DOC" "generic_cse_open=0"
require_line "$DOC" "capsule_flattening_open=0"
require_line "$DOC" "source_rewrite=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

require_source_line "box HakoAllocObjectLifecycleAllocResult"
require_source_line "box HakoAllocObjectLifecycleReleaseResult"
require_source_line "me.last_page_id = -1"
require_source_line "me.last_block_id = -1"
require_source_line "me.last_reason = HakoAllocObjectLifecycleFacadeReason.ok()"
require_source_line "me.last_ok = 0"

cat <<REPORT
output_contract=result-capsule-reset-field-batching-guard-surface-v0
selected_owner=result_capsule_reset_field_batching
new_runtime_helper_symbol=nyash.object.exact_slot_set4_i64_hiiiii
planned_net_helper_call_delta=6
next_row=result_capsule_reset_field_batching_implementation
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

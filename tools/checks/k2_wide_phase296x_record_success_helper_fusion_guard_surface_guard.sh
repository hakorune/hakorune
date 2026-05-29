#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-281-RECORD-SUCCESS-HELPER-FUSION-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-280-CAPSULE-VALUE-RESULT-CALLER-REGION-INVENTORY.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row281-record-success-helper-fusion] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=record-success-helper-fusion-guard-surface-v0"
require_line "$DOC" "input_contract=capsule-value-result-caller-region-inventory-v0"
require_line "$DOC" "selected_owner=record_success_helper_fusion"
require_line "$DOC" "selected_owner_kind=runtime_exact_slot_record_success_helper"
require_line "$DOC" "value_aggregate_rejected=1"
require_line "$DOC" "target_method_count=2"
require_line "$DOC" "target_method_0=HakoAllocObjectLifecycleAllocResult.recordSuccess/1"
require_line "$DOC" "target_method_0_shape=branch_aware_selected_kind"
require_line "$DOC" "target_method_0_runtime_helper=nyash.object.exact_slot_record_alloc_success_hii"
require_line "$DOC" "target_method_0_helper_contract=handle_selected_kind"
require_line "$DOC" "target_method_1=HakoAllocObjectLifecycleReleaseResult.recordSuccess/2"
require_line "$DOC" "target_method_1_shape=straightline_page_block"
require_line "$DOC" "target_method_1_runtime_helper=nyash.object.exact_slot_record_release_success_hiii"
require_line "$DOC" "target_method_1_helper_contract=handle_page_id_block_id"
require_line "$DOC" "planned_erased_exact_slot_get_set_count=14"
require_line "$DOC" "planned_added_record_success_helper_count=2"
require_line "$DOC" "planned_net_helper_call_delta=12"
require_line "$DOC" "planned_net_helper_call_delta_positive=1"
require_line "$DOC" "requires_new_runtime_symbols=1"
require_line "$DOC" "requires_c_abi_same_module_emit=1"
require_line "$DOC" "requires_hako_source_change=0"
require_line "$DOC" "generic_typed_field_residence_open=0"
require_line "$DOC" "generic_cse_open=0"
require_line "$DOC" "capsule_value_aggregate_open=0"
require_line "$DOC" "source_rewrite=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat <<REPORT
output_contract=record-success-helper-fusion-guard-surface-v0
selected_owner=record_success_helper_fusion
planned_net_helper_call_delta=12
selected_next=record_success_helper_fusion_implementation
summary=ok
REPORT

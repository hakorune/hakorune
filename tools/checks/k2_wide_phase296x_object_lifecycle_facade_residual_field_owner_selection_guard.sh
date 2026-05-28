#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-236-OBJECT-LIFECYCLE-FACADE-RESIDUAL-FIELD-OWNER-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-235-OBJECT-LIFECYCLE-FACADE-RESIDUAL-EXACT-SLOT-FIELD-INVENTORY.md"
TOOL="$ROOT_DIR/tools/allocator/object_lifecycle_facade_residual_field_owner_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row236_facade_residual_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

INV="$TMP_DIR/inventory.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row236-facade-residual-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=object-lifecycle-facade-residual-field-owner-selection-v0"
require_line "$DOC" "input_contract=object-lifecycle-facade-residual-exact-slot-field-inventory-v0"
require_line "$DOC" "dominant_field_family=facade_receiver_state"
require_line "$DOC" "facade_receiver_state_count=16"
require_line "$DOC" "page_queue_bridge_count=9"
require_line "$DOC" "positive_net_cache_candidate_count=4"
require_line "$DOC" "selected_owner=page_queue_exact_slot_field_inventory"
require_line "$DOC" "next_diagnostic=page_queue_exact_slot_field_inventory"
require_line "$DOC" "rejected_owner=residual_facade_same_block_get_set_retry"
require_line "$DOC" "rejected_owner_1=generic_typed_field_residence_retry"
require_line "$DOC" "rejected_owner_2=facade_method_local_scalar_cache"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "summary=ok"

cat >"$INV" <<'REPORT'
output_contract=object-lifecycle-facade-residual-exact-slot-field-inventory-v0
input_contract=post-facade-exact-slot-callsite-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
target_family=object_lifecycle_facade
target_family_pct=17.36
facade_method_count=5
facade_exact_slot_get_count=21
facade_exact_slot_set_count=9
facade_exact_slot_field_op_count=30
top_facade_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
top_facade_method_pct=11.12
dominant_field_family=facade_receiver_state
dominant_field_family_count=16
field_family.facade_receiver_state_count=16
field_family.page_model_bridge_count=1
field_family.page_queue_bridge_count=9
field_family.alloc_result_capsule_count=4
field_family.release_result_capsule_count=0
field_family.temporary_status_result_count=0
field_family.unknown_count=0
pattern.same_block_get_set_count=3
pattern.same_receiver_repeated_get_count=1
pattern.write_only_field_count=6
pattern.read_only_field_count=17
pattern.positive_net_cache_candidate_count=4
selected_next=facade_field_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

python3 "$TOOL" --inventory-report "$INV" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=object-lifecycle-facade-residual-field-owner-selection-v0"
require_line "$REPORT" "input_contract=object-lifecycle-facade-residual-exact-slot-field-inventory-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "dominant_field_family=facade_receiver_state"
require_line "$REPORT" "facade_receiver_state_count=16"
require_line "$REPORT" "page_queue_bridge_count=9"
require_line "$REPORT" "positive_net_cache_candidate_count=4"
require_line "$REPORT" "selected_owner=page_queue_exact_slot_field_inventory"
require_line "$REPORT" "selected_reason=residual_facade_positive_net_surface_not_growing_and_page_queue_is_next_bridge_family"
require_line "$REPORT" "next_diagnostic=page_queue_exact_slot_field_inventory"
require_line "$REPORT" "rejected_owner=residual_facade_same_block_get_set_retry"
require_line "$REPORT" "rejected_owner_1=generic_typed_field_residence_retry"
require_line "$REPORT" "rejected_owner_2=facade_method_local_scalar_cache"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

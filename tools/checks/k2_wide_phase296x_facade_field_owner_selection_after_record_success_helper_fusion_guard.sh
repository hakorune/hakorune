#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-287-FACADE-FIELD-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-286-FACADE-EXACT-SLOT-IR-SHAPE-DIFF-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
TOOL="$ROOT_DIR/tools/allocator/object_lifecycle_facade_field_owner_selection_after_rollback.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row287_facade_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

INVENTORY="$TMP_DIR/inventory.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row287-facade-owner-after-record-success] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=facade-field-owner-selection-after-record-success-helper-fusion-v0"
require_line "$DOC" "input_contract=object-lifecycle-facade-exact-slot-field-inventory-v0"
require_line "$DOC" "dominant_field_family=facade_receiver_state"
require_line "$DOC" "facade_receiver_state_count=14"
require_line "$DOC" "page_queue_bridge_count=8"
require_line "$DOC" "alloc_result_capsule_count=4"
require_line "$DOC" "same_block_get_set_count=3"
require_line "$DOC" "same_receiver_repeated_get_count=1"
require_line "$DOC" "positive_net_cache_candidate_count=4"
require_line "$DOC" "previous_selected_facade_get_set_keeper_landed=1"
require_line "$DOC" "previous_selected_facade_get_set_measurement_row=296x-231"
require_line "$DOC" "selected_owner=post_facade_inventory_owner_refresh"
require_line "$DOC" "next_diagnostic=post_facade_inventory_owner_refresh_after_record_success_helper_fusion"
require_line "$DOC" "rejected_owner=repeat_selected_facade_same_block_get_set_fusion"
require_line "$DOC" "rejected_owner_1=generic_typed_field_residence_retry"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$INVENTORY" <<'REPORT'
output_contract=object-lifecycle-facade-exact-slot-field-inventory-v0
input_contract=weighted-exact-slot-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
target_family=object_lifecycle_facade
target_family_pct=13.47
dominant_field_family=facade_receiver_state
field_family.facade_receiver_state_count=14
field_family.page_queue_bridge_count=8
field_family.alloc_result_capsule_count=4
pattern.same_block_get_set_count=3
pattern.same_receiver_repeated_get_count=1
pattern.positive_net_cache_candidate_count=4
selected_next=facade_field_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

python3 "$TOOL" --context after-record-success-helper-fusion --inventory-report "$INVENTORY" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=facade-field-owner-selection-after-record-success-helper-fusion-v0"
require_line "$REPORT" "input_contract=object-lifecycle-facade-exact-slot-field-inventory-v0"
require_line "$REPORT" "dominant_field_family=facade_receiver_state"
require_line "$REPORT" "facade_receiver_state_count=14"
require_line "$REPORT" "page_queue_bridge_count=8"
require_line "$REPORT" "alloc_result_capsule_count=4"
require_line "$REPORT" "positive_net_cache_candidate_count=4"
require_line "$REPORT" "previous_selected_facade_get_set_keeper_landed=1"
require_line "$REPORT" "selected_owner=post_facade_inventory_owner_refresh"
require_line "$REPORT" "selected_reason=selected_facade_fusion_already_landed_and_positive_net_surface_still_4"
require_line "$REPORT" "next_diagnostic=post_facade_inventory_owner_refresh_after_record_success_helper_fusion"
require_line "$REPORT" "rejected_owner=repeat_selected_facade_same_block_get_set_fusion"
require_line "$REPORT" "rejected_reason=same_block_get_set_candidate_count_3_already_exercised_by_row231_keeper"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

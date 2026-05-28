#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-238-PAGE-QUEUE-FIELD-OWNER-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-237-PAGE-QUEUE-EXACT-SLOT-FIELD-INVENTORY.md"
TOOL="$ROOT_DIR/tools/allocator/page_queue_field_owner_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row238_page_queue_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

INV="$TMP_DIR/inventory.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row238-page-queue-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-queue-field-owner-selection-v0"
require_line "$DOC" "input_contract=page-queue-exact-slot-field-inventory-v0"
require_line "$DOC" "same_block_get_set_count=12"
require_line "$DOC" "same_receiver_repeated_get_count=4"
require_line "$DOC" "positive_net_cache_candidate_count=16"
require_line "$DOC" "selected_owner=selected_page_queue_same_block_get_set_fusion"
require_line "$DOC" "planned_net_helper_call_delta=12"
require_line "$DOC" "planned_net_helper_call_delta_positive=1"
require_line "$DOC" "rejected_owner=page_queue_method_local_scalar_cache"
require_line "$DOC" "rejected_owner_1=generic_typed_field_residence_retry"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "summary=ok"

cat >"$INV" <<'REPORT'
output_contract=page-queue-exact-slot-field-inventory-v0
input_contract=object-lifecycle-facade-residual-field-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
target_family=page_queue_helpers
target_family_pct=13.19
page_queue_method_count=3
page_queue_exact_slot_get_count=15
page_queue_exact_slot_set_count=20
page_queue_exact_slot_field_op_count=35
top_page_queue_method=HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
top_page_queue_method_pct=10.44
dominant_field_family=page_queue_receiver_state
dominant_field_family_count=34
field_family.page_queue_receiver_state_count=34
field_family.page_model_bridge_count=1
field_family.alloc_result_capsule_count=0
field_family.facade_bridge_count=0
field_family.unknown_count=0
pattern.same_block_get_set_count=12
pattern.same_receiver_repeated_get_count=4
pattern.write_only_field_count=8
pattern.read_only_field_count=3
pattern.positive_net_cache_candidate_count=16
selected_next=page_queue_field_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

python3 "$TOOL" --inventory-report "$INV" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=page-queue-field-owner-selection-v0"
require_line "$REPORT" "input_contract=page-queue-exact-slot-field-inventory-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "same_block_get_set_count=12"
require_line "$REPORT" "same_receiver_repeated_get_count=4"
require_line "$REPORT" "positive_net_cache_candidate_count=16"
require_line "$REPORT" "selected_owner=selected_page_queue_same_block_get_set_fusion"
require_line "$REPORT" "selected_reason=same_block_get_set_candidates_dominate_page_queue_positive_net_surface"
require_line "$REPORT" "next_diagnostic=selected_page_queue_same_block_get_set_guard_surface"
require_line "$REPORT" "planned_erased_get_set_helper_calls=24"
require_line "$REPORT" "planned_added_fused_helper_calls=12"
require_line "$REPORT" "planned_net_helper_call_delta=12"
require_line "$REPORT" "planned_net_helper_call_delta_positive=1"
require_line "$REPORT" "rejected_owner=page_queue_method_local_scalar_cache"
require_line "$REPORT" "rejected_owner_1=generic_typed_field_residence_retry"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-237-PAGE-QUEUE-EXACT-SLOT-FIELD-INVENTORY.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-236-OBJECT-LIFECYCLE-FACADE-RESIDUAL-FIELD-OWNER-SELECTION.md"
TOOL="$ROOT_DIR/tools/allocator/page_queue_exact_slot_field_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row237_page_queue_inventory.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

OWNER="$TMP_DIR/owner.out"
PERF="$TMP_DIR/perf.report"
MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row237-page-queue-inventory] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-queue-exact-slot-field-inventory-v0"
require_line "$DOC" "input_contract=object-lifecycle-facade-residual-field-owner-selection-v0"
require_line "$DOC" "target_family=page_queue_helpers"
require_line "$DOC" "target_family_pct=13.19"
require_line "$DOC" "page_queue_method_count=3"
require_line "$DOC" "page_queue_exact_slot_get_count=15"
require_line "$DOC" "page_queue_exact_slot_set_count=20"
require_line "$DOC" "page_queue_exact_slot_field_op_count=35"
require_line "$DOC" "dominant_field_family=page_queue_receiver_state"
require_line "$DOC" "field_family.page_queue_receiver_state_count=34"
require_line "$DOC" "field_family.page_model_bridge_count=1"
require_line "$DOC" "pattern.positive_net_cache_candidate_count=16"
require_line "$DOC" "selected_next=page_queue_field_owner_selection"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "summary=ok"

cat >"$OWNER" <<'REPORT'
output_contract=object-lifecycle-facade-residual-field-owner-selection-v0
input_contract=object-lifecycle-facade-residual-exact-slot-field-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
dominant_field_family=facade_receiver_state
facade_receiver_state_count=16
page_queue_bridge_count=9
positive_net_cache_candidate_count=4
selected_owner=page_queue_exact_slot_field_inventory
selected_reason=residual_facade_positive_net_surface_not_growing_and_page_queue_is_next_bridge_family
next_diagnostic=page_queue_exact_slot_field_inventory
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

cat >"$PERF" <<'REPORT'
    19.44%  app.exe  app.exe               [.] nyash.object.exact_slot_set_i64_hii
               |--3.54%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--2.06%--HakoAllocObjectLifecyclePageQueue.beginSelection/0
    10.38%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
               |--2.77%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
     8.96%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
               |--1.34%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--0.69%--HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
     4.89%  app.exe  app.exe               [.] nyash.object.exact_slot_set_u64_hiu
               |--2.79%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
REPORT

cat >"$MIR" <<'JSON'
{
  "functions": [
    {
      "name": "HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3",
      "metadata": {
        "value_types": {
          "1": {"box_type": "HakoAllocObjectLifecyclePageQueue", "kind": "handle"},
          "2": {"box_type": "HakoAllocPageModel", "kind": "handle"}
        }
      },
      "blocks": [
        {"id": 10, "instructions": [
          {"op": "field_get", "box": 1, "field": "accepted_count"},
          {"op": "field_get", "box": 1, "field": "accepted_count"},
          {"op": "field_set", "box": 1, "field": "accepted_count"},
          {"op": "field_set", "box": 1, "field": "accepted_count"},
          {"op": "field_get", "box": 1, "field": "last_selected_index"},
          {"op": "field_set", "box": 1, "field": "last_selected_index"},
          {"op": "field_get", "box": 1, "field": "last_selected_kind"},
          {"op": "field_set", "box": 1, "field": "last_selected_kind"},
          {"op": "field_set", "box": 1, "field": "last_selected_page"},
          {"op": "field_set", "box": 1, "field": "last_selected_page_id"},
          {"op": "field_get", "box": 2, "field": "page_id"}
        ]}
      ]
    },
    {
      "name": "HakoAllocObjectLifecyclePageQueue.beginSelection/0",
      "metadata": {
        "value_types": {
          "1": {"box_type": "HakoAllocObjectLifecyclePageQueue", "kind": "handle"}
        }
      },
      "blocks": [
        {"id": 20, "instructions": [
          {"op": "field_get", "box": 1, "field": "selection_count"},
          {"op": "field_get", "box": 1, "field": "selection_count"},
          {"op": "field_set", "box": 1, "field": "selection_count"},
          {"op": "field_set", "box": 1, "field": "selection_count"}
        ]}
      ]
    },
    {
      "name": "HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0",
      "metadata": {
        "value_types": {
          "1": {"box_type": "HakoAllocObjectLifecyclePageQueue", "kind": "handle"}
        }
      },
      "blocks": [
        {"id": 30, "instructions": [
          {"op": "field_get", "box": 1, "field": "slot_a"},
          {"op": "field_get", "box": 1, "field": "slot_a"},
          {"op": "field_get", "box": 1, "field": "slot_a"},
          {"op": "field_set", "box": 1, "field": "slot_a"},
          {"op": "field_set", "box": 1, "field": "slot_a"},
          {"op": "field_set", "box": 1, "field": "slot_a"},
          {"op": "field_get", "box": 1, "field": "slot_b"},
          {"op": "field_set", "box": 1, "field": "slot_b"},
          {"op": "field_get", "box": 1, "field": "slot_c"},
          {"op": "field_set", "box": 1, "field": "slot_c"},
          {"op": "field_get", "box": 1, "field": "slot_d"},
          {"op": "field_set", "box": 1, "field": "slot_d"},
          {"op": "field_set", "box": 1, "field": "write_only_0"},
          {"op": "field_set", "box": 1, "field": "write_only_1"},
          {"op": "field_set", "box": 1, "field": "write_only_2"},
          {"op": "field_set", "box": 1, "field": "write_only_3"},
          {"op": "field_set", "box": 1, "field": "write_only_4"},
          {"op": "field_set", "box": 1, "field": "write_only_5"},
          {"op": "field_get", "box": 1, "field": "read_only_0"},
          {"op": "field_get", "box": 1, "field": "read_only_1"}
        ]}
      ]
    }
  ]
}
JSON

python3 "$TOOL" --mir-json "$MIR" --perf-report "$PERF" --owner-selection-report "$OWNER" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=page-queue-exact-slot-field-inventory-v0"
require_line "$REPORT" "input_contract=object-lifecycle-facade-residual-field-owner-selection-v0"
require_line "$REPORT" "target_family=page_queue_helpers"
require_line "$REPORT" "target_family_pct=13.19"
require_line "$REPORT" "page_queue_method_count=3"
require_line "$REPORT" "page_queue_exact_slot_get_count=15"
require_line "$REPORT" "page_queue_exact_slot_set_count=20"
require_line "$REPORT" "page_queue_exact_slot_field_op_count=35"
require_line "$REPORT" "dominant_field_family=page_queue_receiver_state"
require_line "$REPORT" "field_family.page_queue_receiver_state_count=34"
require_line "$REPORT" "field_family.page_model_bridge_count=1"
require_line "$REPORT" "pattern.same_block_get_set_count=12"
require_line "$REPORT" "pattern.same_receiver_repeated_get_count=4"
require_line "$REPORT" "pattern.positive_net_cache_candidate_count=16"
require_line "$REPORT" "selected_next=page_queue_field_owner_selection"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

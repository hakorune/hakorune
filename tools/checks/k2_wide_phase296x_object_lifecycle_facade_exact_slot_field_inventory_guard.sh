#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-227-OBJECT-LIFECYCLE-FACADE-EXACT-SLOT-FIELD-INVENTORY.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-226-TYPED-OBJECT-EXACT-SLOT-CALLSITE-OWNER-SELECTION.md"
TOOL="$ROOT_DIR/tools/allocator/object_lifecycle_facade_exact_slot_field_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row227_facade_inventory.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

OWNER="$TMP_DIR/owner.out"
PERF="$TMP_DIR/perf.report"
MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row227-facade-inventory] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=object-lifecycle-facade-exact-slot-field-inventory-v0"
require_line "$DOC" "input_contract=typed-object-exact-slot-callsite-owner-selection-v0"
require_line "$DOC" "target_family=object_lifecycle_facade"
require_line "$DOC" "facade_method_count=3"
require_line "$DOC" "facade_exact_slot_get_count=16"
require_line "$DOC" "facade_exact_slot_set_count=9"
require_line "$DOC" "facade_exact_slot_field_op_count=25"
require_line "$DOC" "dominant_field_family=facade_receiver_state"
require_line "$DOC" "field_family.facade_receiver_state_count=12"
require_line "$DOC" "field_family.page_queue_bridge_count=8"
require_line "$DOC" "field_family.alloc_result_capsule_count=4"
require_line "$DOC" "pattern.positive_net_cache_candidate_count=4"
require_line "$DOC" "selected_next=facade_field_owner_selection"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "summary=ok"

cat >"$OWNER" <<'REPORT'
output_contract=typed-object-exact-slot-callsite-owner-selection-v0
input_contract=typed-object-exact-slot-callsite-attribution-v0
workload_id=representative-object-lifecycle-small-block-v0
dominant_family=object_lifecycle_facade
dominant_family_pct=18.52
top_callsite_symbol=HakoAllocPageModel.acquire_usize/1
top_callsite_pct=4.54
selected_owner=object_lifecycle_facade_exact_slot_field_inventory
selected_reason=dominant_family_object_lifecycle_facade
next_diagnostic=object_lifecycle_facade_exact_slot_field_inventory
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

cat >"$PERF" <<'REPORT'
    16.00%  app.exe  app.exe               [.] nyash.object.exact_slot_set_i64_hii
               |--2.30%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
    11.42%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
               |--0.78%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
               |--0.75%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
     9.53%  app.exe  app.exe               [.] nyash.object.exact_slot_get_handle_hii
               |--1.67%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
               |--1.53%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--1.53%--HakoAllocObjectLifecycleFacade.resetSmallAllocResult/0
     6.94%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
               |--2.33%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--2.28%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
     6.88%  app.exe  app.exe               [.] nyash.object.exact_slot_set_handle_hii
               |--3.81%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--1.54%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
REPORT

cat >"$MIR" <<'JSON'
{
  "functions": [
    {
      "name": "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2",
      "metadata": {
        "value_types": {
          "3": {"box_type": "HakoAllocObjectLifecycleFacade", "kind": "handle"},
          "12": {"box_type": "HakoAllocObjectLifecycleFacade", "kind": "handle"},
          "22": {"box_type": "HakoAllocObjectLifecycleFacade", "kind": "handle"},
          "36": {"box_type": "HakoAllocPageModel", "kind": "handle"},
          "46": {"box_type": "HakoAllocObjectLifecycleFacade", "kind": "handle"}
        }
      },
      "blocks": [
        {"id": 503, "instructions": [{"op": "field_get", "box": 3, "field": "last_alloc_page_id"}]},
        {"id": 504, "instructions": [{"op": "field_get", "box": 12, "field": "last_alloc_page_index"}]},
        {"id": 507, "instructions": [{"op": "field_get", "box": 22, "field": "last_alloc_page"}]},
        {"id": 510, "instructions": [{"op": "field_get", "box": 36, "field": "page_id"}]},
        {"id": 513, "instructions": [
          {"op": "field_set", "box": 46, "field": "release_known_page"},
          {"op": "field_get", "box": 46, "field": "release_known_page_fast_path_count"},
          {"op": "field_set", "box": 46, "field": "release_known_page_fast_path_count"}
        ]}
      ]
    },
    {
      "name": "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
      "metadata": {
        "value_types": {
          "3": {"box_type": "HakoAllocObjectLifecycleFacade", "kind": "handle"},
          "7": {"box_type": "HakoAllocObjectLifecycleAllocResult", "kind": "handle"},
          "19": {"box_type": "HakoAllocObjectLifecyclePageQueue", "kind": "handle"},
          "30": {"box_type": "HakoAllocObjectLifecyclePageQueue", "kind": "handle"},
          "56": {"box_type": "HakoAllocObjectLifecyclePageQueue", "kind": "handle"},
          "80": {"box_type": "HakoAllocObjectLifecyclePageQueue", "kind": "handle"},
          "104": {"box_type": "HakoAllocObjectLifecyclePageQueue", "kind": "handle"},
          "106": {"box_type": "HakoAllocObjectLifecycleAllocResult", "kind": "handle"},
          "203": {"box_type": "HakoAllocObjectLifecycleAllocResult", "kind": "handle"},
          "230": {"box_type": "HakoAllocObjectLifecyclePageQueue", "kind": "handle"},
          "232": {"box_type": "HakoAllocObjectLifecycleFacade", "kind": "handle"}
        }
      },
      "blocks": [
        {"id": 552, "instructions": [
          {"op": "field_get", "box": 3, "field": "alloc_result"},
          {"op": "field_get", "box": 7, "field": "attempt_count"},
          {"op": "field_set", "box": 7, "field": "attempt_count"},
          {"op": "field_get", "box": 3, "field": "object_lifecycle_queue"},
          {"op": "field_get", "box": 19, "field": "page_count"}
        ]},
        {"id": 553, "instructions": [
          {"op": "field_get", "box": 30, "field": "request_count"},
          {"op": "field_set", "box": 30, "field": "request_count"}
        ]},
        {"id": 555, "instructions": [{"op": "field_get", "box": 56, "field": "last_selected_index"}]},
        {"id": 557, "instructions": [{"op": "field_get", "box": 80, "field": "last_selected_page"}]},
        {"id": 560, "instructions": [
          {"op": "field_get", "box": 104, "field": "last_selected_page_id"},
          {"op": "field_set", "box": 106, "field": "last_page_id"},
          {"op": "field_get", "box": 104, "field": "last_selected_kind"}
        ]},
        {"id": 570, "instructions": [{"op": "field_set", "box": 203, "field": "last_block_id"}]},
        {"id": 575, "instructions": [
          {"op": "field_get", "box": 230, "field": "last_selected_page_id"},
          {"op": "field_set", "box": 232, "field": "last_alloc_page_index"},
          {"op": "field_set", "box": 232, "field": "last_alloc_page_id"},
          {"op": "field_set", "box": 232, "field": "last_alloc_page"}
        ]}
      ]
    },
    {
      "name": "HakoAllocObjectLifecycleFacade.resetSmallAllocResult/0",
      "metadata": {
        "value_types": {
          "1": {"box_type": "HakoAllocObjectLifecycleFacade", "kind": "handle"}
        }
      },
      "blocks": [
        {"id": 600, "instructions": [{"op": "field_get", "box": 1, "field": "alloc_result"}]}
      ]
    }
  ]
}
JSON

python3 "$TOOL" --mir-json "$MIR" --perf-report "$PERF" --owner-selection-report "$OWNER" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=object-lifecycle-facade-exact-slot-field-inventory-v0"
require_line "$REPORT" "input_contract=typed-object-exact-slot-callsite-owner-selection-v0"
require_line "$REPORT" "target_family=object_lifecycle_facade"
require_line "$REPORT" "facade_method_count=3"
require_line "$REPORT" "facade_exact_slot_get_count=16"
require_line "$REPORT" "facade_exact_slot_set_count=9"
require_line "$REPORT" "facade_exact_slot_field_op_count=25"
require_line "$REPORT" "dominant_field_family=facade_receiver_state"
require_line "$REPORT" "field_family.facade_receiver_state_count=12"
require_line "$REPORT" "field_family.page_queue_bridge_count=8"
require_line "$REPORT" "field_family.alloc_result_capsule_count=4"
require_line "$REPORT" "pattern.positive_net_cache_candidate_count=4"
require_line "$REPORT" "selected_next=facade_field_owner_selection"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

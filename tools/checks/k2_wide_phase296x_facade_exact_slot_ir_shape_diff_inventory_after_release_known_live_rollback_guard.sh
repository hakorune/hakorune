#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-272-FACADE-EXACT-SLOT-IR-SHAPE-DIFF-INVENTORY-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-271-WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK.md"
TOOL="$ROOT_DIR/tools/allocator/object_lifecycle_facade_exact_slot_field_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row272_facade_inventory.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

OWNER="$TMP_DIR/owner.out"
PERF="$TMP_DIR/perf.report"
MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row272-facade-inventory-after-rollback] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=object-lifecycle-facade-exact-slot-field-inventory-v0"
require_line "$DOC" "input_contract=weighted-exact-slot-owner-selection-v0"
require_line "$DOC" "target_family=object_lifecycle_facade"
require_line "$DOC" "target_family_pct=15.68"
require_line "$DOC" "facade_method_count=4"
require_line "$DOC" "facade_exact_slot_get_count=20"
require_line "$DOC" "facade_exact_slot_set_count=9"
require_line "$DOC" "facade_exact_slot_field_op_count=29"
require_line "$DOC" "top_facade_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "$DOC" "top_facade_method_pct=10.53"
require_line "$DOC" "dominant_field_family=facade_receiver_state"
require_line "$DOC" "field_family.facade_receiver_state_count=15"
require_line "$DOC" "field_family.page_queue_bridge_count=9"
require_line "$DOC" "field_family.alloc_result_capsule_count=4"
require_line "$DOC" "pattern.positive_net_cache_candidate_count=4"
require_line "$DOC" "selected_next=facade_field_owner_selection"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$OWNER" <<'REPORT'
output_contract=weighted-exact-slot-owner-selection-v0
input_contract=weighted-exact-slot-callsite-attribution-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
dominant_family=object_lifecycle_facade
dominant_family_pct=15.68
recent_nonkeeper_family=page_model_hotpath
recent_nonkeeper_row=296x-268
selected_family=object_lifecycle_facade
selected_owner=facade_exact_slot_ir_shape_diff_inventory
selected_reason=dominant_family_not_recent_nonkeeper
next_diagnostic=facade_exact_slot_ir_shape_diff_inventory
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

cat >"$PERF" <<'REPORT'
    13.85%  app.exe  app.exe               [.] nyash.object.exact_slot_set_i64_hii
               |--3.40%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--1.76%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2
     9.58%  app.exe  app.exe               [.] nyash.object.exact_slot_set_u64_hiu
               |--1.67%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--0.81%--HakoAllocObjectLifecycleFacade.resetReleaseResult/0
     8.74%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
               |--1.74%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
               |--1.40%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
     6.99%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
               |--2.33%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--0.84%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
     6.96%  app.exe  app.exe               [.] nyash.object.exact_slot_get_handle_hii
               |--1.73%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
     2.64%  app.exe  app.exe               [.] nyash.object.exact_slot_set_handle_hii
               |--1.76%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
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
      "name": "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2",
      "metadata": {
        "value_types": {
          "2": {"box_type": "HakoAllocObjectLifecycleFacade", "kind": "handle"},
          "8": {"box_type": "HakoAllocObjectLifecycleFacade", "kind": "handle"},
          "9": {"box_type": "HakoAllocObjectLifecyclePageQueue", "kind": "handle"}
        }
      },
      "blocks": [
        {"id": 520, "instructions": [
          {"op": "field_get", "box": 2, "field": "object_lifecycle_queue"},
          {"op": "field_set", "box": 8, "field": "release_attempt_count"},
          {"op": "field_get", "box": 9, "field": "last_selected_page_id"},
          {"op": "field_set", "box": 8, "field": "release_success_count"}
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
      "name": "HakoAllocObjectLifecycleFacade.resetReleaseResult/0",
      "metadata": {
        "value_types": {
          "1": {"box_type": "HakoAllocObjectLifecycleFacade", "kind": "handle"}
        }
      },
      "blocks": [
        {"id": 600, "instructions": [{"op": "field_get", "box": 1, "field": "release_result"}]}
      ]
    }
  ]
}
JSON

python3 "$TOOL" --mir-json "$MIR" --perf-report "$PERF" --owner-selection-report "$OWNER" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=object-lifecycle-facade-exact-slot-field-inventory-v0"
require_line "$REPORT" "input_contract=weighted-exact-slot-owner-selection-v0"
require_line "$REPORT" "target_family=object_lifecycle_facade"
require_line "$REPORT" "target_family_pct=15.68"
require_line "$REPORT" "dominant_field_family=facade_receiver_state"
require_line "$REPORT" "selected_next=facade_field_owner_selection"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

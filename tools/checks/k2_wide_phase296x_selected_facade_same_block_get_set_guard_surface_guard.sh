#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-229-SELECTED-FACADE-SAME-BLOCK-GET-SET-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-228-OBJECT-LIFECYCLE-FACADE-FIELD-OWNER-SELECTION.md"
TOOL="$ROOT_DIR/tools/allocator/selected_facade_same_block_get_set_guard_surface.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row229_facade_guard.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

OWNER="$TMP_DIR/owner.out"
MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row229-facade-guard] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=selected-facade-same-block-get-set-guard-surface-v0"
require_line "$DOC" "input_contract=object-lifecycle-facade-field-owner-selection-v0"
require_line "$DOC" "candidate_count=6"
require_line "$DOC" "candidate_usize_count=6"
require_line "$DOC" "planned_net_helper_call_delta=6"
require_line "$DOC" "runtime_storage_owner_preserved=1"
require_line "$DOC" "generic_residence_open=0"
require_line "$DOC" "source_rewrite=0"
require_line "$DOC" "selected_next=selected_facade_same_block_get_set_keeper"
require_line "$DOC" "summary=ok"

cat >"$OWNER" <<'REPORT'
output_contract=object-lifecycle-facade-field-owner-selection-v0
input_contract=object-lifecycle-facade-exact-slot-field-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_owner=selected_facade_same_block_get_set_fusion
selected_reason=same_block_get_set_candidates_dominate_positive_net_surface
next_diagnostic=selected_facade_same_block_get_set_guard_surface
optimization_open=0
summary=ok
REPORT

cat >"$MIR" <<'JSON'
{
  "typed_object_plans": [
    {"box_name": "HakoAllocObjectLifecycleFacade", "fields": [
      {"name": "release_known_page_fast_path_count", "storage": "usize"},
      {"name": "release_known_page_fallback_count", "storage": "usize"}
    ]},
    {"box_name": "HakoAllocObjectLifecycleAllocResult", "fields": [
      {"name": "attempt_count", "storage": "usize"}
    ]},
    {"box_name": "HakoAllocObjectLifecyclePageQueue", "fields": [
      {"name": "request_count", "storage": "usize"}
    ]}
  ],
  "functions": [
    {
      "name": "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2",
      "metadata": {"value_types": {"1": {"box_type": "HakoAllocObjectLifecycleFacade", "kind": "handle"}}},
      "blocks": [
        {"id": 513, "instructions": [
          {"op": "field_get", "box": 1, "dst": 10, "field": "release_known_page_fast_path_count"},
          {"op": "binop", "operation": "+", "lhs": 10, "rhs": 11, "dst": 12},
          {"op": "field_set", "box": 1, "field": "release_known_page_fast_path_count", "value": 12}
        ]}
      ]
    },
    {
      "name": "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseKnownPageIndex/1",
      "metadata": {"value_types": {"1": {"box_type": "HakoAllocObjectLifecycleFacade", "kind": "handle"}}},
      "blocks": [
        {"id": 525, "instructions": [
          {"op": "field_get", "box": 1, "dst": 20, "field": "release_known_page_fallback_count"},
          {"op": "binop", "operation": "+", "lhs": 20, "rhs": 21, "dst": 22},
          {"op": "field_set", "box": 1, "field": "release_known_page_fallback_count", "value": 22}
        ]},
        {"id": 535, "instructions": [
          {"op": "field_get", "box": 1, "dst": 30, "field": "release_known_page_fast_path_count"},
          {"op": "binop", "operation": "+", "lhs": 30, "rhs": 31, "dst": 32},
          {"op": "field_set", "box": 1, "field": "release_known_page_fast_path_count", "value": 32}
        ]}
      ]
    },
    {
      "name": "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
      "metadata": {"value_types": {
        "1": {"box_type": "HakoAllocObjectLifecycleAllocResult", "kind": "handle"},
        "2": {"box_type": "HakoAllocObjectLifecyclePageQueue", "kind": "handle"}
      }},
      "blocks": [
        {"id": 552, "instructions": [
          {"op": "field_get", "box": 1, "dst": 40, "field": "attempt_count"},
          {"op": "binop", "operation": "+", "lhs": 40, "rhs": 41, "dst": 42},
          {"op": "field_set", "box": 1, "field": "attempt_count", "value": 42}
        ]},
        {"id": 553, "instructions": [
          {"op": "field_get", "box": 2, "dst": 50, "field": "request_count"},
          {"op": "binop", "operation": "+", "lhs": 50, "rhs": 51, "dst": 52},
          {"op": "field_set", "box": 2, "field": "request_count", "value": 52}
        ]}
      ]
    },
    {
      "name": "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAllocAligned/2",
      "metadata": {"value_types": {"1": {"box_type": "HakoAllocObjectLifecycleAllocResult", "kind": "handle"}}},
      "blocks": [
        {"id": 578, "instructions": [
          {"op": "field_get", "box": 1, "dst": 60, "field": "attempt_count"},
          {"op": "binop", "operation": "+", "lhs": 60, "rhs": 61, "dst": 62},
          {"op": "field_set", "box": 1, "field": "attempt_count", "value": 62}
        ]}
      ]
    }
  ]
}
JSON

python3 "$TOOL" --mir-json "$MIR" --owner-selection-report "$OWNER" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=selected-facade-same-block-get-set-guard-surface-v0"
require_line "$REPORT" "input_contract=object-lifecycle-facade-field-owner-selection-v0"
require_line "$REPORT" "candidate_count=6"
require_line "$REPORT" "candidate_usize_count=6"
require_line "$REPORT" "planned_erased_get_set_helper_calls=12"
require_line "$REPORT" "planned_added_fused_helper_calls=6"
require_line "$REPORT" "planned_net_helper_call_delta=6"
require_line "$REPORT" "selected_next=selected_facade_same_block_get_set_keeper"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

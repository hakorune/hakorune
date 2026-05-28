#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-239-SELECTED-PAGE-QUEUE-SAME-BLOCK-GET-SET-GUARD-SURFACE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-238-PAGE-QUEUE-FIELD-OWNER-SELECTION.md"
TOOL="$ROOT_DIR/tools/allocator/selected_page_queue_same_block_get_set_guard_surface.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row239_page_queue_guard.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

OWNER="$TMP_DIR/owner.out"
MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row239-page-queue-guard] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=selected-page-queue-same-block-get-set-guard-surface-v0"
require_line "$DOC" "input_contract=page-queue-field-owner-selection-v0"
require_line "$DOC" "candidate_count=21"
require_line "$DOC" "candidate_usize_count=21"
require_line "$DOC" "planned_net_helper_call_delta=21"
require_line "$DOC" "runtime_storage_owner_preserved=1"
require_line "$DOC" "generic_residence_open=0"
require_line "$DOC" "source_rewrite=0"
require_line "$DOC" "candidate_method_3=HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0"
require_line "$DOC" "candidate_method_3_count=9"
require_line "$DOC" "candidate_field_3=HakoAllocObjectLifecyclePageQueue.miss_count"
require_line "$DOC" "candidate_field_3_count=5"
require_line "$DOC" "selected_next=selected_page_queue_same_block_get_set_keeper"
require_line "$DOC" "by_name_special_case=0"
require_line "$DOC" "summary=ok"

cat >"$OWNER" <<'REPORT'
output_contract=page-queue-field-owner-selection-v0
input_contract=page-queue-exact-slot-field-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
same_block_get_set_count=12
same_receiver_repeated_get_count=4
positive_net_cache_candidate_count=16
selected_owner=selected_page_queue_same_block_get_set_fusion
selected_reason=same_block_get_set_candidates_dominate_page_queue_positive_net_surface
next_diagnostic=selected_page_queue_same_block_get_set_guard_surface
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

python3 - "$MIR" <<'PY'
import json
import sys

path = sys.argv[1]
fields = [
    "active_select_count",
    "add_count",
    "decommitted_skip_count",
    "miss_count",
    "page_count",
    "reject_count",
    "request_count",
    "retired_skip_count",
    "reuse_select_count",
    "select_count",
    "single_page_fallback_count",
    "single_page_fast_path_count",
    "unavailable_skip_count",
]
plans = {
    "box_name": "HakoAllocObjectLifecyclePageQueue",
    "fields": [{"name": field, "storage": "usize"} for field in fields],
}
method_specs = [
    (
        "HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3",
        ["select_count", "reuse_select_count", "active_select_count"],
    ),
    (
        "HakoAllocObjectLifecyclePageQueue.addPage/1",
        ["reject_count", "page_count", "add_count"],
    ),
    (
        "HakoAllocObjectLifecyclePageQueue.selectPage/0",
        [
            "request_count",
            "single_page_fallback_count",
            "decommitted_skip_count",
            "retired_skip_count",
            "unavailable_skip_count",
            "miss_count",
        ],
    ),
    (
        "HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0",
        [
            "miss_count",
            "decommitted_skip_count",
            "miss_count",
            "single_page_fast_path_count",
            "retired_skip_count",
            "miss_count",
            "single_page_fast_path_count",
            "unavailable_skip_count",
            "miss_count",
        ],
    ),
]
functions = []
next_reg = 10
next_block = 100
for name, candidate_fields in method_specs:
    instructions = []
    for field in candidate_fields:
        get_dst = next_reg
        binop_dst = next_reg + 2
        delta = next_reg + 1
        next_reg += 3
        instructions.extend(
            [
                {"op": "field_get", "box": 1, "dst": get_dst, "field": field},
                {"op": "binop", "operation": "+", "lhs": get_dst, "rhs": delta, "dst": binop_dst},
                {"op": "field_set", "box": 1, "field": field, "value": binop_dst},
            ]
        )
    functions.append(
        {
            "name": name,
            "metadata": {
                "value_types": {
                    "1": {
                        "box_type": "HakoAllocObjectLifecyclePageQueue",
                        "kind": "handle",
                    }
                }
            },
            "blocks": [{"id": next_block, "instructions": instructions}],
        }
    )
    next_block += 1

module = {"typed_object_plans": [plans], "functions": functions}
with open(path, "w", encoding="utf-8") as f:
    json.dump(module, f, indent=2)
PY

python3 "$TOOL" --mir-json "$MIR" --owner-selection-report "$OWNER" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=selected-page-queue-same-block-get-set-guard-surface-v0"
require_line "$REPORT" "input_contract=page-queue-field-owner-selection-v0"
require_line "$REPORT" "selected_owner=selected_page_queue_same_block_get_set_fusion"
require_line "$REPORT" "target_family=page_queue_helpers"
require_line "$REPORT" "candidate_count=21"
require_line "$REPORT" "candidate_i64_count=0"
require_line "$REPORT" "candidate_usize_count=21"
require_line "$REPORT" "candidate_u64_count=0"
require_line "$REPORT" "planned_erased_get_set_helper_calls=42"
require_line "$REPORT" "planned_added_fused_helper_calls=21"
require_line "$REPORT" "planned_net_helper_call_delta=21"
require_line "$REPORT" "planned_net_helper_call_delta_positive=1"
require_line "$REPORT" "runtime_storage_owner_preserved=1"
require_line "$REPORT" "helper_free_direct_op_rejected=1"
require_line "$REPORT" "generic_residence_open=0"
require_line "$REPORT" "source_rewrite=0"
require_line "$REPORT" "candidate_method_0=HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3"
require_line "$REPORT" "candidate_method_0_count=3"
require_line "$REPORT" "candidate_method_1=HakoAllocObjectLifecyclePageQueue.addPage/1"
require_line "$REPORT" "candidate_method_1_count=3"
require_line "$REPORT" "candidate_method_2=HakoAllocObjectLifecyclePageQueue.selectPage/0"
require_line "$REPORT" "candidate_method_2_count=6"
require_line "$REPORT" "candidate_method_3=HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0"
require_line "$REPORT" "candidate_method_3_count=9"
require_line "$REPORT" "candidate_field_3=HakoAllocObjectLifecyclePageQueue.miss_count"
require_line "$REPORT" "candidate_field_3_count=5"
require_line "$REPORT" "selected_next=selected_page_queue_same_block_get_set_keeper"
require_line "$REPORT" "by_name_special_case=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

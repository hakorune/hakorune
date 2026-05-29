#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-295-ALLOC-RESULT-CAPSULE-IR-SHAPE-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-294-POST-RELEASE-RESULT-CAPSULE-OWNER-REFRESH-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
TOOL="$ROOT_DIR/tools/allocator/alloc_result_capsule_ir_shape_inventory_after_record_success.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row295_alloc_result.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

OWNER="$TMP_DIR/owner.out"
MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row295-alloc-result-capsule-inventory] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=alloc-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0"
require_line "$DOC" "input_contract=post-release-result-capsule-owner-refresh-after-record-success-helper-fusion-v0"
require_line "$DOC" "alloc_result_method_count=13"
require_line "$DOC" "alloc_result_field_op_count=31"
require_line "$DOC" "alloc_result_call_count=0"
require_line "$DOC" "top_alloc_hot_method=HakoAllocObjectLifecycleAllocResult.recordSuccess/1"
require_line "$DOC" "reset_batching_landed=1"
require_line "$DOC" "record_success_helper_fusion_landed=1"
require_line "$DOC" "record_success_repeat_closed=1"
require_line "$DOC" "remaining_family_is_small=1"
require_line "$DOC" "selected_next=alloc_result_capsule_owner_selection_after_record_success_helper_fusion"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$OWNER" <<'REPORT'
output_contract=post-release-result-capsule-owner-refresh-after-record-success-helper-fusion-v0
input_contract=release-result-capsule-owner-selection-after-record-success-helper-fusion-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_family=alloc_result_capsule
selected_owner=alloc_result_capsule_ir_shape_inventory_after_record_success_helper_fusion
remaining_family_is_small=1
summary=ok
REPORT

cat >"$MIR" <<'JSON'
{
  "functions": [
    {
      "name": "HakoAllocObjectLifecycleAllocResult.birth/0",
      "blocks": [{"id": 1, "instructions": [
        {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"},
        {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"},
        {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}
      ]}]
    },
    {
      "name": "HakoAllocObjectLifecycleAllocResult.recordSuccess/1",
      "blocks": [{"id": 2, "instructions": [
        {"op": "field_get"}, {"op": "field_get"}, {"op": "field_get"},
        {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"},
        {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "branch"}, {"op": "phi"}
      ]}]
    },
    {
      "name": "HakoAllocObjectLifecycleAllocResult.reset/0",
      "blocks": [{"id": 3, "instructions": [
        {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"},
        {"op": "copy"}, {"op": "copy"}
      ]}]
    },
    {"name": "HakoAllocObjectLifecycleAllocResult.recordFailure/1", "blocks": [{"id": 4, "instructions": [{"op": "field_set"}, {"op": "field_set"}, {"op": "copy"}, {"op": "branch"}, {"op": "phi"}]}]},
    {"name": "HakoAllocObjectLifecycleAllocResult.reason/0", "blocks": [{"id": 5, "instructions": [{"op": "field_get"}, {"op": "copy"}]}]},
    {"name": "HakoAllocObjectLifecycleAllocResult.pageId/0", "blocks": [{"id": 6, "instructions": [{"op": "field_get"}, {"op": "copy"}]}]},
    {"name": "HakoAllocObjectLifecycleAllocResult.blockId/0", "blocks": [{"id": 7, "instructions": [{"op": "field_get"}, {"op": "copy"}]}]},
    {"name": "HakoAllocObjectLifecycleAllocResult.ok/0", "blocks": [{"id": 8, "instructions": [{"op": "field_get"}, {"op": "copy"}]}]},
    {"name": "HakoAllocObjectLifecycleAllocResult.attempt/0", "blocks": [{"id": 9, "instructions": [{"op": "field_get"}, {"op": "copy"}]}]},
    {"name": "HakoAllocObjectLifecycleAllocResult.clearKnown/0", "blocks": [{"id": 10, "instructions": [{"op": "field_set"}]}]},
    {"name": "HakoAllocObjectLifecycleAllocResult.markKnown/1", "blocks": [{"id": 11, "instructions": [{"op": "field_set"}]}]},
    {"name": "HakoAllocObjectLifecycleAllocResult.touch/0", "blocks": [{"id": 12, "instructions": [{"op": "field_set"}]}]},
    {"name": "HakoAllocObjectLifecycleAllocResult.noop/0", "blocks": [{"id": 13, "instructions": [
      {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}
    ]}]}
  ]
}
JSON

python3 "$TOOL" --mir-json "$MIR" --owner-refresh-report "$OWNER" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=alloc-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0"
require_line "$REPORT" "alloc_result_method_count=13"
require_line "$REPORT" "alloc_result_field_get_count=8"
require_line "$REPORT" "alloc_result_field_set_count=23"
require_line "$REPORT" "alloc_result_field_op_count=31"
require_line "$REPORT" "alloc_result_copy_count=22"
require_line "$REPORT" "alloc_result_call_count=0"
require_line "$REPORT" "top_alloc_hot_method=HakoAllocObjectLifecycleAllocResult.recordSuccess/1"
require_line "$REPORT" "reset_batching_landed=1"
require_line "$REPORT" "record_success_helper_fusion_landed=1"
require_line "$REPORT" "record_success_repeat_closed=1"
require_line "$REPORT" "remaining_family_is_small=1"
require_line "$REPORT" "selected_next=alloc_result_capsule_owner_selection_after_record_success_helper_fusion"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

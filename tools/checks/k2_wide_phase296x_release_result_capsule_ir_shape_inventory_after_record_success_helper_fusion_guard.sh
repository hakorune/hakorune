#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-292-RELEASE-RESULT-CAPSULE-IR-SHAPE-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-291-POST-PAGE-MODEL-HOTPATH-OWNER-REFRESH-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
TOOL="$ROOT_DIR/tools/allocator/release_result_capsule_ir_shape_inventory_after_record_success.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row292_release_result.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

OWNER="$TMP_DIR/owner.out"
MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row292-release-result-capsule-inventory] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=release-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0"
require_line "$DOC" "input_contract=post-page-model-hotpath-owner-refresh-after-record-success-helper-fusion-v0"
require_line "$DOC" "release_result_method_count=11"
require_line "$DOC" "release_result_field_op_count=25"
require_line "$DOC" "release_result_call_count=0"
require_line "$DOC" "top_release_hot_method=HakoAllocObjectLifecycleReleaseResult.recordSuccess/2"
require_line "$DOC" "record_success_helper_fusion_landed=1"
require_line "$DOC" "record_success_repeat_closed=1"
require_line "$DOC" "selected_next=release_result_capsule_owner_selection_after_record_success_helper_fusion"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$OWNER" <<'REPORT'
output_contract=post-page-model-hotpath-owner-refresh-after-record-success-helper-fusion-v0
input_contract=page-model-hotpath-shape-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_family=release_result_capsule
selected_owner=release_result_capsule_ir_shape_inventory_after_record_success_helper_fusion
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

cat >"$MIR" <<'JSON'
{
  "functions": [
    {
      "name": "HakoAllocObjectLifecycleReleaseResult.birth/0",
      "blocks": [{"id": 1, "instructions": [
        {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"},
        {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"},
        {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}
      ]}]
    },
    {
      "name": "HakoAllocObjectLifecycleReleaseResult.recordSuccess/2",
      "blocks": [{"id": 2, "instructions": [
        {"op": "field_get"}, {"op": "field_get"},
        {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"},
        {"op": "copy"}, {"op": "copy"}, {"op": "copy"}
      ]}]
    },
    {
      "name": "HakoAllocObjectLifecycleReleaseResult.reset/0",
      "blocks": [{"id": 3, "instructions": [
        {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"},
        {"op": "copy"}, {"op": "copy"}
      ]}]
    },
    {"name": "HakoAllocObjectLifecycleReleaseResult.recordFailure/1", "blocks": [{"id": 4, "instructions": [{"op": "field_set"}, {"op": "field_set"}, {"op": "copy"}]}]},
    {"name": "HakoAllocObjectLifecycleReleaseResult.reason/0", "blocks": [{"id": 5, "instructions": [{"op": "field_get"}, {"op": "copy"}]}]},
    {"name": "HakoAllocObjectLifecycleReleaseResult.pageId/0", "blocks": [{"id": 6, "instructions": [{"op": "field_get"}, {"op": "copy"}]}]},
    {"name": "HakoAllocObjectLifecycleReleaseResult.blockId/0", "blocks": [{"id": 7, "instructions": [{"op": "field_get"}, {"op": "copy"}]}]},
    {"name": "HakoAllocObjectLifecycleReleaseResult.ok/0", "blocks": [{"id": 8, "instructions": [{"op": "field_get"}, {"op": "copy"}]}]},
    {"name": "HakoAllocObjectLifecycleReleaseResult.clearKnown/0", "blocks": [{"id": 9, "instructions": [{"op": "field_set"}]}]},
    {"name": "HakoAllocObjectLifecycleReleaseResult.markKnown/1", "blocks": [{"id": 10, "instructions": [{"op": "field_set"}]}]},
    {"name": "HakoAllocObjectLifecycleReleaseResult.touch/0", "blocks": [{"id": 11, "instructions": [{"op": "field_set"}]}]}
  ]
}
JSON

python3 "$TOOL" --mir-json "$MIR" --owner-refresh-report "$OWNER" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=release-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0"
require_line "$REPORT" "release_result_method_count=11"
require_line "$REPORT" "release_result_field_get_count=6"
require_line "$REPORT" "release_result_field_set_count=19"
require_line "$REPORT" "release_result_field_op_count=25"
require_line "$REPORT" "release_result_copy_count=14"
require_line "$REPORT" "release_result_call_count=0"
require_line "$REPORT" "top_release_hot_method=HakoAllocObjectLifecycleReleaseResult.recordSuccess/2"
require_line "$REPORT" "record_success_helper_fusion_landed=1"
require_line "$REPORT" "record_success_repeat_closed=1"
require_line "$REPORT" "selected_next=release_result_capsule_owner_selection_after_record_success_helper_fusion"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

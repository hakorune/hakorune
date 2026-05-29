#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-289-PAGE-MODEL-HOTPATH-IR-SHAPE-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-288-POST-FACADE-INVENTORY-OWNER-REFRESH-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
TOOL="$ROOT_DIR/tools/allocator/page_model_hotpath_ir_shape_diff_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row289_page_model_shape.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

OWNER="$TMP_DIR/owner.out"
PERF="$TMP_DIR/perf.report"
MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row289-page-model-shape-after-record-success] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-model-hotpath-ir-shape-diff-inventory-v0"
require_line "$DOC" "input_contract=post-facade-inventory-owner-refresh-after-record-success-helper-fusion-v0"
require_line "$DOC" "target_family_pct=11.73"
require_line "$DOC" "page_model_method_count=5"
require_line "$DOC" "page_model_mir_field_op_count=36"
require_line "$DOC" "page_model_mir_copy_count=47"
require_line "$DOC" "page_model_mir_call_count=5"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_method_pct=6.31"
require_line "$DOC" "selected_method_shape_owner=copy_materialization"
require_line "$DOC" "selected_next=page_model_hotpath_shape_owner_selection_after_record_success_helper_fusion"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$OWNER" <<'REPORT'
output_contract=post-facade-inventory-owner-refresh-after-record-success-helper-fusion-v0
input_contract=facade-field-owner-selection-after-record-success-helper-fusion-v0
workload_id=representative-object-lifecycle-small-block-v0
source_exact_slot_get_set_pct=50.97
selected_family=page_model_hotpath
selected_family_pct=11.73
selected_owner=page_model_hotpath_ir_shape_inventory_after_record_success_helper_fusion
next_diagnostic=page_model_hotpath_ir_shape_inventory_after_record_success_helper_fusion
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

cat >"$PERF" <<'REPORT'
    11.77%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
               |--1.26%--HakoAllocPageModel.isDecommitted/0
               |--0.64%--HakoAllocPageModel.acquire_usize/1
                --0.63%--HakoAllocPageModel.isRetired/0
     9.88%  app.exe  app.exe               [.] nyash.object.exact_slot_get_handle_hii
               |--1.89%--HakoAllocPageModel.releaseLocalKnownLive/1
                --1.26%--HakoAllocPageModel.acquire_usize/1
     9.02%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
               |--1.89%--HakoAllocPageModel.acquire_usize/1
                --0.64%--HakoAllocPageModel.freeCount/0
     7.75%  app.exe  app.exe               [.] nyash.object.exact_slot_set_u64_hiu
               |--2.52%--HakoAllocPageModel.acquire_usize/1
                --1.00%--HakoAllocPageModel.releaseLocalKnownLive/1
REPORT

cat >"$MIR" <<'JSON'
{
  "functions": [
    {
      "name": "HakoAllocPageModel.acquire_usize/1",
      "blocks": [
        {
          "id": 1,
          "instructions": [
            {"op": "field_get"}, {"op": "field_get"}, {"op": "field_get"}, {"op": "field_get"}, {"op": "field_get"},
            {"op": "field_get"}, {"op": "field_get"}, {"op": "field_get"}, {"op": "field_get"}, {"op": "field_get"},
            {"op": "field_get"}, {"op": "field_get"}, {"op": "field_get"},
            {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"},
            {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"},
            {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"},
            {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"},
            {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"},
            {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"},
            {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"},
            {"op": "copy"},
            {"op": "mir_call"}, {"op": "mir_call"}, {"op": "mir_call"}, {"op": "phi"}
          ]
        }
      ]
    },
    {
      "name": "HakoAllocPageModel.releaseLocalKnownLive/1",
      "blocks": [
        {
          "id": 2,
          "instructions": [
            {"op": "field_get"}, {"op": "field_get"}, {"op": "field_get"}, {"op": "field_get"},
            {"op": "field_get"}, {"op": "field_get"}, {"op": "field_get"},
            {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"}, {"op": "field_set"},
            {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"},
            {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"}, {"op": "copy"},
            {"op": "copy"}, {"op": "mir_call"}, {"op": "mir_call"}
          ]
        }
      ]
    },
    {"name": "HakoAllocPageModel.isDecommitted/0", "blocks": [{"id": 3, "instructions": [{"op": "field_get"}, {"op": "copy"}]}]},
    {"name": "HakoAllocPageModel.freeCount/0", "blocks": [{"id": 4, "instructions": [{"op": "field_get"}, {"op": "copy"}]}]},
    {"name": "HakoAllocPageModel.isRetired/0", "blocks": [{"id": 5, "instructions": [{"op": "field_get"}, {"op": "copy"}]}]}
  ]
}
JSON

python3 "$TOOL" --mir-json "$MIR" --perf-report "$PERF" --owner-selection-report "$OWNER" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=page-model-hotpath-ir-shape-diff-inventory-v0"
require_line "$REPORT" "input_contract=post-facade-inventory-owner-refresh-after-record-success-helper-fusion-v0"
require_line "$REPORT" "target_family=page_model_hotpath"
require_line "$REPORT" "target_family_pct=11.73"
require_line "$REPORT" "page_model_method_count=5"
require_line "$REPORT" "missing_page_model_method_count=0"
require_line "$REPORT" "page_model_exact_slot_perf_pct=11.73"
require_line "$REPORT" "page_model_mir_field_get_count=23"
require_line "$REPORT" "page_model_mir_field_set_count=13"
require_line "$REPORT" "page_model_mir_field_op_count=36"
require_line "$REPORT" "page_model_mir_copy_count=47"
require_line "$REPORT" "page_model_mir_call_count=5"
require_line "$REPORT" "page_model_mir_phi_count=1"
require_line "$REPORT" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "selected_method_pct=6.31"
require_line "$REPORT" "selected_method_shape_owner=copy_materialization"
require_line "$REPORT" "selected_next=page_model_hotpath_shape_owner_selection_after_record_success_helper_fusion"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

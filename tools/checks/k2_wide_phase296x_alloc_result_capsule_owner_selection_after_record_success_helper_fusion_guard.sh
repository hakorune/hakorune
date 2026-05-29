#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-296-ALLOC-RESULT-CAPSULE-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-295-ALLOC-RESULT-CAPSULE-IR-SHAPE-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
TOOL="$ROOT_DIR/tools/allocator/alloc_result_capsule_owner_selection_after_record_success.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row296_alloc_result_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

INV="$TMP_DIR/inventory.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row296-alloc-result-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=alloc-result-capsule-owner-selection-after-record-success-helper-fusion-v0"
require_line "$DOC" "selected_owner=micro_helper_lane_closeout_and_representation_direct_lowering_selection"
require_line "$DOC" "micro_helper_lane_has_remaining_small_keeper=0"
require_line "$DOC" "representation_direct_lowering_required=1"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$INV" <<'REPORT'
output_contract=alloc-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0
input_contract=post-release-result-capsule-owner-refresh-after-record-success-helper-fusion-v0
workload_id=representative-object-lifecycle-small-block-v0
alloc_result_field_op_count=31
top_alloc_method=HakoAllocObjectLifecycleAllocResult.birth/0
top_alloc_method_field_op_count=9
top_alloc_hot_method=HakoAllocObjectLifecycleAllocResult.recordSuccess/1
top_alloc_hot_method_field_op_count=8
reset_batching_landed=1
record_success_helper_fusion_landed=1
record_success_repeat_closed=1
remaining_family_is_small=1
summary=ok
REPORT

python3 "$TOOL" --inventory-report "$INV" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=alloc-result-capsule-owner-selection-after-record-success-helper-fusion-v0"
require_line "$REPORT" "input_contract=alloc-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0"
require_line "$REPORT" "alloc_result_field_op_count=31"
require_line "$REPORT" "selected_owner=micro_helper_lane_closeout_and_representation_direct_lowering_selection"
require_line "$REPORT" "rejected_owner=alloc_result_reset_batching_repeat"
require_line "$REPORT" "rejected_owner_1=alloc_result_record_success_helper_fusion_repeat"
require_line "$REPORT" "rejected_owner_2=alloc_result_birth_batching"
require_line "$REPORT" "rejected_owner_3=generic_capsule_flattening"
require_line "$REPORT" "micro_helper_lane_has_remaining_small_keeper=0"
require_line "$REPORT" "representation_direct_lowering_required=1"
require_line "$REPORT" "implementation_open=0"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

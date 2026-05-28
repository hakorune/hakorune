#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-262-PAGE-MODEL-HOTPATH-IR-SHAPE-DIFF-REFRESH-AFTER-RESULT-CAPSULE-RESET.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-261-WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RESULT-CAPSULE-RESET.md"
OLD="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-246-PAGE-MODEL-HOTPATH-IR-SHAPE-DIFF-INVENTORY.md"
TOOL="$ROOT_DIR/tools/allocator/page_model_hotpath_ir_shape_diff_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row262_page_model_shape.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

OWNER="$TMP_DIR/owner.out"
PERF="$TMP_DIR/perf.report"
MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row262-page-model-shape-refresh] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$OLD" "Status: Landed"
require_line "$DOC" "output_contract=page-model-hotpath-ir-shape-diff-inventory-v0"
require_line "$DOC" "input_contract=weighted-exact-slot-owner-selection-after-result-capsule-reset-v0"
require_line "$DOC" "target_family_pct=16.81"
require_line "$DOC" "page_model_method_count=5"
require_line "$DOC" "page_model_mir_field_op_count=36"
require_line "$DOC" "page_model_mir_copy_count=47"
require_line "$DOC" "page_model_mir_call_count=5"
require_line "$DOC" "previous_page_model_field_op_count=58"
require_line "$DOC" "previous_page_model_copy_count=62"
require_line "$DOC" "previous_page_model_call_count=8"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_method_pct=9.05"
require_line "$DOC" "selected_method_shape_owner=copy_materialization"
require_line "$DOC" "selected_method_prior_no_material_effect_row=296x-252"
require_line "$DOC" "selected_next=page_model_hotpath_shape_owner_selection_after_result_capsule_reset"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$OWNER" <<'REPORT'
output_contract=weighted-exact-slot-owner-selection-after-result-capsule-reset-v0
input_contract=post-result-capsule-reset-batching-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
dominant_family=page_model_hotpath
dominant_family_pct=16.81
dominant_family_is_recent_nonkeeper=0
recent_nonkeeper_family=page_queue_helpers
recent_nonkeeper_row=296x-241
page_queue_immediate_retry_blocked=1
top_unblocked_family=page_model_hotpath
top_unblocked_family_pct=16.81
selected_family=page_model_hotpath
selected_owner=page_model_hotpath_ir_shape_diff_refresh
next_diagnostic=page_model_hotpath_ir_shape_diff_refresh
summary=ok
REPORT

cat >"$PERF" <<'REPORT'
    11.77%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
               |--1.81%--HakoAllocPageModel.isDecommitted/0
               |--0.91%--HakoAllocPageModel.acquire_usize/1
                --0.90%--HakoAllocPageModel.isRetired/0
     9.88%  app.exe  app.exe               [.] nyash.object.exact_slot_get_handle_hii
               |--2.71%--HakoAllocPageModel.releaseLocalKnownLive/1
                --1.81%--HakoAllocPageModel.acquire_usize/1
     9.02%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
               |--2.71%--HakoAllocPageModel.acquire_usize/1
                --0.91%--HakoAllocPageModel.freeCount/0
     7.75%  app.exe  app.exe               [.] nyash.object.exact_slot_set_u64_hiu
               |--3.62%--HakoAllocPageModel.acquire_usize/1
                --1.43%--HakoAllocPageModel.releaseLocalKnownLive/1
REPORT

if [ ! -x "$ROOT_DIR/target/release/hakorune" ]; then
  cargo build --release --bin hakorune >/tmp/hakorune_row262_hakorune_build.log
fi

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR" \
    "$APP" >/tmp/hakorune_row262_mir_emit.log

python3 "$TOOL" --mir-json "$MIR" --perf-report "$PERF" --owner-selection-report "$OWNER" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=page-model-hotpath-ir-shape-diff-inventory-v0"
require_line "$REPORT" "input_contract=weighted-exact-slot-owner-selection-after-result-capsule-reset-v0"
require_line "$REPORT" "target_family=page_model_hotpath"
require_line "$REPORT" "target_family_pct=16.81"
require_line "$REPORT" "page_model_method_count=5"
require_line "$REPORT" "missing_page_model_method_count=0"
require_line "$REPORT" "page_model_exact_slot_perf_pct=16.81"
require_line "$REPORT" "page_model_mir_field_get_count=23"
require_line "$REPORT" "page_model_mir_field_set_count=13"
require_line "$REPORT" "page_model_mir_field_op_count=36"
require_line "$REPORT" "page_model_mir_copy_count=47"
require_line "$REPORT" "page_model_mir_call_count=5"
require_line "$REPORT" "page_model_mir_phi_count=1"
require_line "$REPORT" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "selected_method_pct=9.05"
require_line "$REPORT" "selected_method_field_get_count=13"
require_line "$REPORT" "selected_method_field_set_count=8"
require_line "$REPORT" "selected_method_field_op_count=21"
require_line "$REPORT" "selected_method_copy_count=31"
require_line "$REPORT" "selected_method_call_count=3"
require_line "$REPORT" "selected_method_phi_count=1"
require_line "$REPORT" "selected_method_shape_owner=copy_materialization"
require_line "$REPORT" "recent_selected_method_rmw_keeper_already_applied=1"
require_line "$REPORT" "direct_op_previous_rejected=1"
require_line "$REPORT" "page_queue_recent_nonkeeper_retry_closed=1"
require_line "$REPORT" "ir_shape_diff_inventory_only=1"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "selected_next=page_model_hotpath_shape_owner_selection"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

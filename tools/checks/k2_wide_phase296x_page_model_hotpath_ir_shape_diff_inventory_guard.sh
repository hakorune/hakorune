#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-246-PAGE-MODEL-HOTPATH-IR-SHAPE-DIFF-INVENTORY.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-245-WEIGHTED-EXACT-SLOT-OWNER-SELECTION.md"
TOOL="$ROOT_DIR/tools/allocator/page_model_hotpath_ir_shape_diff_inventory.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row246_page_model_shape.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

OWNER="$TMP_DIR/owner.out"
PERF="$TMP_DIR/perf.report"
MIR="$TMP_DIR/app.mir.json"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row246-page-model-shape] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-model-hotpath-ir-shape-diff-inventory-v0"
require_line "$DOC" "page_model_method_count=5"
require_line "$DOC" "page_model_mir_field_op_count=58"
require_line "$DOC" "page_model_mir_copy_count=62"
require_line "$DOC" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$DOC" "selected_method_shape_owner=copy_materialization"
require_line "$DOC" "recent_selected_method_rmw_keeper_already_applied=1"
require_line "$DOC" "direct_op_previous_rejected=1"
require_line "$DOC" "ir_shape_diff_inventory_only=1"
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
dominant_family=page_queue_helpers
dominant_family_pct=16.45
recent_nonkeeper_family=page_queue_helpers
recent_nonkeeper_row=296x-241
recent_nonkeeper_candidate_count=21
recent_nonkeeper_hot_per_candidate_pct=0.78
dominant_family_is_recent_nonkeeper=1
top_unblocked_family=page_model_hotpath
top_unblocked_family_pct=15.29
selected_family=page_model_hotpath
selected_owner=page_model_hotpath_ir_shape_diff_inventory
next_diagnostic=page_model_hotpath_ir_shape_diff_inventory
summary=ok
REPORT

cat >"$PERF" <<'REPORT'
    17.17%  app.exe  app.exe               [.] nyash.object.exact_slot_set_i64_hii
               |--3.52%--HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
               |--3.42%--HakoAllocObjectLifecycleAllocResult.reset/0
               |--2.57%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--2.56%--HakoAllocObjectLifecyclePageQueue.beginSelection/0
               |--2.53%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--1.71%--HakoAllocObjectLifecycleReleaseResult.reset/0
                --0.86%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
    15.51%  app.exe  app.exe               [.] nyash.object.exact_slot_get_i64_hii
               |--4.60%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
               |--4.18%--HakoAllocPageModel.acquire_usize/1
               |--3.39%--HakoAllocPageModel.isRetired/0
               |--1.66%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
               |--0.85%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
                --0.83%--HakoAllocPageModel.isDecommitted/0
    11.38%  app.exe  app.exe               [.] nyash.object.exact_slot_get_u64_hii
               |--3.66%--HakoAllocObjectLifecyclePageQueue.selectSinglePageFastPath/0
               |--3.44%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--1.74%--HakoAllocPageModel.acquire_usize/1
               |--1.70%--HakoAllocObjectLifecycleAllocResult.recordSuccess/1
                --0.85%--HakoAllocPageModel.releaseLocalKnownLive/1
     4.23%  app.exe  app.exe               [.] nyash.object.exact_slot_set_u64_hiu
               |--1.70%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
               |--0.86%--HakoAllocPageModel.acquire_usize/1
               |--0.85%--HakoAllocPageModel.resetToFresh/0
                --0.82%--HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
     3.44%  app.exe  app.exe               [.] nyash.object.exact_slot_get_handle_hii
               |--1.74%--HakoAllocPageModel.acquire_usize/1
               |--0.85%--HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
                --0.85%--HakoAllocPageModel.releaseLocalKnownLive/1
     2.56%  app.exe  app.exe               [.] nyash.object.exact_slot_set_handle_hii
               |--0.86%--HakoAllocObjectLifecyclePageQueue.beginSelection/0
               |--0.85%--HakoAllocObjectLifecyclePageQueue.acceptSelectedPage/3
                --0.85%--HakoAllocObjectLifecycleFacade.objectLifecycleReleaseDirectCachedPage/2
REPORT

if [ ! -x "$ROOT_DIR/target/release/hakorune" ]; then
  cargo build --release --bin hakorune >/tmp/hakorune_row246_hakorune_build.log
fi

NYASH_FEATURES=rune \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/target/release/hakorune" \
    --backend mir \
    --emit-mir-json "$MIR" \
    "$APP" >/tmp/hakorune_row246_mir_emit.log

python3 "$TOOL" --mir-json "$MIR" --perf-report "$PERF" --owner-selection-report "$OWNER" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=page-model-hotpath-ir-shape-diff-inventory-v0"
require_line "$REPORT" "input_contract=weighted-exact-slot-owner-selection-v0"
require_line "$REPORT" "target_family=page_model_hotpath"
require_line "$REPORT" "target_family_pct=15.29"
require_line "$REPORT" "page_model_method_count=5"
require_line "$REPORT" "missing_page_model_method_count=0"
require_line "$REPORT" "page_model_exact_slot_perf_pct=15.29"
require_line "$REPORT" "page_model_mir_field_get_count=26"
require_line "$REPORT" "page_model_mir_field_set_count=32"
require_line "$REPORT" "page_model_mir_field_op_count=58"
require_line "$REPORT" "page_model_mir_copy_count=62"
require_line "$REPORT" "page_model_mir_call_count=8"
require_line "$REPORT" "selected_method=HakoAllocPageModel.acquire_usize/1"
require_line "$REPORT" "selected_method_pct=8.52"
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

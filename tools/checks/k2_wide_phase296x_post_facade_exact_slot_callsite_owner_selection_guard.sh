#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-234-POST-FACADE-EXACT-SLOT-CALLSITE-OWNER-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-233-POST-FACADE-EXACT-SLOT-CALLSITE-ATTRIBUTION-REFRESH.md"
TOOL="$ROOT_DIR/tools/allocator/post_facade_exact_slot_callsite_owner_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row234_owner_selection.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

ATTR="$TMP_DIR/attr.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row234-owner-selection] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=post-facade-exact-slot-callsite-owner-selection-v0"
require_line "$DOC" "input_contract=typed-object-exact-slot-callsite-attribution-v0"
require_line "$DOC" "dominant_family=object_lifecycle_facade"
require_line "$DOC" "dominant_family_pct=17.36"
require_line "$DOC" "top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "$DOC" "selected_owner=object_lifecycle_facade_residual_exact_slot_field_inventory"
require_line "$DOC" "next_diagnostic=object_lifecycle_facade_residual_exact_slot_field_inventory"
require_line "$DOC" "rejected_owner=repeat_selected_facade_same_block_get_set_fusion"
require_line "$DOC" "rejected_owner_1=generic_typed_field_residence_retry"
require_line "$DOC" "rejected_owner_2=page_queue_followon_keeper"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$ATTR" <<'REPORT'
output_contract=typed-object-exact-slot-callsite-attribution-v0
input_contract=post-selected-facade-get-set-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
attribution_source=perf_callgraph
callgraph_attribution_available=1
exact_slot_get_set_pct=56.37
attributed_callsite_count=29
top_callsite_pct=4.15
top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
top_callsite_helper=nyash.object.exact_slot_get_i64_hii
dominant_family=object_lifecycle_facade
dominant_family_pct=17.36
selected_boundary=exact_slot_callsite_owner_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
REPORT

python3 "$TOOL" --attribution-report "$ATTR" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=post-facade-exact-slot-callsite-owner-selection-v0"
require_line "$REPORT" "input_contract=typed-object-exact-slot-callsite-attribution-v0"
require_line "$REPORT" "workload_id=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "dominant_family=object_lifecycle_facade"
require_line "$REPORT" "dominant_family_pct=17.36"
require_line "$REPORT" "top_callsite_symbol=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "$REPORT" "selected_owner=object_lifecycle_facade_residual_exact_slot_field_inventory"
require_line "$REPORT" "selected_reason=dominant_facade_family_remains_after_selected_fusion"
require_line "$REPORT" "next_diagnostic=object_lifecycle_facade_residual_exact_slot_field_inventory"
require_line "$REPORT" "rejected_owner=repeat_selected_facade_same_block_get_set_fusion"
require_line "$REPORT" "rejected_reason=selected_facade_get_set_fusion_already_landed_and_residual_shape_needs_inventory"
require_line "$REPORT" "rejected_owner_1=generic_typed_field_residence_retry"
require_line "$REPORT" "rejected_owner_2=page_queue_followon_keeper"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

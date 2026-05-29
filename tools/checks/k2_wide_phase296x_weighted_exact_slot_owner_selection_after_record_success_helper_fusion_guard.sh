#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-285-WEIGHTED-EXACT-SLOT-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-284-POST-RECORD-SUCCESS-HELPER-FUSION-OWNER-REFRESH.md"
TOOL="$ROOT_DIR/tools/allocator/weighted_exact_slot_owner_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row285_weighted_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

ATTR="$TMP_DIR/weighted.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row285-weighted-owner-after-record-success] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=weighted-exact-slot-owner-selection-v0"
require_line "$DOC" "input_contract=weighted-exact-slot-callsite-attribution-refresh-v0"
require_line "$DOC" "dominant_family=page_queue_helpers"
require_line "$DOC" "dominant_family_pct=14.32"
require_line "$DOC" "recent_nonkeeper_family=page_queue_helpers"
require_line "$DOC" "recent_nonkeeper_row=296x-241"
require_line "$DOC" "dominant_family_is_recent_nonkeeper=1"
require_line "$DOC" "top_unblocked_family=object_lifecycle_facade"
require_line "$DOC" "top_unblocked_family_pct=13.47"
require_line "$DOC" "selected_family=object_lifecycle_facade"
require_line "$DOC" "selected_owner=facade_exact_slot_ir_shape_diff_inventory"
require_line "$DOC" "selected_reason=dominant_family_is_recent_nonkeeper_select_top_unblocked_family_with_ir_shape_diff"
require_line "$DOC" "next_diagnostic=facade_exact_slot_ir_shape_diff_inventory"
require_line "$DOC" "rejected_owner=page_queue_immediate_retry"
require_line "$DOC" "rejected_owner_2=implementation_without_ir_shape_diff"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$ATTR" <<'REPORT'
output_contract=weighted-exact-slot-callsite-attribution-refresh-v0
input_contract=post-record-success-helper-fusion-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
dominant_family=page_queue_helpers
dominant_family_pct=14.32
recent_nonkeeper_family=page_queue_helpers
recent_nonkeeper_row=296x-241
recent_nonkeeper_candidate_count=21
recent_nonkeeper_hot_per_candidate_pct=0.68
dominant_family_is_recent_nonkeeper=1
top_unblocked_family=object_lifecycle_facade
top_unblocked_family_pct=13.47
static_candidate_count_only_rejected=1
weighted_hot_candidate_score_required=1
ir_shape_diff_required_before_next_keeper=1
summary=ok
REPORT

python3 "$TOOL" --weighted-attribution-report "$ATTR" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=weighted-exact-slot-owner-selection-v0"
require_line "$REPORT" "input_contract=weighted-exact-slot-callsite-attribution-refresh-v0"
require_line "$REPORT" "dominant_family=page_queue_helpers"
require_line "$REPORT" "recent_nonkeeper_family=page_queue_helpers"
require_line "$REPORT" "dominant_family_is_recent_nonkeeper=1"
require_line "$REPORT" "selected_family=object_lifecycle_facade"
require_line "$REPORT" "selected_owner=facade_exact_slot_ir_shape_diff_inventory"
require_line "$REPORT" "selected_reason=dominant_family_is_recent_nonkeeper_select_top_unblocked_family_with_ir_shape_diff"
require_line "$REPORT" "next_diagnostic=facade_exact_slot_ir_shape_diff_inventory"
require_line "$REPORT" "weighted_hot_candidate_score_required=1"
require_line "$REPORT" "ir_shape_diff_required_before_next_keeper=1"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

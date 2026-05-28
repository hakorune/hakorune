#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-265-PAGE-MODEL-RELEASE-KNOWN-LIVE-OWNER-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-264-PAGE-MODEL-RELEASE-KNOWN-LIVE-FIELD-TRAFFIC-PROBE.md"
TOOL="$ROOT_DIR/tools/allocator/page_model_release_known_live_owner_selection.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row265_release_known_live_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

PROBE="$TMP_DIR/probe.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row265-release-known-live-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=page-model-release-known-live-owner-selection-v0"
require_line "$DOC" "rmw_single_use_candidate_count=2"
require_line "$DOC" "rmw_multi_use_candidate_count=2"
require_line "$DOC" "array_bridge_field_get_count=2"
require_line "$DOC" "selected_owner=page_model_release_known_live_single_use_rmw_guard_surface"
require_line "$DOC" "rejected_owner=page_model_release_known_live_multi_use_rmw_fusion"
require_line "$DOC" "rejected_owner_1=page_model_release_known_live_array_bridge_implementation"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$PROBE" <<'REPORT'
output_contract=page-model-release-known-live-field-traffic-probe-v0
input_contract=page-model-hotpath-shape-owner-selection-v0
workload_id=representative-object-lifecycle-small-block-v0
target_method=HakoAllocPageModel.releaseLocalKnownLive/1
target_method_pct=4.14
rmw_candidate_count=4
rmw_single_use_candidate_count=2
rmw_multi_use_candidate_count=2
array_bridge_field_get_count=2
implementation_open=0
summary=ok
REPORT

python3 "$TOOL" --probe-report "$PROBE" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=page-model-release-known-live-owner-selection-v0"
require_line "$REPORT" "input_contract=page-model-release-known-live-field-traffic-probe-v0"
require_line "$REPORT" "target_method=HakoAllocPageModel.releaseLocalKnownLive/1"
require_line "$REPORT" "target_method_pct=4.14"
require_line "$REPORT" "rmw_candidate_count=4"
require_line "$REPORT" "rmw_single_use_candidate_count=2"
require_line "$REPORT" "rmw_multi_use_candidate_count=2"
require_line "$REPORT" "array_bridge_field_get_count=2"
require_line "$REPORT" "multi_use_rmw_immediate_implementation_blocked=1"
require_line "$REPORT" "array_bridge_immediate_implementation_blocked=1"
require_line "$REPORT" "selected_owner=page_model_release_known_live_single_use_rmw_guard_surface"
require_line "$REPORT" "selected_reason=single_use_rmw_candidates_have_positive_helper_call_delta"
require_line "$REPORT" "next_row=page_model_release_known_live_single_use_rmw_guard_surface"
require_line "$REPORT" "rejected_owner=page_model_release_known_live_multi_use_rmw_fusion"
require_line "$REPORT" "rejected_owner_1=page_model_release_known_live_array_bridge_implementation"
require_line "$REPORT" "implementation_open=0"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

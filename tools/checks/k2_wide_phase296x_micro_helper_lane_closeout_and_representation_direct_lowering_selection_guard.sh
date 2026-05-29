#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-297-MICRO-HELPER-LANE-CLOSEOUT-AND-REPRESENTATION-DIRECT-LOWERING-SELECTION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-296-ALLOC-RESULT-CAPSULE-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row297-micro-helper-closeout] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=micro-helper-lane-closeout-and-representation-direct-lowering-selection-v0"
require_line "$DOC" "input_contract=alloc-result-capsule-owner-selection-after-record-success-helper-fusion-v0"
require_line "$DOC" "row284_exact_slot_get_set_pct=50.97"
require_line "$DOC" "excluded_family_0=page_queue_helpers"
require_line "$DOC" "excluded_reason_0=row241_recent_no_effect"
require_line "$DOC" "excluded_family_1=object_lifecycle_facade"
require_line "$DOC" "excluded_family_2=page_model_hotpath"
require_line "$DOC" "excluded_family_3=release_result_capsule"
require_line "$DOC" "excluded_family_4=alloc_result_capsule"
require_line "$DOC" "remaining_small_helper_keeper_count=0"
require_line "$DOC" "selected_owner=representation_direct_lowering_ssot"
require_line "$DOC" "selected_reason=helper_calls_remain_large_but_small_helper_owner_table_is_exhausted"
require_line "$DOC" "next_row=representation_direct_lowering_ssot"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

echo "[row297-micro-helper-closeout] ok"

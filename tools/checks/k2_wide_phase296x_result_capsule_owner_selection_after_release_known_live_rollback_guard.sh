#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-276-RESULT-CAPSULE-OWNER-SELECTION-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-275-ALLOC-RESULT-CAPSULE-IR-SHAPE-INVENTORY-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK.md"
TOOL="$ROOT_DIR/tools/allocator/result_capsule_owner_selection_after_rollback.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row276_result_capsule_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row276-result-capsule-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=result-capsule-owner-selection-after-release-known-live-rollback-v0"
require_line "$DOC" "input_contract=alloc-result-capsule-ir-shape-inventory-after-release-known-live-rollback-v0"
require_line "$DOC" "selected_owner=result_capsule_record_success_shape_guard_surface"
require_line "$DOC" "selected_owner_kind=branch_aware_exact_slot_rmw_and_status_set_plan"
require_line "$DOC" "alloc_record_success_field_op_count=8"
require_line "$DOC" "release_record_success_field_op_count=6"
require_line "$DOC" "record_success_combined_field_op_count=14"
require_line "$DOC" "requires_guard_surface_before_implementation=1"
require_line "$DOC" "requires_hako_source_change=0"
require_line "$DOC" "selected_next=result_capsule_record_success_shape_guard_surface"
require_line "$DOC" "rejected_owner=result_capsule_reset_field_batching"
require_line "$DOC" "rejected_reason=result_capsule_reset_field_batching_already_landed_in_row259"
require_line "$DOC" "rejected_owner_4=source_inline_success_result_fast_path"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

python3 "$TOOL" --inventory-report "$PREV" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=result-capsule-owner-selection-after-release-known-live-rollback-v0"
require_line "$REPORT" "selected_owner=result_capsule_record_success_shape_guard_surface"
require_line "$REPORT" "selected_owner_kind=branch_aware_exact_slot_rmw_and_status_set_plan"
require_line "$REPORT" "alloc_record_success_field_op_count=8"
require_line "$REPORT" "release_record_success_field_op_count=6"
require_line "$REPORT" "record_success_combined_field_op_count=14"
require_line "$REPORT" "selected_next=result_capsule_record_success_shape_guard_surface"
require_line "$REPORT" "rejected_owner=result_capsule_reset_field_batching"
require_line "$REPORT" "rejected_owner_4=source_inline_success_result_fast_path"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-291-POST-PAGE-MODEL-HOTPATH-OWNER-REFRESH-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-290-PAGE-MODEL-HOTPATH-SHAPE-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
SOURCE="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-284-POST-RECORD-SUCCESS-HELPER-FUSION-OWNER-REFRESH.md"
TOOL="$ROOT_DIR/tools/allocator/post_page_model_hotpath_owner_refresh_after_record_success.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row291_owner_refresh.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row291-post-page-model-owner-refresh] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$SOURCE" "Status: Landed"
require_line "$DOC" "output_contract=post-page-model-hotpath-owner-refresh-after-record-success-helper-fusion-v0"
require_line "$DOC" "input_contract=page-model-hotpath-shape-owner-selection-v0"
require_line "$DOC" "source_exact_slot_get_set_pct=50.97"
require_line "$DOC" "excluded_family_0=page_queue_helpers"
require_line "$DOC" "excluded_family_1=object_lifecycle_facade"
require_line "$DOC" "excluded_family_2=page_model_hotpath"
require_line "$DOC" "selected_family=release_result_capsule"
require_line "$DOC" "selected_family_pct=2.59"
require_line "$DOC" "selected_owner=release_result_capsule_ir_shape_inventory_after_record_success_helper_fusion"
require_line "$DOC" "next_diagnostic=release_result_capsule_ir_shape_inventory_after_record_success_helper_fusion"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

python3 "$TOOL" \
  --owner-refresh-report "$SOURCE" \
  --page-model-selection-report "$PREV" \
  --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=post-page-model-hotpath-owner-refresh-after-record-success-helper-fusion-v0"
require_line "$REPORT" "input_contract=page-model-hotpath-shape-owner-selection-v0"
require_line "$REPORT" "source_exact_slot_get_set_pct=50.97"
require_line "$REPORT" "selected_family=release_result_capsule"
require_line "$REPORT" "selected_family_pct=2.59"
require_line "$REPORT" "selected_owner=release_result_capsule_ir_shape_inventory_after_record_success_helper_fusion"
require_line "$REPORT" "next_diagnostic=release_result_capsule_ir_shape_inventory_after_record_success_helper_fusion"
require_line "$REPORT" "family_0_name=page_queue_helpers"
require_line "$REPORT" "family_1_name=object_lifecycle_facade"
require_line "$REPORT" "family_2_name=page_model_hotpath"
require_line "$REPORT" "family_3_name=release_result_capsule"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-294-POST-RELEASE-RESULT-CAPSULE-OWNER-REFRESH-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-293-RELEASE-RESULT-CAPSULE-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
SOURCE="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-284-POST-RECORD-SUCCESS-HELPER-FUSION-OWNER-REFRESH.md"
TOOL="$ROOT_DIR/tools/allocator/post_release_result_capsule_owner_refresh_after_record_success.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row294_owner_refresh.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row294-post-release-result-owner-refresh] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$SOURCE" "Status: Landed"
require_line "$DOC" "output_contract=post-release-result-capsule-owner-refresh-after-record-success-helper-fusion-v0"
require_line "$DOC" "source_exact_slot_get_set_pct=50.97"
require_line "$DOC" "excluded_family_0=page_queue_helpers"
require_line "$DOC" "excluded_family_1=object_lifecycle_facade"
require_line "$DOC" "excluded_family_2=page_model_hotpath"
require_line "$DOC" "excluded_family_3=release_result_capsule"
require_line "$DOC" "selected_family=alloc_result_capsule"
require_line "$DOC" "selected_family_pct=2.19"
require_line "$DOC" "selected_owner=alloc_result_capsule_ir_shape_inventory_after_record_success_helper_fusion"
require_line "$DOC" "remaining_family_is_small=1"
require_line "$DOC" "micro_helper_stop_line_near=1"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

python3 "$TOOL" \
  --owner-refresh-report "$SOURCE" \
  --release-selection-report "$PREV" \
  --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=post-release-result-capsule-owner-refresh-after-record-success-helper-fusion-v0"
require_line "$REPORT" "input_contract=release-result-capsule-owner-selection-after-record-success-helper-fusion-v0"
require_line "$REPORT" "source_exact_slot_get_set_pct=50.97"
require_line "$REPORT" "selected_family=alloc_result_capsule"
require_line "$REPORT" "selected_family_pct=2.19"
require_line "$REPORT" "selected_owner=alloc_result_capsule_ir_shape_inventory_after_record_success_helper_fusion"
require_line "$REPORT" "remaining_family_is_small=1"
require_line "$REPORT" "micro_helper_stop_line_near=1"
require_line "$REPORT" "family_4_name=alloc_result_capsule"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

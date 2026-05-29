#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-293-RELEASE-RESULT-CAPSULE-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-292-RELEASE-RESULT-CAPSULE-IR-SHAPE-INVENTORY-AFTER-RECORD-SUCCESS-HELPER-FUSION.md"
TOOL="$ROOT_DIR/tools/allocator/release_result_capsule_owner_selection_after_record_success.py"
TMP_DIR="$(mktemp -d /tmp/hakorune_row293_release_result_owner.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

INV="$TMP_DIR/inventory.out"
REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row293-release-result-owner] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=release-result-capsule-owner-selection-after-record-success-helper-fusion-v0"
require_line "$DOC" "release_result_field_op_count=25"
require_line "$DOC" "record_success_helper_fusion_landed=1"
require_line "$DOC" "record_success_repeat_closed=1"
require_line "$DOC" "selected_owner=post_release_result_capsule_owner_refresh_after_record_success_helper_fusion"
require_line "$DOC" "rejected_owner=release_result_record_success_helper_fusion_repeat"
require_line "$DOC" "rejected_owner_1=release_result_birth_batching"
require_line "$DOC" "implementation_open=0"
require_line "$DOC" "optimization_open=0"
require_line "$DOC" "winner_claim=0"
require_line "$DOC" "replacement_active=0"
require_line "$DOC" "hook_installed=0"
require_line "$DOC" "global_allocator=0"
require_line "$DOC" "summary=ok"

cat >"$INV" <<'REPORT'
output_contract=release-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0
input_contract=post-page-model-hotpath-owner-refresh-after-record-success-helper-fusion-v0
workload_id=representative-object-lifecycle-small-block-v0
release_result_field_op_count=25
top_release_method=HakoAllocObjectLifecycleReleaseResult.birth/0
top_release_method_field_op_count=6
top_release_hot_method=HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
top_release_hot_method_field_op_count=6
record_success_helper_fusion_landed=1
record_success_repeat_closed=1
summary=ok
REPORT

python3 "$TOOL" --inventory-report "$INV" --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=release-result-capsule-owner-selection-after-record-success-helper-fusion-v0"
require_line "$REPORT" "input_contract=release-result-capsule-ir-shape-inventory-after-record-success-helper-fusion-v0"
require_line "$REPORT" "release_result_field_op_count=25"
require_line "$REPORT" "record_success_helper_fusion_landed=1"
require_line "$REPORT" "record_success_repeat_closed=1"
require_line "$REPORT" "selected_owner=post_release_result_capsule_owner_refresh_after_record_success_helper_fusion"
require_line "$REPORT" "selected_reason=record_success_already_fused_and_remaining_birth_setup_shape_is_not_current_hot_keeper"
require_line "$REPORT" "next_diagnostic=post_release_result_capsule_owner_refresh_after_record_success_helper_fusion"
require_line "$REPORT" "rejected_owner=release_result_record_success_helper_fusion_repeat"
require_line "$REPORT" "rejected_owner_1=release_result_birth_batching"
require_line "$REPORT" "implementation_open=0"
require_line "$REPORT" "optimization_open=0"
require_line "$REPORT" "winner_claim=0"
require_line "$REPORT" "replacement_active=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator=0"
require_line "$REPORT" "summary=ok"

cat "$REPORT"

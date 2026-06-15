#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-728-EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-727-EXACT-OBJECT-FLATTENED-NESTED-FIELD-PLAN-EXPORT-001.md"
SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
TOOL="tools/allocator/hako_exact_object_flattened_nested_field_boundary_consumer.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_flattened_nested_field_boundary_consumer_guard.sh"

[[ -f "$CARD" ]] || { echo "[flattened-nested-boundary-consumer] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[flattened-nested-boundary-consumer] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$SSOT" ]] || { echo "[flattened-nested-boundary-consumer] missing SSOT: $SSOT" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[flattened-nested-boundary-consumer] missing tool: $TOOL" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[flattened-nested-boundary-consumer] row728 card must be Active or Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[flattened-nested-boundary-consumer] row727 card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[flattened-nested-boundary-consumer] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[flattened-nested-boundary-consumer] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-flattened-nested-field-boundary-consumer-v0"
require_line_in_file "$CARD" "source_evidence=296x-727"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "object_storage_plan_mir_json_export_enabled=1"
require_line_in_file "$CARD" "boundary_driver_flattened_nested_consumer=1"
require_line_in_file "$CARD" "uses_object_storage_plan_metadata=1"
require_line_in_file "$CARD" "alignment_result_last_requested_consumed=1"
require_line_in_file "$CARD" "alignment_result_last_normalized_consumed=1"
require_line_in_file "$CARD" "alignment_result_last_reason_consumed=1"
require_line_in_file "$CARD" "alignment_result_last_supported_consumed=1"
require_line_in_file "$CARD" "field_access_lowering_connected=1"
require_line_in_file "$CARD" "nested_method_lowering_connected=1"
require_line_in_file "$CARD" "generated_artifact_reachability_proven=1"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "benchmark_name_branch_count=0"
require_line_in_file "$CARD" "helper_name_branch_count=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "fallback_to_generic_box_supported=1"
require_line_in_file "$CARD" "backend_lowering_enabled=1"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-PILOT-001V"

require_line_in_file "$SSOT" "global_arc_retirement_enabled=0"
require_line_in_file "$SSOT" "object_boundary_removal_owner=exact_aot_backend"
require_line_in_file "$SSOT" "mirbuilder_object_boundary_removal_owner=0"

python3 -m py_compile "$TOOL"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.txt"

python3 "$TOOL" --repo-root "$ROOT" --out "$report"

require_line_in_file "$report" "output_contract=hako-exact-object-flattened-nested-field-boundary-consumer-v0"
require_line_in_file "$report" "boundary_driver_flattened_nested_consumer=1"
require_line_in_file "$report" "uses_object_storage_plan_metadata=1"
require_line_in_file "$report" "field_access_lowering_connected=1"
require_line_in_file "$report" "nested_method_lowering_connected=1"
require_line_in_file "$report" "generated_artifact_reachability_proven=1"
require_line_in_file "$report" "mirbuilder_object_management_enabled=0"
require_line_in_file "$report" "benchmark_name_branch_count=0"
require_line_in_file "$report" "helper_name_branch_count=0"
require_line_in_file "$report" "product_default_changed=0"
require_line_in_file "$report" "fallback_to_generic_box_supported=1"
require_line_in_file "$report" "backend_lowering_enabled=1"
require_line_in_file "$report" "selected_next=EXACT-OBJECT-PILOT-001V"
require_line_in_file "$report" "summary=ok"

bash tools/build_hako_llvmc_ffi.sh

echo "[flattened-nested-boundary-consumer] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-727-EXACT-OBJECT-FLATTENED-NESTED-FIELD-PLAN-EXPORT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-726-EXACT-OBJECT-FLATTENED-NESTED-FIELD-BACKEND-REACHABILITY-001.md"
TOOL="tools/allocator/hako_exact_object_flattened_nested_field_plan_export.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_flattened_nested_field_plan_export_guard.sh"

[[ -f "$CARD" ]] || { echo "[flattened-nested-plan-export] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[flattened-nested-plan-export] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[flattened-nested-plan-export] missing tool: $TOOL" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || { echo "[flattened-nested-plan-export] row727 card must be Active or Landed" >&2; exit 1; }
grep -q '^Status: Landed / Blocked$' "$PREV_CARD" || { echo "[flattened-nested-plan-export] row726 card must be Landed / Blocked" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[flattened-nested-plan-export] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[flattened-nested-plan-export] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-flattened-nested-field-plan-export-v0"
require_line_in_file "$CARD" "source_evidence=296x-726"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "object_storage_plan_mir_json_export_enabled=1"
require_line_in_file "$CARD" "backend_lowering_enabled=0"
require_line_in_file "$CARD" "boundary_driver_flattened_nested_consumer=0"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "benchmark_name_branch_count=0"
require_line_in_file "$CARD" "helper_name_branch_count=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001"

cargo test -q collect_object_storage_plan_values --lib
cargo test -q build_mir_json_root_includes_object_storage_plans_surface --lib

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
report="$tmp_dir/report.txt"
cat > "$mir_json" <<'JSON'
{
  "functions": [],
  "object_storage_plans": [
    {
      "representation": "flattened_nested_fields",
      "source_evidence": "296x-726",
      "owner_box": "HakoAllocObjectLifecycleFacade",
      "owner_field": "alignment_result",
      "owner_layout_id": 3,
      "nested_box": "HakoAllocObjectLifecycleAlignmentResult",
      "nested_layout_id": 1,
      "flattened_field_count": 4,
      "fields": [
        {"flattened_field": "alignment_result.last_requested"},
        {"flattened_field": "alignment_result.last_normalized"},
        {"flattened_field": "alignment_result.last_reason"},
        {"flattened_field": "alignment_result.last_supported"}
      ],
      "backend_lowering_enabled": false,
      "boundary_driver_flattened_nested_consumer": false,
      "mirbuilder_object_management_enabled": false,
      "product_default_changed": false
    }
  ]
}
JSON

python3 "$TOOL" --mir-json "$mir_json" --out "$report"

require_line_in_file "$report" "output_contract=hako-exact-object-flattened-nested-field-plan-export-v0"
require_line_in_file "$report" "flattened_nested_plan_count=1"
require_line_in_file "$report" "flattened_nested_field_count=4"
require_line_in_file "$report" "alignment_result_last_requested_exported=1"
require_line_in_file "$report" "alignment_result_last_normalized_exported=1"
require_line_in_file "$report" "alignment_result_last_reason_exported=1"
require_line_in_file "$report" "alignment_result_last_supported_exported=1"
require_line_in_file "$report" "backend_lowering_enabled=0"
require_line_in_file "$report" "boundary_driver_flattened_nested_consumer=0"
require_line_in_file "$report" "mirbuilder_object_management_enabled=0"
require_line_in_file "$report" "benchmark_name_branch_count=0"
require_line_in_file "$report" "helper_name_branch_count=0"
require_line_in_file "$report" "product_default_changed=0"
require_line_in_file "$report" "selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001"
require_line_in_file "$report" "summary=ok"

echo "[flattened-nested-plan-export] ok"

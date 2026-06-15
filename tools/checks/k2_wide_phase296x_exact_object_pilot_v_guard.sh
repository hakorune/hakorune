#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-729-EXACT-OBJECT-PILOT-001V.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-728-EXACT-OBJECT-FLATTENED-NESTED-FIELD-BOUNDARY-CONSUMER-001.md"
TOOL="tools/allocator/hako_exact_object_pilot_v_preflight.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_pilot_v_guard.sh"

[[ -f "$CARD" ]] || { echo "[exact-object-pilot-v] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[exact-object-pilot-v] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[exact-object-pilot-v] missing tool: $TOOL" >&2; exit 1; }

grep -Eq '^Status: (Active|Landed)$' "$CARD" || {
  echo "[exact-object-pilot-v] row729 card must be Active or Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[exact-object-pilot-v] row728 card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[exact-object-pilot-v] check index missing guard entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[exact-object-pilot-v] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-pilot-v-v0"
require_line_in_file "$CARD" "source_evidence=296x-728"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "nested_owner=HakoAllocObjectLifecycleFacade.alignment_result"
require_line_in_file "$CARD" "nested_object=HakoAllocObjectLifecycleAlignmentResult"
require_line_in_file "$CARD" "representation_choice=flattened_nested_fields"
require_line_in_file "$CARD" "boundary_driver_flattened_nested_consumer=1"
require_line_in_file "$CARD" "field_access_lowering_connected=1"
require_line_in_file "$CARD" "nested_method_lowering_connected=1"
require_line_in_file "$CARD" "generated_artifact_reachability_proven=1"
require_line_in_file "$CARD" "backend_lowering_enabled=1"
require_line_in_file "$CARD" "object_storage_plan_execution_enabled=1"
require_line_in_file "$CARD" "pilot_exact_object_enabled=1"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "benchmark_name_branch_count=0"
require_line_in_file "$CARD" "helper_name_branch_count=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "fallback_to_generic_box_supported=1"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-PILOT-MEASUREMENT-002"
require_line_in_file "$CARD" "summary=ok"

python3 -m py_compile "$TOOL"

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.txt"

python3 "$TOOL" --repo-root "$ROOT" --out "$report"

require_line_in_file "$report" "output_contract=hako-exact-object-pilot-v-v0"
require_line_in_file "$report" "boundary_driver_flattened_nested_consumer=1"
require_line_in_file "$report" "field_access_lowering_connected=1"
require_line_in_file "$report" "nested_method_lowering_connected=1"
require_line_in_file "$report" "generated_artifact_reachability_proven=1"
require_line_in_file "$report" "backend_lowering_enabled=1"
require_line_in_file "$report" "object_storage_plan_execution_enabled=1"
require_line_in_file "$report" "pilot_exact_object_enabled=1"
require_line_in_file "$report" "mirbuilder_object_management_enabled=0"
require_line_in_file "$report" "benchmark_name_branch_count=0"
require_line_in_file "$report" "helper_name_branch_count=0"
require_line_in_file "$report" "product_default_changed=0"
require_line_in_file "$report" "fallback_to_generic_box_supported=1"
require_line_in_file "$report" "selected_next=EXACT-OBJECT-PILOT-MEASUREMENT-002"
require_line_in_file "$report" "summary=ok"

bash tools/checks/k2_wide_phase296x_exact_object_flattened_nested_field_boundary_consumer_guard.sh

echo "[exact-object-pilot-v] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-715-EXACT-OBJECT-PILOT-001R.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-714-EXACT-OBJECT-NESTED-PUBLICATION-PLAN-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-716-EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001.md"
SSOT="docs/development/current/main/design/object-storage-plan-boundary-ssot.md"
TOOL="tools/allocator/hako_exact_object_pilot_retry_preflight.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_pilot_retry_guard.sh"

[[ -f "$CARD" ]] || { echo "[exact-object-pilot-r] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[exact-object-pilot-r] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[exact-object-pilot-r] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$SSOT" ]] || { echo "[exact-object-pilot-r] missing ssot: $SSOT" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[exact-object-pilot-r] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[exact-object-pilot-r] row715 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[exact-object-pilot-r] row714 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[exact-object-pilot-r] row716 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[exact-object-pilot-r] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[exact-object-pilot-r] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-pilot-r-v0"
require_line_in_file "$CARD" "source_evidence=296x-714"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "nested_owner=HakoAllocObjectLifecycleFacade.alignment_result"
require_line_in_file "$CARD" "nested_object=HakoAllocObjectLifecycleAlignmentResult"
require_line_in_file "$CARD" "representation_choice=flatten_nested_fields"
require_line_in_file "$CARD" "object_storage_plan_execution_enabled=0"
require_line_in_file "$CARD" "pilot_exact_object_enabled=0"
require_line_in_file "$CARD" "flattened_nested_field_count=4"
require_line_in_file "$CARD" "nested_receiver_call_count=7"
require_line_in_file "$CARD" "backend_flattened_nested_field_consumer=0"
require_line_in_file "$CARD" "existing_known_receiver_direct_call_requires_handle=1"
require_line_in_file "$CARD" "local_aggregate_published_nested_consumer=0"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "mirbuilder_special_case_count=0"
require_line_in_file "$CARD" "benchmark_name_branch_count=0"
require_line_in_file "$CARD" "helper_name_branch_count=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "fallback_to_generic_box_supported=1"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001"
require_line_in_file "$CARD" "summary=blocked"

require_line_in_file "$NEXT_CARD" "Task: EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001"
require_line_in_file "$NEXT_CARD" "output_contract=hako-exact-object-flattened-nested-field-layout-ssot-v0"
require_line_in_file "$NEXT_CARD" "selected_next=EXACT-OBJECT-FLATTENED-NESTED-FIELD-SHADOW-001"

grep -q 'EXACT-OBJECT-FLATTENED-NESTED-FIELD-LAYOUT-SSOT-001' "$SSOT" || { echo "[exact-object-pilot-r] ssot missing layout task" >&2; exit 1; }
grep -q 'output_contract=hako-exact-object-pilot-r-v0' "$TOOL" || { echo "[exact-object-pilot-r] tool missing output contract" >&2; exit 1; }
grep -q 'backend_flattened_nested_field_consumer' "$TOOL" || { echo "[exact-object-pilot-r] tool missing backend consumer evidence" >&2; exit 1; }
grep -q 'existing_known_receiver_direct_call_requires_handle' "$TOOL" || { echo "[exact-object-pilot-r] tool missing handle-receiver evidence" >&2; exit 1; }

echo "[exact-object-pilot-r] ok"

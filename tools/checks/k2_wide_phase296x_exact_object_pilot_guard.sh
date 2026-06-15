#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-713-EXACT-OBJECT-PILOT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-712-EXACT-OBJECT-PLAN-SHADOW-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-714-EXACT-OBJECT-NESTED-PUBLICATION-PLAN-001.md"
TOOL="tools/allocator/hako_exact_object_pilot_preflight.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_pilot_guard.sh"

[[ -f "$CARD" ]] || { echo "[exact-object-pilot] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[exact-object-pilot] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[exact-object-pilot] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[exact-object-pilot] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[exact-object-pilot] row713 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[exact-object-pilot] row712 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[exact-object-pilot] row714 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[exact-object-pilot] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[exact-object-pilot] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-pilot-v0"
require_line_in_file "$CARD" "source_evidence=296x-712"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "pilot_candidate=HakoAllocObjectLifecycleAlignmentResult"
require_line_in_file "$CARD" "object_storage_plan_execution_enabled=0"
require_line_in_file "$CARD" "pilot_exact_object_enabled=0"
require_line_in_file "$CARD" "closed_world_plan_required=1"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "mirbuilder_special_case_count=0"
require_line_in_file "$CARD" "benchmark_name_branch_count=0"
require_line_in_file "$CARD" "helper_name_branch_count=0"
require_line_in_file "$CARD" "observed_publication_boundary=Facade.alignment_result_handle_field"
require_line_in_file "$CARD" "publication_boundary_count=8"
require_line_in_file "$CARD" "facade_alignment_result_set_count=1"
require_line_in_file "$CARD" "facade_alignment_result_get_count=7"
require_line_in_file "$CARD" "candidate_method_call_count=8"
require_line_in_file "$CARD" "candidate_birth_call_count=1"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "runtime_object_changed=0"
require_line_in_file "$CARD" "fallback_to_generic_box_supported=1"
require_line_in_file "$CARD" "selected_next=EXACT-OBJECT-NESTED-PUBLICATION-PLAN-001"
require_line_in_file "$CARD" "summary=blocked"
require_line_in_file "$NEXT_CARD" "Task: EXACT-OBJECT-NESTED-PUBLICATION-PLAN-001"

grep -q 'output_contract=hako-exact-object-pilot-v0' "$TOOL" || { echo "[exact-object-pilot] tool missing output contract" >&2; exit 1; }
grep -q 'object_storage_plan_execution_enabled=0' "$TOOL" || { echo "[exact-object-pilot] preflight must keep execution disabled" >&2; exit 1; }

echo "[exact-object-pilot] ok"

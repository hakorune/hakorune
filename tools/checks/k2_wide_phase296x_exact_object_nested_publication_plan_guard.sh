#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-714-EXACT-OBJECT-NESTED-PUBLICATION-PLAN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-713-EXACT-OBJECT-PILOT-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-715-EXACT-OBJECT-PILOT-001R.md"
TOOL="tools/allocator/hako_exact_object_nested_publication_plan.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_nested_publication_plan_guard.sh"

[[ -f "$CARD" ]] || { echo "[exact-object-nested-publication] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[exact-object-nested-publication] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[exact-object-nested-publication] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[exact-object-nested-publication] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[exact-object-nested-publication] row714 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[exact-object-nested-publication] row713 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[exact-object-nested-publication] row715 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[exact-object-nested-publication] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[exact-object-nested-publication] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-nested-publication-plan-v0"
require_line_in_file "$CARD" "source_evidence=296x-713"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "nested_owner=HakoAllocObjectLifecycleFacade.alignment_result"
require_line_in_file "$CARD" "nested_object=HakoAllocObjectLifecycleAlignmentResult"
require_line_in_file "$CARD" "publication_boundary_count=8"
require_line_in_file "$CARD" "facade_nested_field_set_count=1"
require_line_in_file "$CARD" "facade_nested_field_get_count=7"
require_line_in_file "$CARD" "nested_receiver_call_count=7"
require_line_in_file "$CARD" "nested_handle_escape_count=0"
require_line_in_file "$CARD" "representation_choice=flatten_nested_fields"
require_line_in_file "$CARD" "mirbuilder_object_management_enabled=0"
require_line_in_file "$CARD" "benchmark_name_branch_count=0"
require_line_in_file "$CARD" "helper_name_branch_count=0"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "fallback_to_generic_box_supported=1"
require_line_in_file "$CARD" "summary=ok"
require_line_in_file "$NEXT_CARD" "Task: EXACT-OBJECT-PILOT-001R"

grep -q 'output_contract=hako-exact-object-nested-publication-plan-v0' "$TOOL" || { echo "[exact-object-nested-publication] tool missing output contract" >&2; exit 1; }
grep -q 'flatten_nested_fields' "$TOOL" || { echo "[exact-object-nested-publication] tool must know flatten_nested_fields" >&2; exit 1; }

echo "[exact-object-nested-publication] ok"

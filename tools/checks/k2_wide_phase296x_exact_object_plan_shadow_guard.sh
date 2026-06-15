#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-712-EXACT-OBJECT-PLAN-SHADOW-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-711-OBJECT-STORAGE-PLAN-SSOT-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-713-EXACT-OBJECT-PILOT-001.md"
TOOL="tools/allocator/hako_exact_object_plan_shadow.py"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_exact_object_plan_shadow_guard.sh"

[[ -f "$CARD" ]] || { echo "[exact-object-plan-shadow] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[exact-object-plan-shadow] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[exact-object-plan-shadow] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[exact-object-plan-shadow] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[exact-object-plan-shadow] row712 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[exact-object-plan-shadow] row711 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[exact-object-plan-shadow] row713 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[exact-object-plan-shadow] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[exact-object-plan-shadow] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-exact-object-plan-shadow-v0"
require_line_in_file "$CARD" "source_evidence=296x-711"
require_line_in_file "$CARD" "target_front=object_lifecycle_body"
require_line_in_file "$CARD" "object_storage_plan_vocabulary_defined=1"
require_line_in_file "$CARD" "object_storage_plan_execution_enabled=0"
require_line_in_file "$CARD" "exact_object_shadow_enabled=1"
require_line_in_file "$CARD" "generic_box_plan_count=1"
require_line_in_file "$CARD" "host_handle_escaped_plan_count=4"
require_line_in_file "$CARD" "arc_dynbox_plan_count=2"
require_line_in_file "$CARD" "exact_stack_object_plan_count=5"
require_line_in_file "$CARD" "exact_native_struct_plan_count=9"
require_line_in_file "$CARD" "scalarized_plan_count=5"
require_line_in_file "$CARD" "selected_pilot_candidate=HakoAllocObjectLifecycleAlignmentResult"
require_line_in_file "$CARD" "selected_pilot_confidence=medium"
require_line_in_file "$CARD" "pilot_allowed=1"
require_line_in_file "$CARD" "product_default_changed=0"
require_line_in_file "$CARD" "compiler_lowering_changed=0"
require_line_in_file "$CARD" "runtime_object_changed=0"
require_line_in_file "$CARD" "summary=ok"
require_line_in_file "$NEXT_CARD" "Task: EXACT-OBJECT-PILOT-001"
require_line_in_file "$NEXT_CARD" "pilot_candidate=HakoAllocObjectLifecycleAlignmentResult"

grep -q 'output_contract=hako-exact-object-plan-shadow-v0' "$TOOL" || { echo "[exact-object-plan-shadow] tool missing output contract" >&2; exit 1; }
grep -q 'object_storage_plan_execution_enabled=0' "$TOOL" || { echo "[exact-object-plan-shadow] tool must keep execution disabled" >&2; exit 1; }

echo "[exact-object-plan-shadow] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-692-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-691-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-DESIGN-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-693-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-IMPLEMENTATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_dominance_required_forwarding_guard_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[call-operand-dominance-guard-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-dominance-guard-surface] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[call-operand-dominance-guard-surface] missing next card: $NEXT_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[call-operand-dominance-guard-surface] row692 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[call-operand-dominance-guard-surface] row691 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[call-operand-dominance-guard-surface] row693 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[call-operand-dominance-guard-surface] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[call-operand-dominance-guard-surface] missing line in $file: $expected" >&2
    exit 1
  fi
}

for file in "$CARD" "$NEXT_CARD"; do
  require_line_in_file "$file" "selected_keeper_shape=dominance_guarded_receiver_operand_forwarding"
  require_line_in_file "$file" "arg_forwarding_enabled=0"
  require_line_in_file "$file" "requires_dominance_guard=1"
  require_line_in_file "$file" "helper_name_special_case=0"
  require_line_in_file "$file" "variable_map_semantics_changed=0"
  require_line_in_file "$file" "phi_lifecycle_changed=0"
  require_line_in_file "$file" "optimization_open=0"
  require_line_in_file "$file" "winner_claim=0"
done

require_line_in_file "$CARD" "output_contract=hako-mimalloc-call-operand-dominance-required-forwarding-guard-surface-v0"
require_line_in_file "$CARD" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line_in_file "$CARD" "source_evidence=296x-691"
require_line_in_file "$CARD" "pre_selected_keeper_candidate_count=13"
require_line_in_file "$CARD" "post_selected_keeper_candidate_count_target=0"
require_line_in_file "$CARD" "post_call_operand_unique_copy_count_upper_bound=14"
require_line_in_file "$CARD" "implementation_started=0"
require_line_in_file "$CARD" "next_task=call_operand_dominance_required_forwarding_implementation"
require_line_in_file "$CARD" "summary=ok"

require_line_in_file "$NEXT_CARD" "Task: CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-IMPLEMENTATION-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-692"
require_line_in_file "$NEXT_CARD" "post_selected_keeper_candidate_count=13"
require_line_in_file "$NEXT_CARD" "post_call_operand_unique_copy_count_upper_bound=14"
require_line_in_file "$NEXT_CARD" "implementation_started=0"
require_line_in_file "$NEXT_CARD" "summary=pending"

echo "[call-operand-dominance-guard-surface] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-686-CALL-OPERAND-MATERIALIZATION-FORWARDING-GUARD-SURFACE-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-685-CALL-OPERAND-MATERIALIZATION-FORWARDING-DESIGN-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-687-CALL-OPERAND-MATERIALIZATION-FORWARDING-IMPLEMENTATION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_materialization_forwarding_guard_surface_guard.sh"

[[ -f "$CARD" ]] || { echo "[call-operand-forwarding-guard-surface] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-forwarding-guard-surface] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[call-operand-forwarding-guard-surface] missing next card: $NEXT_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[call-operand-forwarding-guard-surface] row686 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[call-operand-forwarding-guard-surface] row685 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[call-operand-forwarding-guard-surface] row687 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[call-operand-forwarding-guard-surface] check index missing guard entry" >&2; exit 1; }

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$CARD"; then
    echo "[call-operand-forwarding-guard-surface] missing card line: $expected" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-call-operand-materialization-forwarding-guard-surface-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "source_evidence=296x-685"
require_line "selected_keeper_shape=same_block_root_receiver_operand_forwarding"
require_line "pre_selected_keeper_candidate_count=2"
require_line "post_selected_keeper_candidate_count_target=0"
require_line "post_call_operand_unique_copy_count_upper_bound=27"
require_line "arg_forwarding_enabled=0"
require_line "helper_name_special_case=0"
require_line "requires_dominance_guard=0"
require_line "variable_map_semantics_changed=0"
require_line "phi_lifecycle_changed=0"
require_line "implementation_started=0"
require_line "optimization_open=0"
require_line "winner_claim=0"
require_line "summary=ok"

grep -q 'do not enable Arg forwarding' "$CARD" || {
  echo "[call-operand-forwarding-guard-surface] card must keep Arg forwarding closed" >&2
  exit 1
}
grep -q 'do not forward cross-block/root chains' "$CARD" || {
  echo "[call-operand-forwarding-guard-surface] card must keep cross-root chains closed" >&2
  exit 1
}
grep -q 'do not special-case helper names' "$CARD" || {
  echo "[call-operand-forwarding-guard-surface] card must forbid helper-name special cases" >&2
  exit 1
}

echo "[call-operand-forwarding-guard-surface] ok"

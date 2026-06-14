#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-688-POST-CALL-OPERAND-MATERIALIZATION-FORWARDING-MEASUREMENT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-687-CALL-OPERAND-MATERIALIZATION-FORWARDING-IMPLEMENTATION-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-689-POST-CALL-OPERAND-MATERIALIZATION-FORWARDING-OWNER-REFRESH-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_post_call_operand_materialization_forwarding_measurement_guard.sh"

[[ -f "$CARD" ]] || { echo "[post-call-operand-measurement] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[post-call-operand-measurement] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[post-call-operand-measurement] missing next card: $NEXT_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[post-call-operand-measurement] row688 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[post-call-operand-measurement] row687 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[post-call-operand-measurement] row689 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[post-call-operand-measurement] check index missing guard entry" >&2; exit 1; }

require_line() {
  local expected="$1"
  if ! grep -q "^${expected}$" "$CARD"; then
    echo "[post-call-operand-measurement] missing card line: $expected" >&2
    exit 1
  fi
}

require_line "output_contract=hako-mimalloc-post-call-operand-materialization-forwarding-measurement-v0"
require_line "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
require_line "source_evidence=296x-687"
require_line "post_selected_keeper_candidate_count=0"
require_line "post_call_operand_unique_copy_count=27"
require_line "hako_body_elapsed_ns=375000000"
require_line "c_body_elapsed_ns=3255360"
require_line "body_elapsed_ratio=115.195"
require_line "body_elapsed_gap_ns=371744640"
require_line "gap_owner=compiler_lowering"
require_line "dominant_copy_owner=local_ssa_copy_materialization"
require_line "dominant_position=call_adjacent"
require_line "dominant_route_carrier_role=call_operand"
require_line "winner_claim=0"
require_line "selected_next_owner=post_call_operand_materialization_forwarding_owner_refresh"
require_line "selected_owner_confidence=low"
require_line "next_task=post_call_operand_materialization_forwarding_owner_refresh"
require_line "optimization_open=0"
require_line "summary=ok"

echo "[post-call-operand-measurement] ok"

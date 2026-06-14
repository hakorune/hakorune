#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-690-CALL-OPERAND-RESIDUAL-POLICY-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-689-POST-CALL-OPERAND-MATERIALIZATION-FORWARDING-OWNER-REFRESH-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-691-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-DESIGN-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_residual_policy_selection_guard.sh"
TOOL="tools/allocator/hako_mimalloc_call_operand_residual_policy_selection.py"

[[ -f "$CARD" ]] || { echo "[call-operand-residual-policy] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-residual-policy] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[call-operand-residual-policy] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[call-operand-residual-policy] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[call-operand-residual-policy] row690 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[call-operand-residual-policy] row689 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[call-operand-residual-policy] row691 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[call-operand-residual-policy] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_call_operand_residual_policy.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"

python3 "$TOOL" --owner-refresh "$PREV_CARD" --out "$report"

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[call-operand-residual-policy] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$report" "$CARD"; do
  require_line_in_file "$file" "output_contract=hako-mimalloc-call-operand-residual-policy-selection-v0"
  require_line_in_file "$file" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
  require_line_in_file "$file" "source_evidence=296x-689"
  require_line_in_file "$file" "arg_same_block_root_call_operand_chain_count=7"
  require_line_in_file "$file" "dominance_required_candidate_count=14"
  require_line_in_file "$file" "unknown_root_call_operand_chain_count=3"
  require_line_in_file "$file" "receiver_cross_block_root_call_operand_chain_count=13"
  require_line_in_file "$file" "selected_policy_family=dominance_required_call_operand_forwarding"
  require_line_in_file "$file" "selected_policy_candidate_count=14"
  require_line_in_file "$file" "rejected_policy_family=arg_same_block_root_forwarding"
  require_line_in_file "$file" "rejected_policy_candidate_count=7"
  require_line_in_file "$file" "selected_next_owner=call_operand_dominance_required_forwarding_design"
  require_line_in_file "$file" "selected_owner_confidence=medium"
  require_line_in_file "$file" "next_task=call_operand_dominance_required_forwarding_design"
  require_line_in_file "$file" "implementation_started=0"
  require_line_in_file "$file" "optimization_open=0"
  require_line_in_file "$file" "winner_claim=0"
  require_line_in_file "$file" "summary=ok"
done

require_line_in_file "$NEXT_CARD" "Task: CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-DESIGN-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-690"
require_line_in_file "$NEXT_CARD" "implementation_started=0"
require_line_in_file "$NEXT_CARD" "winner_claim=0"

echo "[call-operand-residual-policy] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-685-CALL-OPERAND-MATERIALIZATION-FORWARDING-DESIGN-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-684-CALL-OPERAND-MATERIALIZATION-COPY-CHAIN-INVENTORY-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-686-CALL-OPERAND-MATERIALIZATION-FORWARDING-GUARD-SURFACE-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_call_operand_materialization_forwarding_design_guard.sh"
TOOL="tools/allocator/hako_mimalloc_call_operand_materialization_forwarding_design.py"

[[ -f "$CARD" ]] || { echo "[call-operand-forwarding-design] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[call-operand-forwarding-design] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[call-operand-forwarding-design] missing next card: $NEXT_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[call-operand-forwarding-design] missing tool: $TOOL" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[call-operand-forwarding-design] row685 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[call-operand-forwarding-design] row684 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[call-operand-forwarding-design] row686 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[call-operand-forwarding-design] check index missing guard entry" >&2; exit 1; }

tmp_dir="$(mktemp -d /tmp/hakorune_call_operand_forwarding_design.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"

python3 "$TOOL" --inventory "$PREV_CARD" --out "$report"

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[call-operand-forwarding-design] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$report" "$CARD"; do
  require_line_in_file "$file" "output_contract=hako-mimalloc-call-operand-materialization-forwarding-design-v0"
  require_line_in_file "$file" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
  require_line_in_file "$file" "source_evidence=296x-684"
  require_line_in_file "$file" "call_operand_chain_count=26"
  require_line_in_file "$file" "safe_forwarding_candidate_count=9"
  require_line_in_file "$file" "dominance_required_candidate_count=14"
  require_line_in_file "$file" "unknown_root_call_operand_chain_count=3"
  require_line_in_file "$file" "selected_keeper_shape=same_block_root_receiver_operand_forwarding"
  require_line_in_file "$file" "selected_keeper_candidate_count=2"
  require_line_in_file "$file" "receiver_same_block_root_candidate_count=2"
  require_line_in_file "$file" "arg_same_block_root_candidate_count=7"
  require_line_in_file "$file" "rejected_arg_forwarding_count=9"
  require_line_in_file "$file" "rejected_unknown_root_count=3"
  require_line_in_file "$file" "rejected_dominance_required_count=14"
  require_line_in_file "$file" "rejected_receiver_nonlocal_root_count=15"
  require_line_in_file "$file" "requires_dominance_guard=0"
  require_line_in_file "$file" "arg_forwarding_enabled=0"
  require_line_in_file "$file" "helper_name_special_case=0"
  require_line_in_file "$file" "variable_map_semantics_changed=0"
  require_line_in_file "$file" "phi_lifecycle_changed=0"
  require_line_in_file "$file" "implementation_started=0"
  require_line_in_file "$file" "optimization_open=0"
  require_line_in_file "$file" "winner_claim=0"
  require_line_in_file "$file" "next_task=call_operand_materialization_forwarding_guard_surface"
  require_line_in_file "$file" "summary=ok"
done

require_line_in_file "$NEXT_CARD" "Task: CALL-OPERAND-MATERIALIZATION-FORWARDING-GUARD-SURFACE-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-685"
require_line_in_file "$NEXT_CARD" "implementation_started=0"
require_line_in_file "$NEXT_CARD" "winner_claim=0"

echo "[call-operand-forwarding-design] ok"

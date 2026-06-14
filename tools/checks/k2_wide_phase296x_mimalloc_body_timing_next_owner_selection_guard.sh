#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-700-MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-699-MIMALLOC-BODY-TIMING-CFG-STABLE-RECEIVER-REWRITE-CLOSEOUT-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-701-MIMALLOC-COMPILER-LOWERING-OPTIMIZATION-CHECKPOINT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_body_timing_next_owner_selection_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-next-owner-selection] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[mimalloc-next-owner-selection] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[mimalloc-next-owner-selection] missing next card: $NEXT_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[mimalloc-next-owner-selection] row700 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[mimalloc-next-owner-selection] row699 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[mimalloc-next-owner-selection] row701 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[mimalloc-next-owner-selection] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[mimalloc-next-owner-selection] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$CARD"; do
  require_line_in_file "$file" "output_contract=hako-mimalloc-body-timing-next-owner-selection-v0"
  require_line_in_file "$file" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
  require_line_in_file "$file" "source_evidence=296x-699"
  require_line_in_file "$file" "current_body_elapsed_ratio=1.865"
  require_line_in_file "$file" "receiver_operand_copy_chain_owner_closed=1"
  require_line_in_file "$file" "fresh_gap_owner=hako_runtime_baseline"
  require_line_in_file "$file" "fresh_gap_confidence=low"
  require_line_in_file "$file" "selected_next_owner=pause_compiler_lowering_optimization"
  require_line_in_file "$file" "selected_owner_confidence=low"
  require_line_in_file "$file" "implementation_started=0"
  require_line_in_file "$file" "startup_lane_reopened=0"
  require_line_in_file "$file" "source_hako_changed=0"
  require_line_in_file "$file" "winner_claim=0"
  require_line_in_file "$file" "next_task=mimalloc_compiler_lowering_optimization_checkpoint"
  require_line_in_file "$file" "summary=ok"
done

require_line_in_file "$NEXT_CARD" "Task: MIMALLOC-COMPILER-LOWERING-OPTIMIZATION-CHECKPOINT-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-700"
require_line_in_file "$NEXT_CARD" "compiler_lowering_optimization_pause=1"

echo "[mimalloc-next-owner-selection] ok"

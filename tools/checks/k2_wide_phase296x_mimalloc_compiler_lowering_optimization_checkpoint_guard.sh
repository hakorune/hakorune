#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-701-MIMALLOC-COMPILER-LOWERING-OPTIMIZATION-CHECKPOINT-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_compiler_lowering_optimization_checkpoint_guard.sh"

[[ -f "$CARD" ]] || { echo "[mimalloc-compiler-lowering-checkpoint] missing card: $CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[mimalloc-compiler-lowering-checkpoint] row701 card must be Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[mimalloc-compiler-lowering-checkpoint] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[mimalloc-compiler-lowering-checkpoint] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line_in_file "$CARD" "output_contract=hako-mimalloc-compiler-lowering-optimization-checkpoint-v0"
require_line_in_file "$CARD" "source_evidence=296x-700"
require_line_in_file "$CARD" "compiler_lowering_optimization_pause=1"
require_line_in_file "$CARD" "receiver_operand_copy_chain_owner_closed=1"
require_line_in_file "$CARD" "stable_body_elapsed_ratio=1.790"
require_line_in_file "$CARD" "fresh_body_elapsed_ratio=1.865"
require_line_in_file "$CARD" "winner_keeper=cfg_stable_dominance_guarded_receiver_operand_rewrite"
require_line_in_file "$CARD" "next_compiler_owner_selected=0"
require_line_in_file "$CARD" "startup_lane_reopened=0"
require_line_in_file "$CARD" "source_hako_changed=0"
require_line_in_file "$CARD" "summary=ok"

echo "[mimalloc-compiler-lowering-checkpoint] ok"

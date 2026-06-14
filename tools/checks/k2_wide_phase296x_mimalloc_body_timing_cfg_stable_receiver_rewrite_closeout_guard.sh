#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-699-MIMALLOC-BODY-TIMING-CFG-STABLE-RECEIVER-REWRITE-CLOSEOUT-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-698-POST-CFG-STABLE-RECEIVER-REWRITE-STABILITY-MEASUREMENT-001.md"
NEXT_CARD="docs/development/current/main/phases/phase-296x/296x-700-MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-001.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_mimalloc_body_timing_cfg_stable_receiver_rewrite_closeout_guard.sh"

[[ -f "$CARD" ]] || { echo "[cfg-stable-receiver-closeout] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[cfg-stable-receiver-closeout] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$NEXT_CARD" ]] || { echo "[cfg-stable-receiver-closeout] missing next card: $NEXT_CARD" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || { echo "[cfg-stable-receiver-closeout] row699 card must be Landed" >&2; exit 1; }
grep -q '^Status: Landed$' "$PREV_CARD" || { echo "[cfg-stable-receiver-closeout] row698 card must be Landed" >&2; exit 1; }
grep -Eq '^Status: (Active|Landed)$' "$NEXT_CARD" || { echo "[cfg-stable-receiver-closeout] row700 card must be Active or Landed" >&2; exit 1; }
grep -q "$SELF_SCRIPT" "$INDEX" || { echo "[cfg-stable-receiver-closeout] check index missing guard entry" >&2; exit 1; }

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[cfg-stable-receiver-closeout] missing line in $file: $expected" >&2
    echo "--- $file ---" >&2
    cat "$file" >&2
    exit 1
  fi
}

for file in "$CARD"; do
  require_line_in_file "$file" "output_contract=hako-mimalloc-body-timing-cfg-stable-receiver-rewrite-closeout-v0"
  require_line_in_file "$file" "target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
  require_line_in_file "$file" "source_evidence=296x-698"
  require_line_in_file "$file" "keeper=cfg_stable_dominance_guarded_receiver_operand_rewrite"
  require_line_in_file "$file" "keeper_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite"
  require_line_in_file "$file" "pre_selected_keeper_candidate_count=13"
  require_line_in_file "$file" "post_selected_keeper_candidate_count=0"
  require_line_in_file "$file" "post_call_operand_unique_copy_count=13"
  require_line_in_file "$file" "stable_hako_body_elapsed_ns=6000000"
  require_line_in_file "$file" "stable_c_body_elapsed_ns=3352143"
  require_line_in_file "$file" "stable_body_elapsed_ratio=1.790"
  require_line_in_file "$file" "winner_claim=1"
  require_line_in_file "$file" "receiver_operand_copy_chain_owner_closed=1"
  require_line_in_file "$file" "startup_lane_reopened=0"
  require_line_in_file "$file" "source_hako_changed=0"
  require_line_in_file "$file" "next_task=mimalloc_body_timing_next_owner_selection"
  require_line_in_file "$file" "summary=ok"
done

require_line_in_file "$NEXT_CARD" "Task: MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-001"
require_line_in_file "$NEXT_CARD" "source_evidence=296x-699"
require_line_in_file "$NEXT_CARD" "implementation_started=0"

echo "[cfg-stable-receiver-closeout] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-summary-no-winner-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-294x/294x-169-MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-294x/294x-168-MIMALLOC-COMPARISON-SUMMARY-NO-WINNER.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SUMMARY_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_summary_no_winner_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_summary_no_winner_closeout_guard.sh"

echo "[$TAG] checking no-winner summary closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SUMMARY_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SUMMARY_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "no-winner summary row must be landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$CARD" "closeout card must be landed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-CLOSEOUT-001' "$CARD" "closeout card must identify the current blocker"
guard_expect_in_file "$TAG" 'HAKO-ALLOC-USIZE-FIELD-GROUP-169' "$CARD" "closeout card must return to usize field-group selection"
guard_expect_in_file "$TAG" 'HAKO-ALLOC-USIZE-FIELD-GROUP-169' "$TASKBOARD" "taskboard must expose next usize field-group selection"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this closeout guard"
guard_expect_in_file "$TAG" 'winner claims' "$CARD" "closeout must keep no-winner contract explicit"

tmp_out="$(mktemp /tmp/${TAG}.summary.XXXXXX)"
trap 'rm -f "$tmp_out"' EXIT
bash "$SUMMARY_GUARD" >"$tmp_out"
rg -F -q 'mimalloc_comparison_summary_no_winner=1' "$tmp_out"
rg -F -q 'winner_claim=0' "$tmp_out"

echo "[$TAG] ok"

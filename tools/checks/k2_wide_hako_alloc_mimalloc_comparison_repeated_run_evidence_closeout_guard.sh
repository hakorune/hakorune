#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-repeated-run-evidence-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-294x/294x-167-MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-294x/294x-166-MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
REPEATED_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_repeated_run_evidence_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_repeated_run_evidence_closeout_guard.sh"

echo "[$TAG] checking repeated-run evidence closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$REPEATED_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$REPEATED_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "repeated-run evidence row must be landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$CARD" "closeout card must be landed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-CLOSEOUT-001' "$CARD" "closeout card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-001' "$CARD" "closeout card must select no-winner summary follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SUMMARY-NO-WINNER-001' "$TASKBOARD" "taskboard must expose no-winner summary follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this closeout guard"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "closeout must keep no-winner contract explicit"

bash "$REPEATED_GUARD"

echo "[$TAG] ok"

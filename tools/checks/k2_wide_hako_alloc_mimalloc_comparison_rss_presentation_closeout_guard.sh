#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-mimalloc-comparison-rss-presentation-closeout"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-294x/294x-165-MIMALLOC-COMPARISON-RSS-PRESENTATION-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-294x/294x-164-MIMALLOC-COMPARISON-RSS-PRESENTATION.md"
TASKBOARD="docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
RSS_GUARD="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_rss_presentation_guard.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_mimalloc_comparison_rss_presentation_closeout_guard.sh"

echo "[$TAG] checking RSS presentation closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$RSS_GUARD" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$RSS_GUARD" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "RSS presentation row must be landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$CARD" "closeout card must be landed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RSS-PRESENTATION-CLOSEOUT-001' "$CARD" "closeout card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-001' "$CARD" "closeout card must select repeated-run evidence follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-RUN-EVIDENCE-001' "$TASKBOARD" "taskboard must expose repeated-run evidence follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check index must list this closeout guard"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "closeout must keep no-winner contract explicit"

bash "$RSS_GUARD"

echo "[$TAG] ok"

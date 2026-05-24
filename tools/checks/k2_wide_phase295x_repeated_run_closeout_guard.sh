#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-repeated-run-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-11-MIMALLOC-COMPARISON-REPEATED-RUN-CLOSEOUT.md"
REFRESH_CARD="docs/development/current/main/phases/phase-295x/295x-10-MIMALLOC-COMPARISON-REPEATED-RUN-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_repeated_run_closeout_guard.sh"
REFRESH_GUARD="tools/checks/k2_wide_phase295x_repeated_run_refresh_guard.sh"

echo "[$TAG] checking phase-295x repeated-run closeout"

guard_require_files "$TAG" "$CARD" "$REFRESH_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$REFRESH_GUARD"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$REFRESH_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-RUN-CLOSEOUT-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-COUNT-EVIDENCE-SELECTION-295X-001' "$CARD" "card must select count-evidence selection"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-RUN-CLOSEOUT-295X-001' "$REFRESH_CARD" "refresh row must select this closeout"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-COUNT-EVIDENCE-SELECTION-295X-001' "$TASKBOARD" "taskboard must expose count-evidence selection"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'winner claim' "$CARD" "closeout must keep winner claim wording explicit"

bash "$REFRESH_GUARD"

echo "[$TAG] ok"

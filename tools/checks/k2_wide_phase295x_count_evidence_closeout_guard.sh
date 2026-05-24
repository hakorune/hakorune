#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-count-evidence-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-14-MIMALLOC-COMPARISON-COUNT-EVIDENCE-CLOSEOUT.md"
REFRESH_CARD="docs/development/current/main/phases/phase-295x/295x-13-MIMALLOC-COMPARISON-HAKO-COUNT-EVIDENCE-REFRESH.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_count_evidence_closeout_guard.sh"
REFRESH_GUARD="tools/checks/k2_wide_phase295x_hako_count_evidence_refresh_guard.sh"

echo "[$TAG] checking phase-295x count evidence closeout"

guard_require_files "$TAG" "$CARD" "$REFRESH_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$REFRESH_GUARD"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$REFRESH_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-COUNT-EVIDENCE-CLOSEOUT-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NEXT-WORKLOAD-SEAM-SELECTION-295X-001' "$CARD" "card must select next workload seam selection"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-COUNT-EVIDENCE-CLOSEOUT-295X-001' "$REFRESH_CARD" "refresh row must select this closeout"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NEXT-WORKLOAD-SEAM-SELECTION-295X-001' "$TASKBOARD" "taskboard must expose next workload seam selection"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'allocation_count_delta=0' "$CARD" "closeout must record allocation count delta"
guard_expect_in_file "$TAG" 'free_count_delta=0' "$CARD" "closeout must record free count delta"

bash "$REFRESH_GUARD"

echo "[$TAG] ok"

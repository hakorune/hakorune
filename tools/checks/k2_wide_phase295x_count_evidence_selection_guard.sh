#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-count-evidence-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-12-MIMALLOC-COMPARISON-COUNT-EVIDENCE-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-11-MIMALLOC-COMPARISON-REPEATED-RUN-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_count_evidence_selection_guard.sh"

echo "[$TAG] checking phase-295x count evidence selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-COUNT-EVIDENCE-SELECTION-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-COUNT-EVIDENCE-295X-REFRESH-001' "$CARD" "card must select hako count evidence refresh"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-COUNT-EVIDENCE-SELECTION-295X-001' "$PREV_CARD" "previous row must select this count-evidence selection"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-COUNT-EVIDENCE-295X-REFRESH-001' "$TASKBOARD" "taskboard must expose hako count evidence refresh"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'allocation_count' "$CARD" "selection must name allocation count evidence"
guard_expect_in_file "$TAG" 'free_count' "$CARD" "selection must name free count evidence"

echo "[$TAG] ok"

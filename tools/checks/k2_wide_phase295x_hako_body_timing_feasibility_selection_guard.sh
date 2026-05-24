#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-hako-body-timing-feasibility-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-77-MIMALLOC-COMPARISON-HAKO-BODY-TIMING-FEASIBILITY-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-76-MIMALLOC-COMPARISON-C-BODY-TIMING-PILOT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_hako_body_timing_feasibility_selection_guard.sh"

echo "[$TAG] checking phase-295x hako body timing feasibility selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-BODY-TIMING-FEASIBILITY-SELECTION-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PORT-RESUME-SEAM-SELECTION-295X-001' "$CARD" "card must select port resume seam"
guard_expect_in_file "$TAG" 'Do not open `.hako` body timing in this row.' "$CARD" "card must keep hako body timing closed"
guard_expect_in_file "$TAG" 'hako_body_timing_available=1' "$CARD" "card must reserve future hako timing evidence"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-BODY-TIMING-FEASIBILITY-SELECTION-295X-001' "$PREV_CARD" "previous card must select this feasibility row"
guard_expect_in_file "$TAG" '295x-78' "$TASKBOARD" "taskboard must expose selected port seam selection"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

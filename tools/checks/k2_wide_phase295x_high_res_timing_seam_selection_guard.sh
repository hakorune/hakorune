#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-high-res-timing-seam-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-69-MIMALLOC-COMPARISON-HIGH-RES-TIMING-SEAM-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-68-MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_high_res_timing_seam_selection_guard.sh"

echo "[$TAG] checking phase-295x high-resolution timing seam selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HIGH-RES-TIMING-SEAM-SELECTION-295X-001' "$CARD" "card must identify selection blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-PACK-295X-001' "$CARD" "card must select long process-repeat timing pack"
guard_expect_in_file "$TAG" 'timing_repeat_kind=process-invocation-v0' "$CARD" "card must name timing repeat kind"
guard_expect_in_file "$TAG" 'operation_repeat=128' "$CARD" "card must fix selected repeat count"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HIGH-RES-TIMING-SEAM-SELECTION-295X-001' "$PREV_CARD" "previous closeout must select this blocker"
guard_expect_in_file "$TAG" '295x-70' "$TASKBOARD" "taskboard must include the selected follow-on row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

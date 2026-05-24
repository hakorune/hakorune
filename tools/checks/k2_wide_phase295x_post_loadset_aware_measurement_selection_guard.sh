#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-post-loadset-aware-measurement-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-66-MIMALLOC-COMPARISON-POST-LOADSET-AWARE-MEASUREMENT-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_post_loadset_aware_measurement_selection_guard.sh"

echo "[$TAG] checking phase-295x post loadset-aware measurement selection"

guard_require_files "$TAG" "$CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-POST-LOADSET-AWARE-MEASUREMENT-SELECTION-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-PACK-295X-001' "$CARD" "card must select speed/stability observation pack"
guard_expect_in_file "$TAG" 'representative-small-block-v0' "$CARD" "card must keep small-block bench"
guard_expect_in_file "$TAG" 'representative-realloc-aligned-v0' "$CARD" "card must keep realloc/aligned bench"
guard_expect_in_file "$TAG" 'representative-mixed-small-v0' "$CARD" "card must keep mixed-small bench"
guard_expect_in_file "$TAG" 'representative-huge-ish-v0' "$CARD" "card must keep huge-ish bench"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-SPEED-STABILITY-OBSERVATION-PACK-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

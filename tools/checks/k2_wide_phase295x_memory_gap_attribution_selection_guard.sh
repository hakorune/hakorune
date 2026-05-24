#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-memory-gap-attribution-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-33-MIMALLOC-COMPARISON-MEMORY-GAP-ATTRIBUTION-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-32-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PRESENTATION.md"
SSOT="docs/development/current/main/design/mimalloc-comparison-execution-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_memory_gap_attribution_selection_guard.sh"

echo "[$TAG] checking phase-295x memory gap attribution selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$SSOT" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-GAP-BASELINE-295X-001' "$CARD" "card must select baseline follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-GAP-ATTRIBUTION-SELECTION-295X-001' "$PREV_CARD" "previous row must select attribution selection"
guard_expect_in_file "$TAG" 'fixed_process_runtime_baseline' "$CARD" "card must name fixed baseline"
guard_expect_in_file "$TAG" 'workload_incremental_rss' "$CARD" "card must name incremental RSS"
guard_expect_in_file "$TAG" 'unattributed_residual' "$CARD" "card must name residual"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-GAP-BASELINE-295X-001' "$TASKBOARD" "taskboard must expose baseline follow-on"
guard_expect_in_file "$TAG" 'Memory Gap Attribution' "$SSOT" "SSOT must define attribution policy"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

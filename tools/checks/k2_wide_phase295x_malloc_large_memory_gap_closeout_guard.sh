#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-malloc-large-memory-gap-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-193-MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-192-MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-INCREMENTAL.md"
SSOT="docs/development/current/main/design/mimalloc-comparison-execution-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_malloc_large_memory_gap_closeout_guard.sh"

echo "[$TAG] checking phase-295x malloc-large memory gap closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$SSOT" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Current' "$CARD" "card must remain current while the follow-on selection is open"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-BASELINE-BREAKDOWN-SELECTION-295X-001' "$CARD" "card must select baseline breakdown follow-on"
guard_expect_fixed_in_file "$TAG" 'fixed_process_runtime_baseline_delta + workload_incremental_delta' "$CARD" "card must preserve the attribution equation"
guard_expect_fixed_in_file "$TAG" 'Winner claims remain closed.' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT-295X-001' "$PREV_CARD" "previous row must select this closeout row"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous row must be landed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-BASELINE-BREAKDOWN-SELECTION-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" '| 192 | `MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-INCREMENTAL-295X-001` | Landed |' "$TASKBOARD" "taskboard must mark incremental landed"
guard_expect_in_file "$TAG" '| 193 | `MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT-295X-001` | Current |' "$TASKBOARD" "taskboard must expose the closeout row"
guard_expect_in_file "$TAG" 'Memory Gap Attribution' "$SSOT" "SSOT must define attribution policy"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

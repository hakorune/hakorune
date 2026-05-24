#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-memory-gap-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-36-MIMALLOC-COMPARISON-MEMORY-GAP-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-35-MIMALLOC-COMPARISON-MEMORY-GAP-INCREMENTAL.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_memory_gap_closeout_guard.sh"
INCREMENTAL_SCRIPT="tools/allocator/mimalloc_memory_gap_incremental.py"

echo "[$TAG] checking phase-295x memory gap closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$INCREMENTAL_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$INCREMENTAL_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-GAP-CLOSEOUT-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-BASELINE-BREAKDOWN-SELECTION-295X-001' "$CARD" "card must select baseline breakdown follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MEMORY-GAP-CLOSEOUT-295X-001' "$PREV_CARD" "previous row must select this closeout row"
guard_expect_fixed_in_file "$TAG" 'fixed_process_runtime_baseline_delta + workload_incremental_delta' "$CARD" "card must preserve the attribution equation"
guard_expect_in_file "$TAG" 'winner claims' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-HAKO-BASELINE-BREAKDOWN-SELECTION-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

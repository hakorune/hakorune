#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-baseline-breakdown-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-219-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-BASELINE-BREAKDOWN-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-218-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_baseline_breakdown_selection_guard.sh"

echo "[$TAG] checking phase-295x abandoned-heap stress baseline breakdown selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-BASELINE-BREAKDOWN-SELECTION-295X-002' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC-295X-002' "$CARD" "card must select empty exact-EXE footprint diagnostic"
guard_expect_fixed_in_file "$TAG" 'empty no-output exact-EXE RSS control' "$CARD" "card must include no-output control"
guard_expect_fixed_in_file "$TAG" 'exact-EXE file / PT_LOAD / section footprint' "$CARD" "card must include exact-EXE footprint inventory"
guard_expect_fixed_in_file "$TAG" 'C empty runner reference footprint' "$CARD" "card must include C reference"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-BASELINE-BREAKDOWN-SELECTION-295X-002' "$PREV_CARD" "previous row must select this row"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD" "card must be landed after selecting the diagnostic follow-on"
guard_expect_fixed_in_file "$TAG" '| 219 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-BASELINE-BREAKDOWN-SELECTION-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the selection row as landed"
guard_expect_in_file "$TAG" '| 220 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the selected diagnostic follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-malloc-large-nyrt-plugin-host-baseline-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-199-MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-BASELINE-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-198-MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-RUN.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_malloc_large_nyrt_plugin_host_baseline_selection_guard.sh"

echo "[$TAG] checking phase-295x malloc-large NyRT plugin-host baseline selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Current' "$CARD" "card must remain current until the plugin-host substage seam is opened"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-002' "$CARD" "card must select substage diagnostic follow-on"
guard_expect_in_file "$TAG" 'after_runtime_hooks' "$CARD" "card must name previous lower checkpoint"
guard_expect_in_file "$TAG" 'after_plugin_host' "$CARD" "card must name previous upper checkpoint"
guard_expect_in_file "$TAG" 'HAKO_NYRT_RSS_CHECKPOINTS=1' "$CARD" "card must keep env gate"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002' "$PREV_CARD" "previous row must select this row"
guard_expect_in_file "$TAG" '| 198 | `MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-RUN-295X-002` | Landed |' "$TASKBOARD" "taskboard must retain the run row as landed"
guard_expect_in_file "$TAG" '| 199 | `MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the plugin-host baseline row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-nyrt-plugin-loadset-footprint-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-226-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-225-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_loadset_footprint_selection_guard.sh"

echo "[$TAG] checking phase-295x abandoned-heap stress NyRT plugin load-set footprint selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD" "card must remain current while the load-set selection row is exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-002' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC-295X-002' "$CARD" "card must select the diagnostic follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-002' "$PREV_CARD" "previous row must select this row"
guard_expect_in_file "$TAG" 'empty_config' "$CARD" "card must include the empty config case"
guard_expect_in_file "$TAG" 'root_current' "$CARD" "card must include the root current case"
guard_expect_in_file "$TAG" '| 225 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-002` | Landed |' "$TASKBOARD" "taskboard must retain the plugin-host substage row as landed"
guard_expect_in_file "$TAG" '| 226 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the plugin load-set selection row as current"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-malloc-large-nyrt-plugin-loadset-footprint-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-201-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-200-MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_malloc_large_nyrt_plugin_loadset_footprint_selection_guard.sh"

echo "[$TAG] checking phase-295x malloc-large NyRT plugin load-set footprint selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Current' "$CARD" "card must remain current while the selection row is being exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-002' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC-295X-002' "$CARD" "card must select the diagnostic follow-on"
guard_expect_in_file "$TAG" 'empty_config' "$CARD" "card must include the empty config case"
guard_expect_in_file "$TAG" 'root_current' "$CARD" "card must include the root current case"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-002' "$PREV_CARD" "previous row must select this row"
guard_expect_in_file "$TAG" '| 200 | `MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-002` | Landed |' "$TASKBOARD" "taskboard must retain the plugin-host substage row as landed"
guard_expect_in_file "$TAG" '| 201 | `MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the plugin load-set selection row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

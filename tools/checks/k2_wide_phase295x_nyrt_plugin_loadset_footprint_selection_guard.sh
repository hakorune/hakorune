#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-nyrt-plugin-loadset-footprint-selection"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-44-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-43-MIMALLOC-COMPARISON-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_nyrt_plugin_loadset_footprint_selection_guard.sh"

echo "[$TAG] checking phase-295x NyRT plugin load-set footprint selection"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC-295X-001' "$CARD" "card must select diagnostic follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-001' "$PREV_CARD" "previous row must select this row"
guard_expect_in_file "$TAG" 'empty_config' "$CARD" "card must include empty config case"
guard_expect_in_file "$TAG" 'core_six_existing' "$CARD" "card must include core-six case"
guard_expect_in_file "$TAG" '| 45 | `295x-45` | Landed | Ran generated-config plugin load-set RSS diagnostic. |' "$TASKBOARD" "taskboard must retain selected follow-on as landed"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-nyrt-plugin-loadset-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-46-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-45-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_nyrt_plugin_loadset_closeout_guard.sh"

echo "[$TAG] checking phase-295x NyRT plugin load-set closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-PILOT-295X-001' "$CARD" "card must select minimal config pilot"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-001' "$PREV_CARD" "previous row must select this closeout"
guard_expect_in_file "$TAG" 'empty_config' "$PREV_CARD" "previous row must record empty config evidence"
guard_expect_in_file "$TAG" 'root_current' "$PREV_CARD" "previous row must record root current evidence"
guard_expect_in_file "$TAG" 'single_libnyash_python_parser_plugin_so' "$PREV_CARD" "previous row must record top single plugin evidence"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-PILOT-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

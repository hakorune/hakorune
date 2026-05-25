#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-malloc-large-nyrt-plugin-loadset-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-203-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-202-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_malloc_large_nyrt_plugin_loadset_closeout_guard.sh"

echo "[$TAG] checking phase-295x malloc-large NyRT plugin load-set closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Current' "$CARD" "card must remain current while the closeout row is being exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-002' "$CARD" "card must identify the closeout blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-PILOT-295X-002' "$CARD" "card must select the smaller-default-load-set pilot"
guard_expect_in_file "$TAG" 'empty_config' "$CARD" "card must preserve the empty_config diagnostic evidence"
guard_expect_in_file "$TAG" 'root_current' "$CARD" "card must preserve the root_current diagnostic evidence"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous diagnostic row must be landed before closeout"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-002' "$PREV_CARD" "previous row must select this closeout row"
guard_expect_in_file "$TAG" '| 202 | `MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the diagnostic row as landed"
guard_expect_in_file "$TAG" '| 203 | `MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the closeout row as current"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

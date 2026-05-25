#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-malloc-large-empty-exe-footprint-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-196-MIMALLOC-COMPARISON-MALLOC-LARGE-EMPTY-EXE-FOOTPRINT-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-195-MIMALLOC-COMPARISON-MALLOC-LARGE-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_malloc_large_empty_exe_footprint_closeout_guard.sh"

echo "[$TAG] checking phase-295x malloc-large empty EXE footprint closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-EMPTY-EXE-FOOTPRINT-CLOSEOUT-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-001' "$CARD" "card must select the NyRT RSS checkpoint follow-on"
guard_expect_in_file "$TAG" 'HAKO_NYRT_RSS_CHECKPOINTS=1' "$CARD" "card must name the env-gated diagnostic"
guard_expect_in_file "$TAG" '[nyrt/rss]' "$CARD" "card must define stable checkpoint tag"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MALLOC-LARGE-EMPTY-EXE-FOOTPRINT-CLOSEOUT-295X-001' "$PREV_CARD" "previous row must select this closeout row"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous row must be landed"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_fixed_in_file "$TAG" '| 195 | `MIMALLOC-COMPARISON-MALLOC-LARGE-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC-295X-001` | Landed |' "$TASKBOARD" "taskboard must mark diagnostic landed"
guard_expect_fixed_in_file "$TAG" '| 196 | `MIMALLOC-COMPARISON-MALLOC-LARGE-EMPTY-EXE-FOOTPRINT-CLOSEOUT-295X-001` | Current |' "$TASKBOARD" "taskboard must expose the closeout row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-process-repeat-pack-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-239-MIMALLOC-COMPARISON-PROCESS-REPEAT-PACK-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-238-MIMALLOC-COMPARISON-MIXED-SMALL-PROCESS-REPEAT-PACK.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_process_repeat_pack_closeout_guard.sh"

echo "[$TAG] checking phase-295x process-repeat pack closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD" "closeout card must be current"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-PROCESS-REPEAT-PACK-CLOSEOUT-295X-002' "$CARD" "closeout blocker must be fixed"
guard_expect_fixed_in_file "$TAG" 'representative-reuse-cycle-small-v0' "$CARD" "closeout must include reuse-cycle small evidence"
guard_expect_fixed_in_file "$TAG" 'representative-realloc-aligned-v0' "$CARD" "closeout must include realloc/aligned evidence"
guard_expect_fixed_in_file "$TAG" 'representative-mixed-small-v0' "$CARD" "closeout must include mixed-small evidence"
guard_expect_fixed_in_file "$TAG" 'Further rows that only add more medians under the same runner, schema,' "$CARD" "closeout must park same-policy median-only rows"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-NEXT-SEMANTIC-SEAM-SELECTION-295X-002' "$CARD" "closeout must select semantic seam follow-on"
guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous mixed-small pack must be landed"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-PROCESS-REPEAT-PACK-CLOSEOUT-295X-002' "$PREV_CARD" "previous card must select this closeout"

guard_expect_fixed_in_file "$TAG" '| 236 | `MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-PROCESS-REPEAT-PACK-295X-002` | Landed |' "$TASKBOARD" "taskboard must keep reuse-cycle pack landed"
guard_expect_fixed_in_file "$TAG" '| 237 | `MIMALLOC-COMPARISON-REALLOC-ALIGNED-PROCESS-REPEAT-PACK-295X-002` | Landed |' "$TASKBOARD" "taskboard must keep realloc/aligned pack landed"
guard_expect_fixed_in_file "$TAG" '| 238 | `MIMALLOC-COMPARISON-MIXED-SMALL-PROCESS-REPEAT-PACK-295X-002` | Landed |' "$TASKBOARD" "taskboard must keep mixed-small pack landed"
guard_expect_fixed_in_file "$TAG" '| 239 | `MIMALLOC-COMPARISON-PROCESS-REPEAT-PACK-CLOSEOUT-295X-002` | Current |' "$TASKBOARD" "taskboard must expose closeout row as current"

guard_expect_fixed_in_file "$TAG" '295x-239-MIMALLOC-COMPARISON-PROCESS-REPEAT-PACK-CLOSEOUT' "$CURRENT_STATE" "current state must point at the closeout card"
guard_expect_fixed_in_file "$TAG" 'MIMALLOC-COMPARISON-PROCESS-REPEAT-PACK-CLOSEOUT-295X-002' "$CURRENT_STATE" "current state must expose closeout blocker"
guard_expect_fixed_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list closeout guard"

echo "[$TAG] ok"

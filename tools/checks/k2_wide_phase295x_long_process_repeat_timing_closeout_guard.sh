#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-long-process-repeat-timing-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-71-MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-70-MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-PACK.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_long_process_repeat_timing_closeout_guard.sh"

echo "[$TAG] checking phase-295x long process-repeat timing closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-LONG-PROCESS-REPEAT-TIMING-CLOSEOUT-295X-001' "$CARD" "card must identify closeout blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-POST-LONG-TIMING-SELECTION-295X-001' "$CARD" "card must select post-long-timing decision row"
guard_expect_in_file "$TAG" 'operation_repeat=128' "$CARD" "card must preserve repeat count"
guard_expect_in_file "$TAG" 'timing_repeat_kind=process-invocation-v0' "$CARD" "card must preserve repeat kind"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'allocator-body-only' "$CARD" "card must not blur process-repeat with body-only timing"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-POST-LONG-TIMING-SELECTION-295X-001' "$TASKBOARD" "taskboard must expose next blocker"
guard_expect_in_file "$TAG" '295x-71' "$CURRENT_STATE" "current state must point at closeout card"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-POST-LONG-TIMING-SELECTION-295X-001' "$CURRENT_STATE" "current state must expose selected next blocker"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

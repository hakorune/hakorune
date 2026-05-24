#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-process-timing-presentation-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-74-MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-73-MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-PACK.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_process_timing_presentation_closeout_guard.sh"
PACK_GUARD="tools/checks/k2_wide_phase295x_process_timing_presentation_pack_guard.sh"

echo "[$TAG] checking phase-295x process timing presentation closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$PACK_GUARD"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$PACK_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-CLOSEOUT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ALLOCATOR-BODY-TIMING-CONTRACT-295X-001' "$CARD" "card must select body timing contract"
guard_expect_in_file "$TAG" 'allocator_body_timing=0' "$CARD" "card must keep allocator-body timing closed"
guard_expect_in_file "$TAG" 'process_runtime_cost_included=1' "$CARD" "card must preserve process runtime cost boundary"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-CLOSEOUT-295X-001' "$PREV_CARD" "previous card must select closeout"
guard_expect_in_file "$TAG" '295x-75' "$TASKBOARD" "taskboard must expose selected body timing contract"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

bash "$PACK_GUARD"

echo "[$TAG] ok"

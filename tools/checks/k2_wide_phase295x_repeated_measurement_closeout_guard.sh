#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-repeated-measurement-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-31-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-30-MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PACK-RUN.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_repeated_measurement_closeout_guard.sh"
PACK_GUARD="tools/checks/k2_wide_phase295x_repeated_measurement_pack_run_guard.sh"

echo "[$TAG] checking phase-295x repeated measurement closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$PACK_GUARD"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$PACK_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-CLOSEOUT-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PRESENTATION-295X-001' "$CARD" "card must select presentation follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-CLOSEOUT-295X-001' "$PREV_CARD" "previous row must select this closeout"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REPEATED-MEASUREMENT-PRESENTATION-295X-001' "$TASKBOARD" "taskboard must expose presentation follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" "$PACK_GUARD" "$INDEX" "check script index must list pack guard"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "closeout must keep winner claims closed"

bash "$PACK_GUARD"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-loadset-aware-repeated-measurement-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-65-MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-CLOSEOUT.md"
RUN_CARD="docs/development/current/main/phases/phase-295x/295x-64-MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-PACK.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_loadset_aware_repeated_measurement_closeout_guard.sh"
RUN_GUARD="tools/checks/k2_wide_phase295x_loadset_aware_repeated_measurement_pack_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"

echo "[$TAG] checking phase-295x loadset-aware repeated measurement closeout"

guard_require_files "$TAG" "$CARD" "$RUN_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUN_GUARD" "$RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUN_GUARD" "$RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-CLOSEOUT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-POST-LOADSET-AWARE-MEASUREMENT-SELECTION-295X-001' "$CARD" "card must select post measurement selection"
guard_expect_in_file "$TAG" 'hako_selected_loadset=empty' "$CARD" "card must close empty selected loadset evidence"
guard_expect_in_file "$TAG" 'hako_selected_library_count=0' "$CARD" "card must close zero selected library evidence"
guard_expect_in_file "$TAG" 'sample_count=5' "$CARD" "card must preserve full sample count"
guard_expect_in_file "$TAG" 'warmup_count=1' "$CARD" "card must preserve warmup"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" 'hako_selected_loadset' "$RUNNER" "runner must still emit selected loadset"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-POST-LOADSET-AWARE-MEASUREMENT-SELECTION-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

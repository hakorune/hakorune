#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-speed-stability-observation-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-232-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-231-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-PACK.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_speed_stability_observation_closeout_guard.sh"
PACK_GUARD="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_speed_stability_observation_pack_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"

echo "[$TAG] checking phase-295x abandoned-heap stress speed/stability observation closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$CURRENT_STATE" "$INDEX" "$SELF_SCRIPT" "$PACK_GUARD" "$RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$PACK_GUARD" "$RUNNER"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD" "card must be landed after the observation row is exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-CLOSEOUT-295X-002' "$CARD" "card must identify closeout blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-HIGH-RES-TIMING-SEAM-SELECTION-295X-002' "$CARD" "card must select high-resolution timing seam"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claims closed"
guard_expect_in_file "$TAG" '1ms / 1ms' "$CARD" "card must record elapsed floor observation"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous observation row must be landed before closeout"
guard_expect_in_file "$TAG" 'external_elapsed_median_ms' "$PREV_CARD" "previous card must define elapsed evidence"
guard_expect_in_file "$TAG" 'external_elapsed_ms' "$RUNNER" "runner must still expose elapsed evidence"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-HIGH-RES-TIMING-SEAM-SELECTION-295X-002' "$TASKBOARD" "taskboard must expose next blocker"
guard_expect_in_file "$TAG" '| 231 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-PACK-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the observation pack landed"
guard_expect_in_file "$TAG" '| 232 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SPEED-STABILITY-OBSERVATION-CLOSEOUT-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the closeout row landed"
guard_expect_in_file "$TAG" '| 233 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-HIGH-RES-TIMING-SEAM-SELECTION-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the high-res timing seam row as current"
guard_expect_in_file "$TAG" '233' "$CURRENT_STATE" "current state must point at the selection card"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-LONG-PROCESS-REPEAT-TIMING-PACK-295X-002' "$CURRENT_STATE" "current state must expose the selected next blocker"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

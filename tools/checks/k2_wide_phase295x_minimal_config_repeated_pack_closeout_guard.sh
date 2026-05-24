#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-minimal-config-repeated-pack-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-52-MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-CLOSEOUT.md"
RUN_CARD="docs/development/current/main/phases/phase-295x/295x-51-MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-RUN.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_minimal_config_repeated_pack_closeout_guard.sh"
RUN_GUARD="tools/checks/k2_wide_phase295x_minimal_config_repeated_pack_run_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"

echo "[$TAG] checking phase-295x minimal-config repeated pack closeout"

guard_require_files "$TAG" "$CARD" "$RUN_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUN_GUARD" "$RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUN_GUARD" "$RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-PACK-CLOSEOUT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-MEASUREMENT-PACK-295X-001' "$CARD" "card must select repeated measurement follow-on"
guard_expect_in_file "$TAG" 'sample_count=5' "$CARD" "card must select full repeated sample count follow-on"
guard_expect_in_file "$TAG" 'warmup_count=1' "$CARD" "card must select warmup follow-on"
guard_expect_in_file "$TAG" 'winner_claim=0' "$CARD" "card must keep winner claim closed"
guard_expect_in_file "$TAG" '--hako-runtime-config' "$RUNNER" "runner must keep runtime config profile option"
guard_expect_in_file "$TAG" 'hako_runtime_config_profile' "$RUNNER" "runner must keep runtime config evidence field"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-MINIMAL-CONFIG-REPEATED-MEASUREMENT-PACK-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

bash "$RUN_GUARD"

echo "[$TAG] ok"

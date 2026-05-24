#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-allocator-body-timing-contract"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-75-MIMALLOC-COMPARISON-ALLOCATOR-BODY-TIMING-CONTRACT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-74-MIMALLOC-COMPARISON-PROCESS-TIMING-PRESENTATION-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_allocator_body_timing_contract_guard.sh"

echo "[$TAG] checking phase-295x allocator body timing contract"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ALLOCATOR-BODY-TIMING-CONTRACT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-C-BODY-TIMING-PILOT-295X-001' "$CARD" "card must select C body timing pilot"
guard_expect_in_file "$TAG" 'timing_repeat_kind=process-invocation-v0' "$CARD" "card must keep process timing vocabulary"
guard_expect_in_file "$TAG" 'body_timing_repeat_kind=workload-body-monotonic-v0' "$CARD" "card must define body timing repeat kind"
guard_expect_in_file "$TAG" 'body_timing_is_process_timing=0' "$CARD" "card must forbid confusing body timing with process timing"
guard_expect_in_file "$TAG" 'hako_body_timing_available=0' "$CARD" "card must keep hako body timing out of first pilot"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ALLOCATOR-BODY-TIMING-CONTRACT-295X-001' "$PREV_CARD" "previous card must select this contract"
guard_expect_in_file "$TAG" '295x-76' "$TASKBOARD" "taskboard must expose C body timing pilot"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

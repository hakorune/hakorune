#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-runner-loadset-evidence-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-59-MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-58-MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_runner_loadset_evidence_closeout_guard.sh"
EVIDENCE_GUARD="tools/checks/k2_wide_phase295x_runner_loadset_evidence_guard.sh"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"

echo "[$TAG] checking phase-295x runner loadset evidence closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$EVIDENCE_GUARD" "$RUNNER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$EVIDENCE_GUARD" "$RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-CLOSEOUT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-STANDALONE-EXE-ROUTE-CONTRACT-295X-001' "$CARD" "card must select standalone route contract follow-on"
guard_expect_in_file "$TAG" 'hako_selected_loadset' "$CARD" "card must close selected loadset evidence"
guard_expect_in_file "$TAG" 'hako_selected_library_count' "$CARD" "card must close selected library count evidence"
guard_expect_in_file "$TAG" 'hako_plugin_load_policy=eager_selected' "$CARD" "card must close eager selected evidence"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-STANDALONE-EXE-ROUTE-CONTRACT-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

bash "$EVIDENCE_GUARD"

echo "[$TAG] ok"

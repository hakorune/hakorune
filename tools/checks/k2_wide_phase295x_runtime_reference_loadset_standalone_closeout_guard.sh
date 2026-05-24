#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-runtime-reference-loadset-standalone-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-63-MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-62-MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_runtime_reference_loadset_standalone_closeout_guard.sh"
REF_GUARD="tools/checks/k2_wide_phase295x_runtime_reference_loadset_standalone_guard.sh"

echo "[$TAG] checking phase-295x runtime reference loadset/standalone closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$REF_GUARD"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$REF_GUARD"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNTIME-REFERENCE-LOADSET-STANDALONE-CLOSEOUT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-PACK-295X-001' "$CARD" "card must select loadset-aware repeated pack"
guard_expect_in_file "$TAG" 'Do not add standalone labels' "$CARD" "card must keep standalone evidence labels out"
guard_expect_in_file "$TAG" 'hako_selected_loadset' "$CARD" "card must point to concrete loadset evidence"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-LOADSET-AWARE-REPEATED-MEASUREMENT-PACK-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

bash "$REF_GUARD"

echo "[$TAG] ok"

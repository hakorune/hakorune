#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-plugin-loadset-preflight-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-57-MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-56-MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-PLAN.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_plugin_loadset_preflight_closeout_guard.sh"
PLAN_GUARD="tools/checks/k2_wide_phase295x_plugin_loadset_preflight_plan_guard.sh"
PLAN_TOOL="tools/allocator/hako_plugin_loadset_plan.py"

echo "[$TAG] checking phase-295x plugin loadset preflight closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$PLAN_GUARD" "$PLAN_TOOL"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$PLAN_GUARD" "$PLAN_TOOL"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-CLOSEOUT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-295X-001' "$CARD" "card must select runner evidence follow-on"
guard_expect_in_file "$TAG" 'selected_loadset=empty' "$CARD" "card must preserve empty plan evidence"
guard_expect_in_file "$TAG" 'selected_loadset=root' "$CARD" "card must preserve root plan evidence"
guard_expect_in_file "$TAG" 'library_count=0' "$CARD" "card must preserve empty library count"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-RUNNER-LOADSET-EVIDENCE-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

bash "$PLAN_GUARD"

echo "[$TAG] ok"

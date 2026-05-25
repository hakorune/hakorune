#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-nyrt-plugin-loadset-smaller-default-set-closeout"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-230-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-CLOSEOUT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-229-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-EVIDENCE.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
RUNNER="tools/allocator/mimalloc_repeated_measurement_runner.py"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_loadset_smaller_default_set_closeout_guard.sh"

echo "[$TAG] checking phase-295x abandoned-heap stress smaller-default load-set closeout"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$RUNNER" "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$RUNNER" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Current' "$CARD" "card must remain current while the closeout row is being exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-CLOSEOUT-295X-002' "$CARD" "card must identify the closeout blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-POST-LOADSET-SELECTION-295X-002' "$CARD" "card must select the post-loadset follow-on"
guard_expect_in_file "$TAG" 'empty default runtime config stays materially smaller than explicit root compatibility' "$CARD" "card must preserve the evidence summary"
guard_expect_in_file "$TAG" 'representative-small-block-v0' "$CARD" "card must preserve the evidence table"
guard_expect_in_file "$TAG" 'representative-realloc-aligned-v0' "$CARD" "card must preserve the realloc/aligned evidence"
guard_expect_in_file "$TAG" 'representative-mixed-small-v0' "$CARD" "card must preserve the mixed-small evidence"
guard_expect_in_file "$TAG" 'representative-huge-ish-v0' "$CARD" "card must preserve the huge-ish evidence"
guard_expect_in_file "$TAG" 'hako_runtime_config_default=empty' "$CARD" "card must record the runner default"
guard_expect_in_file "$TAG" 'Status: Landed' "$PREV_CARD" "previous evidence row must be landed before closeout"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-CLOSEOUT-295X-002' "$PREV_CARD" "previous row must select this closeout row"
guard_expect_in_file "$TAG" '| 229 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-EVIDENCE-295X-002` | Landed |' "$TASKBOARD" "taskboard must mark the evidence row as landed"
guard_expect_in_file "$TAG" '| 230 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-SMALLER-DEFAULT-SET-CLOSEOUT-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the closeout row as current"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

echo "[$TAG] ok"

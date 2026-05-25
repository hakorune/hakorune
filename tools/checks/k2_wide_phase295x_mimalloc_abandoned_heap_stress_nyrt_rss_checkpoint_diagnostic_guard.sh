#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-nyrt-rss-checkpoint-diagnostic"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-222-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-DIAGNOSTIC.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-221-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
ENV_DOC="docs/reference/environment-variables.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_rss_checkpoint_diagnostic_guard.sh"
RSS_MOD="crates/nyash_kernel/src/rss_observe.rs"
RUNTIME_RSS="src/runtime/rss_observe.rs"
ENTRY="crates/nyash_kernel/src/entry.rs"

echo "[$TAG] checking phase-295x abandoned-heap stress NyRT RSS checkpoint diagnostic"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$ENV_DOC" "$SELF_SCRIPT" "$RSS_MOD" "$RUNTIME_RSS" "$ENTRY"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Landed' "$CARD" "card must be landed after the diagnostic row is exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-002' "$CARD" "card must identify the diagnostic blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-RUN-295X-002' "$CARD" "card must select checkpoint run follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT-295X-002' "$PREV_CARD" "previous row must select this diagnostic"
guard_expect_in_file "$TAG" 'HAKO_NYRT_RSS_CHECKPOINTS=1' "$CARD" "card must name the env gate"
guard_expect_in_file "$TAG" 'HAKO_NYRT_RSS_CHECKPOINTS' "$ENV_DOC" "environment reference must document the env gate"
guard_expect_in_file "$TAG" 'HAKO_NYRT_RSS_CHECKPOINTS' "$RUNTIME_RSS" "runtime rss module must read the env gate"
guard_expect_fixed_in_file "$TAG" 'tagged_checkpoint("nyrt/rss", label)' "$RSS_MOD" "rss module must emit stable tag"
guard_expect_fixed_in_file "$TAG" 'checkpoint("entry_start")' "$ENTRY" "entry must include entry_start checkpoint"
guard_expect_fixed_in_file "$TAG" 'checkpoint("after_plugin_host")' "$ENTRY" "entry must include plugin host checkpoint"
guard_expect_fixed_in_file "$TAG" 'checkpoint("before_ny_main")' "$ENTRY" "entry must include before_ny_main checkpoint"
guard_expect_fixed_in_file "$TAG" 'checkpoint("after_ny_main")' "$ENTRY" "entry must include after_ny_main checkpoint"
guard_expect_in_file "$TAG" '| 221 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT-295X-002` | Landed |' "$TASKBOARD" "taskboard must retain the closeout row as landed"
guard_expect_in_file "$TAG" '| 222 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-002` | Landed |' "$TASKBOARD" "taskboard must retain the NyRT diagnostic row as landed"
guard_expect_in_file "$TAG" '| 223 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-RUN-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the NyRT run row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

cargo check -p nyash_kernel

echo "[$TAG] ok"

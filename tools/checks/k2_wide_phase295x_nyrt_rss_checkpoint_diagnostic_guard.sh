#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-nyrt-rss-checkpoint-diagnostic"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-40-MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-DIAGNOSTIC.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-39-MIMALLOC-COMPARISON-HAKO-EMPTY-EXE-FOOTPRINT-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
ENV_DOC="docs/reference/environment-variables.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_nyrt_rss_checkpoint_diagnostic_guard.sh"
RSS_MOD="crates/nyash_kernel/src/rss_observe.rs"
ENTRY="crates/nyash_kernel/src/entry.rs"

echo "[$TAG] checking phase-295x NyRT RSS checkpoint diagnostic"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$ENV_DOC" "$SELF_SCRIPT" "$RSS_MOD" "$ENTRY"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-001' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-RUN-295X-001' "$CARD" "card must select checkpoint run follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-001' "$PREV_CARD" "previous row must select this diagnostic"
guard_expect_in_file "$TAG" 'HAKO_NYRT_RSS_CHECKPOINTS=1' "$CARD" "card must name the env gate"
guard_expect_in_file "$TAG" 'HAKO_NYRT_RSS_CHECKPOINTS' "$ENV_DOC" "environment reference must document the env gate"
guard_expect_in_file "$TAG" 'HAKO_NYRT_RSS_CHECKPOINTS' "$RSS_MOD" "rss module must read the env gate"
guard_expect_fixed_in_file "$TAG" '[nyrt/rss]' "$RSS_MOD" "rss module must emit stable tag"
guard_expect_fixed_in_file "$TAG" 'checkpoint("entry_start")' "$ENTRY" "entry must include entry_start checkpoint"
guard_expect_fixed_in_file "$TAG" 'checkpoint("after_plugin_host")' "$ENTRY" "entry must include plugin host checkpoint"
guard_expect_fixed_in_file "$TAG" 'checkpoint("before_ny_main")' "$ENTRY" "entry must include before_ny_main checkpoint"
guard_expect_fixed_in_file "$TAG" 'checkpoint("after_ny_main")' "$ENTRY" "entry must include after_ny_main checkpoint"
guard_expect_in_file "$TAG" '| 41 | `295x-41` | Landed | Ran empty no-output exact-EXE checkpoint diagnostic. |' "$TASKBOARD" "taskboard must retain selected follow-on as landed"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

cargo check -p nyash_kernel

echo "[$TAG] ok"

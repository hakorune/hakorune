#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-nyrt-rss-checkpoint-run"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-223-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-RUN.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-222-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-DIAGNOSTIC.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_rss_checkpoint_run_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-empty-noio-exe-proof/main.hako"

echo "[$TAG] checking phase-295x abandoned-heap stress NyRT RSS checkpoint run"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$APP"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Current' "$CARD" "card must remain current while the run row is being exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-RUN-295X-002' "$CARD" "card must identify the current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002' "$CARD" "card must select plugin-host follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-RUN-295X-002' "$PREV_CARD" "previous row must select this run"
guard_expect_in_file "$TAG" 'after_plugin_host' "$CARD" "card must identify plugin-host jump"
guard_expect_in_file "$TAG" '| 222 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-002` | Landed |' "$TASKBOARD" "taskboard must retain the diagnostic row as landed"
guard_expect_in_file "$TAG" '| 223 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-RUN-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the run row as current"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_nyrt_rss_run.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir="$tmp_dir/app.mir.json"
exe="$tmp_dir/app.exe"
stderr="$tmp_dir/run.stderr"

cargo build --release -p nyash_kernel >/dev/null
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir" "$APP" >/dev/null
python3 tools/checks/pure_first_route_preflight.py "$mir" >/dev/null
NYASH_DISABLE_PLUGINS=1 \
  tools/selfhost/selfhost_build.sh --mir-in "$mir" --exe "$exe" >/dev/null

HAKO_NYRT_RSS_CHECKPOINTS=1 NYASH_NYRT_SILENT_RESULT=1 "$exe" >/dev/null 2>"$stderr"

python3 - "$stderr" <<'PY'
import re
import sys

text = open(sys.argv[1], encoding="utf-8", errors="replace").read()
labels = [
    "entry_start",
    "after_ring0",
    "after_runtime_hooks",
    "after_plugin_host",
    "before_ny_main",
    "after_ny_main",
]
values = {}
for line in text.splitlines():
    match = re.search(r"\[nyrt/rss\] checkpoint=([a-z0-9_]+) rss_bytes=(\d+)", line)
    if match:
        values[match.group(1)] = int(match.group(2))

missing = [label for label in labels if label not in values]
if missing:
    raise SystemExit(f"missing RSS checkpoints: {missing}")
if any(values[label] <= 0 for label in labels):
    raise SystemExit("all RSS checkpoints must be positive")
if values["after_plugin_host"] < values["after_runtime_hooks"]:
    raise SystemExit("plugin host checkpoint must not decrease from runtime hooks")
if values["before_ny_main"] != values["after_ny_main"]:
    raise SystemExit("empty no-output main should not change RSS checkpoints")

plugin_delta = values["after_plugin_host"] - values["after_runtime_hooks"]
print(f"[phase295x-nyrt-rss-checkpoint-run] plugin_delta_bytes={plugin_delta}")
PY

cat "$stderr"
echo "[$TAG] ok"

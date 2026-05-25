#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-mimalloc-abandoned-heap-stress-nyrt-plugin-host-substage-diagnostic"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-225-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-224-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-BASELINE-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
ENV_DOC="docs/reference/environment-variables.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_plugin_host_substage_diagnostic_guard.sh"
APP="apps/hako-alloc-mimalloc-comparison-empty-noio-exe-proof/main.hako"
RUNTIME_RSS="src/runtime/rss_observe.rs"
KERNEL_RSS="crates/nyash_kernel/src/rss_observe.rs"
UNIFIED="src/runtime/plugin_loader_unified.rs"
LOADER="src/runtime/plugin_loader_v2/enabled/loader/library.rs"

echo "[$TAG] checking phase-295x abandoned-heap stress NyRT plugin host substage diagnostic"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$ENV_DOC" "$SELF_SCRIPT" "$APP" "$RUNTIME_RSS" "$KERNEL_RSS" "$UNIFIED" "$LOADER"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_fixed_in_file "$TAG" 'Status: Landed' "$CARD" "card must be landed after the substage diagnostic row is exercised"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-002' "$CARD" "card must identify the substage diagnostic blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-002' "$CARD" "card must select loadset follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002' "$PREV_CARD" "previous row must select this diagnostic"
guard_expect_in_file "$TAG" 'HAKO_NYRT_RSS_CHECKPOINTS=1' "$ENV_DOC" "environment reference must document the shared env gate"
guard_expect_fixed_in_file "$TAG" 'tagged_checkpoint("runtime/rss", label)' "$RUNTIME_RSS" "runtime RSS observer must emit stable tag"
guard_expect_fixed_in_file "$TAG" 'nyash_rust::runtime::rss_observe::tagged_checkpoint("nyrt/rss", label)' "$KERNEL_RSS" "NyRT observer must delegate to runtime RSS owner"
guard_expect_fixed_in_file "$TAG" 'checkpoint("plugin_host_after_host_config_parse")' "$UNIFIED" "unified host must expose host config parse checkpoint"
guard_expect_fixed_in_file "$TAG" 'checkpoint("plugin_host_after_load_all_plugins")' "$UNIFIED" "unified host must expose load_all completion checkpoint"
guard_expect_fixed_in_file "$TAG" 'checkpoint("plugin_loader_after_library_loop")' "$LOADER" "v2 loader must expose library loop checkpoint"
guard_expect_fixed_in_file "$TAG" 'checkpoint("plugin_loader_after_prebirth_singletons")' "$LOADER" "v2 loader must expose prebirth checkpoint"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-002' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" '| 224 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-BASELINE-SELECTION-295X-002` | Landed |' "$TASKBOARD" "taskboard must retain the baseline-selection row as landed"
guard_expect_in_file "$TAG" '| 225 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-HOST-SUBSTAGE-DIAGNOSTIC-295X-002` | Landed |' "$TASKBOARD" "taskboard must retain the substage diagnostic row as landed"
guard_expect_in_file "$TAG" '| 226 | `MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION-295X-002` | Current |' "$TASKBOARD" "taskboard must expose the load-set selection row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_plugin_substage.XXXXXX)"
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
values = {}
for line in text.splitlines():
    match = re.search(r"\[(nyrt/rss|runtime/rss)\] checkpoint=([a-z0-9_]+) rss_bytes=(\d+)", line)
    if match:
        values[match.group(2)] = int(match.group(3))

required = [
    "after_runtime_hooks",
    "plugin_host_load_libraries_start",
    "plugin_host_after_loader_load_config",
    "plugin_host_after_host_config_parse",
    "plugin_loader_load_all_start",
    "plugin_loader_after_library_loop",
    "plugin_loader_after_plugin_root_loop",
    "plugin_loader_before_prebirth_singletons",
    "plugin_loader_after_prebirth_singletons",
    "plugin_host_after_load_all_plugins",
    "after_plugin_host",
    "before_ny_main",
    "after_ny_main",
]
missing = [label for label in required if label not in values]
if missing:
    raise SystemExit(f"missing RSS checkpoints: {missing}")
if any(values[label] <= 0 for label in required):
    raise SystemExit("all RSS checkpoints must be positive")
if values["after_plugin_host"] != values["plugin_host_after_load_all_plugins"]:
    raise SystemExit("NyRT after_plugin_host must match plugin host completion checkpoint")
if values["before_ny_main"] != values["after_ny_main"]:
    raise SystemExit("empty no-output main should not change RSS checkpoints")

config_delta = values["plugin_host_after_host_config_parse"] - values["plugin_host_load_libraries_start"]
library_delta = values["plugin_loader_after_library_loop"] - values["plugin_loader_load_all_start"]
prebirth_delta = values["plugin_loader_after_prebirth_singletons"] - values["plugin_loader_before_prebirth_singletons"]
total_delta = values["plugin_host_after_load_all_plugins"] - values["after_runtime_hooks"]
print(f"[phase295x-plugin-substage] config_delta_bytes={config_delta}")
print(f"[phase295x-plugin-substage] library_loop_delta_bytes={library_delta}")
print(f"[phase295x-plugin-substage] prebirth_delta_bytes={prebirth_delta}")
print(f"[phase295x-plugin-substage] total_plugin_host_delta_bytes={total_delta}")
PY

cat "$stderr"
echo "[$TAG] ok"

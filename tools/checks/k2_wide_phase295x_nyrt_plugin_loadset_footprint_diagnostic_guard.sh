#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-nyrt-plugin-loadset-footprint-diagnostic"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-45-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-44-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-SELECTION.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_nyrt_plugin_loadset_footprint_diagnostic_guard.sh"
TOOL="tools/allocator/nyrt_plugin_loadset_footprint.py"
APP="apps/hako-alloc-mimalloc-comparison-empty-noio-exe-proof/main.hako"

echo "[$TAG] checking phase-295x NyRT plugin load-set footprint diagnostic"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$TOOL" "$APP"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$TOOL"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT-295X-001' "$CARD" "card must select closeout follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-FOOTPRINT-DIAGNOSTIC-295X-001' "$PREV_CARD" "previous row must select this diagnostic"
guard_expect_in_file "$TAG" 'nyrt-plugin-loadset-footprint-v0' "$TOOL" "tool must emit stable output contract"
guard_expect_in_file "$TAG" 'empty_config' "$TOOL" "tool must include empty config case"
guard_expect_in_file "$TAG" 'root_current' "$TOOL" "tool must include root current case"
guard_expect_in_file "$TAG" '| 46 | `295x-46` | Landed | Closed plugin load-set diagnostic and selected exact-EXE minimal config pilot. |' "$TASKBOARD" "taskboard must retain selected follow-on as landed"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_plugin_loadset.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir="$tmp_dir/app.mir.json"
exe="$tmp_dir/app.exe"
report="$tmp_dir/report.json"

cargo build --release -p nyash_kernel >/dev/null
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 \
  target/release/hakorune --backend mir --emit-mir-json "$mir" "$APP" >/dev/null
python3 tools/checks/pure_first_route_preflight.py "$mir" >/dev/null
NYASH_DISABLE_PLUGINS=1 \
  tools/selfhost/selfhost_build.sh --mir-in "$mir" --exe "$exe" >/dev/null

python3 "$TOOL" --repo-root "$ROOT_DIR" --exe "$exe" >"$report"

python3 - "$report" <<'PY'
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf-8"))
if data.get("output_contract") != "nyrt-plugin-loadset-footprint-v0":
    raise SystemExit("unexpected output_contract")
rows = {row["case"]: row for row in data.get("rows", [])}
required = [
    "empty_config",
    "console_only",
    "core_six_existing",
    "regex_only",
    "all_existing",
    "root_current",
]
missing = [case for case in required if case not in rows]
if missing:
    raise SystemExit(f"missing cases: {missing}")
single_rows = {case: row for case, row in rows.items() if case.startswith("single_")}
if len(single_rows) < 10:
    raise SystemExit(f"expected at least 10 single plugin cases, got {len(single_rows)}")
for case, row in rows.items():
    if row["total_plugin_host_delta_bytes"] < 0:
        raise SystemExit(f"{case}: negative total delta")
    if row["library_loop_delta_bytes"] < 0:
        raise SystemExit(f"{case}: negative library delta")
if rows["empty_config"]["library_loop_delta_bytes"] != 0:
    raise SystemExit("empty_config should not load dynamic plugin libraries")
if rows["all_existing"]["library_loop_delta_bytes"] <= rows["console_only"]["library_loop_delta_bytes"]:
    raise SystemExit("all_existing should cost more than console_only")
if rows["root_current"]["library_loop_delta_bytes"] <= 0:
    raise SystemExit("root_current must show dynamic plugin load cost")

for case in required:
    row = rows[case]
    print(
        "[phase295x-plugin-loadset] "
        f"case={case} "
        f"config_delta_bytes={row['config_delta_bytes']} "
        f"library_loop_delta_bytes={row['library_loop_delta_bytes']} "
        f"total_plugin_host_delta_bytes={row['total_plugin_host_delta_bytes']}"
    )
for case, row in sorted(single_rows.items(), key=lambda item: item[1]["library_loop_delta_bytes"], reverse=True)[:5]:
    print(
        "[phase295x-plugin-loadset-top] "
        f"case={case} "
        f"library_loop_delta_bytes={row['library_loop_delta_bytes']} "
        f"total_plugin_host_delta_bytes={row['total_plugin_host_delta_bytes']}"
    )
PY

echo "[$TAG] ok"

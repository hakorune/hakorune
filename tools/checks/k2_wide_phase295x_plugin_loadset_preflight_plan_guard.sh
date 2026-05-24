#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-plugin-loadset-preflight-plan"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-56-MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-PLAN.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-55-MIMALLOC-COMPARISON-PLUGIN-LOADSET-CONTRACT.md"
SSOT="docs/development/current/main/design/plugin-loadset-linking-ssot.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_plugin_loadset_preflight_plan_guard.sh"
TOOL="tools/allocator/hako_plugin_loadset_plan.py"

echo "[$TAG] checking phase-295x plugin loadset preflight plan"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$SSOT" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$TOOL"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$TOOL"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-PLAN-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-CLOSEOUT-295X-001' "$CARD" "card must select preflight closeout"
guard_expect_in_file "$TAG" 'output_contract=hako-plugin-loadset-plan-v0' "$CARD" "card must define output contract"
guard_expect_in_file "$TAG" 'plugin_load_policy=eager_selected' "$CARD" "card must keep eager selected policy"
guard_expect_in_file "$TAG" 'does not call `dlopen`' "$CARD" "card must keep no-dlopen stop line"
guard_expect_in_file "$TAG" 'hako-plugin-loadset-plan-v0' "$TOOL" "tool must emit plan contract"
guard_expect_in_file "$TAG" 'PLUGIN_LOAD_POLICY = "eager_selected"' "$TOOL" "tool must use eager selected policy"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-PLUGIN-LOADSET-PREFLIGHT-CLOSEOUT-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_plugin_loadset_plan.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
empty_json="$tmp_dir/empty.json"
root_json="$tmp_dir/root.json"

python3 "$TOOL" --config hako.toml --loadset empty --out "$empty_json" > /dev/null
python3 "$TOOL" --config hako.toml --loadset root --out "$root_json" > /dev/null

python3 - "$empty_json" "$root_json" <<'PY'
import json
import sys
from pathlib import Path

empty = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
root = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))

def require_common(report):
    if report.get("output_contract") != "hako-plugin-loadset-plan-v0":
        raise SystemExit("bad output_contract")
    if report.get("plugin_load_policy") != "eager_selected":
        raise SystemExit("bad plugin_load_policy")
    for key in ("provider_activation", "host_replacement", "hook_installed", "global_allocator_installed", "winner_claim"):
        if report.get(key) != 0:
            raise SystemExit(f"{key} must remain 0")

require_common(empty)
require_common(root)

if empty.get("selected_loadset") != "empty":
    raise SystemExit("empty selected_loadset mismatch")
if empty.get("library_count") != 0 or empty.get("missing_library_count") != 0 or empty.get("preflight_ok") != 1:
    raise SystemExit("empty loadset must be an ok zero-library plan")

if root.get("selected_loadset") != "root":
    raise SystemExit("root selected_loadset mismatch")
if root.get("library_count", 0) <= 0:
    raise SystemExit("root loadset must see configured libraries")
if len(root.get("libraries", [])) != root.get("library_count"):
    raise SystemExit("root library list length mismatch")

names = [row.get("name") for row in root["libraries"]]
if names != sorted(names):
    raise SystemExit("root libraries must be sorted")
if "libnyash_filebox_plugin.so" not in names:
    raise SystemExit("root plan must include filebox plugin entry")

print("[phase295x-plugin-loadset-preflight-plan] ok")
PY

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-phase295x-exact-exe-minimal-config-pilot"
cd "$ROOT_DIR"
source tools/checks/lib/guard_common.sh

CARD="docs/development/current/main/phases/phase-295x/295x-47-MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-PILOT.md"
PREV_CARD="docs/development/current/main/phases/phase-295x/295x-46-MIMALLOC-COMPARISON-NYRT-PLUGIN-LOADSET-CLOSEOUT.md"
TASKBOARD="docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase295x_exact_exe_minimal_config_pilot_guard.sh"
RUNNER="tools/allocator/hako_exe_memory_runner.sh"
APP="apps/hako-alloc-mimalloc-comparison-empty-exe-proof/main.hako"

echo "[$TAG] checking phase-295x exact-EXE minimal config pilot"

guard_require_files "$TAG" "$CARD" "$PREV_CARD" "$TASKBOARD" "$INDEX" "$SELF_SCRIPT" "$RUNNER" "$APP"
guard_require_exec_files "$TAG" "$SELF_SCRIPT" "$RUNNER"

guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-PILOT-295X-001' "$CARD" "card must identify current blocker"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-EVIDENCE-295X-001' "$CARD" "card must select evidence follow-on"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-PILOT-295X-001' "$PREV_CARD" "previous row must select this pilot"
guard_expect_in_file "$TAG" '--runtime-config root|empty' "$RUNNER" "runner usage must expose runtime config profile"
guard_expect_in_file "$TAG" 'runtime_config_profile=' "$RUNNER" "runner output must expose runtime config profile"
guard_expect_in_file "$TAG" '[libraries]' "$RUNNER" "runner must generate an empty runtime config"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-EVIDENCE-295X-001' "$TASKBOARD" "taskboard must expose selected follow-on"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"

tmp_dir="$(mktemp -d /tmp/hakorune_phase295x_min_config.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
root_out="$tmp_dir/root.out"
empty_out="$tmp_dir/empty.out"

bash "$RUNNER" --app "$APP" --workload representative-empty-noio-v0 --runtime-config root --out "$root_out" >/dev/null
bash "$RUNNER" --app "$APP" --workload representative-empty-noio-v0 --runtime-config empty --out "$empty_out" >/dev/null

python3 - "$root_out" "$empty_out" <<'PY'
import sys

def read(path):
    values = {}
    for line in open(path, encoding="utf-8", errors="replace"):
        line = line.strip()
        if line and "=" in line:
            k, v = line.split("=", 1)
            values[k] = v
    return values

root = read(sys.argv[1])
empty = read(sys.argv[2])
for label, values, expected in (("root", root, "root"), ("empty", empty, "empty")):
    if values.get("output_contract") != "hako-exe-memory-evidence-v0":
        raise SystemExit(f"{label}: bad output contract")
    if values.get("summary") != "ok":
        raise SystemExit(f"{label}: summary must be ok")
    if values.get("runtime_config_profile") != expected:
        raise SystemExit(f"{label}: runtime_config_profile mismatch")
    if int(values.get("external_peak_rss_bytes", "0")) <= 0:
        raise SystemExit(f"{label}: RSS must be positive")

root_rss = int(root["external_peak_rss_bytes"])
empty_rss = int(empty["external_peak_rss_bytes"])
if empty_rss > root_rss:
    raise SystemExit(f"empty runtime config should not exceed root RSS: empty={empty_rss} root={root_rss}")
print(f"[phase295x-min-config] root_external_peak_rss_bytes={root_rss}")
print(f"[phase295x-min-config] empty_external_peak_rss_bytes={empty_rss}")
print(f"[phase295x-min-config] rss_reduction_bytes={root_rss - empty_rss}")
PY

echo "[$TAG] ok"

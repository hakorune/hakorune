#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-mimalloc-facade-stats-snapshot-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-stats-snapshot-proof/main.hako"
APP_README="apps/mimalloc-facade-stats-snapshot-proof/README.md"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
STATS="lang/src/hako_alloc/memory/object_lifecycle_facade_stats_box.hako"
RESULT="lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-378-MIMAP-018A-FACADE-STATS-SNAPSHOT.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"
ARTIFACT_DIR="$ROOT_DIR/target/checks/$TAG"
TMP_DIR="$ARTIFACT_DIR/tmp"
FORBIDDEN_LOG="$ARTIFACT_DIR/forbidden.log"
INC_LOG="$ARTIFACT_DIR/inc_leak.log"

mkdir -p "$ARTIFACT_DIR"
rm -rf "$TMP_DIR"
mkdir -p "$TMP_DIR"
rm -f "$FORBIDDEN_LOG" "$INC_LOG"

for path in "$APP" "$APP_README" "$FACADE" "$STATS" "$RESULT" "$MODULE" "$CARD" "$INDEX" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_stats_box as HakoAllocObjectLifecycleFacadeStatsBox' "$FACADE"
rg -F -q 'stats_surface: HakoAllocObjectLifecycleFacadeStatsSurface = new HakoAllocObjectLifecycleFacadeStatsSurface()' "$FACADE"
rg -F -q 'objectLifecycleStatsSnapshot()' "$FACADE"
rg -F -q 'objectLifecycleReleaseSuccessCount()' "$FACADE"
rg -F -q 'objectLifecycleReleaseFailureCount()' "$FACADE"
rg -F -q 'box HakoAllocObjectLifecycleFacadeStatsSnapshot' "$STATS"
rg -F -q 'box HakoAllocObjectLifecycleFacadeStatsSurface' "$STATS"
rg -F -q 'attempt_count: usize = 0' "$RESULT"
rg -F -q 'success_count: usize = 0' "$RESULT"
rg -F -q 'failure_count: usize = 0' "$RESULT"
rg -F -q 'reusable_success_count: usize = 0' "$RESULT"
rg -F -q 'active_success_count: usize = 0' "$RESULT"
rg -F -q 'alloc_attempt_count: usize = 0' "$STATS"
rg -F -q 'alloc_success_count: usize = 0' "$STATS"
rg -F -q 'alloc_failure_count: usize = 0' "$STATS"
rg -F -q 'alloc_reusable_success_count: usize = 0' "$STATS"
rg -F -q 'alloc_active_success_count: usize = 0' "$STATS"
rg -F -q 'release_success_count: usize = 0' "$STATS"
rg -F -q 'release_failure_count: usize = 0' "$STATS"
rg -F -q 'memory.object_lifecycle_facade_stats_box = "memory/object_lifecycle_facade_stats_box.hako"' "$MODULE"
rg -F -q 'MIMAP-018A' "$CARD"
rg -F -q 'k2_wide_mimalloc_facade_stats_snapshot_exe_guard.sh' "$INDEX"
rg -F -q 'object_lifecycle_facade_stats_box.hako' "$README"

if rg -n 'PageMap|page_map|lookup\(|copy[A-Za-z0-9_]*\(|byte[A-Za-z0-9_]*\(|memcpy|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(|purge[A-Za-z0-9_]*\(|decommit[A-Za-z0-9_]*\(' \
  "$STATS" >"$FORBIDDEN_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-018A stats surface must stay read-only and policy-free" >&2
  cat "$FORBIDDEN_LOG" >&2
  rm -f "$FORBIDDEN_LOG"
  exit 1
fi
rm -f "$FORBIDDEN_LOG"

if rg -n 'mimalloc-facade-stats-snapshot-proof|objectLifecycleStatsSnapshot|HakoAllocObjectLifecycleFacadeStats' \
  lang/c-abi/shims >"$INC_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-018A matcher leaked into .inc" >&2
  cat "$INC_LOG" >&2
  rm -f "$INC_LOG"
  exit 1
fi
rm -f "$INC_LOG"

pure_first_guard_build_toolchain

mir_json="$TMP_DIR/mimap018a.mir.json"
exe_out="$TMP_DIR/mimap018a.exe"
build_log="$TMP_DIR/build.log"
run_log="$TMP_DIR/run.log"

pure_first_guard_emit_mir "$ROOT_DIR" "$APP" "$mir_json"

python3 - "$mir_json" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as fh:
    data = json.load(fh)

functions = {fn.get("name"): fn for fn in data.get("functions", [])}
required = {
    "main",
    "HakoAllocObjectLifecycleFacade.objectLifecycleStatsSnapshot/0",
    "HakoAllocObjectLifecycleFacadeStatsSurface.snapshot/7",
    "HakoAllocObjectLifecycleFacadeStatsSnapshot.allocTerminalCount/0",
    "HakoAllocObjectLifecycleFacadeStatsSnapshot.releaseTerminalCount/0",
    "HakoAllocObjectLifecycleFacadeStatsSnapshot.totalTerminalCount/0",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in (
    "HakoAllocObjectLifecycleFacade",
    "HakoAllocObjectLifecycleAllocResult",
    "HakoAllocObjectLifecycleReleaseResult",
    "HakoAllocObjectLifecycleFacadeStatsSurface",
    "HakoAllocObjectLifecycleFacadeStatsSnapshot",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

facade_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleFacade"].get("fields", [])
}
stats_surface = facade_fields.get("stats_surface")
if (
    stats_surface is None
    or stats_surface.get("declared_type") != "HakoAllocObjectLifecycleFacadeStatsSurface"
    or stats_surface.get("storage") != "handle"
):
    raise SystemExit(f"facade stats_surface field must be typed handle: {stats_surface}")

snapshot_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleFacadeStatsSnapshot"].get("fields", [])
}
for field in (
    "alloc_attempt_count",
    "alloc_success_count",
    "alloc_failure_count",
    "alloc_reusable_success_count",
    "alloc_active_success_count",
    "release_success_count",
    "release_failure_count",
):
    snapshot_field = snapshot_fields.get(field)
    if snapshot_field is None:
        raise SystemExit(f"missing stats snapshot field: {field}")
    if snapshot_field.get("declared_type") != "usize" or snapshot_field.get("storage") != "usize":
        raise SystemExit(f"stats snapshot {field} must be exact usize storage: {snapshot_field}")

alloc_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleAllocResult"].get("fields", [])
}
for field in (
    "attempt_count",
    "success_count",
    "failure_count",
    "reusable_success_count",
    "active_success_count",
):
    alloc_field = alloc_fields.get(field)
    if alloc_field is None or alloc_field.get("declared_type") != "usize" or alloc_field.get("storage") != "usize":
        raise SystemExit(f"alloc result {field} must be exact usize storage: {alloc_field}")

release_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleReleaseResult"].get("fields", [])
}
for field in ("success_count", "failure_count"):
    release_field = release_fields.get(field)
    if release_field is None or release_field.get("declared_type") != "usize" or release_field.get("storage") != "usize":
        raise SystemExit(f"release result {field} must be exact usize storage: {release_field}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def require_method(fn, box_name, name):
    for callee in iter_calls(fn):
        if (
            callee.get("type") == "Method"
            and callee.get("box_name") == box_name
            and callee.get("name") == name
        ):
            return
    raise SystemExit(f"missing method call {box_name}.{name} in {fn.get('name')}")

require_method(functions["main"], "HakoAllocObjectLifecycleFacade", "objectLifecycleStatsSnapshot")
require_method(
    functions["HakoAllocObjectLifecycleFacade.objectLifecycleStatsSnapshot/0"],
    "HakoAllocObjectLifecycleFacadeStatsSurface",
    "snapshot",
)

print("[mimap018a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-stats-snapshot-proof' "$run_log"
rg -F -q 'alloc=3,2,1' "$run_log"
rg -F -q 'kind=1,1' "$run_log"
rg -F -q 'release=1,1' "$run_log"
rg -F -q 'totals=3,2,5' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

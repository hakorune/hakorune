#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-stats-surface"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-stats-surface-proof/main.hako"
APP_README="apps/hako-alloc-stats-surface-proof/README.md"
APP_TEST="apps/hako-alloc-stats-surface-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-231-M191-HAKO-ALLOC-STATS-SURFACE.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
RECORD_SSOT="docs/development/current/main/design/record-and-packed-array-lowering-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
FACADE="lang/src/hako_alloc/memory/allocator_facade_box.hako"
STATS="lang/src/hako_alloc/memory/stats_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_stats_surface_guard.sh"

echo "[$TAG] checking M191 hako_alloc stats surface"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$RECORD_SSOT" \
  "$PHASE_README" \
  "$TASKBOARD" \
  "$INDEX" \
  "$FACADE" \
  "$STATS" \
  "$MODULE" \
  "$MEMORY_README" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M191 card must be complete"
guard_expect_in_file "$TAG" 'M191 status:' "$PLAN" "mimalloc plan must record M191 status"
guard_expect_in_file "$TAG" '`M191` is complete as' "$RECORD_SSOT" "record SSOT must mark M191 complete"
guard_expect_in_file "$TAG" '`293x-231`' "$PHASE_README" "phase README must list M191 row"
guard_expect_in_file "$TAG" '\[x\] `293x-231`' "$TASKBOARD" "taskboard must mark M191 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M191 guard"

guard_expect_in_file "$TAG" 'memory.stats_box = "memory/stats_box.hako"' "$MODULE" "hako_alloc module must export stats_box"
guard_expect_in_file "$TAG" 'using selfhost.hako_alloc.memory.stats_box as HakoAllocStats' "$FACADE" "facade must import stats owner"
guard_expect_in_file "$TAG" 'stats_surface: HakoAllocStatsSurface = new HakoAllocStatsSurface()' "$FACADE" "facade must own stats surface"
guard_expect_in_file "$TAG" 'statsSnapshot()' "$FACADE" "facade must expose statsSnapshot"
guard_expect_in_file "$TAG" 'box HakoAllocStatsSnapshot' "$STATS" "stats snapshot box must exist"
guard_expect_in_file "$TAG" 'box HakoAllocStatsSurface' "$STATS" "stats surface box must exist"
guard_expect_in_file "$TAG" 'totalPageAllocCount' "$STATS" "stats snapshot must expose total page alloc count"
guard_expect_in_file "$TAG" 'totalPageReleaseCount' "$STATS" "stats snapshot must expose total page release count"
guard_expect_in_file "$TAG" 'stats_box.hako` owns M191 allocator stats snapshots' "$MEMORY_README" "memory README must define stats owner"
guard_expect_in_file "$TAG" 'HakoAllocProductionFacade.statsSnapshot' "$CARD" "M191 card must name facade method"

if rg -n 'NYASH_|HAKO_|std::env|env::|provider_|install_hook|global_allocator|purge[[:space:]]*\\(|decommit[[:space:]]*\\(|InlineRecord|ArrayStorage|PlanProbe|AutoUse|compiler' \
  "$STATS" >/tmp/"$TAG".stats_forbidden 2>&1; then
  echo "[$TAG] ERROR: stats owner must stay read-only and allocator/backend vocabulary free" >&2
  cat /tmp/"$TAG".stats_forbidden >&2
  rm -f /tmp/"$TAG".stats_forbidden
  exit 1
fi
rm -f /tmp/"$TAG".stats_forbidden

if rg -n 'hako-alloc-stats-surface-proof|HakoAllocStatsSurface|HakoAllocStatsSnapshot|statsSnapshot' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M191 app/stats matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_stats_surface_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M191 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m191_hako_alloc_stats.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m191.mir.json"
exe_out="$tmp_dir/m191.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"
vm_log="$tmp_dir/vm.log"

if ! NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 \
  "$ROOT_DIR/target/debug/hakorune" --backend vm "$APP" >"$vm_log" 2>&1; then
  echo "[$TAG] ERROR: VM run failed" >&2
  sed -n '1,160p' "$vm_log" >&2
  exit 1
fi

rg -F -q 'hako-alloc-stats-surface-proof' "$vm_log"
rg -F -q 'shape=6' "$vm_log"
rg -F -q 'facade=3,1,2' "$vm_log"
rg -F -q 'snapshot=3,1,2,72,2' "$vm_log"
rg -F -q 'page=2,1,7,3' "$vm_log"
rg -F -q 'totals=3,1,10' "$vm_log"
rg -F -q 'summary=ok' "$vm_log"

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
    "HakoAllocProductionFacade.statsSnapshot/0",
    "HakoAllocStatsSurface.snapshot/15",
    "HakoAllocStatsSnapshot.totalPageAllocCount/0",
    "HakoAllocStatsSnapshot.totalPageReleaseCount/0",
    "HakoAllocStatsSnapshot.totalFreeCount/0",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in ("HakoAllocProductionFacade", "HakoAllocStatsSurface", "HakoAllocStatsSnapshot"):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

facade_fields = {
    field.get("name"): field
    for field in plans["HakoAllocProductionFacade"].get("fields", [])
}
stats_surface = facade_fields.get("stats_surface")
if (
    stats_surface is None
    or stats_surface.get("declared_type") != "HakoAllocStatsSurface"
    or stats_surface.get("storage") != "handle"
):
    raise SystemExit(f"facade stats_surface field must be HakoAllocStatsSurface handle: {stats_surface}")

snapshot_fields = {
    field.get("name"): field
    for field in plans["HakoAllocStatsSnapshot"].get("fields", [])
}
required_fields = {
    "allocation_count",
    "release_count",
    "reject_count",
    "requested_bytes",
    "outstanding_blocks",
    "small_alloc_count",
    "small_release_count",
    "small_reuse_count",
    "small_peak_used",
    "small_free_count",
    "medium_alloc_count",
    "medium_release_count",
    "medium_reuse_count",
    "medium_peak_used",
    "medium_free_count",
}
missing_fields = sorted(name for name in required_fields if name not in snapshot_fields)
if missing_fields:
    raise SystemExit(f"missing stats snapshot fields: {missing_fields}")

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

require_method(functions["main"], "HakoAllocProductionFacade", "statsSnapshot")
require_method(
    functions["HakoAllocProductionFacade.statsSnapshot/0"],
    "HakoAllocStatsSurface",
    "snapshot",
)

print("[m191-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-stats-surface-proof' "$run_log"
rg -F -q 'shape=6' "$run_log"
rg -F -q 'facade=3,1,2' "$run_log"
rg -F -q 'snapshot=3,1,2,72,2' "$run_log"
rg -F -q 'page=2,1,7,3' "$run_log"
rg -F -q 'totals=3,1,10' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

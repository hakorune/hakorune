#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-production-facade-stress-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-production-facade-stress/main.hako"
APP_README="apps/hako-alloc-production-facade-stress/README.md"
LOWER_STRESS_APP="apps/allocator-stress/main.hako"
FACADE="lang/src/hako_alloc/memory/allocator_facade_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-102-M50-ALLOCATOR-STRESS-PRODUCTION-FACADE-PARITY.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M50 production facade stress EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$LOWER_STRESS_APP" "$FACADE" "$CARD" "$TASKBOARD"

if rg -n 'hako-alloc-production-facade-stress|HakoAllocProductionFacadeStress|HakoAllocProductionFacade|allocator_stress' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: production facade stress matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add|allocator replacement|replace_allocator' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".inactive_rows 2>&1; then
  echo "[$TAG] ERROR: inactive allocator rows must stay inactive in M50" >&2
  cat /tmp/"$TAG".inactive_rows >&2
  rm -f /tmp/"$TAG".inactive_rows
  exit 1
fi
rm -f /tmp/"$TAG".inactive_rows

rg -F -q 'using selfhost.hako_alloc.memory.allocator_facade_box as HakoAllocFacade' "$APP"
rg -F -q 'using selfhost.hako_alloc.memory.page_heap_box as HakoAllocPageHeap' "$LOWER_STRESS_APP"
rg -F -q 'box HakoAllocProductionFacade' "$FACADE"
rg -F -q 'smallAllocCount' "$FACADE"
rg -F -q 'mediumReuseCount' "$FACADE"

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m50_hako_alloc_facade_stress.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m50.mir.json"
exe_out="$tmp_dir/m50.exe"
build_log="$tmp_dir/build.log"
run_log="$tmp_dir/run.log"

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
    "HakoAllocProductionFacadeStress.run/0",
    "HakoAllocProductionFacade.allocate/1",
    "HakoAllocProductionFacade.release/1",
    "HakoAllocProductionFacade.requestedBytes/0",
    "HakoAllocProductionFacade.outstandingBlocks/0",
    "HakoAllocProductionFacade.smallFreeCount/0",
    "HakoAllocProductionFacade.smallAllocCount/0",
    "HakoAllocProductionFacade.smallReleaseCount/0",
    "HakoAllocProductionFacade.smallReuseCount/0",
    "HakoAllocProductionFacade.smallPeakUsed/0",
    "HakoAllocProductionFacade.mediumFreeCount/0",
    "HakoAllocProductionFacade.mediumAllocCount/0",
    "HakoAllocProductionFacade.mediumReleaseCount/0",
    "HakoAllocProductionFacade.mediumReuseCount/0",
    "HakoAllocProductionFacade.mediumPeakUsed/0",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
if plans.get("HakoAllocProductionFacade") is None:
    raise SystemExit("missing facade typed object plan")
if plans.get("HakoAllocProductionFacadeStress") is None:
    raise SystemExit("missing stress typed object plan")

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

main = functions["main"]
run = functions["HakoAllocProductionFacadeStress.run/0"]

require_method(main, "HakoAllocProductionFacadeStress", "run")

for name in (
    "allocate",
    "release",
    "requestedBytes",
    "outstandingBlocks",
    "smallFreeCount",
    "smallAllocCount",
    "smallReleaseCount",
    "smallReuseCount",
    "smallPeakUsed",
    "mediumFreeCount",
    "mediumAllocCount",
    "mediumReleaseCount",
    "mediumReuseCount",
    "mediumPeakUsed",
):
    require_method(run, "HakoAllocProductionFacade", name)

for callee in iter_calls(run):
    if callee.get("type") == "Method" and callee.get("box_name") in {
        "HakoAllocHeap",
        "HakoAllocPage",
    }:
        raise SystemExit(f"stress app must not call heap/page directly: {callee}")

require_method(functions["HakoAllocProductionFacade.allocate/1"], "HakoAllocHeap", "allocate")
require_method(functions["HakoAllocProductionFacade.release/1"], "HakoAllocHeap", "release")

print("[m50-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-production-facade-stress' "$run_log"
rg -F -q 'small_allocs=11 frees=3 reused=3 peak=8 free=0' "$run_log"
rg -F -q 'medium_allocs=6 frees=2 reused=2 peak=4 free=0' "$run_log"
rg -F -q 'facade=17,5,4' "$run_log"
rg -F -q 'requested_bytes=454' "$run_log"
rg -F -q 'outstanding=12' "$run_log"
rg -F -q 'rejects=4' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '| `M50 allocator stress production-facade parity` | `live-narrow` |' "$TASKBOARD"
rg -F -q 'M50 Allocator Stress Production-Facade Parity' "$CARD"
rg -F -q 'HakoAllocProductionFacade' "$APP_README"
rg -F -q 'k2_wide_hako_alloc_production_facade_stress_exe_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"

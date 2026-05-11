#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-production-facade-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-production-facade-proof/main.hako"
APP_README="apps/hako-alloc-production-facade-proof/README.md"
FACADE="lang/src/hako_alloc/memory/allocator_facade_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-098-M46-HAKO-ALLOC-PRODUCTION-FACADE-BOUNDARY.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M46 hako_alloc production facade EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$FACADE" "$MODULE" "$CARD" "$TASKBOARD"

if rg -n 'hako-alloc-production-facade-proof|HakoAllocProductionFacade|allocator_facade_box' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: hako_alloc production facade matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".inactive_pointer_rows 2>&1; then
  echo "[$TAG] ERROR: pointer atomic fetch_add rows must stay inactive in M46" >&2
  cat /tmp/"$TAG".inactive_pointer_rows >&2
  rm -f /tmp/"$TAG".inactive_pointer_rows
  exit 1
fi
rm -f /tmp/"$TAG".inactive_pointer_rows

rg -F -q 'memory.allocator_facade_box = "memory/allocator_facade_box.hako"' "$MODULE"
rg -F -q 'using selfhost.hako_alloc.memory.page_heap_box as HakoAllocPageHeap' "$FACADE"
rg -F -q 'box HakoAllocProductionFacade' "$FACADE"
rg -F -q 'heap: HakoAllocHeap = new HakoAllocHeap()' "$FACADE"
rg -F -q 'alloc_count: i64 = 0' "$FACADE"

if rg -n 'init[[:space:]]*\\{' "$FACADE" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: production facade should use stored fields/initializers, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m46_hako_alloc_facade.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m46.mir.json"
exe_out="$tmp_dir/m46.exe"
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
    "HakoAllocProductionFacade.birth/0",
    "HakoAllocProductionFacade.allocate/1",
    "HakoAllocProductionFacade.release/1",
    "HakoAllocProductionFacade.requestedBytes/0",
    "HakoAllocProductionFacade.outstandingBlocks/0",
    "HakoAllocProductionFacade.smallFreeCount/0",
    "HakoAllocProductionFacade.mediumFreeCount/0",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

main = functions["main"]

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
facade = plans.get("HakoAllocProductionFacade")
if facade is None:
    raise SystemExit("missing typed object plan: HakoAllocProductionFacade")
fields = {
    field.get("name"): field
    for field in facade.get("fields", [])
}
heap = fields.get("heap")
if heap is None or heap.get("declared_type") != "HakoAllocHeap" or heap.get("storage") != "handle":
    raise SystemExit(f"facade heap field must be declared HakoAllocHeap handle: {facade}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            callee = inst.get("mir_call", {}).get("callee", {})
            yield callee

def require_method(fn, box_name, name):
    for callee in iter_calls(fn):
        if (
            callee.get("type") == "Method"
            and callee.get("box_name") == box_name
            and callee.get("name") == name
        ):
            return
    raise SystemExit(f"missing method call {box_name}.{name} in {fn.get('name')}")

for name in (
    "allocate",
    "release",
):
    require_method(main, "HakoAllocProductionFacade", name)

require_method(functions["HakoAllocProductionFacade.allocate/1"], "HakoAllocHeap", "allocate")
require_method(functions["HakoAllocProductionFacade.release/1"], "HakoAllocHeap", "release")

print("[m46-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-production-facade-proof' "$run_log"
rg -F -q 'allocs=3 frees=1 rejects=2' "$run_log"
rg -F -q 'shape=1,1,1,1,1,1' "$run_log"
rg -F -q 'stats=72,2,7,3' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '| `M46 hako_alloc production facade boundary` | `live-narrow` |' "$TASKBOARD"
rg -F -q 'M46 Hako Alloc Production Facade Boundary' "$CARD"
rg -F -q 'HakoAllocProductionFacade' "$APP_README"
rg -F -q 'k2_wide_hako_alloc_production_facade_exe_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"

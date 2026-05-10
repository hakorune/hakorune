#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-local-page-policy-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-local-page-policy-proof/main.hako"
APP_README="apps/hako-alloc-local-page-policy-proof/README.md"
FACADE="lang/src/hako_alloc/memory/allocator_facade_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-099-M47-ALLOCATOR-LOCAL-PAGE-POLICY-PROOF.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M47 hako_alloc local page policy EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$FACADE" "$CARD" "$TASKBOARD"

if rg -n 'hako-alloc-local-page-policy-proof|HakoAllocProductionFacade|AllocatorLocalPagePolicy' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: hako_alloc local page policy matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".inactive_pointer_rows 2>&1; then
  echo "[$TAG] ERROR: pointer atomic fetch_add rows must stay inactive in M47" >&2
  cat /tmp/"$TAG".inactive_pointer_rows >&2
  rm -f /tmp/"$TAG".inactive_pointer_rows
  exit 1
fi
rm -f /tmp/"$TAG".inactive_pointer_rows

rg -F -q 'using selfhost.hako_alloc.memory.allocator_facade_box as HakoAllocFacade' "$APP"
rg -F -q 'box HakoAllocProductionFacade' "$FACADE"
rg -F -q 'release(handle)' "$FACADE"

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m47_hako_alloc_local_page.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m47.mir.json"
exe_out="$tmp_dir/m47.exe"
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
main = functions.get("main")
if main is None:
    raise SystemExit("missing main")

for required in (
    "HakoAllocProductionFacade.allocate/1",
    "HakoAllocProductionFacade.release/1",
    "HakoAllocProductionFacade.requestedBytes/0",
    "HakoAllocProductionFacade.outstandingBlocks/0",
    "HakoAllocProductionFacade.smallFreeCount/0",
    "HakoAllocProductionFacade.mediumFreeCount/0",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing facade function: {required}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
facade = plans.get("HakoAllocProductionFacade")
if facade is None:
    raise SystemExit("missing facade typed object plan")

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

for name in ("allocate", "release"):
    require_method(main, "HakoAllocProductionFacade", name)

require_method(functions["HakoAllocProductionFacade.allocate/1"], "HakoAllocHeap", "allocate")
require_method(functions["HakoAllocProductionFacade.release/1"], "HakoAllocHeap", "release")

print("[m47-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-local-page-policy-proof' "$run_log"
rg -F -q 'facade=6,2,2' "$run_log"
rg -F -q 'shape=10' "$run_log"
rg -F -q 'stats=200,4,6,2' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '| `M47 allocator local page policy proof` | `live-narrow` |' "$TASKBOARD"
rg -F -q 'M47 Allocator Local Page Policy Proof' "$CARD"
rg -F -q 'HakoAllocProductionFacade' "$APP_README"
rg -F -q 'k2_wide_hako_alloc_local_page_policy_exe_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"

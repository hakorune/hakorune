#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-lifecycle-route-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-lifecycle-route-proof/main.hako"
APP_README="apps/mimalloc-facade-lifecycle-route-proof/README.md"
FACADE="lang/src/hako_alloc/memory/allocator_facade_box.hako"
QUEUE="lang/src/hako_alloc/memory/page_queue_lifecycle_box.hako"
POLICY="docs/development/current/main/design/mimalloc-backend-acceptance-policy-ssot.md"
LIMITS="docs/development/current/main/design/vm-known-limitations-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-345-MIMAP-011-FACADE-LIFECYCLE-ROUTE-PILOT.md"
SSOT="docs/development/current/main/design/mimalloc-facade-lifecycle-route-ssot.md"
INDEX="docs/tools/check-scripts-index.md"

for path in "$APP" "$APP_README" "$FACADE" "$QUEUE" "$POLICY" "$LIMITS" "$CARD" "$SSOT" "$INDEX"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.allocator_facade_box as HakoAllocFacade' "$APP"
rg -F -q 'using selfhost.hako_alloc.memory.page_queue_lifecycle_box as HakoAllocLifecyclePageQueueBox' "$FACADE"
rg -F -q 'lifecycle_queue: HakoAllocLifecyclePageQueue = new HakoAllocLifecyclePageQueue()' "$FACADE"
rg -F -q 'lifecycleSelectionBegin()' "$FACADE"
rg -F -q 'lifecycleSelectionConsider(page_id, decommitted, retired, reusable, available)' "$FACADE"
rg -F -q 'lifecycleSelectionFinish()' "$FACADE"
rg -F -q 'Acceptance backend: LLVM/EXE primary' "$SSOT"
rg -F -q 'VM-LIM-001 object-heavy page queue/facade route' "$LIMITS"
rg -F -q 'k2_wide_mimalloc_facade_lifecycle_route_exe_guard.sh' "$INDEX"

if rg -n 'hako-alloc-local-page-policy-proof|mimalloc-facade-lifecycle-route-proof|HakoAllocProductionFacade.lifecycleSelection' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: MIMAP-011 matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap011_facade_lifecycle.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap011.mir.json"
exe_out="$tmp_dir/mimap011.exe"
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
    "HakoAllocProductionFacade.lifecycleSelectionBegin/0",
    "HakoAllocProductionFacade.lifecycleSelectionConsider/5",
    "HakoAllocProductionFacade.lifecycleSelectionFinish/0",
    "HakoAllocProductionFacade.lifecycleSelectedKind/0",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing facade lifecycle function: {required}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
if plans.get("HakoAllocProductionFacade") is None:
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

for name in ("lifecycleSelectionBegin", "lifecycleSelectionConsider", "lifecycleSelectionFinish", "lifecycleSelectedKind"):
    require_method(main, "HakoAllocProductionFacade", name)

require_method(functions["HakoAllocProductionFacade.lifecycleSelectionConsider/5"], "HakoAllocLifecyclePageQueue", "considerPage")
require_method(functions["HakoAllocProductionFacade.lifecycleSelectionFinish/0"], "HakoAllocLifecyclePageQueue", "finishSelection")

print("[mimap011-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-lifecycle-route-proof' "$run_log"
rg -F -q 'pages=20,30,-1' "$run_log"
rg -F -q 'kinds=1,2,0' "$run_log"
rg -F -q 'queue=2,1,1,3,0,1' "$run_log"
rg -F -q 'shape=12' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

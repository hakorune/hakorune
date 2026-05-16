#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-object-lifecycle-queue-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-object-lifecycle-queue-proof/main.hako"
APP_README="apps/mimalloc-facade-object-lifecycle-queue-proof/README.md"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
QUEUE="lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
POLICY="docs/development/current/main/design/mimalloc-backend-acceptance-policy-ssot.md"
LIMITS="docs/development/current/main/design/vm-known-limitations-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-356-MIMAP-013-FACADE-OBJECT-LIFECYCLE-QUEUE.md"
SSOT="docs/development/current/main/design/mimalloc-facade-object-lifecycle-queue-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

for path in "$APP" "$APP_README" "$FACADE" "$QUEUE" "$PAGE" "$POLICY" "$LIMITS" "$CARD" "$SSOT" "$INDEX" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_box as HakoAllocObjectLifecycleFacadeBox' "$APP"
rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_page_queue_box as HakoAllocObjectLifecyclePageQueueBox' "$FACADE"
rg -F -q 'object_lifecycle_queue: HakoAllocObjectLifecyclePageQueue = new HakoAllocObjectLifecyclePageQueue()' "$FACADE"
rg -F -q 'memory.object_lifecycle_facade_box = "memory/object_lifecycle_facade_box.hako"' lang/src/hako_alloc/hako_module.toml
rg -F -q 'objectLifecycleAddPage(page)' "$FACADE"
rg -F -q 'objectLifecycleSelectPage()' "$FACADE"
rg -F -q 'objectLifecycleSelectedKind()' "$FACADE"
rg -F -q 'objectLifecycleSelectedPageId()' "$FACADE"
rg -F -q 'box HakoAllocObjectLifecyclePageQueue' "$QUEUE"
rg -F -q 'pages.length()' "$QUEUE"
rg -F -q 'Acceptance backend: LLVM/EXE primary' "$SSOT"
rg -F -q 'VM-LIM-001 object-heavy page queue/facade route' "$LIMITS"
rg -F -q 'object_lifecycle_facade_box.hako' "$README"
rg -F -q 'k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh' "$INDEX"

if rg -n 'OSVM|OsVm|externcall|atomic|RawBuf|provider|global_allocator|install_hook|hook|pageSource|remote' "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-013 proof app must not activate substrate/provider/hook behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'local page[0-9]+ = pages\.get\([0-9]+\)' "$QUEUE" >/tmp/"$TAG".fixed_slots 2>&1; then
  echo "[$TAG] ERROR: MIMAP-040A must not reintroduce fixed page0/page1/page2 selection slots" >&2
  cat /tmp/"$TAG".fixed_slots >&2
  rm -f /tmp/"$TAG".fixed_slots
  exit 1
fi
rm -f /tmp/"$TAG".fixed_slots

if rg -n 'mimalloc-facade-object-lifecycle-queue-proof|objectLifecycle(AddPage|SelectPage|SelectedKind|SelectedPageId)|HakoAllocObjectLifecyclePageQueue' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: MIMAP-013 matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap013_facade_object_queue.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap013.mir.json"
exe_out="$tmp_dir/mimap013.exe"
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
    "HakoAllocObjectLifecycleFacade.objectLifecycleAddPage/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleSelectPage/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleSelectedKind/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleSelectedPageId/0",
    "HakoAllocObjectLifecyclePageQueue.addPage/1",
    "HakoAllocObjectLifecyclePageQueue.selectPage/0",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing MIMAP-013 function: {required}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for required in ("HakoAllocObjectLifecycleFacade", "HakoAllocObjectLifecyclePageQueue"):
    if plans.get(required) is None:
        raise SystemExit(f"missing typed object plan: {required}")

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

for name in ("objectLifecycleAddPage", "objectLifecycleSelectPage", "objectLifecycleSelectedKind"):
    require_method(main, "HakoAllocObjectLifecycleFacade", name)

require_method(functions["HakoAllocObjectLifecycleFacade.objectLifecycleAddPage/1"], "HakoAllocObjectLifecyclePageQueue", "addPage")
require_method(functions["HakoAllocObjectLifecycleFacade.objectLifecycleSelectPage/0"], "HakoAllocObjectLifecyclePageQueue", "selectPage")
select_fn = functions["HakoAllocObjectLifecyclePageQueue.selectPage/0"]
for name in ("isDecommitted", "isRetired", "canReuse", "freeCount"):
    require_method(select_fn, "HakoAllocPageModel", name)

print("[mimap013-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-object-lifecycle-queue-proof' "$run_log"
rg -F -q 'adds=0,1,2,3' "$run_log"
rg -F -q 'pages=20,40,-1' "$run_log"
rg -F -q 'kinds=1,2,0' "$run_log"
rg -F -q 'queue=4,4,2,1,1,3,0,1' "$run_log"
rg -F -q 'shape=18' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

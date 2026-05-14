#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-object-lifecycle-queue-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-object-lifecycle-queue-proof/main.hako"
APP_README="apps/mimalloc-object-lifecycle-queue-proof/README.md"
QUEUE="lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
POLICY="docs/development/current/main/design/mimalloc-backend-acceptance-policy-ssot.md"
LIMITS="docs/development/current/main/design/vm-known-limitations-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-353-MIMAP-012-OBJECT-LIFECYCLE-QUEUE-PILOT.md"
SSOT="docs/development/current/main/design/mimalloc-object-lifecycle-queue-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
MODULE="lang/src/hako_alloc/hako_module.toml"
README="lang/src/hako_alloc/memory/README.md"

for path in "$APP" "$APP_README" "$QUEUE" "$PAGE" "$POLICY" "$LIMITS" "$CARD" "$SSOT" "$INDEX" "$MODULE" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_page_queue_box as HakoAllocObjectLifecyclePageQueueBox' "$APP"
rg -F -q 'box HakoAllocObjectLifecyclePageQueue' "$QUEUE"
rg -F -q 'pages: ArrayBox = new ArrayBox()' "$QUEUE"
rg -F -q 'addPage(page)' "$QUEUE"
rg -F -q 'selectPage()' "$QUEUE"
rg -F -q 'considerPage(page)' "$QUEUE"
rg -F -q 'page.decommitted != 0' "$QUEUE"
rg -F -q 'page.canReuse() != 0' "$QUEUE"
rg -F -q 'page.reuse() != 0' "$QUEUE"
rg -F -q 'page.freeCount() > 0' "$QUEUE"
rg -F -q 'memory.object_lifecycle_page_queue_box = "memory/object_lifecycle_page_queue_box.hako"' "$MODULE"
rg -F -q 'object_lifecycle_page_queue_box.hako' "$README"
rg -F -q 'Acceptance backend: LLVM/EXE primary' "$SSOT"
rg -F -q 'VM-LIM-001 object-heavy page queue/facade route' "$LIMITS"
rg -F -q 'k2_wide_mimalloc_object_lifecycle_queue_exe_guard.sh' "$INDEX"

if rg -n 'OSVM|OsVm|externcall|atomic|RawBuf|provider|global_allocator|install_hook|hook|pageSource|remote' "$APP" "$QUEUE" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-012 object queue must not activate substrate/provider/hook behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-object-lifecycle-queue-proof|HakoAllocObjectLifecyclePageQueue' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: MIMAP-012 matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap012_object_queue.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap012.mir.json"
exe_out="$tmp_dir/mimap012.exe"
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
    "HakoAllocObjectLifecyclePageQueue.addPage/1",
    "HakoAllocObjectLifecyclePageQueue.selectPage/0",
    "HakoAllocObjectLifecyclePageQueue.considerPage/1",
    "HakoAllocObjectLifecyclePageQueue.recordSelectedPage/2",
    "HakoAllocPageModel.canReuse/0",
    "HakoAllocPageModel.reuse/0",
    "HakoAllocPageModel.freeCount/0",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing object queue function: {required}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
if plans.get("HakoAllocObjectLifecyclePageQueue") is None:
    raise SystemExit("missing object lifecycle queue typed object plan")

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

for name in ("addPage", "selectPage"):
    require_method(main, "HakoAllocObjectLifecyclePageQueue", name)
for name in ("birth", "acquire", "releaseLocal", "decommit"):
    require_method(main, "HakoAllocPageModel", name)

select_fn = functions["HakoAllocObjectLifecyclePageQueue.selectPage/0"]
require_method(select_fn, "ArrayBox", "get")
require_method(functions["HakoAllocObjectLifecyclePageQueue.considerPage/1"], "HakoAllocPageModel", "reuse")

print("[mimap012-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-object-lifecycle-queue-proof' "$run_log"
rg -F -q 'pages=20,30,-1' "$run_log"
rg -F -q 'kinds=1,2,0' "$run_log"
rg -F -q 'state=0,1,0,0' "$run_log"
rg -F -q 'queue=3,2,1,1,3,0,1,3' "$run_log"
rg -F -q 'shape=18' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

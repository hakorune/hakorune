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
ARTIFACT_DIR="$ROOT_DIR/target/checks/$TAG"
TMP_DIR="$ARTIFACT_DIR/tmp"
FORBIDDEN_LOG="$ARTIFACT_DIR/forbidden.log"
FIXED_SLOTS_LOG="$ARTIFACT_DIR/fixed_slots.log"
INC_LOG="$ARTIFACT_DIR/app_specific.inc.log"

mkdir -p "$ARTIFACT_DIR"
rm -rf "$TMP_DIR"
mkdir -p "$TMP_DIR"
rm -f "$FORBIDDEN_LOG" "$FIXED_SLOTS_LOG" "$INC_LOG"

for path in "$APP" "$APP_README" "$FACADE" "$QUEUE" "$PAGE" "$POLICY" "$LIMITS" "$CARD" "$SSOT" "$INDEX" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_box as HakoAllocObjectLifecycleFacadeBox' "$APP"
rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_page_queue_box as HakoAllocObjectLifecyclePageQueueBox' "$FACADE"
rg -F -q 'object_lifecycle_queue: HakoAllocObjectLifecyclePageQueue = new HakoAllocObjectLifecyclePageQueue()' "$FACADE"
rg -F -q 'memory.object_lifecycle_facade_box = "memory/object_lifecycle_facade_box.hako"' lang/src/hako_alloc/hako_module.toml
rg -F -q 'objectLifecycleAddPage(page)' "$FACADE"
rg -F -q 'objectLifecycleRequestCount()' "$FACADE"
rg -F -q 'objectLifecycleSelectPage()' "$FACADE"
rg -F -q 'objectLifecycleSelectedKind()' "$FACADE"
rg -F -q 'objectLifecycleSelectedPageId()' "$FACADE"
rg -F -q 'box HakoAllocObjectLifecyclePageQueue' "$QUEUE"
rg -F -q 'page_count: usize = 0' "$QUEUE"
rg -F -q 'request_count: usize = 0' "$QUEUE"
rg -F -q 'last_selected_page_id: i64 = -1' "$QUEUE"
rg -F -q 'local count = me.page_count' "$QUEUE"
rg -F -q 'Acceptance backend: LLVM/EXE primary' "$SSOT"
rg -F -q 'VM-LIM-001 object-heavy page queue/facade route' "$LIMITS"
rg -F -q 'object_lifecycle_facade_box.hako' "$README"
rg -F -q 'k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh' "$INDEX"

if rg -n 'OSVM|OsVm|externcall|atomic|RawBuf|provider|global_allocator|install_hook|hook|pageSource|remote' "$APP" >"$FORBIDDEN_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-013 proof app must not activate substrate/provider/hook behavior" >&2
  cat "$FORBIDDEN_LOG" >&2
  rm -f "$FORBIDDEN_LOG"
  exit 1
fi
rm -f "$FORBIDDEN_LOG"

if rg -n 'local page[0-9]+ = pages\.get\([0-9]+\)' "$QUEUE" >"$FIXED_SLOTS_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-040A must not reintroduce fixed page0/page1/page2 selection slots" >&2
  cat "$FIXED_SLOTS_LOG" >&2
  rm -f "$FIXED_SLOTS_LOG"
  exit 1
fi
rm -f "$FIXED_SLOTS_LOG"

if rg -n 'mimalloc-facade-object-lifecycle-queue-proof|objectLifecycle(AddPage|SelectPage|SelectedKind|SelectedPageId)|HakoAllocObjectLifecyclePageQueue' \
  lang/c-abi/shims >"$INC_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-013 matcher leaked into .inc" >&2
  cat "$INC_LOG" >&2
  rm -f "$INC_LOG"
  exit 1
fi
rm -f "$INC_LOG"

pure_first_guard_build_toolchain

mir_json="$TMP_DIR/mimap013.mir.json"
exe_out="$TMP_DIR/mimap013.exe"
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
queue = plans["HakoAllocObjectLifecyclePageQueue"]
fields = {field.get("name"): field for field in queue.get("fields", [])}
for name in (
    "page_count",
    "add_count",
    "request_count",
    "select_count",
    "reuse_select_count",
    "active_select_count",
    "decommitted_skip_count",
    "retired_skip_count",
    "unavailable_skip_count",
    "miss_count",
    "reject_count",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"queue {name} must be exact usize storage: {field}")
for name in ("last_selected_index", "last_selected_page_id", "last_selected_kind"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"queue {name} must remain signed storage: {field}")

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

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
ARTIFACT_DIR="$ROOT_DIR/target/checks/$TAG"
TMP_DIR="$ARTIFACT_DIR/tmp"
FORBIDDEN_LOG="$ARTIFACT_DIR/forbidden.log"
FIXED_SLOTS_LOG="$ARTIFACT_DIR/fixed_slots.log"
INC_LOG="$ARTIFACT_DIR/app_specific.inc.log"

mkdir -p "$ARTIFACT_DIR"
rm -rf "$TMP_DIR"
mkdir -p "$TMP_DIR"
rm -f "$FORBIDDEN_LOG" "$FIXED_SLOTS_LOG" "$INC_LOG"

for path in "$APP" "$APP_README" "$QUEUE" "$PAGE" "$POLICY" "$LIMITS" "$CARD" "$SSOT" "$INDEX" "$MODULE" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_page_queue_box as HakoAllocObjectLifecyclePageQueueBox' "$APP"
rg -F -q 'box HakoAllocObjectLifecyclePageQueue' "$QUEUE"
rg -F -q 'pages: ArrayBox = new ArrayBox()' "$QUEUE"
rg -F -q 'page_count: usize = 0' "$QUEUE"
rg -F -q 'add_count: usize = 0' "$QUEUE"
rg -F -q 'request_count: usize = 0' "$QUEUE"
rg -F -q 'select_count: usize = 0' "$QUEUE"
rg -F -q 'reuse_select_count: usize = 0' "$QUEUE"
rg -F -q 'active_select_count: usize = 0' "$QUEUE"
rg -F -q 'decommitted_skip_count: usize = 0' "$QUEUE"
rg -F -q 'retired_skip_count: usize = 0' "$QUEUE"
rg -F -q 'unavailable_skip_count: usize = 0' "$QUEUE"
rg -F -q 'miss_count: usize = 0' "$QUEUE"
rg -F -q 'reject_count: usize = 0' "$QUEUE"
rg -F -q 'last_selected_index: i64 = -1' "$QUEUE"
rg -F -q 'last_selected_page_id: i64 = -1' "$QUEUE"
rg -F -q 'last_selected_kind: i64 = 0' "$QUEUE"
rg -F -q 'addPage(page)' "$QUEUE"
rg -F -q 'selectPage()' "$QUEUE"
rg -F -q 'pages.get(' "$QUEUE"
rg -F -q 'local count = me.page_count' "$QUEUE"
rg -F -q 'isDecommitted() != 0' "$QUEUE"
rg -F -q 'canReuse() != 0' "$QUEUE"
rg -F -q 'freeCount() > 0' "$QUEUE"
rg -F -q 'memory.object_lifecycle_page_queue_box = "memory/object_lifecycle_page_queue_box.hako"' "$MODULE"
rg -F -q 'object_lifecycle_page_queue_box.hako' "$README"
rg -F -q 'Acceptance backend: LLVM/EXE primary' "$SSOT"
rg -F -q 'VM-LIM-001 object-heavy page queue/facade route' "$LIMITS"
rg -F -q 'k2_wide_mimalloc_object_lifecycle_queue_exe_guard.sh' "$INDEX"

if rg -n 'OSVM|OsVm|externcall|atomic|RawBuf|provider|global_allocator|install_hook|hook|pageSource|remote' "$APP" "$QUEUE" >"$FORBIDDEN_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-012 object queue must not activate substrate/provider/hook behavior" >&2
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

if rg -n 'mimalloc-object-lifecycle-queue-proof|HakoAllocObjectLifecyclePageQueue' \
  lang/c-abi/shims >"$INC_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-012 matcher leaked into .inc" >&2
  cat "$INC_LOG" >&2
  rm -f "$INC_LOG"
  exit 1
fi
rm -f "$INC_LOG"

pure_first_guard_build_toolchain

mir_json="$TMP_DIR/mimap012.mir.json"
exe_out="$TMP_DIR/mimap012.exe"
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
    "HakoAllocObjectLifecyclePageQueue.addPage/1",
    "HakoAllocObjectLifecyclePageQueue.selectPage/0",
    "HakoAllocPageModel.isDecommitted/0",
    "HakoAllocPageModel.isRetired/0",
    "HakoAllocPageModel.canReuse/0",
    "HakoAllocPageModel.reuse/0",
    "HakoAllocPageModel.freeCount/0",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing object queue function: {required}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
queue = plans.get("HakoAllocObjectLifecyclePageQueue")
if queue is None:
    raise SystemExit("missing object lifecycle queue typed object plan")
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

def require_lowering_symbol(fn, symbol):
    for entry in fn.get("metadata", {}).get("lowering_plan", []):
        if entry.get("symbol") == symbol:
            return entry
    raise SystemExit(f"missing lowering symbol {symbol} in {fn.get('name')}")

for name in ("addPage", "selectPage"):
    require_method(main, "HakoAllocObjectLifecyclePageQueue", name)
for name in ("birth", "acquire", "releaseLocal", "decommit"):
    require_method(main, "HakoAllocPageModel", name)
require_lowering_symbol(main, "HakoAllocPageModel.reuse/0")

select_fn = functions["HakoAllocObjectLifecyclePageQueue.selectPage/0"]
select_route = require_lowering_symbol(main, "HakoAllocObjectLifecyclePageQueue.selectPage/0")
if select_route.get("return_shape") != "object_handle":
    raise SystemExit(f"selectPage route must return object_handle, got {select_route.get('return_shape')}")
if select_route.get("target_result_box_name") != "HakoAllocPageModel":
    raise SystemExit(f"selectPage route must publish HakoAllocPageModel, got {select_route.get('target_result_box_name')}")
require_method(select_fn, "HakoAllocPageModel", "isDecommitted")
require_method(select_fn, "HakoAllocPageModel", "isRetired")
require_method(select_fn, "HakoAllocPageModel", "canReuse")
require_method(select_fn, "HakoAllocPageModel", "freeCount")

print("[mimap012-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-object-lifecycle-queue-proof' "$run_log"
rg -F -q 'pages=20,40,-1' "$run_log"
rg -F -q 'kinds=1,2,0' "$run_log"
rg -F -q 'queue=4,4,3,2,1,1,3,0,5,1,0' "$run_log"
rg -F -q 'shape=11' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

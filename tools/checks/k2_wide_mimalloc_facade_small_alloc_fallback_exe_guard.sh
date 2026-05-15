#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-small-alloc-fallback-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-small-alloc-fallback-proof/main.hako"
APP_README="apps/mimalloc-facade-small-alloc-fallback-proof/README.md"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
QUEUE="lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-358-MIMAP-014B-FACADE-SMALL-ALLOC-FALLBACK.md"
SSOT="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

for path in "$APP" "$APP_README" "$FACADE" "$QUEUE" "$PAGE" "$CARD" "$SSOT" "$INDEX" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_box as HakoAllocObjectLifecycleFacadeBox' "$APP"
rg -F -q 'objectLifecycleSmallAlloc(size)' "$FACADE"
rg -F -q 'local selected_kind = me.object_lifecycle_queue.last_selected_kind' "$FACADE"
rg -F -q 'if selected_kind == 1' "$FACADE"
rg -F -q 'if selected_kind != 2' "$FACADE"
rg -F -q 'page.reuse()' "$FACADE"
rg -F -q 'page.acquire(size)' "$FACADE"
rg -F -q 'me.reuse_select_count = me.reuse_select_count + 1' "$QUEUE"
rg -F -q 'me.active_select_count = me.active_select_count + 1' "$QUEUE"
rg -F -q 'MIMAP-014B active-page fallback and allocation miss' "$SSOT"
rg -F -q 'k2_wide_mimalloc_facade_small_alloc_fallback_exe_guard.sh' "$INDEX"
rg -F -q 'MIMAP-014B' "$README"

if rg -n 'releaseLocal\(|realloc[A-Za-z0-9_]*\(|align[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(' "$FACADE" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-014B facade must not activate release/realloc/substrate/provider/hook behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -F -q 'local page = me.object_lifecycle_queue.selectPage()' "$FACADE"; then
  echo "[$TAG] ERROR: MIMAP-014B must not bind the selected page object returned by selectPage" >&2
  exit 1
fi

if rg -n 'realloc[A-Za-z0-9_]*\(|align[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(' "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-014B proof app must not activate realloc/substrate/provider/hook behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-small-alloc-fallback-proof|objectLifecycleSmallAlloc|objectLifecycleAlloc(PageId|Reason|Ok)|objectLifecycleSelectedKind' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: MIMAP-014B matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap014b_facade_small_alloc_fallback.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap014b.mir.json"
exe_out="$tmp_dir/mimap014b.exe"
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
    "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocPageId/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocReason/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleSelectedKind/0",
    "HakoAllocObjectLifecyclePageQueue.selectPage/0",
    "HakoAllocPageModel.reuse/0",
    "HakoAllocPageModel.acquire/1",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing MIMAP-014B function: {required}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for required in ("HakoAllocObjectLifecycleFacade", "HakoAllocObjectLifecyclePageQueue", "HakoAllocPageModel"):
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

for name in (
    "objectLifecycleAddPage",
    "objectLifecycleSmallAlloc",
    "objectLifecycleAllocPageId",
    "objectLifecycleAllocReason",
    "objectLifecycleSelectedKind",
):
    require_method(main, "HakoAllocObjectLifecycleFacade", name)

small_alloc = functions["HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"]
require_method(small_alloc, "HakoAllocObjectLifecycleFacade", "resetSmallAllocResult")
require_method(small_alloc, "HakoAllocObjectLifecyclePageQueue", "selectPage")
require_method(small_alloc, "HakoAllocPageModel", "reuse")
require_method(small_alloc, "HakoAllocPageModel", "acquire")

select_fn = functions["HakoAllocObjectLifecyclePageQueue.selectPage/0"]
require_method(select_fn, "HakoAllocPageModel", "canReuse")
require_method(select_fn, "HakoAllocPageModel", "freeCount")

print("[mimap014b-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-small-alloc-fallback-proof' "$run_log"
rg -F -q 'reusable_page=50' "$run_log"
rg -F -q 'active_page=60' "$run_log"
rg -F -q 'miss_reason=1' "$run_log"
rg -F -q 'kinds=1,2,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

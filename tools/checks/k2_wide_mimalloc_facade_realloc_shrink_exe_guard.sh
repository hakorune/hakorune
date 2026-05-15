#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-realloc-shrink-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-realloc-shrink-proof/main.hako"
APP_README="apps/mimalloc-facade-realloc-shrink-proof/README.md"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-364-MIMAP-017A-REALLOC-SHRINK-SAME-PAGE.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

for path in "$APP" "$APP_README" "$FACADE" "$PAGE" "$CARD" "$INDEX" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.page_box as HakoAllocPageBox' "$APP"
rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_box as HakoAllocObjectLifecycleFacadeBox' "$APP"
rg -F -q 'last_realloc_page_id: i64 = -1' "$FACADE"
rg -F -q 'last_realloc_block_id: i64 = -1' "$FACADE"
rg -F -q 'last_realloc_requested_size: i64 = 0' "$FACADE"
rg -F -q 'last_realloc_reason: i64 = 0' "$FACADE"
rg -F -q 'last_realloc_ok: i64 = 0' "$FACADE"
rg -F -q 'validateReallocShrinkPage(page, block_id, requested_size)' "$FACADE"
rg -F -q 'objectLifecycleReallocShrink(page_id, block_id, requested_size)' "$FACADE"
rg -F -q 'return me.recordReallocFailure(4)' "$FACADE"
rg -F -q 'return me.recordReallocFailure(5)' "$FACADE"
rg -F -q 'page.block_used.get(block_id)' "$FACADE"
rg -F -q 'objectLifecycleReallocRequestedSize()' "$FACADE"
rg -F -q 'MIMAP-017A' "$CARD"
rg -F -q 'MIMAP-017A' "$README"
rg -F -q 'k2_wide_mimalloc_facade_realloc_shrink_exe_guard.sh' "$INDEX"

if rg -n 'copy[A-Za-z0-9_]*\(|byte[A-Za-z0-9_]*\(|memcpy|register[A-Za-z0-9_]*\(|unregister[A-Za-z0-9_]*\(|PageMap|page_map|lookup\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(' "$FACADE" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-017A facade must not activate copy/page-map/substrate/provider behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'objectLifecycleSmallAllocAligned\(|objectLifecycleReleaseBlock\(|releaseLocal\(|page\.acquire\(|objectLifecycleReallocGrow[A-Za-z0-9_]*\(|copy[A-Za-z0-9_]*\(|byte[A-Za-z0-9_]*\(|memcpy|register[A-Za-z0-9_]*\(|unregister[A-Za-z0-9_]*\(|PageMap|page_map|lookup\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(' "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-017A proof app must stay same-page shrink only" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-realloc-shrink-proof|objectLifecycleReallocShrink|objectLifecycleRealloc(PageId|BlockId|RequestedSize|Reason|Ok)' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: MIMAP-017A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap017a_facade_realloc_shrink.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap017a.mir.json"
exe_out="$tmp_dir/mimap017a.exe"
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
    "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReallocShrink/3",
    "HakoAllocObjectLifecycleFacade.validateReallocShrinkPage/3",
    "HakoAllocObjectLifecycleFacade.recordReallocFailure/1",
    "HakoAllocObjectLifecycleFacade.recordReallocSuccess/3",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReallocPageId/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReallocBlockId/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReallocRequestedSize/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReallocReason/0",
    "HakoAllocPageModel.acquire/1",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing MIMAP-017A function: {required}")

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
    "objectLifecycleSmallAlloc",
    "objectLifecycleReallocShrink",
    "objectLifecycleReallocPageId",
    "objectLifecycleReallocBlockId",
    "objectLifecycleReallocRequestedSize",
    "objectLifecycleReallocReason",
):
    require_method(main, "HakoAllocObjectLifecycleFacade", name)

realloc_fn = functions["HakoAllocObjectLifecycleFacade.objectLifecycleReallocShrink/3"]
require_method(realloc_fn, "HakoAllocObjectLifecycleFacade", "resetReallocResult")
require_method(realloc_fn, "HakoAllocObjectLifecycleFacade", "recordReallocFailure")
require_method(realloc_fn, "HakoAllocObjectLifecycleFacade", "validateReallocShrinkPage")

validate_fn = functions["HakoAllocObjectLifecycleFacade.validateReallocShrinkPage/3"]
require_method(validate_fn, "RuntimeDataBox", "get")
require_method(validate_fn, "HakoAllocObjectLifecycleFacade", "recordReallocFailure")
require_method(validate_fn, "HakoAllocObjectLifecycleFacade", "recordReallocSuccess")

print("[mimap017a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-realloc-shrink-proof' "$run_log"
rg -F -q 'shrink=1,150,0,4,0' "$run_log"
rg -F -q 'reject=0,4' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

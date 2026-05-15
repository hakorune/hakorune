#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-release-one-block-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-release-one-block-proof/main.hako"
APP_README="apps/mimalloc-facade-release-one-block-proof/README.md"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
REASON="lang/src/hako_alloc/memory/object_lifecycle_facade_reason_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-360-MIMAP-015A-FACADE-RELEASE-ONE-BLOCK.md"
SSOT="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

for path in "$APP" "$APP_README" "$FACADE" "$REASON" "$PAGE" "$CARD" "$SSOT" "$INDEX" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_box as HakoAllocObjectLifecycleFacadeBox' "$APP"
rg -F -q 'objectLifecycleReleaseBlock(page_id, block_id)' "$FACADE"
rg -F -q 'objectLifecycleKnownPageIndexById(page_id)' "$FACADE"
rg -F -q 'recordReleaseFailure(reason)' "$FACADE"
rg -F -q 'recordReleaseSuccess(page_id, block_id)' "$FACADE"
rg -F -q 'page.releaseLocal(block_id)' "$FACADE"
rg -F -q 'release_no_page()' "$REASON"
rg -F -q 'release_bad_block()' "$REASON"
rg -F -q 'objectLifecycleReleasePageId()' "$FACADE"
rg -F -q 'objectLifecycleReleaseBlockId()' "$FACADE"
rg -F -q 'objectLifecycleReleaseReason()' "$FACADE"
rg -F -q 'objectLifecycleReleaseOk()' "$FACADE"
rg -F -q 'MIMAP-015A' "$CARD"
rg -F -q 'MIMAP-015A' "$SSOT"
rg -F -q 'k2_wide_mimalloc_facade_release_one_block_exe_guard.sh' "$INDEX"
rg -F -q 'MIMAP-015A' "$README"

if rg -n 'allocateAligned[A-Za-z0-9_]*\(|aligned_good_size[A-Za-z0-9_]*\(|padded_request_size[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(|PageMap|page_map|lookup\(' "$FACADE" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-015A facade must not activate substrate/provider/page-map behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'releaseLocal\(|realloc[A-Za-z0-9_]*\(|align[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(|PageMap|page_map|lookup\(' "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-015A proof app must route release through the facade only" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-release-one-block-proof|objectLifecycleRelease(Block|Page|Reason|Ok)' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: MIMAP-015A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap015a_facade_release.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap015a.mir.json"
exe_out="$tmp_dir/mimap015a.exe"
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
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2",
    "HakoAllocObjectLifecycleFacade.recordReleaseFailure/1",
    "HakoAllocObjectLifecycleFacade.recordReleaseSuccess/2",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleasePageId/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlockId/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseReason/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseOk/0",
    "HakoAllocPageModel.releaseLocal/1",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing MIMAP-015A function: {required}")

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
    "objectLifecycleReleaseBlock",
    "objectLifecycleReleasePageId",
    "objectLifecycleReleaseBlockId",
    "objectLifecycleReleaseReason",
    "objectLifecycleReleaseOk",
):
    require_method(main, "HakoAllocObjectLifecycleFacade", name)

release_fn = functions["HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2"]
require_method(release_fn, "HakoAllocObjectLifecycleFacade", "resetReleaseResult")
require_method(release_fn, "HakoAllocObjectLifecycleFacade", "recordReleaseFailure")
require_method(release_fn, "HakoAllocObjectLifecycleFacade", "recordReleaseSuccess")
require_method(release_fn, "HakoAllocPageModel", "releaseLocal")

print("[mimap015a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-release-one-block-proof' "$run_log"
rg -F -q 'alloc=90,0' "$run_log"
rg -F -q 'release=90,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

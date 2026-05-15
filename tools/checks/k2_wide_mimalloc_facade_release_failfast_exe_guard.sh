#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-release-failfast-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-release-failfast-proof/main.hako"
APP_README="apps/mimalloc-facade-release-failfast-proof/README.md"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-361-MIMAP-015B-FACADE-RELEASE-FAILFAST.md"
SSOT="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

for path in "$APP" "$APP_README" "$FACADE" "$CARD" "$SSOT" "$INDEX" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_box as HakoAllocObjectLifecycleFacadeBox' "$APP"
rg -F -q 'objectLifecycleReleaseBlock(page_id, block_id)' "$FACADE"
rg -F -q 'return me.recordReleaseFailure(3)' "$FACADE"
rg -F -q 'return me.recordReleaseFailure(1)' "$FACADE"
rg -F -q 'MIMAP-015B' "$CARD"
rg -F -q 'MIMAP-015B' "$README"
rg -F -q 'k2_wide_mimalloc_facade_release_failfast_exe_guard.sh' "$INDEX"

if rg -n 'realloc[A-Za-z0-9_]*\(|align[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(|PageMap|page_map|lookup\(' "$FACADE" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-015B facade must not activate realloc/alignment/substrate/provider/page-map behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'releaseLocal\(|realloc[A-Za-z0-9_]*\(|align[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(|PageMap|page_map|lookup\(' "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-015B proof app must route release through the facade only" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-release-failfast-proof|objectLifecycleReleaseBlock' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: MIMAP-015B matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap015b_facade_release_failfast.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap015b.mir.json"
exe_out="$tmp_dir/mimap015b.exe"
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
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseReason/0",
    "HakoAllocPageModel.releaseLocal/1",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing MIMAP-015B function: {required}")

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

require_method(main, "HakoAllocObjectLifecycleFacade", "objectLifecycleSmallAlloc")
require_method(main, "HakoAllocObjectLifecycleFacade", "objectLifecycleReleaseBlock")
require_method(main, "HakoAllocObjectLifecycleFacade", "objectLifecycleReleaseReason")

release_fn = functions["HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2"]
require_method(release_fn, "HakoAllocPageModel", "releaseLocal")
require_method(release_fn, "HakoAllocObjectLifecycleFacade", "recordReleaseFailure")

print("[mimap015b-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-release-failfast-proof' "$run_log"
rg -F -q 'double=0,3' "$run_log"
rg -F -q 'stale=0,1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

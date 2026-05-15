#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-aligned-alloc-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-aligned-alloc-proof/main.hako"
APP_README="apps/mimalloc-facade-aligned-alloc-proof/README.md"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
REASON="lang/src/hako_alloc/memory/object_lifecycle_facade_reason_box.hako"
ALIGNMENT="lang/src/hako_alloc/memory/alignment_policy_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-363-MIMAP-016B-ALIGNED-ALLOC-SUCCESS-FAIL.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

for path in "$APP" "$APP_README" "$FACADE" "$REASON" "$ALIGNMENT" "$PAGE" "$CARD" "$INDEX" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.page_box as HakoAllocPageBox' "$APP"
rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_box as HakoAllocObjectLifecycleFacadeBox' "$APP"
rg -F -q 'objectLifecycleSmallAllocAligned(size, alignment)' "$FACADE"
rg -F -q 'objectLifecycleRecordAlignmentRequest(alignment)' "$FACADE"
rg -F -q 'return me.recordSmallAllocFailure(HakoAllocObjectLifecycleFacadeReason.small_alignment_unsupported())' "$FACADE"
rg -F -q 'small_alignment_unsupported()' "$REASON"
rg -F -q 'return me.objectLifecycleSmallAlloc(size)' "$FACADE"
rg -F -q 'HakoAllocAlignmentPolicy.normalize_alignment(alignment)' "$FACADE"
rg -F -q 'MIMAP-016B' "$CARD"
rg -F -q 'MIMAP-016B' "$README"
rg -F -q 'k2_wide_mimalloc_facade_aligned_alloc_exe_guard.sh' "$INDEX"

if rg -n 'aligned_good_size[A-Za-z0-9_]*\(|padded_request_size[A-Za-z0-9_]*\(|PageMap|page_map|lookup\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(' "$FACADE" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-016B facade must not activate padded/native/page-map/substrate/provider behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'objectLifecycleSmallAlloc\(|objectLifecycleReleaseBlock\(|releaseLocal\(|page\.acquire\(|aligned_good_size[A-Za-z0-9_]*\(|padded_request_size[A-Za-z0-9_]*\(|PageMap|page_map|lookup\(|realloc[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(' "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-016B proof app must route through the aligned facade method only" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-aligned-alloc-proof|objectLifecycleSmallAllocAligned' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: MIMAP-016B matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap016b_facade_aligned_alloc.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap016b.mir.json"
exe_out="$tmp_dir/mimap016b.exe"
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
    "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAllocAligned/2",
    "HakoAllocObjectLifecycleFacade.objectLifecycleRecordAlignmentRequest/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
    "HakoAllocObjectLifecycleFacade.recordSmallAllocFailure/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocPageId/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocBlockId/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocReason/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAlignmentNormalized/0",
    "HakoAllocAlignmentPolicy.normalize_alignment/1",
    "HakoAllocPageModel.acquire/1",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing MIMAP-016B function: {required}")

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
    "objectLifecycleSmallAllocAligned",
    "objectLifecycleAllocPageId",
    "objectLifecycleAllocBlockId",
    "objectLifecycleAllocReason",
    "objectLifecycleAlignmentNormalized",
):
    require_method(main, "HakoAllocObjectLifecycleFacade", name)

aligned = functions["HakoAllocObjectLifecycleFacade.objectLifecycleSmallAllocAligned/2"]
require_method(aligned, "HakoAllocObjectLifecycleFacade", "objectLifecycleRecordAlignmentRequest")
require_method(aligned, "HakoAllocObjectLifecycleFacade", "resetSmallAllocResult")
require_method(aligned, "HakoAllocObjectLifecycleFacade", "recordSmallAllocFailure")
require_method(aligned, "HakoAllocObjectLifecycleFacade", "objectLifecycleSmallAlloc")

small_alloc = functions["HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"]
require_method(small_alloc, "HakoAllocPageModel", "acquire")

print("[mimap016b-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-aligned-alloc-proof' "$run_log"
rg -F -q 'aligned=1,140,0,8,0' "$run_log"
rg -F -q 'reject=0,-1,5' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

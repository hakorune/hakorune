#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-alignment-metadata-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-alignment-metadata-proof/main.hako"
APP_README="apps/mimalloc-facade-alignment-metadata-proof/README.md"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
RESULT="lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako"
ALIGNMENT="lang/src/hako_alloc/memory/alignment_policy_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-362-MIMAP-016A-ALIGNMENT-METADATA-OBSERVERS.md"
SSOT="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

for path in "$APP" "$APP_README" "$FACADE" "$RESULT" "$ALIGNMENT" "$CARD" "$SSOT" "$INDEX" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_box as HakoAllocObjectLifecycleFacadeBox' "$APP"
rg -F -q 'using selfhost.hako_alloc.memory.alignment_policy_box as HakoAllocAlignmentPolicy' "$FACADE"
rg -F -q 'alignment_result: HakoAllocObjectLifecycleAlignmentResult = new HakoAllocObjectLifecycleAlignmentResult()' "$FACADE"
rg -F -q 'last_requested: i64 = -1' "$RESULT"
rg -F -q 'last_normalized: i64 = -1' "$RESULT"
rg -F -q 'last_reason: i64 = 0' "$RESULT"
rg -F -q 'last_supported: i64 = 0' "$RESULT"
rg -F -q 'objectLifecycleRecordAlignmentRequest(alignment)' "$FACADE"
rg -F -q 'HakoAllocAlignmentPolicy.normalize_alignment(alignment)' "$FACADE"
rg -F -q 'objectLifecycleAlignmentRequested()' "$FACADE"
rg -F -q 'objectLifecycleAlignmentNormalized()' "$FACADE"
rg -F -q 'objectLifecycleAlignmentReason()' "$FACADE"
rg -F -q 'objectLifecycleAlignmentSupported()' "$FACADE"
rg -F -q 'MIMAP-016A' "$CARD"
rg -F -q 'MIMAP-016A' "$README"
rg -F -q 'k2_wide_mimalloc_facade_alignment_metadata_exe_guard.sh' "$INDEX"

if rg -n 'allocateAligned[A-Za-z0-9_]*\(|aligned_good_size[A-Za-z0-9_]*\(|padded_request_size[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(|PageMap|page_map|lookup\(' "$FACADE" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-016A facade must not activate substrate/provider/page-map behavior" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'objectLifecycleSmallAlloc\(|objectLifecycleReleaseBlock\(|releaseLocal\(|HakoAllocPageModel|objectLifecycleAligned[A-Za-z0-9_]*\(|allocateAligned[A-Za-z0-9_]*\(|aligned_good_size[A-Za-z0-9_]*\(|padded_request_size[A-Za-z0-9_]*\(|realloc[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(|PageMap|page_map|lookup\(' "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-016A proof app must stay metadata-only" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-alignment-metadata-proof|objectLifecycleRecordAlignmentRequest|objectLifecycleAlignment(Requested|Normalized|Reason|Supported)' \
  lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: MIMAP-016A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap016a_facade_alignment_metadata.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap016a.mir.json"
exe_out="$tmp_dir/mimap016a.exe"
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
    "HakoAllocObjectLifecycleFacade.objectLifecycleRecordAlignmentRequest/1",
    "HakoAllocObjectLifecycleFacade.recordAlignmentFailure/2",
    "HakoAllocObjectLifecycleFacade.recordAlignmentSuccess/2",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAlignmentRequested/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAlignmentNormalized/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAlignmentReason/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAlignmentSupported/0",
    "HakoAllocAlignmentPolicy.normalize_alignment/1",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing MIMAP-016A function: {required}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def require_call(fn, box_name, name):
    for callee in iter_calls(fn):
        if callee.get("box_name") == box_name and callee.get("name") == name:
            return
    raise SystemExit(f"missing call {box_name}.{name} in {fn.get('name')}")

def require_global(fn, name):
    for callee in iter_calls(fn):
        if callee.get("type") == "Global" and callee.get("name") == name:
            return
    raise SystemExit(f"missing global call {name} in {fn.get('name')}")

for name in (
    "objectLifecycleRecordAlignmentRequest",
    "objectLifecycleAlignmentRequested",
    "objectLifecycleAlignmentNormalized",
    "objectLifecycleAlignmentReason",
    "objectLifecycleAlignmentSupported",
):
    require_call(main, "HakoAllocObjectLifecycleFacade", name)

record = functions["HakoAllocObjectLifecycleFacade.objectLifecycleRecordAlignmentRequest/1"]
require_global(record, "HakoAllocAlignmentPolicy.normalize_alignment/1")
require_call(record, "HakoAllocObjectLifecycleFacade", "recordAlignmentFailure")
require_call(record, "HakoAllocObjectLifecycleFacade", "recordAlignmentSuccess")

print("[mimap016a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-alignment-metadata-proof' "$run_log"
rg -F -q 'align=4,8,0' "$run_log"
rg -F -q 'unsupported=3,-1,1' "$run_log"
rg -F -q 'support=1,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

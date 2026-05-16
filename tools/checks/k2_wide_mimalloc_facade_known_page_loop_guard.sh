#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-known-page-loop"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-known-page-loop-proof/main.hako"
APP_README="apps/mimalloc-facade-known-page-loop-proof/README.md"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-469-MIMAP-038A-FACADE-KNOWN-PAGE-LOOP.md"
SSOT="docs/development/current/main/design/mimalloc-facade-known-page-loop-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

echo "[$TAG] running MIMAP-038A facade known-page loop guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$FACADE" "$CARD" "$SSOT" "$INDEX" "$README"
guard_require_exec_files "$TAG" "$0"

guard_expect_in_file "$TAG" "Status: landed" "$CARD" "MIMAP-038A card must be landed after implementation"
guard_expect_in_file "$TAG" "Decision: accepted" "$SSOT" "MIMAP-038A SSOT must be accepted"
guard_expect_in_file "$TAG" "objectLifecycleKnownPageIndexById\\(page_id\\)" "$FACADE" "facade lookup method missing"
guard_expect_in_file "$TAG" "local count = pages\\.length\\(\\)" "$FACADE" "facade lookup must use queue length"
guard_expect_in_file "$TAG" "loop\\(i < count\\)" "$FACADE" "facade lookup must use a loop"
guard_expect_in_file "$TAG" "local page = pages\\.get\\(i\\)" "$FACADE" "facade lookup must read by loop index"
guard_expect_in_file "$TAG" "return i" "$FACADE" "facade lookup must return the matched loop index"
guard_expect_in_file "$TAG" "mimalloc-facade-known-page-loop-proof" "$APP" "proof app marker missing"
guard_expect_in_file "$TAG" "objectLifecycleKnownPageIndexById\\(40\\)" "$APP" "proof app must check fourth page lookup"
guard_expect_in_file "$TAG" "objectLifecycleReleaseBlock\\(40, block3\\)" "$APP" "proof app must use fourth-page lookup through release"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list MIMAP-038A guard"
guard_expect_in_file "$TAG" "objectLifecycleKnownPageIndexById\\(page_id\\)" "$README" "memory README must name lookup contract"

if rg -n 'local page[012] = pages\.get\([012]\)' "$FACADE" >/tmp/"$TAG".fixed_lookup 2>&1; then
  cat /tmp/"$TAG".fixed_lookup >&2
  rm -f /tmp/"$TAG".fixed_lookup
  guard_fail "$TAG" "facade known-page lookup must not return to fixed three-page shape"
fi
rm -f /tmp/"$TAG".fixed_lookup

if rg -n 'OSVM|OsVm|externcall|atomic|RawBuf|provider|global_allocator|install_hook|hook|pageSource|remote' "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  guard_fail "$TAG" "proof app must not activate substrate/provider/hook behavior"
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-known-page-loop|objectLifecycleKnownPageIndexById' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  guard_fail "$TAG" "MIMAP-038A matcher leaked into .inc"
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap038a_known_page_loop.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap038a.mir.json"
exe_out="$tmp_dir/mimap038a.exe"
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

required = (
    "HakoAllocObjectLifecycleFacade.objectLifecycleAddPage/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleKnownPageIndexById/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseOk/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseReason/0",
)
for name in required:
    if functions.get(name) is None:
        raise SystemExit(f"missing MIMAP-038A function: {name}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for required_box in ("HakoAllocObjectLifecycleFacade", "HakoAllocObjectLifecyclePageQueue", "HakoAllocPageModel"):
    if plans.get(required_box) is None:
        raise SystemExit(f"missing typed object plan: {required_box}")

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

def require_method_name(fn, name):
    for callee in iter_calls(fn):
        if callee.get("type") == "Method" and callee.get("name") == name:
            return
    raise SystemExit(f"missing method call *.{name} in {fn.get('name')}")

for name in (
    "objectLifecycleAddPage",
    "objectLifecycleKnownPageIndexById",
    "objectLifecycleReleaseBlock",
    "objectLifecycleReleaseOk",
    "objectLifecycleReleaseReason",
):
    require_method(main, "HakoAllocObjectLifecycleFacade", name)

lookup_fn = functions["HakoAllocObjectLifecycleFacade.objectLifecycleKnownPageIndexById/1"]
require_method(lookup_fn, "ArrayBox", "length")
require_method_name(lookup_fn, "get")
release_fn = functions["HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2"]
require_method(release_fn, "HakoAllocObjectLifecycleFacade", "objectLifecycleKnownPageIndexById")
require_method(release_fn, "HakoAllocPageModel", "releaseLocal")

print("[mimap038a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-known-page-loop-proof' "$run_log"
rg -F -q 'adds=0,1,2,3' "$run_log"
rg -F -q 'idx=0,3,-1' "$run_log"
rg -F -q 'release=1,1,0' "$run_log"
rg -F -q 'shape=11' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-release-failfast-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-release-failfast-proof/main.hako"
APP_README="apps/mimalloc-facade-release-failfast-proof/README.md"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
REASON="lang/src/hako_alloc/memory/object_lifecycle_facade_reason_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-361-MIMAP-015B-FACADE-RELEASE-FAILFAST.md"
SSOT="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"
RESULT="lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako"
ARTIFACT_DIR="$ROOT_DIR/target/checks/$TAG"
TMP_DIR="$ARTIFACT_DIR/tmp"
FORBIDDEN_LOG="$ARTIFACT_DIR/forbidden.log"
APP_FORBIDDEN_LOG="$ARTIFACT_DIR/app.forbidden.log"
INC_LOG="$ARTIFACT_DIR/app_specific.inc.log"

mkdir -p "$ARTIFACT_DIR"
rm -rf "$TMP_DIR"
mkdir -p "$TMP_DIR"
rm -f "$FORBIDDEN_LOG" "$APP_FORBIDDEN_LOG" "$INC_LOG"

for path in "$APP" "$APP_README" "$FACADE" "$REASON" "$RESULT" "$CARD" "$SSOT" "$INDEX" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_box as HakoAllocObjectLifecycleFacadeBox' "$APP"
rg -F -q 'objectLifecycleReleaseBlock(page_id, block_id)' "$FACADE"
rg -F -q 'return me.recordReleaseFailure(HakoAllocObjectLifecycleFacadeReason.release_page_reject())' "$FACADE"
rg -F -q 'return me.recordReleaseFailure(HakoAllocObjectLifecycleFacadeReason.release_no_page())' "$FACADE"
rg -F -q 'objectLifecycleReleaseSuccessCount()' "$FACADE"
rg -F -q 'objectLifecycleReleaseFailureCount()' "$FACADE"
rg -F -q 'success_count: usize = 0' "$RESULT"
rg -F -q 'failure_count: usize = 0' "$RESULT"
rg -F -q 'release_page_reject()' "$REASON"
rg -F -q 'release_no_page()' "$REASON"
rg -F -q 'MIMAP-015B' "$CARD"
rg -F -q 'MIMAP-015B' "$README"
rg -F -q 'k2_wide_mimalloc_facade_release_failfast_exe_guard.sh' "$INDEX"

if rg -n 'allocateAligned[A-Za-z0-9_]*\(|aligned_good_size[A-Za-z0-9_]*\(|padded_request_size[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(|PageMap|page_map|lookup\(' "$FACADE" >"$FORBIDDEN_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-015B facade must not activate substrate/provider/page-map behavior" >&2
  cat "$FORBIDDEN_LOG" >&2
  rm -f "$FORBIDDEN_LOG"
  exit 1
fi
rm -f "$FORBIDDEN_LOG"

if rg -n 'releaseLocal\(|realloc[A-Za-z0-9_]*\(|align[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(|PageMap|page_map|lookup\(' "$APP" >"$APP_FORBIDDEN_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-015B proof app must route release through the facade only" >&2
  cat "$APP_FORBIDDEN_LOG" >&2
  rm -f "$APP_FORBIDDEN_LOG"
  exit 1
fi
rm -f "$APP_FORBIDDEN_LOG"

if rg -n 'mimalloc-facade-release-failfast-proof|objectLifecycleRelease(Block|SuccessCount|FailureCount)' \
  lang/c-abi/shims >"$INC_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-015B matcher leaked into .inc" >&2
  cat "$INC_LOG" >&2
  rm -f "$INC_LOG"
  exit 1
fi
rm -f "$INC_LOG"

pure_first_guard_build_toolchain

mir_json="$TMP_DIR/mimap015b.mir.json"
exe_out="$TMP_DIR/mimap015b.exe"
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
    "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseReason/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseSuccessCount/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseFailureCount/0",
    "HakoAllocPageModel.releaseLocal/1",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing MIMAP-015B function: {required}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
result = plans.get("HakoAllocObjectLifecycleReleaseResult")
if result is None:
    raise SystemExit("missing typed object plan: HakoAllocObjectLifecycleReleaseResult")
fields = {field.get("name"): field for field in result.get("fields", [])}
for name in ("success_count", "failure_count"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"release result {name} must be exact usize storage: {field}")
for name in ("last_page_id", "last_block_id", "last_reason", "last_ok"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"release result {name} must remain signed storage: {field}")

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
require_method(main, "HakoAllocObjectLifecycleFacade", "objectLifecycleReleaseSuccessCount")
require_method(main, "HakoAllocObjectLifecycleFacade", "objectLifecycleReleaseFailureCount")

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
rg -F -q 'release_counts=1,2' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

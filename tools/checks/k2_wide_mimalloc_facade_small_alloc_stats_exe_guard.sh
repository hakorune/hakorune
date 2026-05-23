#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-small-alloc-stats-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-small-alloc-stats-proof/main.hako"
APP_README="apps/mimalloc-facade-small-alloc-stats-proof/README.md"
FACADE="lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
RESULT="lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-359-MIMAP-014C-ALLOC-STATS-OBSERVERS.md"
SSOT="docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"
ARTIFACT_DIR="$ROOT_DIR/target/checks/$TAG"
TMP_DIR="$ARTIFACT_DIR/tmp"
FORBIDDEN_LOG="$ARTIFACT_DIR/forbidden.log"
APP_FORBIDDEN_LOG="$ARTIFACT_DIR/app.forbidden.log"
INC_LOG="$ARTIFACT_DIR/app_specific.inc.log"

mkdir -p "$ARTIFACT_DIR"
rm -rf "$TMP_DIR"
mkdir -p "$TMP_DIR"
rm -f "$FORBIDDEN_LOG" "$APP_FORBIDDEN_LOG" "$INC_LOG"

for path in "$APP" "$APP_README" "$FACADE" "$RESULT" "$CARD" "$SSOT" "$INDEX" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'using selfhost.hako_alloc.memory.object_lifecycle_facade_box as HakoAllocObjectLifecycleFacadeBox' "$APP"
rg -F -q 'alloc_result: HakoAllocObjectLifecycleAllocResult = new HakoAllocObjectLifecycleAllocResult()' "$FACADE"
rg -F -q 'attempt_count: usize = 0' "$RESULT"
rg -F -q 'success_count: usize = 0' "$RESULT"
rg -F -q 'failure_count: usize = 0' "$RESULT"
rg -F -q 'reusable_success_count: usize = 0' "$RESULT"
rg -F -q 'active_success_count: usize = 0' "$RESULT"
rg -F -q 'recordSmallAllocFailure(reason)' "$FACADE"
rg -F -q 'recordSmallAllocSuccess(selected_kind)' "$FACADE"
rg -F -q 'objectLifecycleAllocAttemptCount()' "$FACADE"
rg -F -q 'objectLifecycleAllocSuccessCount()' "$FACADE"
rg -F -q 'objectLifecycleAllocFailureCount()' "$FACADE"
rg -F -q 'objectLifecycleAllocReusableSuccessCount()' "$FACADE"
rg -F -q 'objectLifecycleAllocActiveSuccessCount()' "$FACADE"
rg -F -q 'MIMAP-014C allocation fast-path stats observers' "$SSOT"
rg -F -q 'k2_wide_mimalloc_facade_small_alloc_stats_exe_guard.sh' "$INDEX"
rg -F -q 'MIMAP-014C' "$README"

if rg -n 'allocateAligned[A-Za-z0-9_]*\(|aligned_good_size[A-Za-z0-9_]*\(|padded_request_size[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(' "$FACADE" >"$FORBIDDEN_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-014C facade must not activate substrate/provider/hook behavior" >&2
  cat "$FORBIDDEN_LOG" >&2
  rm -f "$FORBIDDEN_LOG"
  exit 1
fi
rm -f "$FORBIDDEN_LOG"

if rg -n 'objectLifecycleReleaseBlock\(|realloc[A-Za-z0-9_]*\(|align[A-Za-z0-9_]*\(|OSVM|OsVm|externcall|atomic[A-Za-z0-9_]*\(|RawBuf|provider[A-Za-z0-9_]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*\(|pageSource|remote[A-Za-z0-9_]*\(' "$APP" >"$APP_FORBIDDEN_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-014C proof app must not activate facade release/realloc/substrate/provider/hook behavior" >&2
  cat "$APP_FORBIDDEN_LOG" >&2
  rm -f "$APP_FORBIDDEN_LOG"
  exit 1
fi
rm -f "$APP_FORBIDDEN_LOG"

if rg -n 'mimalloc-facade-small-alloc-stats-proof|objectLifecycleAlloc(Attempt|Success|Failure|ReusableSuccess|ActiveSuccess)Count' \
  lang/c-abi/shims >"$INC_LOG" 2>&1; then
  echo "[$TAG] ERROR: MIMAP-014C matcher leaked into .inc" >&2
  cat "$INC_LOG" >&2
  rm -f "$INC_LOG"
  exit 1
fi
rm -f "$INC_LOG"

pure_first_guard_build_toolchain

mir_json="$TMP_DIR/mimap014c.mir.json"
exe_out="$TMP_DIR/mimap014c.exe"
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
    "HakoAllocObjectLifecycleFacade.recordSmallAllocFailure/1",
    "HakoAllocObjectLifecycleFacade.recordSmallAllocSuccess/1",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocAttemptCount/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocSuccessCount/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocFailureCount/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocReusableSuccessCount/0",
    "HakoAllocObjectLifecycleFacade.objectLifecycleAllocActiveSuccessCount/0",
):
    if functions.get(required) is None:
        raise SystemExit(f"missing MIMAP-014C function: {required}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
result = plans.get("HakoAllocObjectLifecycleAllocResult")
if result is None:
    raise SystemExit("missing typed object plan: HakoAllocObjectLifecycleAllocResult")
fields = {field.get("name"): field for field in result.get("fields", [])}
for name in (
    "attempt_count",
    "success_count",
    "failure_count",
    "reusable_success_count",
    "active_success_count",
):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"alloc result {name} must be exact usize storage: {field}")
for name in ("last_page_id", "last_block_id", "last_reason", "last_ok"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"alloc result {name} must remain signed storage: {field}")

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
    "objectLifecycleAllocAttemptCount",
    "objectLifecycleAllocSuccessCount",
    "objectLifecycleAllocFailureCount",
    "objectLifecycleAllocReusableSuccessCount",
    "objectLifecycleAllocActiveSuccessCount",
):
    require_method(main, "HakoAllocObjectLifecycleFacade", name)

small_alloc = functions["HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"]
require_method(small_alloc, "HakoAllocObjectLifecycleFacade", "recordSmallAllocFailure")
require_method(small_alloc, "HakoAllocObjectLifecycleFacade", "recordSmallAllocSuccess")

print("[mimap014c-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-small-alloc-stats-proof' "$run_log"
rg -F -q 'attempts=3' "$run_log"
rg -F -q 'successes=2' "$run_log"
rg -F -q 'failures=1' "$run_log"
rg -F -q 'by_kind=1,1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

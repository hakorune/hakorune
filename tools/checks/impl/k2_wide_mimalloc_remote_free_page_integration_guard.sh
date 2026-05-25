#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="k2-wide-mimalloc-remote-free-page-integration"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

INBOX="lang/src/hako_alloc/memory/remote_free_page_integration_box.hako"
FACADE="lang/src/hako_alloc/memory/allocator_facade_box.hako"
REMOTE_POLICY="lang/src/hako_alloc/memory/remote_free_policy_box.hako"
PAGE_BOX="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
APP="apps/mimalloc-remote-free-page-integration-proof/main.hako"
APP_TEST="apps/mimalloc-remote-free-page-integration-proof/test.sh"
APP_README="apps/mimalloc-remote-free-page-integration-proof/README.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-178-M170-MIMALLOC-REMOTE-FREE-INTEGRATION.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_remote_free_page_integration_guard.sh"

echo "[$TAG] running M170 mimalloc remote-free page integration guard"

guard_require_files \
  "$TAG" \
  "$INBOX" \
  "$FACADE" \
  "$REMOTE_POLICY" \
  "$PAGE_BOX" \
  "$MODULE" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$PLAN" \
  "$CARD" \
  "$INDEX"

guard_expect_in_file "$TAG" 'memory.remote_free_page_integration_box = "memory/remote_free_page_integration_box.hako"' "$MODULE" "hako module must export M170 integration owner"
guard_expect_in_file "$TAG" 'box HakoAllocRemoteFreePageInbox' "$INBOX" "M170 integration owner must exist"
guard_expect_in_file "$TAG" 'box HakoAllocRemoteFreePagePort' "$INBOX" "M170 integration owner must expose facade-backed page port"
guard_expect_in_file "$TAG" 'box HakoAllocRemoteFreePagePortSnapshot' "$INBOX" "M170 integration owner must expose a port snapshot capsule"
guard_expect_in_file "$TAG" 'box HakoAllocRemoteFreePageExerciseReport' "$INBOX" "M170 integration owner must expose an exercise report capsule"
guard_expect_in_file "$TAG" 'captureSnapshot\(snapshot\)' "$INBOX" "M170 exercise report must capture the port snapshot contract"
guard_expect_in_file "$TAG" 'captureFacadeBlocks\(alloc, port\)' "$INBOX" "M170 exercise report must capture acquired block results through the facade seam"
guard_expect_in_file "$TAG" 'me\.page = new HakoAllocPageModel' "$INBOX" "M170 port must own page creation"
guard_expect_in_file "$TAG" 'me\.inbox = new HakoAllocRemoteFreePageInbox' "$INBOX" "M170 port must compose inbox creation"
guard_expect_in_file "$TAG" 'captureObservedPointers\(head, next_b, next_a, block_a, block_b\)' "$INBOX" "M170 exercise report must capture pointer observations"
guard_expect_in_file "$TAG" 'captureFacadePublish\(alloc, port, block_a, block_b\)' "$INBOX" "M170 exercise report must capture publish results through the facade seam"
guard_expect_in_file "$TAG" 'captureFacadeObservedPointers\(alloc, port, block_a, block_b\)' "$INBOX" "M170 exercise report must capture pointer observations through the facade seam"
guard_expect_in_file "$TAG" 'captureFacadeCollect\(alloc, port\)' "$INBOX" "M170 exercise report must capture collect results through the facade seam"
guard_expect_in_file "$TAG" 'captureFacadeSnapshot\(alloc, port\)' "$INBOX" "M170 exercise report must capture the port snapshot through the facade seam"
guard_expect_in_file "$TAG" 'captureFacadeExercise\(alloc, head_cell, block_a, block_b\)' "$INBOX" "M170 exercise report must orchestrate the facade exercise seam"
guard_expect_in_file "$TAG" 'summaryLine\(\)' "$INBOX" "M170 exercise report must expose a summary render helper"
guard_expect_in_file "$TAG" 'blocksLine\(\)' "$INBOX" "M170 exercise report must expose a blocks render helper"
guard_expect_in_file "$TAG" 'snapshot\(\)' "$INBOX" "M170 port must expose snapshot access"
guard_expect_in_file "$TAG" 'remotePagePort\(page_id, block_size, capacity, reserved, head_cell\)' "$FACADE" "production facade must vend remote-free page ports"
guard_expect_in_file "$TAG" 'remotePageAcquire\(port, requested_size\)' "$FACADE" "production facade must route page-port acquire through the facade seam"
guard_expect_in_file "$TAG" 'remotePagePublishPort\(port, block_ptr, block_id, interferer_ptr\)' "$FACADE" "production facade must route page-port publish through the facade seam"
guard_expect_in_file "$TAG" 'remotePageCollectPendingPort\(port\)' "$FACADE" "production facade must route page-port drain through the facade seam"
guard_expect_in_file "$TAG" 'remotePagePeekPortHead\(port\)' "$FACADE" "production facade must route page-port head peek through the facade seam"
guard_expect_in_file "$TAG" 'remotePagePeekPortNext\(port, block_ptr\)' "$FACADE" "production facade must route page-port next peek through the facade seam"
guard_expect_in_file "$TAG" 'remotePagePortSnapshot\(port\)' "$FACADE" "production facade must route page-port snapshot access through the facade seam"
guard_expect_in_file "$TAG" 'remotePageInbox\(page, head_cell\)' "$FACADE" "production facade must expose remote-free page inbox composition"
guard_expect_in_file "$TAG" 'remotePagePublish\(inbox, block_ptr, block_id, interferer_ptr\)' "$FACADE" "production facade must publish remote-free page inbox operations"
guard_expect_in_file "$TAG" 'remotePageCollectPending\(inbox\)' "$FACADE" "production facade must drain remote-free page inbox operations"
guard_expect_in_file "$TAG" 'HakoAllocRemoteFreePolicy.pushRetry' "$INBOX" "M170 must publish through bounded remote-free retry policy"
guard_expect_in_file "$TAG" 'me\.page\.releaseLocal\(block_id\)' "$INBOX" "M170 must collect into page-owned local-free state"
guard_expect_in_file "$TAG" 'collectPending\(\)' "$INBOX" "M170 integration owner must expose pending drain"
guard_expect_in_file "$TAG" 'report\.captureFacadeExercise\(alloc, head_cell, block_a, block_b\)' "$APP" "proof must orchestrate the facade exercise through the report capsule"
guard_expect_in_file "$TAG" 'print\(report\.blocksLine\(\)\)' "$APP" "proof must render blocks output through the report capsule"
guard_expect_in_file "$TAG" 'print\(report\.summaryLine\(\)\)' "$APP" "proof must render summary output through the report capsule"
guard_expect_in_file "$TAG" 'report\.ok\(\)' "$APP" "proof must use the exercise report contract"
guard_expect_in_file "$TAG" 'caller-provided `block_id`' "$APP_README" "M170 app README must document caller-provided block identity"
guard_expect_in_file "$TAG" '293x-178 M170 Mimalloc Remote-Free Integration' "$CARD" "missing M170 card"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M170 guard"
guard_expect_in_file "$TAG" 'M170 remote-free integration' "$PLAN" "plan must retain M170 row"

if rg -n 'init[[:space:]]*\{' "$INBOX" >/tmp/"$TAG".legacy_init 2>&1; then
  echo "[$TAG] ERROR: M170 inbox must use Unified Members stored fields, not legacy init slots" >&2
  cat /tmp/"$TAG".legacy_init >&2
  rm -f /tmp/"$TAG".legacy_init
  exit 1
fi
rm -f /tmp/"$TAG".legacy_init

if rg -n 'fetch_add|ptr_fetch_add|page_map|PageMap|arbitrary pointer free|replacement|hook|provider|global_allocator|GlobalAlloc|unreserve|release_bytes|hako_osvm_(unreserve|release)' \
  "$INBOX" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: M170 leaked out of bounded remote-free page integration scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".inactive_pointer_rows 2>&1; then
  echo "[$TAG] ERROR: pointer atomic fetch_add rows must stay inactive in M170" >&2
  cat /tmp/"$TAG".inactive_pointer_rows >&2
  rm -f /tmp/"$TAG".inactive_pointer_rows
  exit 1
fi
rm -f /tmp/"$TAG".inactive_pointer_rows

if rg -n 'mimalloc-remote-free-page-integration|HakoAllocRemoteFreePageInbox|remote_free_page_integration' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M170 app/box matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m170_remote_free_page.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m170.mir.json"
exe_out="$tmp_dir/m170.exe"
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
required = {
    "main",
    "HakoAllocProductionFacade.remotePagePort/5",
    "HakoAllocProductionFacade.remotePageAcquire/2",
    "HakoAllocProductionFacade.remotePagePublishPort/4",
    "HakoAllocProductionFacade.remotePageCollectOnePort/1",
    "HakoAllocProductionFacade.remotePageCollectPendingPort/1",
    "HakoAllocProductionFacade.remotePagePeekPortHead/1",
    "HakoAllocProductionFacade.remotePagePeekPortNext/2",
    "HakoAllocProductionFacade.remotePagePeekPortHead/1",
    "HakoAllocProductionFacade.remotePagePeekPortNext/2",
    "HakoAllocProductionFacade.remotePagePortSnapshot/1",
    "HakoAllocProductionFacade.remotePageInbox/2",
    "HakoAllocProductionFacade.remotePagePublish/4",
    "HakoAllocProductionFacade.remotePageCollectOne/1",
    "HakoAllocProductionFacade.remotePageCollectPending/1",
    "HakoAllocProductionFacade.remotePagePeekInboxHead/1",
    "HakoAllocProductionFacade.remotePagePeekInboxNext/2",
    "HakoAllocRemoteFreePagePort.birth/5",
    "HakoAllocRemoteFreePagePort.acquire/1",
    "HakoAllocRemoteFreePagePort.publish/3",
    "HakoAllocRemoteFreePagePort.collectOne/0",
    "HakoAllocRemoteFreePagePort.collectPending/0",
    "HakoAllocRemoteFreePagePort.peekHead/0",
    "HakoAllocRemoteFreePagePort.peekNext/1",
    "HakoAllocRemoteFreePagePort.snapshot/0",
    "HakoAllocRemoteFreePageExerciseReport.captureBlocks/3",
    "HakoAllocRemoteFreePageExerciseReport.captureFacadeBlocks/2",
    "HakoAllocRemoteFreePageExerciseReport.capturePublish/2",
    "HakoAllocRemoteFreePageExerciseReport.captureFacadePublish/4",
    "HakoAllocRemoteFreePageExerciseReport.captureObservedPointers/5",
    "HakoAllocRemoteFreePageExerciseReport.captureFacadeObservedPointers/4",
    "HakoAllocRemoteFreePageExerciseReport.captureCollect/2",
    "HakoAllocRemoteFreePageExerciseReport.captureFacadeCollect/2",
    "HakoAllocRemoteFreePageExerciseReport.captureSnapshot/1",
    "HakoAllocRemoteFreePageExerciseReport.captureFacadeSnapshot/2",
    "HakoAllocRemoteFreePageExerciseReport.captureFacadeExercise/4",
    "HakoAllocRemoteFreePageExerciseReport.blocksLine/0",
    "HakoAllocRemoteFreePageExerciseReport.publishLine/0",
    "HakoAllocRemoteFreePageExerciseReport.listLine/0",
    "HakoAllocRemoteFreePageExerciseReport.collectLine/0",
    "HakoAllocRemoteFreePageExerciseReport.pageLine/0",
    "HakoAllocRemoteFreePageExerciseReport.inboxLine/0",
    "HakoAllocRemoteFreePageExerciseReport.summaryLine/0",
    "HakoAllocRemoteFreePageExerciseReport.ok/0",
    "HakoAllocPageModel.acquire/1",
    "HakoAllocPageModel.releaseLocal/1",
    "HakoAllocRemoteFreePageInbox.birth/2",
    "HakoAllocRemoteFreePageInbox.publish/3",
    "HakoAllocRemoteFreePageInbox.collectOne/0",
    "HakoAllocRemoteFreePageInbox.collectPending/0",
    "HakoAllocRemoteFreePageInbox.peekHead/0",
    "HakoAllocRemoteFreePageInbox.peekNext/1",
    "HakoAllocRemoteFreePolicy.initHead/1",
    "HakoAllocRemoteFreePolicy.pushRetry/3",
    "HakoAllocRemoteFreePolicy.peekHead/1",
    "HakoAllocRemoteFreePolicy.peekNext/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for box_name in ("HakoAllocPageModel", "HakoAllocRemoteFreePageInbox", "HakoAllocRemoteFreePagePort", "HakoAllocRemoteFreePagePortSnapshot", "HakoAllocRemoteFreePageExerciseReport"):
    if plans.get(box_name) is None:
        raise SystemExit(f"missing typed object plan: {box_name}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def require_method(owner_name, box_name, name):
    for callee in iter_calls(functions[owner_name]):
        if (
            callee.get("type") == "Method"
            and callee.get("box_name") == box_name
            and callee.get("name") == name
        ):
            return
    raise SystemExit(f"missing method call in {owner_name}: {box_name}.{name}")

for owner_name, method in (
    ("HakoAllocProductionFacade.remotePagePort/5", "birth"),
    ("HakoAllocProductionFacade.remotePageAcquire/2", "acquire"),
    ("HakoAllocProductionFacade.remotePagePublishPort/4", "publish"),
    ("HakoAllocProductionFacade.remotePageCollectPendingPort/1", "collectPending"),
    ("HakoAllocProductionFacade.remotePageCollectOnePort/1", "collectOne"),
    ("HakoAllocProductionFacade.remotePagePeekPortHead/1", "peekHead"),
    ("HakoAllocProductionFacade.remotePagePeekPortNext/2", "peekNext"),
    ("HakoAllocProductionFacade.remotePagePortSnapshot/1", "snapshot"),
):
    box_name = "HakoAllocRemoteFreePagePort"
    require_method(owner_name, box_name, method)

for method in (
    "captureFacadeExercise",
    "blocksLine",
    "publishLine",
    "listLine",
    "collectLine",
    "pageLine",
    "inboxLine",
    "summaryLine",
    "ok",
):
    require_method("main", "HakoAllocRemoteFreePageExerciseReport", method)

for owner_name, method in (
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadeExercise/4", "remotePagePort"),
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadeExercise/4", "captureFacadeBlocks"),
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadeExercise/4", "captureFacadePublish"),
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadeExercise/4", "captureFacadeObservedPointers"),
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadeExercise/4", "captureFacadeCollect"),
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadeExercise/4", "captureFacadeSnapshot"),
):
    box_name = "HakoAllocRemoteFreePageExerciseReport"
    if method == "remotePagePort":
        box_name = "HakoAllocProductionFacade"
    require_method(owner_name, box_name, method)

for owner_name, method in (
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadeBlocks/2", "remotePageAcquire"),
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadePublish/4", "remotePagePublishPort"),
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadePublish/4", "remotePagePublishPort"),
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadeCollect/2", "remotePageCollectPendingPort"),
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadeCollect/2", "remotePageCollectOnePort"),
):
    require_method(owner_name, "HakoAllocProductionFacade", method)

require_method(
    "HakoAllocRemoteFreePageExerciseReport.captureFacadeBlocks/2",
    "HakoAllocRemoteFreePageExerciseReport",
    "captureBlocks",
)

require_method(
    "HakoAllocRemoteFreePageExerciseReport.captureFacadePublish/4",
    "HakoAllocRemoteFreePageExerciseReport",
    "capturePublish",
)

for owner_name, method in (
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadeObservedPointers/4", "remotePagePeekPortHead"),
    ("HakoAllocRemoteFreePageExerciseReport.captureFacadeObservedPointers/4", "remotePagePeekPortNext"),
):
    require_method(owner_name, "HakoAllocProductionFacade", method)

require_method(
    "HakoAllocRemoteFreePageExerciseReport.captureFacadeObservedPointers/4",
    "HakoAllocRemoteFreePageExerciseReport",
    "captureObservedPointers",
)
require_method(
    "HakoAllocRemoteFreePageExerciseReport.captureFacadeCollect/2",
    "HakoAllocRemoteFreePageExerciseReport",
    "captureCollect",
)
require_method(
    "HakoAllocRemoteFreePageExerciseReport.captureFacadeSnapshot/2",
    "HakoAllocProductionFacade",
    "remotePagePortSnapshot",
)
require_method(
    "HakoAllocRemoteFreePageExerciseReport.captureFacadeSnapshot/2",
    "HakoAllocRemoteFreePageExerciseReport",
    "captureSnapshot",
)
require_method(
    "HakoAllocRemoteFreePageExerciseReport.summaryLine/0",
    "HakoAllocRemoteFreePageExerciseReport",
    "ok",
)

def require_global(owner_name, symbol):
    routes = functions[owner_name].get("metadata", {}).get("global_call_routes", [])
    for route in routes:
        if (
            route.get("symbol") == symbol
            and route.get("target_shape") == "generic_i64_body"
            and route.get("proof") == "typed_global_call_generic_i64"
            and route.get("return_shape") == "ScalarI64"
        ):
            return
    raise SystemExit(f"missing generic-i64 route in {owner_name} -> {symbol}: {routes}")

for owner_name, symbol in (
    ("HakoAllocRemoteFreePageInbox.birth/2", "HakoAllocRemoteFreePolicy.initHead/1"),
    ("HakoAllocRemoteFreePageInbox.publish/3", "HakoAllocRemoteFreePolicy.pushRetry/3"),
    ("HakoAllocRemoteFreePageInbox.peekHead/0", "HakoAllocRemoteFreePolicy.peekHead/1"),
    ("HakoAllocRemoteFreePageInbox.peekNext/1", "HakoAllocRemoteFreePolicy.peekNext/1"),
):
    require_global(owner_name, symbol)

def require_extern(owner_name, symbol, route_id, core_op, arity, ret, demand, effects):
    owner = functions[owner_name]
    routes = owner.get("metadata", {}).get("extern_call_routes", [])
    for route in routes:
        if (
            route.get("route_id") == route_id
            and route.get("core_op") == core_op
            and route.get("symbol") == symbol
            and route.get("return_shape") == ret
            and route.get("value_demand") == demand
            and route.get("effects") == effects
        ):
            break
    else:
        raise SystemExit(f"missing extern route in {owner_name} for {symbol}: {routes}")

    plans = owner.get("metadata", {}).get("lowering_plan", [])
    for plan in plans:
        if (
            plan.get("source") == "extern_call_routes"
            and plan.get("source_route_id") == route_id
            and plan.get("arity") == arity
            and plan.get("symbol") == symbol
        ):
            return
    raise SystemExit(f"missing lowering plan in {owner_name} for {symbol}: {plans}")

for symbol, route_id, core_op, arity, ret, demand, effects in (
    (
        "hako_mem_alloc",
        "extern.hako_mem.alloc",
        "HakoMemAlloc",
        1,
        "native_ptr_nullable",
        "native_ptr_nullable",
        ["hako.mem.alloc"],
    ),
    (
        "hako_mem_free",
        "extern.hako_mem.free",
        "HakoMemFree",
        1,
        "void_sentinel_i64_zero",
        "scalar_i64",
        ["hako.mem.free"],
    ),
):
    require_extern("main", symbol, route_id, core_op, arity, ret, demand, effects)

require_extern(
    "HakoAllocRemoteFreePolicy.initHead/1",
    "hako_atomic_ptr_store_ordered",
    "extern.hako_atomic.ptr_store_ordered",
    "HakoAtomicPtrStoreOrdered",
    3,
    "scalar_i64",
    "native_ptr_nullable",
    ["hako.atomic.ptr_store"],
)

for owner_name in (
    "HakoAllocRemoteFreePolicy.pushRetry/3",
    "HakoAllocRemoteFreePolicy.peekHead/1",
    "HakoAllocRemoteFreePolicy.peekNext/1",
):
    require_extern(
        owner_name,
        "hako_atomic_ptr_load_ordered",
        "extern.hako_atomic.ptr_load_ordered",
        "HakoAtomicPtrLoadOrdered",
        2,
        "native_ptr_nullable",
        "native_ptr_nullable",
        ["hako.atomic.ptr_load"],
    )

require_extern(
    "HakoAllocRemoteFreePolicy.pushRetry/3",
    "hako_atomic_ptr_store_ordered",
    "extern.hako_atomic.ptr_store_ordered",
    "HakoAtomicPtrStoreOrdered",
    3,
    "scalar_i64",
    "native_ptr_nullable",
    ["hako.atomic.ptr_store"],
)
require_extern(
    "HakoAllocRemoteFreePolicy.pushRetry/3",
    "hako_atomic_ptr_cas_ordered",
    "extern.hako_atomic.ptr_cas_ordered",
    "HakoAtomicPtrCasOrdered",
    5,
    "native_ptr_nullable",
    "native_ptr_nullable",
    ["hako.atomic.ptr_cas"],
)

print("[m170-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_store_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_load_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_cas_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-remote-free-page-integration-proof' "$run_log"
rg -F -q 'blocks=1,0,-1' "$run_log"
rg -F -q 'publish=1,1' "$run_log"
rg -F -q 'list=1,1,1' "$run_log"
rg -F -q 'collect=2,0' "$run_log"
rg -F -q 'page=0,2,1,2' "$run_log"
rg -F -q 'inbox=0,2,2,0,1,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"

echo "[$TAG] ok"

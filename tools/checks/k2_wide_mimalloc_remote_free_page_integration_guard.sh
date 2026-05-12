#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-remote-free-page-integration"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

INBOX="lang/src/hako_alloc/memory/remote_free_page_integration_box.hako"
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
guard_expect_in_file "$TAG" 'HakoAllocRemoteFreePolicy.pushRetry' "$INBOX" "M170 must publish through bounded remote-free retry policy"
guard_expect_in_file "$TAG" 'me\.page\.releaseLocal\(block_id\)' "$INBOX" "M170 must collect into page-owned local-free state"
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
    "HakoAllocPageModel.acquire/1",
    "HakoAllocPageModel.releaseLocal/1",
    "HakoAllocRemoteFreePageInbox.birth/2",
    "HakoAllocRemoteFreePageInbox.publish/3",
    "HakoAllocRemoteFreePageInbox.collectOne/0",
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
for box_name in ("HakoAllocPageModel", "HakoAllocRemoteFreePageInbox"):
    if plans.get(box_name) is None:
        raise SystemExit(f"missing typed object plan: {box_name}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def require_main_method(box_name, name):
    for callee in iter_calls(functions["main"]):
        if (
            callee.get("type") == "Method"
            and callee.get("box_name") == box_name
            and callee.get("name") == name
        ):
            return
    raise SystemExit(f"missing main method call: {box_name}.{name}")

for method in ("publish", "collectOne", "peekHead", "peekNext"):
    require_main_method("HakoAllocRemoteFreePageInbox", method)

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
rg -F -q 'collect=1,1,0' "$run_log"
rg -F -q 'page=0,2,1,2' "$run_log"
rg -F -q 'inbox=0,2,2,0,1,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"

echo "[$TAG] ok"

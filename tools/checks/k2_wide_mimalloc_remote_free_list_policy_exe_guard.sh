#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-remote-free-list-policy-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-remote-free-list-policy-proof/main.hako"
APP_README="apps/mimalloc-remote-free-list-policy-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-094-M42-ALLOCATOR-REMOTE-FREE-LIST-POLICY-INTEGRATION-PROOF.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M42 allocator remote-free list policy EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$CARD" "$TASKBOARD"

if rg -n 'mimalloc-remote-free-list-policy-proof' lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: app-specific remote-free list policy matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".inactive_pointer_rows 2>&1; then
  echo "[$TAG] ERROR: pointer atomic fetch_add rows must stay inactive in M42" >&2
  cat /tmp/"$TAG".inactive_pointer_rows >&2
  rm -f /tmp/"$TAG".inactive_pointer_rows
  exit 1
fi
rm -f /tmp/"$TAG".inactive_pointer_rows

cargo test -q refresh_function_extern_call_routes_records_hako_atomic_ptr_store_ordered_route -- --nocapture
cargo test -q refresh_function_extern_call_routes_records_hako_atomic_ptr_load_ordered_route -- --nocapture
cargo test -q refresh_function_extern_call_routes_records_hako_atomic_ptr_cas_ordered_route -- --nocapture
cargo test -q -p nyash_kernel atomic -- --nocapture
pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m42_remote_free_list_policy.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m42.mir.json"
exe_out="$tmp_dir/m42.exe"
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
required_functions = {
    "main",
    "AllocatorRemoteFreeListPolicy.init_head/1",
    "AllocatorRemoteFreeListPolicy.push/2",
    "AllocatorRemoteFreeListPolicy.peek_head/1",
    "AllocatorRemoteFreeListPolicy.peek_next/1",
}
missing = sorted(name for name in required_functions if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

main = functions["main"]
init_head = functions["AllocatorRemoteFreeListPolicy.init_head/1"]
push = functions["AllocatorRemoteFreeListPolicy.push/2"]
peek_head = functions["AllocatorRemoteFreeListPolicy.peek_head/1"]
peek_next = functions["AllocatorRemoteFreeListPolicy.peek_next/1"]

def require_global(owner, owner_name, symbol):
    routes = owner.get("metadata", {}).get("global_call_routes", [])
    for route in routes:
        if (
            route.get("symbol") == symbol
            and route.get("target_shape") == "generic_i64_body"
            and route.get("proof") == "typed_global_call_generic_i64"
            and route.get("return_shape") == "ScalarI64"
        ):
            return
    raise SystemExit(f"missing generic-i64 route in {owner_name} -> {symbol}: {routes}")

for symbol in (
    "AllocatorRemoteFreeListPolicy.init_head/1",
    "AllocatorRemoteFreeListPolicy.push/2",
    "AllocatorRemoteFreeListPolicy.peek_head/1",
    "AllocatorRemoteFreeListPolicy.peek_next/1",
):
    require_global(main, "main", symbol)

def require_extern(owner, owner_name, symbol, route_id, core_op, arity, ret, demand, effects):
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

require_extern(
    init_head,
    "AllocatorRemoteFreeListPolicy.init_head/1",
    "hako_atomic_ptr_store_ordered",
    "extern.hako_atomic.ptr_store_ordered",
    "HakoAtomicPtrStoreOrdered",
    3,
    "scalar_i64",
    "native_ptr_nullable",
    ["hako.atomic.ptr_store"],
)

for owner, owner_name in (
    (push, "AllocatorRemoteFreeListPolicy.push/2"),
    (peek_head, "AllocatorRemoteFreeListPolicy.peek_head/1"),
    (peek_next, "AllocatorRemoteFreeListPolicy.peek_next/1"),
):
    require_extern(
        owner,
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
    push,
    "AllocatorRemoteFreeListPolicy.push/2",
    "hako_atomic_ptr_store_ordered",
    "extern.hako_atomic.ptr_store_ordered",
    "HakoAtomicPtrStoreOrdered",
    3,
    "scalar_i64",
    "native_ptr_nullable",
    ["hako.atomic.ptr_store"],
)
require_extern(
    push,
    "AllocatorRemoteFreeListPolicy.push/2",
    "hako_atomic_ptr_cas_ordered",
    "extern.hako_atomic.ptr_cas_ordered",
    "HakoAtomicPtrCasOrdered",
    5,
    "native_ptr_nullable",
    "native_ptr_nullable",
    ["hako.atomic.ptr_cas"],
)

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
    require_extern(main, "main", symbol, route_id, core_op, arity, ret, demand, effects)

print("[m42-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_store_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_load_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_cas_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-remote-free-list-policy-proof' "$run_log"
rg -F -q 'init=0' "$run_log"
rg -F -q 'first=1,1,1' "$run_log"
rg -F -q 'second=1,1,1,1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '| `M42 allocator remote-free list policy integration proof` | `live-narrow` |' "$TASKBOARD"
rg -F -q 'M42 Allocator Remote-Free List Policy Integration Proof' "$CARD"
rg -F -q 'AllocatorRemoteFreeListPolicy' "$APP_README"
rg -F -q 'k2_wide_mimalloc_remote_free_list_policy_exe_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-remote-free-policy-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-remote-free-policy-proof/main.hako"
APP_README="apps/mimalloc-remote-free-policy-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-089-M37-ALLOCATOR-REMOTE-FREE-POLICY-INTEGRATION-PROOF.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M37 allocator remote-free policy EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$CARD" "$TASKBOARD"

if rg -n 'mimalloc-remote-free-policy-proof' lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: app-specific remote-free policy matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

if rg -n 'hako_atomic_ptr_cas_ordered|HakoAtomicPtrCasOrdered|extern\\.hako_atomic\\.ptr_cas_ordered|ptr_fetch_add' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".inactive_pointer_rows 2>&1; then
  echo "[$TAG] ERROR: pointer atomic CAS/fetch_add rows must stay inactive after M39" >&2
  cat /tmp/"$TAG".inactive_pointer_rows >&2
  rm -f /tmp/"$TAG".inactive_pointer_rows
  exit 1
fi
rm -f /tmp/"$TAG".inactive_pointer_rows

cargo test -q refresh_function_extern_call_routes_records_hako_tls_cache_slot_routes -- --nocapture
cargo test -q refresh_function_extern_call_routes_records_hako_atomic_ptr_store_ordered_route -- --nocapture
cargo test -q -p nyash_kernel tls -- --nocapture
cargo test -q -p nyash_kernel atomic -- --nocapture
pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m37_remote_free_policy.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m37.mir.json"
exe_out="$tmp_dir/m37.exe"
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
install = functions.get("AllocatorRemoteFreePolicy.install_mailbox/2")
publish = functions.get("AllocatorRemoteFreePolicy.publish_remote_free/2")
release = functions.get("AllocatorRemoteFreePolicy.release_mailbox/1")
for name, fn in (
    ("main", main),
    ("AllocatorRemoteFreePolicy.install_mailbox/2", install),
    ("AllocatorRemoteFreePolicy.publish_remote_free/2", publish),
    ("AllocatorRemoteFreePolicy.release_mailbox/1", release),
):
    if fn is None:
        raise SystemExit(f"missing function: {name}")

expected_tls_helpers = {
    "TlsCoreBox.cache_slot_get_i64/1": (
        "extern.hako_tls.cache_slot_get_i64",
        "HakoTlsCacheSlotGetI64",
        "hako_tls_cache_slot_get_i64",
        1,
        "scalar_i64",
        "runtime_i64",
        ["hako.tls.cache_slot_get"],
    ),
    "TlsCoreBox.cache_slot_set_i64/2": (
        "extern.hako_tls.cache_slot_set_i64",
        "HakoTlsCacheSlotSetI64",
        "hako_tls_cache_slot_set_i64",
        2,
        "scalar_i64",
        "runtime_i64",
        ["hako.tls.cache_slot_set"],
    ),
}

for fn_name, (route_id, core_op, symbol, arity, ret, demand, effects) in expected_tls_helpers.items():
    fn = functions.get(fn_name)
    if fn is None:
        raise SystemExit(f"missing helper function: {fn_name}")
    routes = fn.get("metadata", {}).get("extern_call_routes", [])
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
        raise SystemExit(f"missing extern route for {fn_name}: {routes}")
    plans = fn.get("metadata", {}).get("lowering_plan", [])
    for plan in plans:
        if (
            plan.get("source") == "extern_call_routes"
            and plan.get("source_route_id") == route_id
            and plan.get("arity") == arity
            and plan.get("symbol") == symbol
        ):
            break
    else:
        raise SystemExit(f"missing lowering plan for {fn_name}: {plans}")

def require_generic(owner, owner_name, symbol):
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
    "AllocatorRemoteFreePolicy.install_mailbox/2",
    "AllocatorRemoteFreePolicy.publish_remote_free/2",
    "AllocatorRemoteFreePolicy.release_mailbox/1",
    "TlsCoreBox.cache_slot_get_i64/1",
):
    require_generic(main, "main", symbol)

require_generic(install, "AllocatorRemoteFreePolicy.install_mailbox/2", "TlsCoreBox.cache_slot_set_i64/2")
require_generic(publish, "AllocatorRemoteFreePolicy.publish_remote_free/2", "TlsCoreBox.cache_slot_get_i64/1")
require_generic(release, "AllocatorRemoteFreePolicy.release_mailbox/1", "TlsCoreBox.cache_slot_set_i64/2")

publish_routes = publish.get("metadata", {}).get("extern_call_routes", [])
publish_plans = publish.get("metadata", {}).get("lowering_plan", [])
for route in publish_routes:
    if (
        route.get("route_id") == "extern.hako_atomic.ptr_store_ordered"
        and route.get("core_op") == "HakoAtomicPtrStoreOrdered"
        and route.get("symbol") == "hako_atomic_ptr_store_ordered"
        and route.get("return_shape") == "scalar_i64"
        and route.get("value_demand") == "native_ptr_nullable"
        and route.get("effects") == ["hako.atomic.ptr_store"]
    ):
        break
else:
    raise SystemExit(f"missing pointer-store extern route in policy publish: {publish_routes}")

for plan in publish_plans:
    if (
        plan.get("source") == "extern_call_routes"
        and plan.get("source_route_id") == "extern.hako_atomic.ptr_store_ordered"
        and plan.get("arity") == 3
        and plan.get("symbol") == "hako_atomic_ptr_store_ordered"
    ):
        break
else:
    raise SystemExit(f"missing pointer-store lowering plan in policy publish: {publish_plans}")

main_routes = main.get("metadata", {}).get("extern_call_routes", [])
main_plans = main.get("metadata", {}).get("lowering_plan", [])
expected_main_extern = {
    "hako_mem_alloc": (
        "extern.hako_mem.alloc",
        "HakoMemAlloc",
        1,
        "native_ptr_nullable",
        "native_ptr_nullable",
        ["hako.mem.alloc"],
    ),
    "hako_mem_free": (
        "extern.hako_mem.free",
        "HakoMemFree",
        1,
        "void_sentinel_i64_zero",
        "scalar_i64",
        ["hako.mem.free"],
    ),
}

for symbol, (route_id, core_op, arity, ret, demand, effects) in expected_main_extern.items():
    for route in main_routes:
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
        raise SystemExit(f"missing main extern route for {symbol}: {main_routes}")
    for plan in main_plans:
        if (
            plan.get("source") == "extern_call_routes"
            and plan.get("source_route_id") == route_id
            and plan.get("arity") == arity
            and plan.get("symbol") == symbol
        ):
            break
    else:
        raise SystemExit(f"missing main lowering plan for {symbol}: {main_plans}")

print("[m37-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_tls_cache_slot_get_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_tls_cache_slot_set_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_store_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-remote-free-policy-proof' "$run_log"
rg -F -q 'policy=0,0,0,0' "$run_log"
rg -F -q 'final_mailbox=0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '| `M37 allocator remote-free policy integration proof` | `live-narrow` |' "$TASKBOARD"
rg -F -q 'M37 Allocator Remote-Free Policy Integration Proof' "$CARD"
rg -F -q 'AllocatorRemoteFreePolicy' "$APP_README"
rg -F -q 'k2_wide_mimalloc_remote_free_policy_exe_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"

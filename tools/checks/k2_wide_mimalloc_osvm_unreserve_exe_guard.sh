#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-osvm-unreserve-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-osvm-unreserve-proof/main.hako"
APP_README="apps/mimalloc-osvm-unreserve-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-457-MIMAP-032A-OSVM-UNRESERVE-SUBSTRATE-ROUTE.md"
PAGE_SOURCE_ADAPTER_CARD="docs/development/current/main/phases/phase-293x/293x-459-MIMAP-033A-PAGE-SOURCE-UNRESERVE-ADAPTER.md"
OSVM_CORE="lang/src/runtime/substrate/osvm/osvm_core_box.hako"
PAGE_SOURCE="lang/src/hako_alloc/memory/page_source_policy_box.hako"

echo "[$TAG] running MIMAP-032A OSVM unreserve EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$CARD" "$PAGE_SOURCE_ADAPTER_CARD" "$OSVM_CORE" "$PAGE_SOURCE"

if rg -n 'mimalloc-osvm-unreserve-proof' lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: app-specific OSVM unreserve matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

if rg -q 'Status: landed' "$PAGE_SOURCE_ADAPTER_CARD"; then
  echo "[$TAG] MIMAP-033A has landed; skipping pre-page-source owner scan"
else
  if rg -n 'unreservePage|releasePage' "$PAGE_SOURCE" >/tmp/"$TAG".allocator_owner 2>&1; then
    echo "[$TAG] ERROR: MIMAP-032A must not open allocator page-source unreserve/release owners" >&2
    cat /tmp/"$TAG".allocator_owner >&2
    rm -f /tmp/"$TAG".allocator_owner
    exit 1
  fi
fi
rm -f /tmp/"$TAG".allocator_owner

guard_expect_in_file "$TAG" 'hako_osvm_unreserve_bytes_i64' "$OSVM_CORE" "OsVmCoreBox must own the substrate unreserve extern"
guard_expect_in_file "$TAG" 'hako_osvm_unreserve_bytes_i64' lang/c-abi/include/hako_hostbridge.h "HostBridge header must declare unreserve"
guard_expect_in_file "$TAG" 'hako_osvm_unreserve_bytes_i64' lang/c-abi/shims/hako_kernel.c "C kernel shim must implement unreserve"
guard_expect_in_file "$TAG" 'HakoOsvmUnreserveBytesI64' src/mir/extern_call_route_plan.rs "MIR extern route plan must classify unreserve"
guard_expect_in_file "$TAG" 'hako_osvm_unreserve_bytes_i64' lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc "ny-llvmc route shell must emit unreserve"
guard_expect_in_file "$TAG" 'hako_osvm_unreserve_bytes_i64' lang/src/vm/boxes/mir_vm_s0_call_exec.hako "VM S0 externcall must accept unreserve"

cargo test -q refresh_function_extern_call_routes_records_hako_osvm_routes -- --nocapture
cargo test -q -p nyash_kernel osvm -- --nocapture
pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap032a_osvm_unreserve.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap032a.mir.json"
exe_out="$tmp_dir/mimap032a.exe"
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

expected_helpers = {
    "OsVmCoreBox.reserve_bytes_i64/1": (
        "extern.hako_osvm.reserve_bytes_i64",
        "HakoOsvmReserveBytesI64",
        "hako_osvm_reserve_bytes_i64",
        1,
        "native_ptr_nullable",
        "native_ptr_nullable",
        ["hako.osvm.reserve"],
    ),
    "OsVmCoreBox.commit_bytes_i64/2": (
        "extern.hako_osvm.commit_bytes_i64",
        "HakoOsvmCommitBytesI64",
        "hako_osvm_commit_bytes_i64",
        2,
        "scalar_i64",
        "runtime_i64",
        ["hako.osvm.commit"],
    ),
    "OsVmCoreBox.decommit_bytes_i64/2": (
        "extern.hako_osvm.decommit_bytes_i64",
        "HakoOsvmDecommitBytesI64",
        "hako_osvm_decommit_bytes_i64",
        2,
        "scalar_i64",
        "runtime_i64",
        ["hako.osvm.decommit"],
    ),
    "OsVmCoreBox.unreserve_bytes_i64/2": (
        "extern.hako_osvm.unreserve_bytes_i64",
        "HakoOsvmUnreserveBytesI64",
        "hako_osvm_unreserve_bytes_i64",
        2,
        "scalar_i64",
        "runtime_i64",
        ["hako.osvm.unreserve"],
    ),
}

for fn_name, (route_id, core_op, symbol, arity, ret, demand, effects) in expected_helpers.items():
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

routes = main.get("metadata", {}).get("global_call_routes", [])
for symbol in expected_helpers:
    for route in routes:
        if (
            route.get("symbol") == symbol
            and route.get("target_shape") == "generic_i64_body"
            and route.get("proof") == "typed_global_call_generic_i64"
            and route.get("return_shape") == "ScalarI64"
        ):
            break
    else:
        raise SystemExit(f"missing generic-i64 route main -> {symbol}: {routes}")

print("[mimap032a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_unreserve_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-osvm-unreserve-proof' "$run_log"
rg -F -q 'page=4096 reserved=1' "$run_log"
rg -F -q 'commit=0 decommit=0 unreserve=0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q 'MIMAP-032A OSVM Unreserve Substrate Route' "$CARD"
rg -F -q 'unreserve substrate seam' "$APP_README"
rg -F -q 'k2_wide_mimalloc_osvm_unreserve_exe_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-tls-cache-slot-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-tls-cache-slot-proof/main.hako"
APP_README="apps/mimalloc-tls-cache-slot-proof/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-078-M26-MIMALLOC-TLS-CACHE-SLOT-EXE-PROOF.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"

echo "[$TAG] running M26 mimalloc TLS cache-slot EXE guard"

guard_require_files "$TAG" "$APP" "$APP_README" "$CARD" "$TASKBOARD"

if rg -n 'mimalloc-tls-cache-slot-proof' lang/c-abi/shims >/tmp/"$TAG".app_specific.inc 2>&1; then
  echo "[$TAG] ERROR: app-specific TLS cache-slot matcher leaked into .inc" >&2
  cat /tmp/"$TAG".app_specific.inc >&2
  rm -f /tmp/"$TAG".app_specific.inc
  exit 1
fi
rm -f /tmp/"$TAG".app_specific.inc

cargo test -q refresh_function_extern_call_routes_records_hako_tls_cache_slot_routes -- --nocapture
cargo test -q -p nyash_kernel tls -- --nocapture
pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m26_tls_cache_slot.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m26.mir.json"
exe_out="$tmp_dir/m26.exe"
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

print("[m26-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_global_generic_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_tls_cache_slot_get_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_tls_cache_slot_set_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-tls-cache-slot-proof' "$run_log"
rg -F -q 'before=0 after=4096 final=0' "$run_log"
rg -F -q 'set=0 clear=0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

rg -F -q '`M26 mimalloc TLS cache-slot EXE proof`' "$TASKBOARD"
rg -F -q 'M26 Mimalloc TLS Cache-Slot EXE Proof' "$CARD"
rg -F -q 'cache-slot get/set' "$APP_README"
rg -F -q 'k2_wide_mimalloc_tls_cache_slot_exe_guard.sh' docs/tools/check-scripts-index.md

echo "[$TAG] ok"

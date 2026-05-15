#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-worker-identity-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-worker-identity-proof/main.hako"
APP_TEST="apps/mimalloc-worker-identity-proof/test.sh"
APP_README="apps/mimalloc-worker-identity-proof/README.md"
WORKER_CORE="lang/src/runtime/substrate/worker/worker_core_box.hako"
WORKER_README="lang/src/runtime/substrate/worker/README.md"
WORKER_EXPORT="crates/nyash_kernel/src/exports/worker.rs"
WORKER_EXPORT_MOD="crates/nyash_kernel/src/exports/mod.rs"
IDENTITY="lang/src/hako_alloc/memory/worker_identity_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
SUBSTRATE_README="lang/src/runtime/substrate/README.md"
ROUTE_PLAN="src/mir/extern_call_route_plan.rs"
GENERIC_I64="src/mir/global_call_route_plan/generic_i64_body.rs"
EXTERN_CALLS="src/mir/builder/calls/extern_calls.rs"
METADATA_SSOT="docs/reference/mir/metadata-facts-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-393-MIMAP-WORKER-001-INTERNAL-WORKER-IDENTITY-SUBSTRATE.md"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] running MIMAP-WORKER-001 worker identity guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$WORKER_CORE" \
  "$WORKER_README" \
  "$WORKER_EXPORT" \
  "$WORKER_EXPORT_MOD" \
  "$IDENTITY" \
  "$MODULE" \
  "$MEMORY_README" \
  "$SUBSTRATE_README" \
  "$ROUTE_PLAN" \
  "$GENERIC_I64" \
  "$EXTERN_CALLS" \
  "$METADATA_SSOT" \
  "$CARD" \
  "$INDEX"

guard_expect_in_file "$TAG" 'externcall "hako_worker_current_id_i64"\(0\)' "$WORKER_CORE" "WorkerCoreBox must call the worker-id substrate route with reserved lane 0"
guard_expect_in_file "$TAG" 'hako_worker_current_id_i64' "$WORKER_EXPORT" "nyash kernel must export worker current id"
guard_expect_in_file "$TAG" 'pub\(crate\) mod worker;' "$WORKER_EXPORT_MOD" "nyash kernel export module must include worker"
guard_expect_in_file "$TAG" 'box HakoAllocWorkerIdentity' "$IDENTITY" "allocator-facing worker identity owner missing"
guard_expect_in_file "$TAG" 'WorkerCoreBox.current_id_i64\(\)' "$IDENTITY" "allocator owner must call WorkerCoreBox"
guard_expect_in_file "$TAG" 'memory.worker_identity_box = "memory/worker_identity_box.hako"' "$MODULE" "hako_alloc module must export worker identity"
guard_expect_in_file "$TAG" 'HakoWorkerCurrentIdI64' "$ROUTE_PLAN" "MIR route kind missing"
guard_expect_in_file "$TAG" 'extern.hako_worker.current_id_i64' "$ROUTE_PLAN" "MIR route id missing"
guard_expect_in_file "$TAG" 'HakoWorkerCurrentIdI64' "$GENERIC_I64" "generic i64 route propagation missing"
guard_expect_in_file "$TAG" 'hako_worker_current_id_i64' "$EXTERN_CALLS" "explicit extern return type missing"
guard_expect_in_file "$TAG" 'hako_worker_current_id_i64' "$METADATA_SSOT" "metadata SSOT must list worker route"
guard_expect_in_file "$TAG" 'MIMAP-WORKER-001' "$CARD" "MIMAP-WORKER-001 card missing"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list worker identity guard"

if rg -n 'worker_local|Channel|task_scope|nowait|await|TlsCoreBox|hako_tls|Atomic|hako_atomic|remote[A-Za-z0-9_]*[[:space:]]*\(|PageMap|page_map|lookup[[:space:]]*\(|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$WORKER_CORE" "$IDENTITY" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-WORKER-001 leaked beyond worker identity substrate" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-worker-identity|HakoAllocWorkerIdentity|WorkerCoreBox|hako_worker_current_id_i64' \
  lang/c-abi/shims/*.inc >/tmp/"$TAG".inc_direct 2>&1; then
  if rg -v 'hako_llvmc_ffi_(mir_call_(need_|shell)|pure_compile_generic_lowering_prescan)' /tmp/"$TAG".inc_direct >/tmp/"$TAG".inc_leak 2>&1; then
    echo "[$TAG] ERROR: worker identity leaked into non-route .inc matcher" >&2
    cat /tmp/"$TAG".inc_leak >&2
    rm -f /tmp/"$TAG".inc_direct /tmp/"$TAG".inc_leak
    exit 1
  fi
fi
rm -f /tmp/"$TAG".inc_direct /tmp/"$TAG".inc_leak

cargo test -q refresh_function_extern_call_routes_records_hako_worker_current_id_route -- --nocapture
cargo test -q -p nyash_kernel current_worker_id_is_single_worker_zero -- --nocapture

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap_worker_identity.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap_worker_identity.mir.json"
exe_out="$tmp_dir/mimap_worker_identity.exe"
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
    "WorkerCoreBox.current_id_i64/0",
    "HakoAllocWorkerIdentity.currentWorkerId/0",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
if plans.get("HakoAllocWorkerIdentity") is None:
    raise SystemExit("missing typed object plan: HakoAllocWorkerIdentity")
fields = {field.get("name") for field in plans["HakoAllocWorkerIdentity"].get("fields", [])}
for field in ("call_count", "last_worker_id"):
    if field not in fields:
        raise SystemExit(f"missing HakoAllocWorkerIdentity field: {field}")

worker = functions["WorkerCoreBox.current_id_i64/0"]
routes = worker.get("metadata", {}).get("extern_call_routes", [])
matches = [
    route for route in routes
    if route.get("route_id") == "extern.hako_worker.current_id_i64"
    and route.get("core_op") == "HakoWorkerCurrentIdI64"
    and route.get("symbol") == "hako_worker_current_id_i64"
    and route.get("return_shape") == "scalar_i64"
    and route.get("value_demand") == "runtime_i64"
]
if len(matches) != 1:
    raise SystemExit(f"missing worker extern route: {routes}")

plans = worker.get("metadata", {}).get("lowering_plan", [])
if not any(
    plan.get("source") == "extern_call_routes"
    and plan.get("route_kind") == "extern.hako_worker.current_id_i64"
    and plan.get("symbol") == "hako_worker_current_id_i64"
    and plan.get("arity") == 1
    for plan in plans
):
    raise SystemExit(f"missing worker lowering plan: {plans}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def require_method(fn_name, box_name, name):
    for callee in iter_calls(functions[fn_name]):
        if (
            callee.get("type") == "Method"
            and callee.get("box_name") == box_name
            and callee.get("name") == name
        ):
            return
    raise SystemExit(f"missing method call {box_name}.{name} in {fn_name}")

def require_global(fn_name, name):
    for callee in iter_calls(functions[fn_name]):
        if callee.get("type") == "Global" and callee.get("name") == name:
            return
    raise SystemExit(f"missing global call {name} in {fn_name}")

require_method("main", "HakoAllocWorkerIdentity", "currentWorkerId")
require_global("HakoAllocWorkerIdentity.currentWorkerId/0", "WorkerCoreBox.current_id_i64/0")

for fn_name in ("main", "WorkerCoreBox.current_id_i64/0", "HakoAllocWorkerIdentity.currentWorkerId/0"):
    for callee in iter_calls(functions[fn_name]):
        target = f"{callee.get('box_name') or ''}.{callee.get('name') or ''}"
        if any(part in target for part in ("Tls", "Atomic", "RemoteFree", "PageMap", "Provider")):
            raise SystemExit(f"forbidden call in {fn_name}: {target}")

print("[mimap-worker-identity-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_worker_current_id_i64_emit' "$build_log"
if rg -F -q 'mir_call_hako_tls_cache_slot_' "$build_log"; then
  echo "[$TAG] ERROR: worker identity row must not emit TLS cache-slot calls" >&2
  exit 1
fi
if rg -F -q 'mir_call_hako_atomic_' "$build_log"; then
  echo "[$TAG] ERROR: worker identity row must not emit atomic calls" >&2
  exit 1
fi

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-worker-identity-proof' "$run_log"
rg -F -q 'worker_ids=0,0' "$run_log"
rg -F -q 'state=0,2' "$run_log"
rg -F -q 'shape=4' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"

echo "[$TAG] ok"

#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-worker-tls-cache-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-worker-tls-cache-proof/main.hako"
APP_TEST="apps/mimalloc-worker-tls-cache-proof/test.sh"
APP_README="apps/mimalloc-worker-tls-cache-proof/README.md"
WORKER_IDENTITY="lang/src/hako_alloc/memory/worker_identity_box.hako"
WORKER_TLS_CACHE="lang/src/hako_alloc/memory/worker_tls_cache_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
TLS_CORE="lang/src/runtime/substrate/tls/tls_core_box.hako"
WORKER_CORE="lang/src/runtime/substrate/worker/worker_core_box.hako"
CARD="docs/development/current/main/phases/phase-293x/293x-394-MIMAP-TLS-001-INTERNAL-TLS-CACHE-SLOT-SUBSTRATE.md"
INDEX="docs/tools/check-scripts-index.md"

echo "[$TAG] running MIMAP-TLS-001 worker TLS cache guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$WORKER_IDENTITY" \
  "$WORKER_TLS_CACHE" \
  "$MODULE" \
  "$MEMORY_README" \
  "$TLS_CORE" \
  "$WORKER_CORE" \
  "$CARD" \
  "$INDEX"

guard_expect_in_file "$TAG" 'box HakoAllocWorkerTlsCache' "$WORKER_TLS_CACHE" "allocator worker TLS cache owner missing"
guard_expect_in_file "$TAG" 'HakoAllocWorkerIdentity' "$WORKER_TLS_CACHE" "worker TLS cache must compose worker identity"
guard_expect_in_file "$TAG" 'TlsCoreBox.cache_slot_get_i64' "$WORKER_TLS_CACHE" "worker TLS cache must read the cache slot through TlsCoreBox"
guard_expect_in_file "$TAG" 'TlsCoreBox.cache_slot_set_i64' "$WORKER_TLS_CACHE" "worker TLS cache must write the cache slot through TlsCoreBox"
guard_expect_in_file "$TAG" 'memory.worker_tls_cache_box = "memory/worker_tls_cache_box.hako"' "$MODULE" "hako_alloc module must export worker TLS cache"
guard_expect_in_file "$TAG" "$0" "$INDEX" "check script index must list worker TLS cache guard"

if rg -n 'worker_local|Channel|task_scope|nowait|await|Atomic|hako_atomic|remote[A-Za-z0-9_]*[[:space:]]*\(|PageMap|page_map|lookup[[:space:]]*\(|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$WORKER_TLS_CACHE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-TLS-001 leaked beyond worker TLS cache-slot substrate" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-worker-tls-cache|HakoAllocWorkerTlsCache' \
  lang/c-abi/shims/*.inc >/tmp/"$TAG".inc_direct 2>&1; then
  echo "[$TAG] ERROR: worker TLS cache proof leaked into backend .inc matcher" >&2
  cat /tmp/"$TAG".inc_direct >&2
  rm -f /tmp/"$TAG".inc_direct
  exit 1
fi
rm -f /tmp/"$TAG".inc_direct

cargo test -q refresh_function_extern_call_routes_records_hako_worker_current_id_route -- --nocapture
cargo test -q refresh_function_extern_call_routes_records_hako_tls_cache_slot_routes -- --nocapture
cargo test -q -p nyash_kernel current_worker_id_is_single_worker_zero -- --nocapture
cargo test -q -p nyash_kernel tls -- --nocapture

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap_worker_tls_cache.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap_worker_tls_cache.mir.json"
exe_out="$tmp_dir/mimap_worker_tls_cache.exe"
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
    "TlsCoreBox.cache_slot_get_i64/1",
    "TlsCoreBox.cache_slot_set_i64/2",
    "HakoAllocWorkerIdentity.currentWorkerId/0",
    "HakoAllocWorkerTlsCache.storeSlot/2",
    "HakoAllocWorkerTlsCache.loadSlot/1",
    "HakoAllocWorkerTlsCache.clearSlot/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
cache_plan = plans.get("HakoAllocWorkerTlsCache")
if cache_plan is None:
    raise SystemExit("missing typed object plan: HakoAllocWorkerTlsCache")
fields = {field.get("name") for field in cache_plan.get("fields", [])}
for field in ("identity", "slot_id", "stored_value", "observed_worker_id", "get_count", "set_count"):
    if field not in fields:
        raise SystemExit(f"missing HakoAllocWorkerTlsCache field: {field}")

def require_extern(fn_name, route_id, core_op, symbol, arity, effects):
    fn = functions[fn_name]
    routes = fn.get("metadata", {}).get("extern_call_routes", [])
    for route in routes:
        if (
            route.get("route_id") == route_id
            and route.get("core_op") == core_op
            and route.get("symbol") == symbol
            and route.get("return_shape") == "scalar_i64"
            and route.get("value_demand") == "runtime_i64"
            and route.get("effects") == effects
        ):
            break
    else:
        raise SystemExit(f"missing extern route for {fn_name}: {routes}")
    lowering = fn.get("metadata", {}).get("lowering_plan", [])
    for plan in lowering:
        if (
            plan.get("source") == "extern_call_routes"
            and (plan.get("source_route_id") == route_id or plan.get("route_kind") == route_id)
            and plan.get("symbol") == symbol
            and plan.get("arity") == arity
        ):
            return
    raise SystemExit(f"missing lowering plan for {fn_name}: {lowering}")

require_extern(
    "WorkerCoreBox.current_id_i64/0",
    "extern.hako_worker.current_id_i64",
    "HakoWorkerCurrentIdI64",
    "hako_worker_current_id_i64",
    1,
    ["hako.worker.current_id"],
)
require_extern(
    "TlsCoreBox.cache_slot_get_i64/1",
    "extern.hako_tls.cache_slot_get_i64",
    "HakoTlsCacheSlotGetI64",
    "hako_tls_cache_slot_get_i64",
    1,
    ["hako.tls.cache_slot_get"],
)
require_extern(
    "TlsCoreBox.cache_slot_set_i64/2",
    "extern.hako_tls.cache_slot_set_i64",
    "HakoTlsCacheSlotSetI64",
    "hako_tls_cache_slot_set_i64",
    2,
    ["hako.tls.cache_slot_set"],
)

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def callee_label(callee):
    return ".".join(part for part in (callee.get("box_name"), callee.get("name")) if part)

def require_call(fn_name, fragment):
    labels = [callee_label(callee) for callee in iter_calls(functions[fn_name])]
    if not any(fragment in label for label in labels):
        raise SystemExit(f"missing call {fragment} in {fn_name}: {labels}")

require_call("HakoAllocWorkerTlsCache.storeSlot/2", "HakoAllocWorkerIdentity.currentWorkerId")
require_call("HakoAllocWorkerTlsCache.storeSlot/2", "TlsCoreBox.cache_slot_set_i64")
require_call("HakoAllocWorkerTlsCache.loadSlot/1", "HakoAllocWorkerIdentity.currentWorkerId")
require_call("HakoAllocWorkerTlsCache.loadSlot/1", "TlsCoreBox.cache_slot_get_i64")
require_call("HakoAllocWorkerTlsCache.clearSlot/1", "HakoAllocWorkerTlsCache.storeSlot")

for fn_name in (
    "main",
    "HakoAllocWorkerTlsCache.storeSlot/2",
    "HakoAllocWorkerTlsCache.loadSlot/1",
    "HakoAllocWorkerTlsCache.clearSlot/1",
):
    for callee in iter_calls(functions[fn_name]):
        label = callee_label(callee)
        if any(part in label for part in ("Atomic", "RemoteFree", "PageMap", "Provider")):
            raise SystemExit(f"forbidden call in {fn_name}: {label}")

print("[mimap-worker-tls-cache-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_worker_current_id_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_tls_cache_slot_get_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_tls_cache_slot_set_i64_emit' "$build_log"
if rg -F -q 'mir_call_hako_atomic_' "$build_log"; then
  echo "[$TAG] ERROR: worker TLS cache row must not emit atomic calls" >&2
  exit 1
fi

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-worker-tls-cache-proof' "$run_log"
rg -F -q 'worker=0 identity_calls=5' "$run_log"
rg -F -q 'values=0,8192,0' "$run_log"
rg -F -q 'rc=0,0' "$run_log"
rg -F -q 'state=0,0,0' "$run_log"
rg -F -q 'counts=3,2,5' "$run_log"
rg -F -q 'shape=13' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"

echo "[$TAG] ok"

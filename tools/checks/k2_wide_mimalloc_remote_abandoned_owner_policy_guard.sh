#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-remote-abandoned-owner-policy"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-remote-abandoned-owner-policy-proof/main.hako"
APP_TEST="apps/mimalloc-remote-abandoned-owner-policy-proof/test.sh"
APP_README="apps/mimalloc-remote-abandoned-owner-policy-proof/README.md"
OWNER="lang/src/hako_alloc/memory/remote_free_abandoned_owner_policy_box.hako"
REMOTE_POLICY="lang/src/hako_alloc/memory/remote_free_policy_box.hako"
WORKER_TLS="lang/src/hako_alloc/memory/worker_tls_cache_box.hako"
THREAD_OWNER="lang/src/hako_alloc/memory/thread_heap_owner_inventory_box.hako"
RECLAIM="lang/src/hako_alloc/memory/abandoned_reclaim_inventory_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
CARD="docs/development/current/main/phases/phase-293x/293x-396-MIMAP-REMOTE-001-REMOTE-FREE-ABANDONED-OWNER-POLICY.md"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_remote_abandoned_owner_policy_guard.sh"

echo "[$TAG] running MIMAP-REMOTE-001 remote abandoned-owner policy guard"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_TEST" \
  "$APP_README" \
  "$OWNER" \
  "$REMOTE_POLICY" \
  "$WORKER_TLS" \
  "$THREAD_OWNER" \
  "$RECLAIM" \
  "$MODULE" \
  "$MEMORY_README" \
  "$CARD" \
  "$INDEX" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'memory.remote_free_abandoned_owner_policy_box = "memory/remote_free_abandoned_owner_policy_box.hako"' "$MODULE" "hako_alloc module must export MIMAP-REMOTE-001 owner"
guard_expect_in_file "$TAG" 'box HakoAllocRemoteAbandonedOwnerPolicy' "$OWNER" "remote abandoned-owner policy owner missing"
guard_expect_in_file "$TAG" 'HakoAllocWorkerTlsCache' "$OWNER" "policy must compose worker TLS cache"
guard_expect_in_file "$TAG" 'HakoAllocRemoteFreePolicy.pushRetry' "$OWNER" "policy must reuse bounded remote-free policy"
guard_expect_in_file "$TAG" 'HakoAllocThreadHeapOwnerInventory' "$OWNER" "policy must reuse thread owner inventory"
guard_expect_in_file "$TAG" 'HakoAllocAbandonedReclaimInventory' "$OWNER" "policy must reuse abandoned reclaim inventory"
guard_expect_in_file "$TAG" 'remote_free_abandoned_owner_policy_box.hako` owns MIMAP-REMOTE-001' "$MEMORY_README" "memory README must define MIMAP-REMOTE-001 owner"
guard_expect_in_file "$TAG" 'Existing Proof Role Table' "$CARD" "MIMAP-REMOTE-001 card must include remote-free role table"
guard_expect_in_file "$TAG" 'k2_wide_mimalloc_remote_free_i64_exe_guard.sh' "$CARD" "MIMAP-REMOTE-001 card must require M31 proof guard"
guard_expect_in_file "$TAG" 'k2_wide_mimalloc_remote_free_policy_exe_guard.sh' "$CARD" "MIMAP-REMOTE-001 card must require M37 proof guard"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-REMOTE-001 guard"

if rg -n 'worker_local|lock<T>|Channel|task_scope|nowait|await|spawn[[:space:]]*\(|thread::|global_allocator|install_hook|provider[A-Za-z0-9_]*[[:space:]]*\(|hook[A-Za-z0-9_]*[[:space:]]*\(|page_map|PageMap|lookup[[:space:]]*\(|change_page_owner|execute_reclaim|release_osvm|unreserve' \
  "$OWNER" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-REMOTE-001 leaked beyond policy composition scope" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-remote-abandoned-owner-policy|HakoAllocRemoteAbandonedOwner|remote_free_abandoned_owner_policy' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-REMOTE-001 app/owner matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'hako_atomic_ptr_fetch_add|ptr_fetch_add' \
  src lang/c-abi/shims crates/nyash_kernel >/tmp/"$TAG".ptr_fetch_add 2>&1; then
  echo "[$TAG] ERROR: pointer fetch_add row must stay inactive" >&2
  cat /tmp/"$TAG".ptr_fetch_add >&2
  rm -f /tmp/"$TAG".ptr_fetch_add
  exit 1
fi
rm -f /tmp/"$TAG".ptr_fetch_add

bash tools/checks/k2_wide_mimalloc_remote_free_i64_exe_guard.sh
bash tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap_remote_owner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap_remote_owner.mir.json"
exe_out="$tmp_dir/mimap_remote_owner.exe"
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
    "HakoAllocRemoteAbandonedOwnerPolicy.installMailbox/2",
    "HakoAllocRemoteAbandonedOwnerPolicy.clearMailbox/1",
    "HakoAllocRemoteAbandonedOwnerPolicy.routeFreeOrClassify/11",
    "HakoAllocRemoteAbandonedOwnerPolicy.decision/21",
    "HakoAllocRemoteAbandonedOwnerPolicy.reject/11",
    "HakoAllocWorkerTlsCache.loadSlot/1",
    "HakoAllocWorkerTlsCache.storeSlot/2",
    "HakoAllocWorkerTlsCache.clearSlot/1",
    "HakoAllocRemoteFreePolicy.pushRetry/3",
    "HakoAllocRemoteFreePolicy.peekHead/1",
    "HakoAllocThreadHeapOwnerInventory.classifyOwner/6",
    "HakoAllocAbandonedReclaimInventory.classifyPage/9",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for box_name in (
    "HakoAllocRemoteAbandonedOwnerDecision",
    "HakoAllocRemoteAbandonedOwnerPolicy",
):
    if plans.get(box_name) is None:
        raise SystemExit(f"missing typed object plan: {box_name}")

fields = {field.get("name") for field in plans["HakoAllocRemoteAbandonedOwnerPolicy"].get("fields", [])}
for field in (
    "cache",
    "owner_inventory",
    "reclaim_inventory",
    "classify_count",
    "same_owner_count",
    "remote_publish_count",
    "abandoned_candidate_count",
    "reject_count",
):
    if field not in fields:
        raise SystemExit(f"missing policy field: {field}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def label(callee):
    return ".".join(part for part in (callee.get("box_name"), callee.get("name")) if part)

def require_call(fn_name, fragment):
    labels = [label(callee) for callee in iter_calls(functions[fn_name])]
    if not any(fragment in item for item in labels):
        raise SystemExit(f"missing call {fragment} in {fn_name}: {labels}")

route = "HakoAllocRemoteAbandonedOwnerPolicy.routeFreeOrClassify/11"
for fragment in (
    "HakoAllocWorkerTlsCache.loadSlot",
    "HakoAllocWorkerTlsCache.lastWorkerId",
    "HakoAllocRemoteFreePolicy.pushRetry",
    "HakoAllocRemoteFreePolicy.peekHead",
    "HakoAllocThreadHeapOwnerInventory.classifyOwner",
    "HakoAllocAbandonedReclaimInventory.classifyPage",
):
    require_call(route, fragment)

for fn_name in (
    "main",
    "HakoAllocRemoteAbandonedOwnerPolicy.routeFreeOrClassify/11",
):
    for callee in iter_calls(functions[fn_name]):
        item = label(callee)
        if any(part in item for part in ("PageMap", "Provider", "Hook")):
            raise SystemExit(f"forbidden call in {fn_name}: {item}")

print("[mimap-remote-owner-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_worker_current_id_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_tls_cache_slot_get_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_tls_cache_slot_set_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_load_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_store_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_atomic_ptr_cas_ordered_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-remote-abandoned-owner-policy-proof' "$run_log"
rg -F -q 'same=1,1,0' "$run_log"
rg -F -q 'remote=2,1,1,0' "$run_log"
rg -F -q 'abandoned=3,1,1,1,1' "$run_log"
rg -F -q 'pending=0,6,4,3' "$run_log"
rg -F -q 'counts=4,1,1,1,1' "$run_log"
rg -F -q 'mailbox=0,0,0' "$run_log"
rg -F -q 'shape=9' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"

echo "[$TAG] ok"

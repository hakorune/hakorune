#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-facade-purge-policy-route-exe"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-facade-purge-policy-route-proof/main.hako"
APP_README="apps/mimalloc-facade-purge-policy-route-proof/README.md"
ROUTE="lang/src/hako_alloc/memory/object_lifecycle_facade_purge_policy_box.hako"
STATS="lang/src/hako_alloc/memory/object_lifecycle_facade_stats_box.hako"
PURGE="lang/src/hako_alloc/memory/purge_candidate_policy_box.hako"
RECLAIM="lang/src/hako_alloc/memory/abandoned_reclaim_inventory_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
CARD="docs/development/current/main/phases/phase-293x/293x-379-MIMAP-019A-PURGE-RECLAIM-POLICY-ROUTE.md"
INDEX="docs/tools/check-scripts-index.md"
README="lang/src/hako_alloc/memory/README.md"

for path in "$APP" "$APP_README" "$ROUTE" "$STATS" "$PURGE" "$RECLAIM" "$MODULE" "$CARD" "$INDEX" "$README"; do
  [[ -f "$path" ]] || { echo "[$TAG] ERROR: missing required file: $path" >&2; exit 1; }
done

rg -F -q 'box HakoAllocObjectLifecycleFacadePurgePolicyRoute' "$ROUTE"
rg -F -q 'box HakoAllocObjectLifecycleFacadePurgePolicyDecision' "$ROUTE"
rg -F -q 'HakoAllocPurgeCandidatePolicyInventory' "$ROUTE"
rg -F -q 'HakoAllocAbandonedReclaimInventory' "$ROUTE"
rg -F -q 'box HakoAllocObjectLifecycleFacadePurgePolicyLifecycleReport' "$ROUTE"
rg -F -q 'box HakoAllocObjectLifecycleFacadePurgePolicyPageView' "$ROUTE"
rg -F -q 'classifyPageView(stats, page_view)' "$ROUTE"
rg -F -q 'memory.object_lifecycle_facade_purge_policy_box = "memory/object_lifecycle_facade_purge_policy_box.hako"' "$MODULE"
rg -F -q 'MIMAP-019A' "$CARD"
rg -F -q 'object_lifecycle_facade_purge_policy_box.hako' "$README"
rg -F -q 'k2_wide_mimalloc_facade_purge_policy_route_exe_guard.sh' "$INDEX"

if rg -n 'HakoAllocBoundedPurgeDecommitScheduler|HakoAllocPurgeStateAwareDecommit|HakoAllocBoundedDecommitPolicy|HakoAllocPageSource|HakoAllocPurgeHeapDecommit|OsVmCoreBox|attemptHeapPage[[:space:]]*\(|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|PageMap|page_map|lookup[[:space:]]*\(|provider[A-Za-z0-9_]*[[:space:]]*\(|global_allocator|install_hook|hook[A-Za-z0-9_]*[[:space:]]*\(|remote[A-Za-z0-9_]*[[:space:]]*\(' \
  "$ROUTE" "$APP" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: MIMAP-019A route must stay read-only/policy-only" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc-facade-purge-policy-route-proof|HakoAllocObjectLifecycleFacadePurgePolicy|objectLifecycleFacadePurge' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-019A matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap019a_facade_policy.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap019a.mir.json"
exe_out="$tmp_dir/mimap019a.exe"
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
    "HakoAllocObjectLifecycleFacadePurgePolicyRoute.pageView/13",
    "HakoAllocObjectLifecycleFacadePurgePolicyRoute.classifyPageView/2",
    "HakoAllocObjectLifecycleFacadePurgePolicyRoute.lifecycleReport/17",
    "HakoAllocObjectLifecycleFacadePurgePolicyRoute.emptyDecision/2",
    "HakoAllocObjectLifecycleFacadePurgePolicyRoute.reject/1",
    "HakoAllocPurgeCandidatePolicyInventory.classifyLifecycleReport/1",
    "HakoAllocAbandonedReclaimInventory.classifyPage/9",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in (
    "HakoAllocObjectLifecycleFacadePurgePolicyRoute",
    "HakoAllocObjectLifecycleFacadePurgePolicyLifecycleReport",
    "HakoAllocObjectLifecycleFacadePurgePolicyPageView",
    "HakoAllocObjectLifecycleFacadePurgePolicyDecision",
    "HakoAllocPurgeCandidatePolicyInventory",
    "HakoAllocAbandonedReclaimInventory",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

route_fields = {
    field.get("name"): field
    for field in plans["HakoAllocObjectLifecycleFacadePurgePolicyRoute"].get("fields", [])
}
for field_name, type_name in (
    ("purge_inventory", "HakoAllocPurgeCandidatePolicyInventory"),
    ("reclaim_inventory", "HakoAllocAbandonedReclaimInventory"),
):
    field = route_fields.get(field_name)
    if field is None or field.get("declared_type") != type_name or field.get("storage") != "handle":
        raise SystemExit(f"route field {field_name} must be typed handle {type_name}: {field}")

decision_fields = {
    field.get("name")
    for field in plans["HakoAllocObjectLifecycleFacadePurgePolicyDecision"].get("fields", [])
}
for field in (
    "stats_total_terminal_count",
    "purge_eligible",
    "purge_decommit_candidate",
    "reclaim_eligible",
    "reclaim_forward_to_purge_candidate",
    "would_decommit",
    "would_reclaim",
    "would_unreserve",
    "would_release_osvm",
):
    if field not in decision_fields:
        raise SystemExit(f"missing decision field: {field}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def require_method(fn_name, box_name, name):
    fn = functions[fn_name]
    for callee in iter_calls(fn):
        if (
            callee.get("type") == "Method"
            and callee.get("box_name") == box_name
            and callee.get("name") == name
        ):
            return
    raise SystemExit(f"missing method call {box_name}.{name} in {fn_name}")

route_fn = "HakoAllocObjectLifecycleFacadePurgePolicyRoute.classifyPageView/2"
require_method(route_fn, "HakoAllocObjectLifecycleFacadePurgePolicyRoute", "lifecycleReport")
require_method(route_fn, "HakoAllocPurgeCandidatePolicyInventory", "classifyLifecycleReport")
require_method(route_fn, "HakoAllocAbandonedReclaimInventory", "classifyPage")

for fn_name, fn in functions.items():
    for callee in iter_calls(fn):
        box = callee.get("box_name") or ""
        name = callee.get("name") or ""
        target = f"{box}.{name}"
        forbidden = (
            "HakoAllocBoundedPurgeDecommitScheduler",
            "HakoAllocPurgeStateAwareDecommit",
            "HakoAllocBoundedDecommitPolicy",
            "HakoAllocPageSource",
            "OsVmCoreBox",
        )
        if any(part in target for part in forbidden):
            raise SystemExit(f"forbidden execution call in {fn_name}: {target}")
        if name in {"attemptHeapPage", "decommitPage", "commitPage", "reservePage", "unreserve", "releasePage"}:
            raise SystemExit(f"forbidden execution method in {fn_name}: {target}")

print("[mimap019a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-facade-purge-policy-route-proof' "$run_log"
rg -F -q 'stats=1,1,2' "$run_log"
rg -F -q 'ready=1,0,1,1' "$run_log"
rg -F -q 'rejects=3,2,4,4' "$run_log"
rg -F -q 'would=0,0' "$run_log"
rg -F -q 'counts=3' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

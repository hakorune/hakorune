#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-page-source-decommit-adapter"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-page-source-decommit-adapter-proof/main.hako"
APP_README="apps/hako-alloc-page-source-decommit-adapter-proof/README.md"
APP_TEST="apps/hako-alloc-page-source-decommit-adapter-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-236-M196-PAGE-SOURCE-DECOMMIT-ADAPTER.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
ADAPTER="lang/src/hako_alloc/memory/purge_page_source_decommit_adapter_box.hako"
BOUNDED="lang/src/hako_alloc/memory/purge_bounded_decommit_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_page_source_decommit_adapter_guard.sh"

echo "[$TAG] checking M196 page-source decommit adapter"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$PLAN" \
  "$PHASE_README" \
  "$TASKBOARD" \
  "$INDEX" \
  "$ADAPTER" \
  "$BOUNDED" \
  "$MODULE" \
  "$MEMORY_README" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M196 card must be complete"
guard_expect_in_file "$TAG" 'M196 status:' "$PLAN" "mimalloc plan must record M196 status"
guard_expect_in_file "$TAG" '`293x-236`' "$PHASE_README" "phase README must list M196 row"
guard_expect_in_file "$TAG" '\[x\] `293x-236`' "$TASKBOARD" "taskboard must mark M196 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M196 guard"

guard_expect_in_file "$TAG" 'memory.purge_page_source_decommit_adapter_box = "memory/purge_page_source_decommit_adapter_box.hako"' "$MODULE" "hako_alloc module must export page-source adapter"
guard_expect_in_file "$TAG" 'box HakoAllocPageSourceDecommitAdapter' "$ADAPTER" "page-source decommit adapter box must exist"
guard_expect_in_file "$TAG" 'HakoAllocPageSourcePolicy\.decommitPage\(base, bytes\)' "$ADAPTER" "adapter must delegate only to page-source decommit"
guard_expect_in_file "$TAG" 'purge_page_source_decommit_adapter_box.hako` owns M196 page-source decommit' "$MEMORY_README" "memory README must define M196 owner"

if rg -n 'HakoAllocPageSourcePolicy\.(reservePage|commitPage)|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|hako_osvm_(unreserve|release)' \
  "$ADAPTER" >/tmp/"$TAG".forbidden_adapter 2>&1; then
  echo "[$TAG] ERROR: M196 adapter must not reserve, commit, unreserve, or release OS pages" >&2
  cat /tmp/"$TAG".forbidden_adapter >&2
  rm -f /tmp/"$TAG".forbidden_adapter
  exit 1
fi
rm -f /tmp/"$TAG".forbidden_adapter

if rg -n 'hako-alloc-page-source-decommit-adapter-proof|HakoAllocPageSourceDecommitAdapter' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M196 app/adapter matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_page_source_decommit_adapter_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M196 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m196_hako_alloc_decommit.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m196.mir.json"
exe_out="$tmp_dir/m196.exe"
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
    "HakoAllocPageSourceDecommitAdapter.decommitPage/2",
    "HakoAllocBoundedDecommitPolicy.attemptDecommit/4",
    "HakoAllocPageSourcePolicy.decommitPage/2",
    "OsVmCoreBox.decommit_bytes_i64/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
if plans.get("HakoAllocPageSourceDecommitAdapter") is None:
    raise SystemExit("missing typed object plan: HakoAllocPageSourceDecommitAdapter")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def require_global(owner, owner_name, symbol):
    routes = owner.get("metadata", {}).get("global_call_routes", [])
    for route in routes:
        if route.get("symbol") == symbol and route.get("target_shape") == "generic_i64_body":
            return
    raise SystemExit(f"missing global route in {owner_name} -> {symbol}: {routes}")

require_global(
    functions["HakoAllocPageSourceDecommitAdapter.decommitPage/2"],
    "HakoAllocPageSourceDecommitAdapter.decommitPage/2",
    "HakoAllocPageSourcePolicy.decommitPage/2",
)
require_global(
    functions["HakoAllocPageSourcePolicy.decommitPage/2"],
    "HakoAllocPageSourcePolicy.decommitPage/2",
    "OsVmCoreBox.decommit_bytes_i64/2",
)

print("[m196-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-page-source-decommit-adapter-proof' "$run_log"
rg -F -q 'page=4096 reserved=1 commit=0' "$run_log"
rg -F -q 'bounded=0,1,1,0' "$run_log"
rg -F -q 'adapter=1,1,4096,0' "$run_log"
rg -F -q 'release=0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

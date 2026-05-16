#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-osvm-fast-path-unreserve-route"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-osvm-fast-path-unreserve-route-proof/main.hako"
APP_README="apps/hako-alloc-osvm-fast-path-unreserve-route-proof/README.md"
APP_TEST="apps/hako-alloc-osvm-fast-path-unreserve-route-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-522-MIMAP-045A-OSVM-FAST-PATH-UNRESERVE-ROUTE.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/osvm_fast_path_unreserve_route_box.hako"
REUSE_ROUTE="lang/src/hako_alloc/memory/osvm_fast_path_reuse_route_box.hako"
UNRESERVE_ADAPTER="lang/src/hako_alloc/memory/purge_page_source_unreserve_adapter_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_route_guard.sh"

echo "[$TAG] checking MIMAP-045A OSVM-backed fast-path unreserve route"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$ROOT_README" \
  "$MEMORY_README" \
  "$OWNER" \
  "$REUSE_ROUTE" \
  "$UNRESERVE_ADAPTER" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMAP-045A' "$CARD" "MIMAP-045A card must name the row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-045A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-045A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-045A"
guard_expect_in_file "$TAG" 'memory.osvm_fast_path_unreserve_route_box = "memory/osvm_fast_path_unreserve_route_box.hako"' "$MODULE" "hako_alloc module must export MIMAP-045A owner"
guard_expect_in_file "$TAG" 'HakoAllocOsVmFastPathUnreserveRoute' "$ROOT_README" "root README must name MIMAP-045A owner"
guard_expect_in_file "$TAG" 'osvm_fast_path_unreserve_route_box.hako` owns MIMAP-045A' "$MEMORY_README" "memory README must define MIMAP-045A owner"
guard_expect_in_file "$TAG" 'box HakoAllocOsVmFastPathUnreserveRoute' "$OWNER" "MIMAP-045A owner box must exist"
guard_expect_in_file "$TAG" 'reuse_route: HakoAllocOsVmFastPathReuseRoute' "$OWNER" "MIMAP-045A must own 043A route handle"
guard_expect_in_file "$TAG" 'unreserve_adapter: HakoAllocPageSourceUnreserveAdapter' "$OWNER" "MIMAP-045A must own MIMAP-033A adapter handle"
guard_expect_in_file "$TAG" 'unreservePurgedPage\(page_id\)' "$OWNER" "MIMAP-045A must expose unreserve route"
guard_expect_in_file "$TAG" 'me.unreserve_adapter.unreservePage\(base, bytes\)' "$OWNER" "MIMAP-045A must delegate unreserve through MIMAP-033A"

if rg -n 'HakoAllocPageSourcePolicy|OsVmCoreBox|(^|[^A-Za-z0-9_])(reservePage|commitPage|decommitPage|releasePage)[[:space:]]*\(|unreserve_bytes_i64' \
  "$OWNER" "$APP" >/tmp/"$TAG".direct_execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-045A route/proof must not call page-source or OSVM seams directly" >&2
  cat /tmp/"$TAG".direct_execution_leak >&2
  rm -f /tmp/"$TAG".direct_execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|provider_|install_hook|global_allocator|InlineRecord|ArrayStorage|PlanProbe|AutoUse|compiler|Atomic|Tls|RemoteFree|remote_free|worker_local|spawn[[:space:]]*\(' \
  "$OWNER" "$APP" >/tmp/"$TAG".forbidden_vocab 2>&1; then
  echo "[$TAG] ERROR: MIMAP-045A route/proof must stay provider/backend/concurrency vocabulary free" >&2
  cat /tmp/"$TAG".forbidden_vocab >&2
  rm -f /tmp/"$TAG".forbidden_vocab
  exit 1
fi
rm -f /tmp/"$TAG".forbidden_vocab

if rg -n 'hako-alloc-osvm-fast-path-unreserve-route-proof|HakoAllocOsVmFastPathUnreserveRoute|osvm_fast_path_unreserve_route' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-045A app/route matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap045a_osvm_fast_path_unreserve.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap045a.mir.json"
exe_out="$tmp_dir/mimap045a.exe"
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
    "HakoAllocOsVmFastPathUnreserveRoute.allocate/1",
    "HakoAllocOsVmFastPathUnreserveRoute.release/1",
    "HakoAllocOsVmFastPathUnreserveRoute.purgeOne/1",
    "HakoAllocOsVmFastPathUnreserveRoute.unreservePurgedPage/1",
    "HakoAllocOsVmFastPathReuseRoute.allocate/1",
    "HakoAllocOsVmFastPathReuseRoute.release/1",
    "HakoAllocOsVmFastPathReuseRoute.purgeOne/1",
    "HakoAllocPageSourceUnreserveAdapter.unreservePage/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in (
    "HakoAllocOsVmFastPathUnreserveRoute",
    "HakoAllocOsVmFastPathUnreserveReport",
    "HakoAllocOsVmFastPathReuseRoute",
    "HakoAllocPageSourceUnreserveAdapter",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

route_fields = {
    field.get("name"): field
    for field in plans["HakoAllocOsVmFastPathUnreserveRoute"].get("fields", [])
}
for name in ("reuse_route", "unreserve_adapter"):
    if name not in route_fields:
        raise SystemExit(f"missing route field: {name}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def call_pairs(fn_name):
    return {
        (callee.get("box_name"), callee.get("name"))
        for callee in iter_calls(functions[fn_name])
    }

expected_calls = {
    "HakoAllocOsVmFastPathUnreserveRoute.allocate/1": {
        ("HakoAllocOsVmFastPathReuseRoute", "allocate"),
    },
    "HakoAllocOsVmFastPathUnreserveRoute.release/1": {
        ("HakoAllocOsVmFastPathReuseRoute", "release"),
    },
    "HakoAllocOsVmFastPathUnreserveRoute.purgeOne/1": {
        ("HakoAllocOsVmFastPathReuseRoute", "purgeOne"),
    },
    "HakoAllocOsVmFastPathUnreserveRoute.unreservePurgedPage/1": {
        ("HakoAllocOsVmFastPathReuseRoute", "markerIsMarked"),
        ("HakoAllocOsVmBackedFastPathHeap", "pageBase"),
        ("HakoAllocOsVmBackedFastPathHeap", "pageBackingBytes"),
        ("HakoAllocPageSourceUnreserveAdapter", "unreservePage"),
    },
}
for fn_name, expected in expected_calls.items():
    seen = call_pairs(fn_name)
    missing_calls = sorted(expected - seen)
    if missing_calls:
        raise SystemExit(f"{fn_name} missing required seam calls: {missing_calls}")

for fn_name in (
    "main",
    "HakoAllocOsVmFastPathUnreserveRoute.allocate/1",
    "HakoAllocOsVmFastPathUnreserveRoute.release/1",
    "HakoAllocOsVmFastPathUnreserveRoute.purgeOne/1",
    "HakoAllocOsVmFastPathUnreserveRoute.unreservePurgedPage/1",
):
    for callee in iter_calls(functions[fn_name]):
        if callee.get("box_name") in {"HakoAllocPageSourcePolicy", "OsVmCoreBox"}:
            raise SystemExit(f"MIMAP-045A route/proof bypassed owner seams: {callee}")

print("[mimap045a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_unreserve_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-osvm-fast-path-unreserve-route-proof' "$run_log"
rg -F -q 'initial=1,1,1,1' "$run_log"
rg -F -q 'purge=0,0,0,1,1' "$run_log"
rg -F -q 'unreserve=0,1,1,0' "$run_log"
rg -F -q 'backing=0,' "$run_log"
rg -F -q ',32,1' "$run_log"
rg -F -q 'adapter=1,1,0' "$run_log"
rg -F -q 'route=1,1,0,0,1' "$run_log"
rg -F -q 'heap=1,1,1,1,1,0' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

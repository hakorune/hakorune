#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-osvm-fast-path-unreserve-failfast"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-osvm-fast-path-unreserve-failfast-proof/main.hako"
APP_README="apps/hako-alloc-osvm-fast-path-unreserve-failfast-proof/README.md"
APP_TEST="apps/hako-alloc-osvm-fast-path-unreserve-failfast-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-524-MIMAP-046A-OSVM-FAST-PATH-UNRESERVE-FAILFAST.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
ROOT_README="lang/src/hako_alloc/README.md"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/osvm_fast_path_unreserve_failfast_box.hako"
SUCCESS_ROUTE="lang/src/hako_alloc/memory/osvm_fast_path_unreserve_route_box.hako"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_failfast_guard.sh"

echo "[$TAG] checking MIMAP-046A OSVM-backed fast-path unreserve fail-fast diagnostics"

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
  "$SUCCESS_ROUTE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMAP-046A' "$CARD" "MIMAP-046A card must name the row"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list MIMAP-046A guard"
guard_expect_in_file "$TAG" 'id = "MIMAP-046A"' "$PROOF_MANIFEST" "proof app manifest must list MIMAP-046A"
guard_expect_in_file "$TAG" 'memory.osvm_fast_path_unreserve_failfast_box = "memory/osvm_fast_path_unreserve_failfast_box.hako"' "$MODULE" "hako_alloc module must export MIMAP-046A owner"
guard_expect_in_file "$TAG" 'HakoAllocOsVmFastPathUnreserveFailFastRoute' "$ROOT_README" "root README must name MIMAP-046A owner"
guard_expect_in_file "$TAG" 'osvm_fast_path_unreserve_failfast_box.hako` owns MIMAP-046A' "$MEMORY_README" "memory README must define MIMAP-046A owner"
guard_expect_in_file "$TAG" 'box HakoAllocOsVmFastPathUnreserveFailFastRoute' "$OWNER" "MIMAP-046A owner box must exist"
guard_expect_in_file "$TAG" 'route: HakoAllocOsVmFastPathUnreserveRoute' "$OWNER" "MIMAP-046A must wrap MIMAP-045A success route"
guard_expect_in_file "$TAG" 'duplicate_reason' "$OWNER" "MIMAP-046A must expose duplicate reason"
guard_expect_in_file "$TAG" 'unknown_reason' "$OWNER" "MIMAP-046A must expose unknown reason"
guard_expect_in_file "$TAG" 'not_decommitted_reason' "$OWNER" "MIMAP-046A must expose not-decommitted reason"
guard_expect_in_file "$TAG" 'alreadyUnreserved\(page_id\)' "$OWNER" "MIMAP-046A must track duplicate unreserve"

if rg -n 'HakoAllocPageSourcePolicy|OsVmCoreBox|(^|[^A-Za-z0-9_])(reservePage|commitPage|decommitPage|unreservePage|releasePage)[[:space:]]*\(|unreserve_bytes_i64' \
  "$OWNER" "$APP" >/tmp/"$TAG".direct_execution_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-046A diagnostics/proof must not call page-source or OSVM seams directly" >&2
  cat /tmp/"$TAG".direct_execution_leak >&2
  rm -f /tmp/"$TAG".direct_execution_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_execution_leak

if rg -n 'NYASH_|HAKO_|std::env|env::|provider_|install_hook|global_allocator|InlineRecord|ArrayStorage|PlanProbe|AutoUse|compiler|Atomic|Tls|RemoteFree|remote_free|worker_local|spawn[[:space:]]*\(' \
  "$OWNER" "$APP" >/tmp/"$TAG".forbidden_vocab 2>&1; then
  echo "[$TAG] ERROR: MIMAP-046A diagnostics/proof must stay provider/backend/concurrency vocabulary free" >&2
  cat /tmp/"$TAG".forbidden_vocab >&2
  rm -f /tmp/"$TAG".forbidden_vocab
  exit 1
fi
rm -f /tmp/"$TAG".forbidden_vocab

if rg -n 'hako-alloc-osvm-fast-path-unreserve-failfast-proof|HakoAllocOsVmFastPathUnreserveFailFastRoute|osvm_fast_path_unreserve_failfast' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: MIMAP-046A app/diagnostics matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_mimap046a_osvm_fast_path_unreserve_failfast.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/mimap046a.mir.json"
exe_out="$tmp_dir/mimap046a.exe"
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
    "HakoAllocOsVmFastPathUnreserveFailFastRoute.allocate/1",
    "HakoAllocOsVmFastPathUnreserveFailFastRoute.release/1",
    "HakoAllocOsVmFastPathUnreserveFailFastRoute.purgeOne/1",
    "HakoAllocOsVmFastPathUnreserveFailFastRoute.unreservePurgedPage/1",
    "HakoAllocOsVmFastPathUnreserveFailFastRoute.alreadyUnreserved/1",
    "HakoAllocOsVmFastPathUnreserveRoute.unreservePurgedPage/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
for name in (
    "HakoAllocOsVmFastPathUnreserveFailFastRoute",
    "HakoAllocOsVmFastPathUnreserveFailFastReport",
    "HakoAllocOsVmFastPathUnreserveRoute",
):
    if plans.get(name) is None:
        raise SystemExit(f"missing typed object plan: {name}")

route_fields = {
    field.get("name"): field
    for field in plans["HakoAllocOsVmFastPathUnreserveFailFastRoute"].get("fields", [])
}
for name in ("route", "unreserved_page_ids"):
    if name not in route_fields:
        raise SystemExit(f"missing diagnostics field: {name}")

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

expected = {
    ("HakoAllocOsVmFastPathUnreserveRoute", "unreservePurgedPage"),
    ("HakoAllocOsVmFastPathUnreserveFailFastRoute", "alreadyUnreserved"),
    ("HakoAllocOsVmFastPathUnreserveFailFastRoute", "pageIsKnown"),
}
seen = call_pairs("HakoAllocOsVmFastPathUnreserveFailFastRoute.unreservePurgedPage/1")
missing_calls = sorted(expected - seen)
if missing_calls:
    raise SystemExit(f"unreserve fail-fast route missing seam calls: {missing_calls}")

for fn_name in (
    "main",
    "HakoAllocOsVmFastPathUnreserveFailFastRoute.unreservePurgedPage/1",
    "HakoAllocOsVmFastPathUnreserveFailFastRoute.alreadyUnreserved/1",
):
    for callee in iter_calls(functions[fn_name]):
        if callee.get("box_name") in {"HakoAllocPageSourcePolicy", "OsVmCoreBox"}:
            raise SystemExit(f"MIMAP-046A diagnostics/proof bypassed owner seams: {callee}")

print("[mimap046a-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_unreserve_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-osvm-fast-path-unreserve-failfast-proof' "$run_log"
rg -F -q 'first=1,0,1,1,1' "$run_log"
rg -F -q 'duplicate=0,1,0,1' "$run_log"
rg -F -q 'unknown=0,2,0,1' "$run_log"
rg -F -q 'stale=0,3,0,0' "$run_log"
rg -F -q 'counts=3,1,2,1,1,1' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

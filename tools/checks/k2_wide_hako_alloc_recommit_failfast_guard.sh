#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-recommit-failfast"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-recommit-failfast-proof/main.hako"
APP_README="apps/hako-alloc-recommit-failfast-proof/README.md"
APP_TEST="apps/hako-alloc-recommit-failfast-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-245-M201-RECOMMIT-FAILFAST-ENTRY.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
OWNER="lang/src/hako_alloc/memory/purge_recommit_failfast_box.hako"
PRECONDITION="lang/src/hako_alloc/memory/purge_decommitted_page_reuse_precondition_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_recommit_failfast_guard.sh"

echo "[$TAG] checking M201 recommit fail-fast entry"

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
  "$OWNER" \
  "$PRECONDITION" \
  "$MODULE" \
  "$MEMORY_README" \
  "$PROOF_MANIFEST" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M201 card must be complete"
guard_expect_in_file "$TAG" 'M201 status:' "$PLAN" "mimalloc plan must record M201 status"
guard_expect_in_file "$TAG" '`293x-245`' "$PHASE_README" "phase README must list M201 row"
guard_expect_in_file "$TAG" '\[x\] `293x-245`' "$TASKBOARD" "taskboard must mark M201 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M201 guard"
guard_expect_in_file "$TAG" 'id = "M201"' "$PROOF_MANIFEST" "proof app manifest must list M201"

guard_expect_in_file "$TAG" 'memory.purge_recommit_failfast_box = "memory/purge_recommit_failfast_box.hako"' "$MODULE" "hako_alloc module must export recommit fail-fast entry"
guard_expect_in_file "$TAG" 'box HakoAllocRecommitFailFastEntry' "$OWNER" "recommit fail-fast entry box must exist"
guard_expect_in_file "$TAG" 'attemptHeapPage' "$OWNER" "recommit fail-fast entry must expose attemptHeapPage"
guard_expect_in_file "$TAG" 'HakoAllocDecommittedPageReusePrecondition' "$OWNER" "M201 must read the M200 precondition"
guard_expect_in_file "$TAG" 'recommit_executed = 0' "$OWNER" "M201 report must keep recommit execution closed"
guard_expect_in_file "$TAG" 'source_executed = 0' "$OWNER" "M201 report must keep source execution closed"
guard_expect_in_file "$TAG" 'owns M201 recommit fail-fast' "$MEMORY_README" "memory README must define M201 owner"

if rg -n 'HakoAllocPageSourcePolicy\.|OsVmCoreBox\.|decommitPage[[:space:]]*\(|commitPage[[:space:]]*\(|reservePage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|heap\.decommitPage|addBackedPage[[:space:]]*\(|addFreshPage[[:space:]]*\(' \
  "$OWNER" >/tmp/"$TAG".direct_leak 2>&1; then
  echo "[$TAG] ERROR: M201 recommit fail-fast owner must not call page-source APIs or mutate heap/page state" >&2
  cat /tmp/"$TAG".direct_leak >&2
  rm -f /tmp/"$TAG".direct_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_leak

if rg -n 'marked_page_ids\.|\.push\(|marker\.marked_page_ids|clear|remove' "$OWNER" >/tmp/"$TAG".marker_mutation 2>&1; then
  echo "[$TAG] ERROR: M201 recommit fail-fast owner must not mutate or clear decommit marker state" >&2
  cat /tmp/"$TAG".marker_mutation >&2
  rm -f /tmp/"$TAG".marker_mutation
  exit 1
fi
rm -f /tmp/"$TAG".marker_mutation

if rg -n 'hako-alloc-recommit-failfast-proof|HakoAllocRecommitFailFastEntry' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M201 app/entry matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_recommit_failfast_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M201 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m201_hako_alloc_recommit.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m201.mir.json"
exe_out="$tmp_dir/m201.exe"
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
    "HakoAllocRecommitFailFastEntry.attemptHeapPage/3",
    "HakoAllocDecommittedPageReusePrecondition.classifyHeapPage/3",
    "HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage/2",
    "HakoAllocPurgeDecommitStateMarker.isMarked/1",
    "HakoAllocPageSourceDecommitAdapter.decommitPage/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
entry = plans.get("HakoAllocRecommitFailFastEntry")
if entry is None:
    raise SystemExit("missing typed object plan: HakoAllocRecommitFailFastEntry")
fields = {
    field.get("name"): field
    for field in entry.get("fields", [])
}
for name in ("attempt_count", "no_recommit_count", "blocked_count", "missing_count", "recommit_execution_count", "source_execution_count"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad recommit entry counter field {name}: {field}")

print("[m201-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-recommit-failfast-proof' "$run_log"
rg -F -q 'before=0,0,1,0,0,0' "$run_log"
rg -F -q 'live=2,0' "$run_log"
rg -F -q 'release=1' "$run_log"
rg -F -q 'first=0,1,1' "$run_log"
rg -F -q 'blocked=1,1,1,1,0,0,1,16' "$run_log"
rg -F -q 'missing=2,1,1,0,0' "$run_log"
rg -F -q 'recommit=3,1,2,1,0,0,99' "$run_log"
rg -F -q 'precondition=3,1,2,1' "$run_log"
rg -F -q 'guard=2,1,1,1' "$run_log"
rg -F -q 'adapter=1,1,16' "$run_log"
rg -F -q 'heap=0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

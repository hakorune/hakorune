#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-bounded-recommit-policy"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-bounded-recommit-policy-proof/main.hako"
APP_README="apps/hako-alloc-bounded-recommit-policy-proof/README.md"
APP_TEST="apps/hako-alloc-bounded-recommit-policy-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-246-M202-BOUNDED-RECOMMIT-POLICY.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
OWNER="lang/src/hako_alloc/memory/purge_bounded_recommit_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_bounded_recommit_policy_guard.sh"

echo "[$TAG] checking M202 bounded recommit policy"

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
  "$MODULE" \
  "$MEMORY_README" \
  "$PROOF_MANIFEST" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M202 card must be complete"
guard_expect_in_file "$TAG" 'M202 status:' "$PLAN" "mimalloc plan must record M202 status"
guard_expect_in_file "$TAG" '`293x-246`' "$PHASE_README" "phase README must list M202 row"
guard_expect_in_file "$TAG" '\[x\] `293x-246`' "$TASKBOARD" "taskboard must mark M202 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M202 guard"
guard_expect_in_file "$TAG" 'id = "M202"' "$PROOF_MANIFEST" "proof app manifest must list M202"

guard_expect_in_file "$TAG" 'memory.purge_bounded_recommit_box = "memory/purge_bounded_recommit_box.hako"' "$MODULE" "hako_alloc module must export bounded recommit policy"
guard_expect_in_file "$TAG" 'box HakoAllocBoundedRecommitPolicy' "$OWNER" "bounded recommit policy box must exist"
guard_expect_in_file "$TAG" 'attemptRecommit' "$OWNER" "bounded recommit policy must expose attemptRecommit"
guard_expect_in_file "$TAG" 'source.commitPage' "$OWNER" "M202 may only call a caller-provided commit source"
guard_expect_in_file "$TAG" 'marker_cleared = 0' "$OWNER" "M202 report must keep marker clearing closed"
guard_expect_in_file "$TAG" 'unreserve_executed = 0' "$OWNER" "M202 report must keep unreserve closed"
guard_expect_in_file "$TAG" 'os_release_executed = 0' "$OWNER" "M202 report must keep OS release closed"
guard_expect_in_file "$TAG" 'owns M202 bounded recommit policy' "$MEMORY_README" "memory README must define M202 owner"

if rg -n 'HakoAllocPageSourcePolicy|OsVmCoreBox|reservePage[[:space:]]*\(|decommitPage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|heap\.|addBackedPage[[:space:]]*\(|addFreshPage[[:space:]]*\(' \
  "$OWNER" >/tmp/"$TAG".direct_leak 2>&1; then
  echo "[$TAG] ERROR: M202 bounded recommit policy must not call page-source APIs or mutate heap/page state" >&2
  cat /tmp/"$TAG".direct_leak >&2
  rm -f /tmp/"$TAG".direct_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_leak

if rg -n 'marked_page_ids\.|\.push\(|marker\.marked_page_ids|clearMarked|clearMarker|remove' "$OWNER" >/tmp/"$TAG".marker_mutation 2>&1; then
  echo "[$TAG] ERROR: M202 bounded recommit policy must not mutate or clear decommit marker state" >&2
  cat /tmp/"$TAG".marker_mutation >&2
  rm -f /tmp/"$TAG".marker_mutation
  exit 1
fi
rm -f /tmp/"$TAG".marker_mutation

if rg -n 'hako-alloc-bounded-recommit-policy-proof|HakoAllocBoundedRecommitPolicy' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M202 app/policy matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_bounded_recommit_policy_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M202 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m202_hako_alloc_recommit.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m202.mir.json"
exe_out="$tmp_dir/m202.exe"
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
    "HakoAllocBoundedRecommitPolicy.attemptRecommit/4",
    "HakoAllocFakeRecommitSource.commitPage/2",
    "HakoAllocDecommittedPageReusePrecondition.classifyHeapPage/3",
    "HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
policy = plans.get("HakoAllocBoundedRecommitPolicy")
if policy is None:
    raise SystemExit("missing typed object plan: HakoAllocBoundedRecommitPolicy")
fields = {
    field.get("name"): field
    for field in policy.get("fields", [])
}
for name in ("max_recommit_bytes", "attempt_count", "blocked_count", "recommit_attempt_count", "recommit_success_count", "source_reject_count"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad bounded recommit policy field {name}: {field}")

print("[m202-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-bounded-recommit-policy-proof' "$run_log"
rg -F -q 'before=0,1,0,16' "$run_log"
rg -F -q 'no_need=3,1,0,0,0,0' "$run_log"
rg -F -q 'release=1' "$run_log"
rg -F -q 'decommit=0,1,1' "$run_log"
rg -F -q 'decision=1,1,1,16' "$run_log"
rg -F -q 'success=0,0,1,1,0,0,16' "$run_log"
rg -F -q 'too_big=6,1,0,0,8' "$run_log"
rg -F -q 'fail=7,1,1,0,1' "$run_log"
rg -F -q 'missing=2,1,1,0,0' "$run_log"
rg -F -q 'policy=4,3,2,1,1' "$run_log"
rg -F -q 'limit=1,1,0,0' "$run_log"
rg -F -q 'source=2,1,1,1,16' "$run_log"
rg -F -q 'precondition=3,1,2,1' "$run_log"
rg -F -q 'guard=1,1,1' "$run_log"
rg -F -q 'heap=0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

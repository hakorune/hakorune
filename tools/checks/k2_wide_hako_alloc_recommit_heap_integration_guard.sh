#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-recommit-heap-integration"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-recommit-heap-integration-proof/main.hako"
APP_README="apps/hako-alloc-recommit-heap-integration-proof/README.md"
APP_TEST="apps/hako-alloc-recommit-heap-integration-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-249-M205-RECOMMIT-HEAP-INTEGRATION.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
OWNER="lang/src/hako_alloc/memory/purge_recommit_heap_integration_box.hako"
PAGE="lang/src/hako_alloc/memory/page_box.hako"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
PROOF_MANIFEST="tools/checks/proof_apps.toml"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_recommit_heap_integration_guard.sh"

echo "[$TAG] checking M205 recommit heap integration"

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
  "$PAGE" \
  "$MODULE" \
  "$MEMORY_README" \
  "$PROOF_MANIFEST" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M205 card must be complete"
guard_expect_in_file "$TAG" 'M205 status:' "$PLAN" "mimalloc plan must record M205 status"
guard_expect_in_file "$TAG" '`293x-249`' "$PHASE_README" "phase README must list M205 row"
guard_expect_in_file "$TAG" '\[x\] `293x-249`' "$TASKBOARD" "taskboard must mark M205 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M205 guard"
guard_expect_in_file "$TAG" 'id = "M205"' "$PROOF_MANIFEST" "proof app manifest must list M205"

guard_expect_in_file "$TAG" 'memory.purge_recommit_heap_integration_box = "memory/purge_recommit_heap_integration_box.hako"' "$MODULE" "hako_alloc module must export recommit heap integration"
guard_expect_in_file "$TAG" 'box HakoAllocRecommitHeapIntegration' "$OWNER" "recommit heap integration box must exist"
guard_expect_in_file "$TAG" 'page\.reactivate' "$OWNER" "M205 must reactivate page-local state only after recommit"
guard_expect_in_file "$TAG" 'reactivate' "$PAGE" "page model must expose reactivation seam"
guard_expect_in_file "$TAG" 'M205 recommit heap' "$MEMORY_README" "memory README must define M205 owner"
guard_expect_in_file "$TAG" 'Status codes:' "$OWNER" "M205 owner must document report status codes"
guard_expect_in_file "$TAG" '3 marker transition failed after recommit source execution' "$OWNER" "M205 owner must document partial source-success blocked status"
guard_expect_in_file "$TAG" 'success_count only means marker transition' "$OWNER" "M205 owner must document blocked_count/success_count semantics"

if rg -n 'reservePage[[:space:]]*\(|decommitPage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|addBackedPage[[:space:]]*\(|addFreshPage[[:space:]]*\(' \
  "$OWNER" >/tmp/"$TAG".direct_leak 2>&1; then
  echo "[$TAG] ERROR: M205 recommit heap integration must not source pages or call decommit/unreserve/OS release" >&2
  cat /tmp/"$TAG".direct_leak >&2
  rm -f /tmp/"$TAG".direct_leak
  exit 1
fi
rm -f /tmp/"$TAG".direct_leak

if rg -n 'hako-alloc-recommit-heap-integration-proof|HakoAllocRecommitHeapIntegration' \
  lang/c-abi/shims >/tmp/"$TAG".inc_leak 2>&1; then
  echo "[$TAG] ERROR: M205 app/integration matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc_leak >&2
  rm -f /tmp/"$TAG".inc_leak
  exit 1
fi
rm -f /tmp/"$TAG".inc_leak

if rg -n 'k2_wide_hako_alloc_recommit_heap_integration_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M205 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m205_hako_alloc_recommit.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m205.mir.json"
exe_out="$tmp_dir/m205.exe"
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
    "HakoAllocRecommitHeapIntegration.attemptHeapPage/3",
    "HakoAllocPageModel.reactivate/0",
    "HakoAllocPurgeDecommitStateMarker.markIfRecommitted/2",
    "HakoAllocBoundedRecommitPolicy.attemptRecommit/4",
    "HakoAllocPageSourceRecommitAdapter.commitPage/2",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
page = plans.get("HakoAllocPageModel")
if page is None:
    raise SystemExit("missing typed object plan: HakoAllocPageModel")
fields = {
    field.get("name"): field
    for field in page.get("fields", [])
}
for name in ("reactivate_count", "reactivate_reject_count"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad page reactivation field {name}: {field}")

print("[m205-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-recommit-heap-integration-proof' "$run_log"
rg -F -q 'release=1' "$run_log"
rg -F -q 'decommit=0,1,1' "$run_log"
rg -F -q 'before_acquire=-1' "$run_log"
rg -F -q 'recommit=0,0,1,1,1,1,16' "$run_log"
rg -F -q 'selected=0,0' "$run_log"
rg -F -q 'page=1,0,0,0,1,0' "$run_log"
rg -F -q 'marker=1,1,0' "$run_log"
rg -F -q 'integration=1,1,0,1,1' "$run_log"
rg -F -q 'policy=1,1,0' "$run_log"
rg -F -q 'adapter=1,1,0,16' "$run_log"
rg -F -q 'heap=0,0,0' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

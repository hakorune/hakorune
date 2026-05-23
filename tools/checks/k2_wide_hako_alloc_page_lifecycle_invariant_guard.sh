#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-alloc-page-lifecycle-invariant"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/hako-alloc-page-lifecycle-invariant-proof/main.hako"
APP_README="apps/hako-alloc-page-lifecycle-invariant-proof/README.md"
APP_TEST="apps/hako-alloc-page-lifecycle-invariant-proof/test.sh"
CARD="docs/development/current/main/phases/phase-293x/293x-251-M207-PAGE-LIFECYCLE-INVARIANT-FREEZE.md"
USIZE_SELECTION_CARD="docs/development/current/main/phases/phase-294x/294x-101-HAKO-ALLOC-USIZE-PAGE-LIFECYCLE-OBSERVER-COUNTER-SELECTION.md"
USIZE_CARD="docs/development/current/main/phases/phase-294x/294x-102-HAKO-ALLOC-USIZE-PAGE-LIFECYCLE-OBSERVER-COUNTERS.md"
PLAN="docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md"
PHASE_README="docs/development/current/main/phases/phase-293x/README.md"
TASKBOARD="docs/development/current/main/phases/phase-293x/293x-90-real-app-taskboard.md"
INDEX="docs/tools/check-scripts-index.md"
PROOF_MANIFEST="tools/checks/manifests/proof_apps/hako_alloc_purge_core.toml"
MODULE="lang/src/hako_alloc/hako_module.toml"
MEMORY_README="lang/src/hako_alloc/memory/README.md"
OWNER="lang/src/hako_alloc/memory/page_lifecycle_invariant_box.hako"
DEV_GATE="tools/checks/dev_gate.sh"
ALLOCATOR_GATE="tools/checks/k2_wide_allocator_gate.sh"
SELF_SCRIPT="tools/checks/k2_wide_hako_alloc_page_lifecycle_invariant_guard.sh"

echo "[$TAG] checking M207 page lifecycle invariant freeze"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$CARD" \
  "$USIZE_SELECTION_CARD" \
  "$USIZE_CARD" \
  "$PLAN" \
  "$PHASE_README" \
  "$TASKBOARD" \
  "$INDEX" \
  "$PROOF_MANIFEST" \
  "$MODULE" \
  "$MEMORY_README" \
  "$OWNER" \
  "$DEV_GATE" \
  "$ALLOCATOR_GATE" \
  "$SELF_SCRIPT"

guard_require_exec_files "$TAG" "$APP_TEST" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'Status: Complete' "$CARD" "M207 card must be complete"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_SELECTION_CARD" "294x-101 usize selection card must be landed"
guard_expect_in_file "$TAG" 'Status: Landed' "$USIZE_CARD" "294x-102 usize migration card must be landed"
guard_expect_in_file "$TAG" 'M207 status:' "$PLAN" "mimalloc plan must record M207 status"
guard_expect_in_file "$TAG" '`293x-251`' "$PHASE_README" "phase README must list M207 row"
guard_expect_in_file "$TAG" '\[x\] `293x-251`' "$TASKBOARD" "taskboard must mark M207 complete"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list M207 guard"
guard_expect_in_file "$TAG" 'id = "M207"' "$PROOF_MANIFEST" "proof app manifest must list M207"
guard_expect_in_file "$TAG" 'memory.page_lifecycle_invariant_box = "memory/page_lifecycle_invariant_box.hako"' "$MODULE" "hako_alloc module must export M207 observer"
guard_expect_in_file "$TAG" 'owns M207 page lifecycle invariant' "$MEMORY_README" "memory README must define M207 owner"
guard_expect_in_file "$TAG" 'State codes:' "$OWNER" "M207 owner must document state codes"
guard_expect_in_file "$TAG" 'box HakoAllocPageLifecycleInvariantObserver' "$OWNER" "M207 observer box must exist"
guard_expect_in_file "$TAG" 'observe_count: usize = 0' "$OWNER" "M207 observe counter must be exact usize"
guard_expect_in_file "$TAG" 'missing_count: usize = 0' "$OWNER" "M207 missing counter must be exact usize"
guard_expect_in_file "$TAG" 'active_count: usize = 0' "$OWNER" "M207 active counter must be exact usize"
guard_expect_in_file "$TAG" 'retired_count: usize = 0' "$OWNER" "M207 retired counter must be exact usize"
guard_expect_in_file "$TAG" 'decommitted_count: usize = 0' "$OWNER" "M207 decommitted counter must be exact usize"
guard_expect_in_file "$TAG" 'recommitted_count: usize = 0' "$OWNER" "M207 recommitted counter must be exact usize"
guard_expect_in_file "$TAG" 'last_page_id: i64 = -1' "$OWNER" "M207 last page id must remain signed sentinel"
guard_expect_in_file "$TAG" 'last_state: i64 = 0' "$OWNER" "M207 last state must remain signed state vocabulary"
guard_expect_in_file "$TAG" 'check "m207 page lifecycle"' "$APP" "M207 proof must use labelled check block"

if rg -n 'reservePage[[:space:]]*\(|commitPage[[:space:]]*\(|decommitPage[[:space:]]*\(|unreserve[[:space:]]*\(|releasePage[[:space:]]*\(|attemptHeapPage[[:space:]]*\(|markIf|reactivate[[:space:]]*\(|releaseLocal[[:space:]]*\(|acquire[[:space:]]*\(' \
  "$OWNER" >/tmp/"$TAG".mutation 2>&1; then
  echo "[$TAG] ERROR: M207 lifecycle observer must stay read-only" >&2
  cat /tmp/"$TAG".mutation >&2
  rm -f /tmp/"$TAG".mutation
  exit 1
fi
rm -f /tmp/"$TAG".mutation

if rg -n 'page-lifecycle-invariant|HakoAllocPageLifecycleInvariant' \
  lang/c-abi/shims >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: M207 lifecycle matcher leaked into .inc" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

if rg -n 'k2_wide_hako_alloc_page_lifecycle_invariant_guard\.sh' \
  "$DEV_GATE" "$ALLOCATOR_GATE" >/tmp/"$TAG".gate_growth 2>&1; then
  echo "[$TAG] ERROR: M207 guard must stay local-run/index-listed by default" >&2
  cat /tmp/"$TAG".gate_growth >&2
  rm -f /tmp/"$TAG".gate_growth
  exit 1
fi
rm -f /tmp/"$TAG".gate_growth

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_m207_page_lifecycle.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/m207.mir.json"
exe_out="$tmp_dir/m207.exe"
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
    "HakoAllocPageLifecycleInvariantObserver.observeHeapPage/3",
    "HakoAllocPurgeStateAwareDecommitGuard.attemptHeapPage/2",
    "HakoAllocRecommitHeapIntegration.attemptHeapPage/3",
    "HakoAllocPageModel.acquire/1",
    "HakoAllocPageModel.releaseLocal/1",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {
    plan.get("box_name"): plan
    for plan in data.get("typed_object_plans", [])
}
report = plans.get("HakoAllocPageLifecycleInvariantReport")
if report is None:
    raise SystemExit("missing typed object plan: HakoAllocPageLifecycleInvariantReport")
observer = plans.get("HakoAllocPageLifecycleInvariantObserver")
if observer is None:
    raise SystemExit("missing typed object plan: HakoAllocPageLifecycleInvariantObserver")
fields = {
    field.get("name"): field
    for field in report.get("fields", [])
}
for name in ("state", "acquire_allowed", "decommit_candidate", "recommit_required", "marked_generations", "recommitted_generations"):
    field = fields.get(name)
    if field is None or field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad lifecycle report field {name}: {field}")

observer_fields = {
    field.get("name"): field
    for field in observer.get("fields", [])
}
for name in (
    "observe_count",
    "missing_count",
    "active_count",
    "retired_count",
    "decommitted_count",
    "recommitted_count",
):
    field = observer_fields.get(name)
    if field is None:
        raise SystemExit(f"missing lifecycle observer field: {name}")
    if field.get("declared_type") != "usize" or field.get("storage") != "usize":
        raise SystemExit(f"bad lifecycle observer counter {name}: {field}")
for name in ("last_page_id", "last_state"):
    field = observer_fields.get(name)
    if field is None:
        raise SystemExit(f"missing lifecycle observer field: {name}")
    if field.get("declared_type") != "i64" or field.get("storage") != "i64":
        raise SystemExit(f"bad lifecycle observer signed field {name}: {field}")

print("[m207-mir-json] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_reserve_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_commit_bytes_i64_emit' "$build_log"
rg -F -q 'mir_call_hako_osvm_decommit_bytes_i64_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'hako-alloc-page-lifecycle-invariant-proof' "$run_log"
rg -F -q 'states=1,2,3,4,4' "$run_log"
rg -F -q 'active_rejects=2,0,2,0' "$run_log"
rg -F -q 'decommit=0,1,1,0,-1' "$run_log"
rg -F -q 'recommit=0,1,1,0,1,1' "$run_log"
rg -F -q 'generations=2,2,0,1' "$run_log"
rg -F -q 'observer=5,1,1,1,2,4' "$run_log"
rg -F -q 'page=0,1,0,0,2,2,1' "$run_log"
rg -F -q 'guards=4,2,2,1,2,3,2,1' "$run_log"
rg -F -q 'check=1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

echo "[$TAG] ok"

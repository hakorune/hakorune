#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="phase295x-mimalloc-remote-free-minimum-benchmark-selection"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP="apps/mimalloc-remote-free-minimum-benchmark-selection-proof/main.hako"
APP_README="apps/mimalloc-remote-free-minimum-benchmark-selection-proof/README.md"
APP_TEST="apps/mimalloc-remote-free-minimum-benchmark-selection-proof/test.sh"
FACADE="lang/src/hako_alloc/memory/allocator_facade_box.hako"
INTEGRATION="lang/src/hako_alloc/memory/remote_free_page_integration_box.hako"
CARD_244="docs/development/current/main/phases/phase-295x/295x-244-MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-SELECTION.md"

echo "[$TAG] running remote-free minimum benchmark selection proof"

guard_require_files \
  "$TAG" \
  "$APP" \
  "$APP_README" \
  "$APP_TEST" \
  "$FACADE" \
  "$INTEGRATION" \
  "$CARD_244"

guard_expect_in_file "$TAG" 'benchmark_pack=remote-free-minimum-v0' "$APP_README" "selection proof README must pin the benchmark pack"
guard_expect_in_file "$TAG" 'backend_scope=exact-exe-first' "$APP_README" "selection proof README must pin the backend scope"
guard_expect_in_file "$TAG" 'This catches a changed \.hako contract that the existing remote-free evidence guard cannot observe\.' "$APP_README" "selection proof README must record the primary guard reason"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-RUN-295X-002' "$CARD_244" "244 card must select the benchmark run row next"
guard_expect_in_file "$TAG" 'captureLocalAllocFree\(\)' "$APP" "selection proof must capture the local alloc/free workload"
guard_expect_in_file "$TAG" 'capturePublishOnly\(head_cell, block_a, block_b\)' "$APP" "selection proof must capture the publish-only workload"
guard_expect_in_file "$TAG" 'captureCollectOnly\(head_cell, block_a, block_b\)' "$APP" "selection proof must capture the collect-only workload"
guard_expect_in_file "$TAG" 'capturePublishCollectCycle\(head_cell, block_a, block_b\)' "$APP" "selection proof must capture the publish-collect cycle workload"
guard_expect_in_file "$TAG" 'summaryLine\(\)' "$APP" "selection proof must render the final summary through the report capsule"

if rg -n 'provider_active=1|replacement_active=1|winner_claim=1|global_allocator|GlobalAlloc' \
  "$APP" "$APP_README" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: benchmark selection row leaked closed seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_remote_free_min_bench.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/min_bench.mir.json"
exe_out="$tmp_dir/min_bench.exe"
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
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.captureLocalAllocFree/0",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.capturePublishOnly/3",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.captureCollectOnly/3",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.capturePublishCollectCycle/3",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.benchmarkPackLine/0",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.backendScopeLine/0",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.workloadCountLine/0",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.workload0Line/0",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.workload1Line/0",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.workload2Line/0",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.workload3Line/0",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.policyLine/0",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.stopLine/0",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.selectionLine/0",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.summaryLine/0",
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.ok/0",
    "HakoAllocProductionFacade.allocate/1",
    "HakoAllocProductionFacade.release/1",
    "HakoAllocProductionFacade.outstandingBlocks/0",
    "HakoAllocProductionFacade.remotePagePort/5",
    "HakoAllocProductionFacade.remotePageAcquire/2",
    "HakoAllocProductionFacade.remotePagePublishPort/4",
    "HakoAllocProductionFacade.remotePageCollectPendingPort/1",
    "HakoAllocProductionFacade.remotePageCollectOnePort/1",
    "HakoAllocProductionFacade.remotePagePortSnapshot/1",
    "HakoAllocRemoteFreePageExerciseReport.captureFacadeExercise/4",
}
missing = sorted(name for name in required if functions.get(name) is None)
if missing:
    raise SystemExit(f"missing functions: {missing}")

plans = {plan.get("box_name"): plan for plan in data.get("typed_object_plans", [])}
for box_name in (
    "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport",
    "HakoAllocRemoteFreePageExerciseReport",
    "HakoAllocRemoteFreePagePortSnapshot",
):
    if plans.get(box_name) is None:
        raise SystemExit(f"missing typed object plan: {box_name}")

def iter_calls(fn):
    for block in fn.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            yield inst.get("mir_call", {}).get("callee", {})

def require_method(owner_name, box_name, name):
    for callee in iter_calls(functions[owner_name]):
        if (
            callee.get("type") == "Method"
            and callee.get("box_name") == box_name
            and callee.get("name") == name
        ):
            return
    raise SystemExit(f"missing method call in {owner_name}: {box_name}.{name}")

for method in (
    "captureLocalAllocFree",
    "capturePublishOnly",
    "captureCollectOnly",
    "capturePublishCollectCycle",
    "benchmarkPackLine",
    "backendScopeLine",
    "workloadCountLine",
    "workload0Line",
    "workload1Line",
    "workload2Line",
    "workload3Line",
    "policyLine",
    "stopLine",
    "selectionLine",
    "summaryLine",
    "ok",
):
    require_method("main", "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport", method)

for owner_name, box_name, method in (
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.captureLocalAllocFree/0", "HakoAllocProductionFacade", "allocate"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.captureLocalAllocFree/0", "HakoAllocProductionFacade", "release"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.captureLocalAllocFree/0", "HakoAllocProductionFacade", "outstandingBlocks"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.capturePublishOnly/3", "HakoAllocProductionFacade", "remotePagePort"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.capturePublishOnly/3", "HakoAllocProductionFacade", "remotePageAcquire"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.capturePublishOnly/3", "HakoAllocProductionFacade", "remotePagePublishPort"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.capturePublishOnly/3", "HakoAllocProductionFacade", "remotePagePortSnapshot"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.captureCollectOnly/3", "HakoAllocProductionFacade", "remotePagePort"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.captureCollectOnly/3", "HakoAllocProductionFacade", "remotePageAcquire"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.captureCollectOnly/3", "HakoAllocProductionFacade", "remotePagePublishPort"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.captureCollectOnly/3", "HakoAllocProductionFacade", "remotePageCollectPendingPort"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.captureCollectOnly/3", "HakoAllocProductionFacade", "remotePageCollectOnePort"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.captureCollectOnly/3", "HakoAllocProductionFacade", "remotePagePortSnapshot"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.capturePublishCollectCycle/3", "HakoAllocRemoteFreePageExerciseReport", "captureFacadeExercise"),
    ("HakoAllocRemoteFreeMinimumBenchmarkSelectionReport.summaryLine/0", "HakoAllocRemoteFreeMinimumBenchmarkSelectionReport", "ok"),
):
    require_method(owner_name, box_name, method)

print("[remote-free-minimum-benchmark-selection-mir] ok")
PY

pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$APP" "$mir_json" "$exe_out" "$build_log"
pure_first_guard_assert_clean_build_log "$TAG" "$build_log"

rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"

pure_first_guard_run_exe "$TAG" "$exe_out" "$run_log"

rg -F -q 'mimalloc-remote-free-minimum-benchmark-selection-proof' "$run_log"
rg -F -q 'benchmark_pack=remote-free-minimum-v0' "$run_log"
rg -F -q 'backend_scope=exact-exe-first' "$run_log"
rg -F -q 'workload_count=4' "$run_log"
rg -F -q 'workload0=local-alloc-free-cycle-v0' "$run_log"
rg -F -q 'workload1=remote-free-publish-only-v0' "$run_log"
rg -F -q 'workload2=remote-free-collect-only-v0' "$run_log"
rg -F -q 'workload3=remote-free-publish-collect-cycle-v0' "$run_log"
rg -F -q 'policy=warmup:1,samples:5,summary:min,median,max' "$run_log"
rg -F -q 'stop_line=provider:0,replacement:0,winner:0' "$run_log"
rg -F -q 'selection=1,1,1,1' "$run_log"
rg -F -q 'summary=ok' "$run_log"

cat "$run_log"

echo "[$TAG] ok"

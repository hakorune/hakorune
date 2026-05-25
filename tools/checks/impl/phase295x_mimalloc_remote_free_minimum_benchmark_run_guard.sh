#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../../.." && pwd)"
TAG="phase295x-mimalloc-remote-free-minimum-benchmark-run"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/pure_first_exe_guard.sh"

APP_DIR="apps/mimalloc-remote-free-minimum-benchmark-run-proof"
APP_LOCAL="$APP_DIR/main.hako"
APP_PUBLISH="$APP_DIR/remote_free_publish_only.hako"
APP_COLLECT="$APP_DIR/remote_free_collect_only.hako"
APP_CYCLE="$APP_DIR/remote_free_publish_collect_cycle.hako"
APP_README="$APP_DIR/README.md"
APP_TEST="$APP_DIR/test.sh"
FACADE="lang/src/hako_alloc/memory/allocator_facade_box.hako"
INTEGRATION="lang/src/hako_alloc/memory/remote_free_page_integration_box.hako"
CARD_244="docs/development/current/main/phases/phase-295x/295x-244-MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-SELECTION.md"
CARD_245="docs/development/current/main/phases/phase-295x/295x-245-MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-RUN.md"

echo "[$TAG] running remote-free minimum benchmark run proof"

guard_require_files \
  "$TAG" \
  "$APP_LOCAL" \
  "$APP_PUBLISH" \
  "$APP_COLLECT" \
  "$APP_CYCLE" \
  "$APP_README" \
  "$APP_TEST" \
  "$FACADE" \
  "$INTEGRATION" \
  "$CARD_244" \
  "$CARD_245"

guard_expect_in_file "$TAG" 'operation_repeat=128' "$APP_README" "run proof README must pin the repeat count"
guard_expect_in_file "$TAG" 'timing_repeat_kind=process-invocation-v0' "$APP_README" "run proof README must pin the timing repeat kind"
guard_expect_in_file "$TAG" 'sample_count=5' "$APP_README" "run proof README must pin the sample count"
guard_expect_in_file "$TAG" 'warmup_count=1' "$APP_README" "run proof README must pin the warmup count"
guard_expect_in_file "$TAG" 'main.hako' "$APP_README" "run proof README must list the local alloc/free app"
guard_expect_in_file "$TAG" 'remote_free_publish_only.hako' "$APP_README" "run proof README must list the publish-only app"
guard_expect_in_file "$TAG" 'remote_free_collect_only.hako' "$APP_README" "run proof README must list the collect-only app"
guard_expect_in_file "$TAG" 'remote_free_publish_collect_cycle.hako' "$APP_README" "run proof README must list the publish-collect app"
guard_expect_in_file "$TAG" 'This catches a changed \.hako contract that the benchmark selection proof cannot observe\.' "$APP_README" "run proof README must record the primary guard reason"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-RUN-295X-002' "$CARD_244" "244 card must select the run row next"
guard_expect_in_file "$TAG" 'MIMALLOC-COMPARISON-REMOTE-FREE-BACKEND-SPLIT-SELECTION-295X-002' "$CARD_245" "245 card must select the backend split row next"
guard_expect_in_file "$TAG" 'dedicated no-arg entrypoints' "$CARD_245" "245 card must document the no-arg proof entrypoints"
guard_expect_in_file "$TAG" 'local-alloc-free-cycle-v0' "$APP_LOCAL" "local app must pin workload id"
guard_expect_in_file "$TAG" 'remote-free-publish-only-v0' "$APP_PUBLISH" "publish app must pin workload id"
guard_expect_in_file "$TAG" 'remote-free-collect-only-v0' "$APP_COLLECT" "collect app must pin workload id"
guard_expect_in_file "$TAG" 'remote-free-publish-collect-cycle-v0' "$APP_CYCLE" "cycle app must pin workload id"

if rg -n 'provider_active=1|replacement_active=1|winner_claim=1|global_allocator|GlobalAlloc' \
  "$APP_LOCAL" "$APP_PUBLISH" "$APP_COLLECT" "$APP_CYCLE" "$APP_README" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: benchmark run row leaked closed seams" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

pure_first_guard_build_toolchain

tmp_dir="$(mktemp -d /tmp/hakorune_remote_free_min_bench_run.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

run_once() {
  local exe_out="$1"
  local run_log="$2"
  "$exe_out" >"$run_log" 2>&1
}

check_output_contract() {
  local run_log="$1"
  local workload="$2"
  rg -F -q 'mimalloc-remote-free-minimum-benchmark-run-proof' "$run_log"
  rg -F -q 'output_contract=mimalloc-comparison-remote-free-minimum-benchmark-run-v0' "$run_log"
  rg -F -q 'benchmark_pack=remote-free-minimum-v0' "$run_log"
  rg -F -q 'backend_scope=exact-exe-first' "$run_log"
  rg -F -q 'timing_repeat_kind=process-invocation-v0' "$run_log"
  rg -F -q 'operation_repeat=128' "$run_log"
  rg -F -q 'warmup_count=1' "$run_log"
  rg -F -q 'sample_count=5' "$run_log"
  rg -F -q "workload_id=$workload" "$run_log"
  rg -F -q 'completed_ops=128' "$run_log"
  rg -F -q 'stop_line=provider:0,replacement:0,winner:0' "$run_log"
  rg -F -q 'summary=ok' "$run_log"
}

build_and_measure() {
  local app="$1"
  local workload="$2"
  local label="$3"
  local mir_json="$tmp_dir/${label}.mir.json"
  local exe_out="$tmp_dir/${label}.exe"
  local build_log="$tmp_dir/${label}.build.log"
  local warmup_log="$tmp_dir/${label}.warmup.log"
  local samples=()

  pure_first_guard_emit_mir "$ROOT_DIR" "$app" "$mir_json"
  pure_first_guard_build_exe "$TAG" "$ROOT_DIR" "$app" "$mir_json" "$exe_out" "$build_log"
  pure_first_guard_assert_clean_build_log "$TAG" "$build_log"
  rg -F -q 'mir_call_user_box_method_same_module_emit' "$build_log"

  case "$label" in
    local_alloc_free)
      rg -F -q 'symbol=HakoAllocProductionFacade.allocate/1' "$build_log"
      rg -F -q 'symbol=HakoAllocProductionFacade.release/1' "$build_log"
      rg -F -q 'symbol=HakoAllocProductionFacade.outstandingBlocks/0' "$build_log"
      ;;
    remote_free_publish_only)
      rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
      rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"
      rg -F -q 'symbol=HakoAllocProductionFacade.remotePagePort/5' "$build_log"
      rg -F -q 'symbol=HakoAllocProductionFacade.remotePageAcquire/2' "$build_log"
      rg -F -q 'symbol=HakoAllocProductionFacade.remotePagePublishPort/4' "$build_log"
      rg -F -q 'symbol=HakoAllocProductionFacade.remotePagePortSnapshot/1' "$build_log"
      ;;
    remote_free_collect_only)
      rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
      rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"
      rg -F -q 'symbol=HakoAllocProductionFacade.remotePageCollectPendingPort/1' "$build_log"
      rg -F -q 'symbol=HakoAllocProductionFacade.remotePageCollectOnePort/1' "$build_log"
      rg -F -q 'symbol=HakoAllocProductionFacade.remotePagePortSnapshot/1' "$build_log"
      ;;
    remote_free_publish_collect_cycle)
      rg -F -q 'mir_call_hako_mem_alloc_emit' "$build_log"
      rg -F -q 'mir_call_hako_mem_free_emit' "$build_log"
      rg -F -q 'symbol=HakoAllocRemoteFreePageExerciseReport.captureFacadeExercise/4' "$build_log"
      rg -F -q 'symbol=HakoAllocRemoteFreePageExerciseReport.ok/0' "$build_log"
      ;;
  esac

  run_once "$exe_out" "$warmup_log"
  check_output_contract "$warmup_log" "$workload"

  for sample_idx in 0 1 2 3 4; do
    local run_log="$tmp_dir/${label}.sample${sample_idx}.log"
    local start_ms end_ms elapsed_ms
    start_ms="$(date +%s%3N)"
    run_once "$exe_out" "$run_log"
    end_ms="$(date +%s%3N)"
    elapsed_ms="$((end_ms - start_ms))"
    samples+=("$elapsed_ms")
    check_output_contract "$run_log" "$workload"
  done

  mapfile -t sorted < <(printf '%s\n' "${samples[@]}" | sort -n)
  local min_ms="${sorted[0]}"
  local median_ms="${sorted[2]}"
  local max_ms="${sorted[4]}"
  printf '%s_ms=%s,%s,%s\n' "$label" "$min_ms" "$median_ms" "$max_ms"
}

{
  echo "mimalloc-remote-free-minimum-benchmark-run-proof"
  echo "benchmark_pack=remote-free-minimum-v0"
  echo "backend_scope=exact-exe-first"
  echo "timing_repeat_kind=process-invocation-v0"
  echo "operation_repeat=128"
  echo "warmup_count=1"
  echo "sample_count=5"
  echo "stop_line=provider:0,replacement:0,winner:0"
  build_and_measure "$APP_LOCAL" "local-alloc-free-cycle-v0" "local_alloc_free"
  build_and_measure "$APP_PUBLISH" "remote-free-publish-only-v0" "remote_free_publish_only"
  build_and_measure "$APP_COLLECT" "remote-free-collect-only-v0" "remote_free_collect_only"
  build_and_measure "$APP_CYCLE" "remote-free-publish-collect-cycle-v0" "remote_free_publish_collect_cycle"
  echo "summary=ok"
} >"$tmp_dir/report.log"

python3 - "$tmp_dir/report.log" <<'PY'
import sys

path = sys.argv[1]
lines = {}
with open(path, encoding="utf-8") as fh:
    for raw in fh:
        line = raw.strip()
        if "=" in line:
            key, value = line.split("=", 1)
            lines[key] = value

for key in (
    "local_alloc_free_ms",
    "remote_free_publish_only_ms",
    "remote_free_collect_only_ms",
    "remote_free_publish_collect_cycle_ms",
):
    values = [int(part) for part in lines[key].split(",")]
    if len(values) != 3:
        raise SystemExit(f"bad sample summary for {key}: {lines[key]}")
    if not (values[0] <= values[1] <= values[2]):
        raise SystemExit(f"non-monotonic summary for {key}: {values}")

print("[remote-free-minimum-benchmark-run-report] ok")
PY

cat "$tmp_dir/report.log"

echo "[$TAG] ok"

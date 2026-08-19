#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-meso-bench-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
BENCH="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_bench.c"
REFERENCE="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_meso_reference.c"
VALIDATOR="$ROOT_DIR/tools/perf/s6c_pinned_corridor_meso_bench.py"
PAIRED_PLAN="$ROOT_DIR/tools/perf/s6c_paired_wallclock_plan.py"
PAIRED_BATCH="$ROOT_DIR/tools/perf/s6c_paired_wallclock_batch.py"
PAIRED_STORE="$ROOT_DIR/tools/perf/s6c_paired_wallclock_batch_store.py"
PAIRED_HARNESS="$ROOT_DIR/tools/perf/s6c_paired_wallclock_harness.py"
SMOKE="$ROOT_DIR/tools/checks/s6c_pinned_corridor_meso_bench_smoke.sh"
COUNTER_GUARD="$ROOT_DIR/tools/checks/s6c_native_hwcounter_guard.sh"
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$BENCH" "$REFERENCE" "$VALIDATOR" "$PAIRED_PLAN" \
  "$PAIRED_BATCH" "$PAIRED_HARNESS" "$SMOKE" "$COUNTER_GUARD"
guard_require_files "$TAG" "$PAIRED_STORE"
count_fixed() {
  local needle="$1"
  shift
  (rg -F -o -- "$needle" "$@" || true) | wc -l | tr -d '[:space:]'
}
for needle in CLOCK_MONOTONIC_RAW 'warmup < 10' 'sample < 51' 30000000 \
  hako_text_formal_residence_enter_v1 hako_text_formal_residence_finish_or_abort_v1; do
  [[ "$(count_fixed "$needle" "$BENCH")" -ge 1 ]] || guard_fail "$TAG" "benchmark control missing: $needle"
done
for needle in 32 256 4096 1048576 ascii width2 width3 width4 mixed first middle last miss; do
  [[ "$(count_fixed "$needle" "$BENCH")" -ge 1 ]] || guard_fail "$TAG" "meso corpus missing: $needle"
done
for needle in taskset clang-18 -O3 -fno-lto promotion-test-support address_mod_64 body_sha256; do
  [[ "$(count_fixed "$needle" "$SMOKE")" -ge 1 ]] || guard_fail "$TAG" "smoke control missing: $needle"
done
for needle in robust-valid.csv 'post-warm calibration' 30000000 60000000; do
  [[ "$(count_fixed "$needle" "$SMOKE")" -ge 1 ]] || \
    guard_fail "$TAG" "calibration smoke contract missing: $needle"
done
for needle in 1.15 nearest_rank promotion-evidence-only gated_4k_plus_max_p50 alignment-manifest; do
  [[ "$(count_fixed "$needle" "$VALIDATOR")" -ge 1 ]] || guard_fail "$TAG" "validator contract missing: $needle"
done
for needle in 'PAIR_COUNT = 51' 'BLOCK_COUNT = 3' 'BLOCK_SIZE = 17' \
  retain_all_no_retry_no_outlier_removal development-evidence-only \
  p95_diagnostic order_strata_p50 block_p50 ShortMeasuredArm \
  calibration_target_arm_ns; do
  [[ "$(count_fixed "$needle" "$PAIRED_PLAN")" -ge 1 ]] || \
    guard_fail "$TAG" "paired wall-clock contract missing: $needle"
done
for needle in MANIFEST_SCHEMA incomplete_predecessor confirmatory_development \
  retain_all_completed_pairs_no_replacement native_promotion_authority \
  diagnostic_raw_csv_sha256 wallclock-batch-v3; do
  [[ "$(count_fixed "$needle" "$PAIRED_BATCH")" -ge 1 ]] || \
    guard_fail "$TAG" "paired batch contract missing: $needle"
done
for needle in --robust-case 'argc == 8' 'strlen(argv[5]) != 51' 'sample / 17' \
  'sample % 17' sample_minimum_ns calibration_target_ns; do
  [[ "$(count_fixed "$needle" "$BENCH")" -ge 1 ]] || \
    guard_fail "$TAG" "robust C harness contract missing: $needle"
done
for needle in orders_text parse_case_output taskset wsl_development create_command \
  run_command close-abandoned same_batch_resume_forbidden short_measured_arm; do
  [[ "$(count_fixed "$needle" "$PAIRED_HARNESS")" -ge 1 ]] || \
    guard_fail "$TAG" "paired harness contract missing: $needle"
done
if rg -n -- '--session-index|--aggregate-reports|native_final' "$PAIRED_HARNESS"; then
  guard_fail "$TAG" "retired direct V1/native CLI remains in paired harness"
fi
for needle in create_once exclusive_batch close_abandoned terminal.json \
  meso-bench.frozen controller_interrupted diagnostic.raw.csv; do
  [[ "$(count_fixed "$needle" "$PAIRED_STORE")" -ge 1 ]] || \
    guard_fail "$TAG" "append-only batch store contract missing: $needle"
done
for negative in missing-case short-arm shape-drift threshold-red foreign-outline foreign-alignment; do
  [[ "$(count_fixed "$negative" "$SMOKE")" -ge 1 ]] || guard_fail "$TAG" "negative missing: $negative"
done
if rg -n 'memcmp|builtin|fallback|retry|hako_text_formal|ny_main' "$REFERENCE"; then
  guard_fail "$TAG" "C reference must be direct ptr/len scan only"
fi
for file in "$BENCH" "$REFERENCE" "$VALIDATOR" "$PAIRED_PLAN" "$PAIRED_BATCH" \
  "$PAIRED_STORE" "$PAIRED_HARNESS" "$SMOKE"; do
  lines="$(wc -l <"$file" | tr -d '[:space:]')"
  (( lines < 760 )) || guard_fail "$TAG" "source reached 760-line split trigger: $file=$lines"
done
python3 "$PAIRED_PLAN"
python3 "$PAIRED_BATCH"
python3 "$PAIRED_STORE"
python3 "$PAIRED_HARNESS" self-test
bash "$COUNTER_GUARD"
echo "[$TAG] ok (fixed 80-case paired meso evidence; Residence outside timed region)"

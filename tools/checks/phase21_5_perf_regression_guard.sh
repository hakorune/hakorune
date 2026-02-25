#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BASELINE_FILE="${PERF_REG_GUARD_BASELINE_FILE:-$ROOT_DIR/benchmarks/baselines/phase21_5_perf_guard.latest.json}"
ENTRY_MODE_LIB="${ROOT_DIR}/tools/checks/lib/perf_guard_entry_mode.sh"
APPS_LIB="${ROOT_DIR}/tools/checks/lib/perf_guard_apps.sh"
GUARD_COMMON_LIB="${ROOT_DIR}/tools/checks/lib/guard_common.sh"
source "${ROOT_DIR}/tools/perf/lib/bench_env.sh"

WARMUP="${PERF_REG_GUARD_WARMUP:-1}"
REPEAT="${PERF_REG_GUARD_REPEAT:-1}"
VM_TIMEOUT="${PERF_REG_GUARD_VM_TIMEOUT:-$(perf_vm_timeout_resolve heavy)}"
AOT_MAX_DEGRADE_PCT="${PERF_REG_GUARD_AOT_MAX_DEGRADE_PCT:-15}"
APPS_MAX_DEGRADE_PCT="${PERF_REG_GUARD_APPS_MAX_DEGRADE_PCT:-20}"
PER_APP_MAX_DEGRADE_PCT="${PERF_REG_GUARD_PER_APP_MAX_DEGRADE_PCT:-25}"
APPS_RETRIES="${PERF_REG_GUARD_APPS_RETRIES:-3}"
ENTRY_SOURCE_MAX_DEGRADE_PCT="${PERF_REG_GUARD_ENTRY_SOURCE_MAX_DEGRADE_PCT:-25}"
ENTRY_PREBUILT_MAX_DEGRADE_PCT="${PERF_REG_GUARD_ENTRY_PREBUILT_MAX_DEGRADE_PCT:-25}"
ENTRY_DELTA_MIN_RATIO="${PERF_REG_GUARD_ENTRY_DELTA_MIN_RATIO:-0.50}"
ENTRY_RETRIES="${PERF_REG_GUARD_ENTRY_RETRIES:-3}"
ENTRY_MODE_SAMPLES="${PERF_REG_GUARD_ENTRY_MODE_SAMPLES:-1}"
ENTRY_COMPARE_SCRIPT="${ROOT_DIR}/tools/perf/bench_apps_entry_mode_compare.sh"

tag="phase21_5-perf-regression-guard"

if [[ ! -f "$GUARD_COMMON_LIB" ]]; then
  echo "[$tag] ERROR: guard common lib missing: $GUARD_COMMON_LIB" >&2
  exit 2
fi
source "$GUARD_COMMON_LIB"
guard_require_command "$tag" jq
guard_require_command "$tag" awk
if [[ ! -f "$ENTRY_MODE_LIB" ]]; then
  echo "[$tag] ERROR: entry-mode guard lib missing: $ENTRY_MODE_LIB" >&2
  exit 2
fi
if [[ ! -f "$APPS_LIB" ]]; then
  echo "[$tag] ERROR: apps guard lib missing: $APPS_LIB" >&2
  exit 2
fi
source "$ENTRY_MODE_LIB"
source "$APPS_LIB"

if [[ ! -f "$BASELINE_FILE" ]]; then
  echo "[$tag] ERROR: baseline file missing: $BASELINE_FILE" >&2
  echo "[$tag] hint: seed baseline with:" >&2
  echo "[$tag]   PERF_STABILITY_INCLUDE_MEDIUM=1 PERF_STABILITY_INCLUDE_APPS=1 PERF_STABILITY_WRITE_BASELINE=1 tools/perf/record_baseline_stability_21_5.sh 2 1 1" >&2
  exit 1
fi

baseline_aot="$(jq -r '.numeric_mixed_medium_ny_aot_ms // 0' "$BASELINE_FILE")"
perf_guard_apps_load_baseline "$BASELINE_FILE"
perf_guard_entry_mode_load_baseline "$BASELINE_FILE"
perf_guard_assert_uint_ge "$tag" "numeric_mixed_medium_ny_aot_ms in baseline" "$baseline_aot" 1
perf_guard_apps_validate_baseline "$tag"
perf_guard_entry_mode_validate_baseline "$tag"

BENCH_OUT="$(
  PERF_AOT=1 \
  PERF_SKIP_VM_PREFLIGHT=1 \
  PERF_VM_TIMEOUT="$VM_TIMEOUT" \
  NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  bash "$ROOT_DIR/tools/perf/bench_compare_c_vs_hako.sh" numeric_mixed_medium "$WARMUP" "$REPEAT" 2>&1
)" || {
  echo "[$tag] ERROR: bench_compare failed" >&2
  echo "$BENCH_OUT" >&2
  exit 1
}

aot_line="$(printf '%s\n' "$BENCH_OUT" | grep -E '\[bench\] name=numeric_mixed_medium \(aot\)' | tail -n 1)"
if [[ -z "$aot_line" ]]; then
  echo "[$tag] ERROR: missing numeric_mixed_medium (aot) line" >&2
  echo "$BENCH_OUT" >&2
  exit 1
fi
if [[ "$aot_line" != *"status=ok"* ]]; then
  echo "[$tag] ERROR: AOT status is not ok" >&2
  echo "$aot_line" >&2
  exit 1
fi
current_aot="$(printf '%s\n' "$aot_line" | sed -n 's/.*ny_aot_ms=\([0-9][0-9]*\).*/\1/p')"
if ! [[ "$current_aot" =~ ^[0-9]+$ ]]; then
  echo "[$tag] ERROR: could not parse current AOT ms from line" >&2
  echo "$aot_line" >&2
  exit 1
fi
perf_guard_assert_uint_ge "$tag" "current AOT ms" "$current_aot" 1

perf_guard_apps_collect_current "$tag" "$ROOT_DIR" "$VM_TIMEOUT" "$WARMUP" "$REPEAT" "$APPS_RETRIES"

perf_guard_entry_mode_collect_current "$tag" "$ENTRY_COMPARE_SCRIPT" "$VM_TIMEOUT" "$ENTRY_MODE_SAMPLES" "$WARMUP" "$REPEAT" "$ENTRY_RETRIES"

aot_degrade="$(perf_guard_calc_degrade_pct "$baseline_aot" "$current_aot")"

echo "[$tag] baseline_aot_ms=${baseline_aot} current_aot_ms=${current_aot} degrade_pct=${aot_degrade} limit_pct=${AOT_MAX_DEGRADE_PCT}"
perf_guard_apps_verify_totals "$tag" "$APPS_MAX_DEGRADE_PCT"
perf_guard_assert_max_pct "$tag" "AOT regression exceeded threshold" "$aot_degrade" "$AOT_MAX_DEGRADE_PCT"
perf_guard_entry_mode_verify_and_report "$tag" "$ENTRY_SOURCE_MAX_DEGRADE_PCT" "$ENTRY_PREBUILT_MAX_DEGRADE_PCT" "$ENTRY_DELTA_MIN_RATIO"
perf_guard_apps_verify_per_app_and_hotspot "$tag" "$BASELINE_FILE" "$PER_APP_MAX_DEGRADE_PCT"
perf_guard_entry_mode_print_hotspot "$tag"

echo "[$tag] ok"

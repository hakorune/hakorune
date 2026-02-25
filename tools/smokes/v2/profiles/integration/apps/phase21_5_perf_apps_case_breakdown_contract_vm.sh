#!/bin/bash
# phase21_5_perf_apps_case_breakdown_contract_vm.sh
#
# Contract pin:
# - bench_apps_wallclock JSON must include per-app case breakdown.
# - total_ms must equal sum(cases.*).

set -euo pipefail

SMOKE_NAME="phase21_5_perf_apps_case_breakdown_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/perf_apps_contract.sh"
require_env || exit 2
APP_BENCH="$NYASH_ROOT/tools/perf/bench_apps_wallclock.sh"
source "$NYASH_ROOT/tools/perf/lib/apps_wallclock_cases.sh"

if [ ! -f "$APP_BENCH" ]; then
  test_fail "$SMOKE_NAME: app bench script missing: $APP_BENCH"
  exit 2
fi

OUT="$(
  PERF_APPS_OUTPUT=json \
  PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
  bash "$APP_BENCH" "${PERF_APPS_WARMUP:-1}" "${PERF_APPS_REPEAT:-1}" vm 2>&1
)" || {
  echo "$OUT"
  test_fail "$SMOKE_NAME: app bench command failed"
  exit 1
}

perf_apps_require_json "$SMOKE_NAME" "$OUT" || exit 1

backend="$(perf_apps_json_get "$OUT" '.backend // ""')"
total_ms="$(perf_apps_json_get "$OUT" '.total_ms // 0')"
cases_count="$(perf_apps_json_get "$OUT" '(.cases // {} | keys | length)')"
sum_cases="$(perf_apps_json_get "$OUT" '(.cases // {} | to_entries | map(.value) | add // 0)')"
hotspot_metric="$(perf_apps_json_get "$OUT" '.hotspot.metric // ""')"
hotspot_case="$(perf_apps_json_get "$OUT" '.hotspot.case // ""')"
hotspot_ms="$(perf_apps_json_get "$OUT" '.hotspot.ms // -1')"
expected_count="${#APPS_WALLCLOCK_CASE_NAMES[@]}"

perf_apps_assert_backend_vm "$SMOKE_NAME" "$backend" "$OUT" || exit 1
perf_apps_assert_positive_uint "$SMOKE_NAME" "total_ms" "$total_ms" "$OUT" || exit 1
perf_apps_assert_uint "$SMOKE_NAME" "cases_count" "$cases_count" "$OUT" || exit 1
if [ "$cases_count" -ne "$expected_count" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: invalid case count: ${cases_count} (expected ${expected_count})"
  exit 1
fi
perf_apps_assert_eq_uint "$SMOKE_NAME" "total_ms_vs_sum(cases)" "$total_ms" "$sum_cases" "$OUT" || exit 1
if [ "$hotspot_metric" != "raw" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: hotspot.metric must be raw (got: $hotspot_metric)"
  exit 1
fi
perf_apps_assert_positive_uint "$SMOKE_NAME" "hotspot.ms" "$hotspot_ms" "$OUT" || exit 1
perf_apps_assert_case_name_in_object "$SMOKE_NAME" "hotspot.case" "$hotspot_case" "$OUT" '.cases // {}' || exit 1

for case_name in "${APPS_WALLCLOCK_CASE_NAMES[@]}"; do
  case_ms="$(perf_apps_json_get "$OUT" ".cases[\"${case_name}\"] // 0")"
  perf_apps_assert_positive_uint "$SMOKE_NAME" "cases.${case_name}" "$case_ms" "$OUT" || exit 1
done

hotspot_case_ms="$(perf_apps_json_get "$OUT" ".cases[\"${hotspot_case}\"] // -1")"
perf_apps_assert_eq_uint "$SMOKE_NAME" "hotspot.case_vs_hotspot.ms" "$hotspot_case_ms" "$hotspot_ms" "$OUT" || exit 1

log_info "$SMOKE_NAME: total_ms=${total_ms} cases=${cases_count}"
test_pass "$SMOKE_NAME"

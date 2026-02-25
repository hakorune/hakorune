#!/bin/bash
# phase21_5_perf_apps_entry_mode_contract_vm.sh
#
# Contract pin:
# - bench_apps_wallclock supports app entry mode switch:
#   - source
#   - mir_shape_prebuilt
# - JSON must expose app_entry_mode.
# - invalid mode must fail-fast.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_apps_entry_mode_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/perf_apps_contract.sh"
require_env || exit 2
APP_BENCH="$NYASH_ROOT/tools/perf/bench_apps_wallclock.sh"
source "$NYASH_ROOT/tools/perf/lib/apps_wallclock_cases.sh"

if [ ! -f "$APP_BENCH" ]; then
  test_fail "$SMOKE_NAME: app bench script missing: $APP_BENCH"
  exit 2
fi

run_mode_check() {
  local mode="$1"
  local out
  out="$(
    PERF_APPS_OUTPUT=json \
    PERF_APPS_ENTRY_MODE="$mode" \
    PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
    bash "$APP_BENCH" "${PERF_APPS_WARMUP:-1}" "${PERF_APPS_REPEAT:-1}" vm 2>&1
  )" || {
    echo "$out"
    test_fail "$SMOKE_NAME: mode=${mode} run failed"
    exit 1
  }

  perf_apps_require_json "$SMOKE_NAME" "$out" || exit 1
  backend="$(perf_apps_json_get "$out" '.backend // ""')"
  mode_actual="$(perf_apps_json_get "$out" '.app_entry_mode // ""')"
  total_ms="$(perf_apps_json_get "$out" '.total_ms // 0')"
  cases_count="$(perf_apps_json_get "$out" '(.cases // {} | keys | length)')"
  expected_count="${#APPS_WALLCLOCK_CASE_NAMES[@]}"

  perf_apps_assert_backend_vm "$SMOKE_NAME" "$backend" "$out" || exit 1
  if [ "$mode_actual" != "$mode" ]; then
    echo "$out"
    test_fail "$SMOKE_NAME: mode mismatch expected=${mode} actual=${mode_actual}"
    exit 1
  fi
  perf_apps_assert_positive_uint "$SMOKE_NAME" "total_ms(${mode})" "$total_ms" "$out" || exit 1
  perf_apps_assert_uint "$SMOKE_NAME" "cases_count(${mode})" "$cases_count" "$out" || exit 1
  if [ "$cases_count" -ne "$expected_count" ]; then
    echo "$out"
    test_fail "$SMOKE_NAME: invalid case count mode=${mode} count=${cases_count} expected=${expected_count}"
    exit 1
  fi
}

run_mode_check "source"
run_mode_check "mir_shape_prebuilt"

set +e
bad_out="$(
  PERF_APPS_OUTPUT=json \
  PERF_APPS_ENTRY_MODE="invalid_mode" \
  PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
  bash "$APP_BENCH" "${PERF_APPS_WARMUP:-1}" "${PERF_APPS_REPEAT:-1}" vm 2>&1
)"
bad_rc=$?
set -e

if [ "$bad_rc" -eq 0 ]; then
  test_fail "$SMOKE_NAME: invalid mode should fail"
  exit 1
fi
if ! printf '%s\n' "$bad_out" | grep -q 'PERF_APPS_ENTRY_MODE must be source|mir_shape_prebuilt'; then
  echo "$bad_out"
  test_fail "$SMOKE_NAME: invalid mode error message missing"
  exit 1
fi

test_pass "$SMOKE_NAME"

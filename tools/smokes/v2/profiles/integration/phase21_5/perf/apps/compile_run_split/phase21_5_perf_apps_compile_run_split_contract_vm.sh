#!/bin/bash
# phase21_5_perf_apps_compile_run_split_contract_vm.sh
#
# Contract pin:
# - bench_apps_wallclock JSON exposes timing split fields:
#   timing_ms.prepare / timing_ms.mir_emit / timing_ms.startup_probe / timing_ms.run
# - timing_ms.run must equal total_ms.
# - timing_ms.mir_emit must be <= timing_ms.prepare.
# - prebuilt mode must keep timing_ms.mir_emit == 0.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_apps_compile_run_split_contract_vm"

source "$(dirname "$0")/../../../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../../../lib/perf_apps_contract.sh"
require_env || exit 2
APP_BENCH="$NYASH_ROOT/tools/perf/bench_apps_wallclock.sh"
PREBUILT="$NYASH_ROOT/apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json"

if [ ! -f "$APP_BENCH" ]; then
  test_fail "$SMOKE_NAME: app bench script missing: $APP_BENCH"
  exit 2
fi
if [ ! -f "$PREBUILT" ]; then
  test_fail "$SMOKE_NAME: prebuilt MIR missing: $PREBUILT"
  exit 2
fi

run_mode_contract() {
  local mode="$1"
  local out
  out="$(
    PERF_APPS_OUTPUT=json \
    PERF_APPS_MIR_SHAPE_INPUT_MODE="$mode" \
    PERF_APPS_MIR_SHAPE_PREBUILT="$PREBUILT" \
    PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
    bash "$APP_BENCH" "${PERF_APPS_WARMUP:-1}" "${PERF_APPS_REPEAT:-1}" vm 2>&1
  )" || {
    echo "$out"
    test_fail "$SMOKE_NAME: mode=${mode} bench command failed"
    exit 1
  }

  perf_apps_require_json "$SMOKE_NAME" "$out" || exit 1

  backend="$(perf_apps_json_get "$out" '.backend // ""')"
  mode_actual="$(perf_apps_json_get "$out" '.mir_shape_input_mode // ""')"
  total_ms="$(perf_apps_json_get "$out" '.total_ms // -1')"
  timing_prepare="$(perf_apps_json_get "$out" '.timing_ms.prepare // -1')"
  timing_mir_emit="$(perf_apps_json_get "$out" '.timing_ms.mir_emit // -1')"
  timing_startup_probe="$(perf_apps_json_get "$out" '.timing_ms.startup_probe // -1')"
  timing_run="$(perf_apps_json_get "$out" '.timing_ms.run // -1')"

  perf_apps_assert_backend_vm "$SMOKE_NAME" "$backend" "$out" || exit 1
  if [ "$mode_actual" != "$mode" ]; then
    echo "$out"
    test_fail "$SMOKE_NAME: mode mismatch expected=${mode} actual=${mode_actual}"
    exit 1
  fi
  perf_apps_assert_positive_uint "$SMOKE_NAME" "total_ms(${mode})" "$total_ms" "$out" || exit 1
  perf_apps_assert_uint "$SMOKE_NAME" "timing_ms.prepare(${mode})" "$timing_prepare" "$out" || exit 1
  perf_apps_assert_uint "$SMOKE_NAME" "timing_ms.mir_emit(${mode})" "$timing_mir_emit" "$out" || exit 1
  perf_apps_assert_uint "$SMOKE_NAME" "timing_ms.startup_probe(${mode})" "$timing_startup_probe" "$out" || exit 1
  perf_apps_assert_uint "$SMOKE_NAME" "timing_ms.run(${mode})" "$timing_run" "$out" || exit 1
  perf_apps_assert_eq_uint "$SMOKE_NAME" "timing_ms.run_vs_total_ms(${mode})" "$timing_run" "$total_ms" "$out" || exit 1
  perf_apps_assert_le_uint "$SMOKE_NAME" "timing_ms.mir_emit<=timing_ms.prepare(${mode})" "$timing_mir_emit" "$timing_prepare" "$out" || exit 1

  if [ "$mode" = "prebuilt" ] && [ "$timing_mir_emit" -ne 0 ]; then
    echo "$out"
    test_fail "$SMOKE_NAME: prebuilt mode must keep timing_ms.mir_emit=0 (got: $timing_mir_emit)"
    exit 1
  fi
}

run_mode_contract "emit"
run_mode_contract "prebuilt"

test_pass "$SMOKE_NAME"

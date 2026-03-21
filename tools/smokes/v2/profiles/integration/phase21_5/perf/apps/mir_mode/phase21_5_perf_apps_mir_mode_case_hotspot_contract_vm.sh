#!/bin/bash
# phase21_5_perf_apps_mir_mode_case_hotspot_contract_vm.sh
#
# Contract pin:
# - bench_apps_mir_mode_compare summary contains case-level fields:
#   emit_cases_median_ms / prebuilt_cases_median_ms / case_delta_ms / case_winner
# - hotspot_case_delta points to the max abs(case_delta_ms) case.
# - hotspot_case_delta.significant follows PERF_APPS_MIR_MODE_SIGNIFICANCE_MS.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_apps_mir_mode_case_hotspot_contract_vm"

source "$(dirname "$0")/../../../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../../../lib/perf_apps_contract.sh"
require_env || exit 2
source "$NYASH_ROOT/tools/perf/lib/apps_wallclock_cases.sh"

COMPARE="$NYASH_ROOT/tools/perf/bench_apps_mir_mode_compare.sh"
if [ ! -f "$COMPARE" ]; then
  test_fail "$SMOKE_NAME: compare script missing: $COMPARE"
  exit 2
fi

COUNT="${PERF_APPS_MIR_MODE_CASE_SAMPLES:-3}"
if ! [[ "$COUNT" =~ ^[0-9]+$ ]] || [ "$COUNT" -le 0 ]; then
  test_fail "$SMOKE_NAME: invalid sample count: $COUNT"
  exit 2
fi

OUT="$(
  PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
  PERF_APPS_MIR_MODE_SIGNIFICANCE_MS="${PERF_APPS_MIR_MODE_SIGNIFICANCE_MS:-10}" \
  bash "$COMPARE" "${PERF_APPS_WARMUP:-1}" "${PERF_APPS_REPEAT:-1}" vm --json-lines "$COUNT" 2>&1
)" || {
  echo "$OUT"
  test_fail "$SMOKE_NAME: compare command failed"
  exit 1
}

summary_line="$(printf '%s\n' "$OUT" | jq -c 'select(.kind=="summary")' 2>/dev/null || true)"
if [ -z "$summary_line" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: summary line missing"
  exit 1
fi

backend="$(printf '%s\n' "$summary_line" | jq -r '.backend // ""')"
perf_apps_assert_backend_vm "$SMOKE_NAME" "$backend" "$summary_line" || exit 1

threshold="$(printf '%s\n' "$summary_line" | jq -r '.significance_ms_threshold // -1')"
if ! [[ "$threshold" =~ ^[0-9]+$ ]]; then
  echo "$summary_line"
  test_fail "$SMOKE_NAME: significance threshold must be uint (got: $threshold)"
  exit 1
fi

hotspot_case="$(printf '%s\n' "$summary_line" | jq -r '.hotspot_case_delta.case // ""')"
hotspot_emit="$(printf '%s\n' "$summary_line" | jq -r '.hotspot_case_delta.emit_ms // -1')"
hotspot_prebuilt="$(printf '%s\n' "$summary_line" | jq -r '.hotspot_case_delta.prebuilt_ms // -1')"
hotspot_delta="$(printf '%s\n' "$summary_line" | jq -r '.hotspot_case_delta.delta_ms // -999999')"
hotspot_abs="$(printf '%s\n' "$summary_line" | jq -r '.hotspot_case_delta.delta_ms_abs // -1')"
hotspot_winner="$(printf '%s\n' "$summary_line" | jq -r '.hotspot_case_delta.winner // ""')"
hotspot_significant="$(printf '%s\n' "$summary_line" | jq -r '.hotspot_case_delta.significant // -1')"

perf_apps_assert_case_name_in_object "$SMOKE_NAME" "hotspot_case_delta.case" "$hotspot_case" "$summary_line" '.case_delta_ms // {}' || exit 1
perf_apps_assert_positive_uint "$SMOKE_NAME" "hotspot_case_delta.emit_ms" "$hotspot_emit" "$summary_line" || exit 1
perf_apps_assert_positive_uint "$SMOKE_NAME" "hotspot_case_delta.prebuilt_ms" "$hotspot_prebuilt" "$summary_line" || exit 1
perf_apps_assert_positive_uint "$SMOKE_NAME" "hotspot_case_delta.delta_ms_abs" "$hotspot_abs" "$summary_line" || exit 1
if ! [[ "$hotspot_delta" =~ ^-?[0-9]+$ ]]; then
  echo "$summary_line"
  test_fail "$SMOKE_NAME: hotspot_case_delta.delta_ms must be int (got: $hotspot_delta)"
  exit 1
fi
if [ "$hotspot_winner" != "emit" ] && [ "$hotspot_winner" != "prebuilt" ]; then
  echo "$summary_line"
  test_fail "$SMOKE_NAME: hotspot_case_delta.winner must be emit|prebuilt (got: $hotspot_winner)"
  exit 1
fi
if [ "$hotspot_significant" != "0" ] && [ "$hotspot_significant" != "1" ]; then
  echo "$summary_line"
  test_fail "$SMOKE_NAME: hotspot_case_delta.significant must be 0|1 (got: $hotspot_significant)"
  exit 1
fi

hotspot_abs_expected="$hotspot_delta"
if [ "$hotspot_abs_expected" -lt 0 ]; then
  hotspot_abs_expected=$(( -hotspot_abs_expected ))
fi
if [ "$hotspot_abs" -ne "$hotspot_abs_expected" ]; then
  echo "$summary_line"
  test_fail "$SMOKE_NAME: hotspot abs mismatch actual=$hotspot_abs expected=$hotspot_abs_expected"
  exit 1
fi

expected_hotspot_case=""
expected_hotspot_abs=-1

for case_name in "${APPS_WALLCLOCK_CASE_NAMES[@]}"; do
  emit_ms="$(printf '%s\n' "$summary_line" | jq -r ".emit_cases_median_ms[\"${case_name}\"] // -1")"
  prebuilt_ms="$(printf '%s\n' "$summary_line" | jq -r ".prebuilt_cases_median_ms[\"${case_name}\"] // -1")"
  delta_ms="$(printf '%s\n' "$summary_line" | jq -r ".case_delta_ms[\"${case_name}\"] // -999999")"
  winner="$(printf '%s\n' "$summary_line" | jq -r ".case_winner[\"${case_name}\"] // \"\"")"

  perf_apps_assert_positive_uint "$SMOKE_NAME" "emit_cases_median_ms.${case_name}" "$emit_ms" "$summary_line" || exit 1
  perf_apps_assert_positive_uint "$SMOKE_NAME" "prebuilt_cases_median_ms.${case_name}" "$prebuilt_ms" "$summary_line" || exit 1
  if ! [[ "$delta_ms" =~ ^-?[0-9]+$ ]]; then
    echo "$summary_line"
    test_fail "$SMOKE_NAME: case_delta_ms.${case_name} must be int (got: $delta_ms)"
    exit 1
  fi
  if [ "$winner" != "emit" ] && [ "$winner" != "prebuilt" ]; then
    echo "$summary_line"
    test_fail "$SMOKE_NAME: case_winner.${case_name} must be emit|prebuilt (got: $winner)"
    exit 1
  fi

  expected_delta=$(( prebuilt_ms - emit_ms ))
  if [ "$delta_ms" -ne "$expected_delta" ]; then
    echo "$summary_line"
    test_fail "$SMOKE_NAME: delta mismatch case=${case_name} actual=$delta_ms expected=$expected_delta"
    exit 1
  fi

  expected_winner="prebuilt"
  if [ "$emit_ms" -le "$prebuilt_ms" ]; then
    expected_winner="emit"
  fi
  if [ "$winner" != "$expected_winner" ]; then
    echo "$summary_line"
    test_fail "$SMOKE_NAME: winner mismatch case=${case_name} actual=$winner expected=$expected_winner"
    exit 1
  fi

  abs_delta="$delta_ms"
  if [ "$abs_delta" -lt 0 ]; then
    abs_delta=$(( -abs_delta ))
  fi
  if [ "$abs_delta" -gt "$expected_hotspot_abs" ] || { [ "$abs_delta" -eq "$expected_hotspot_abs" ] && { [ -z "$expected_hotspot_case" ] || [[ "$case_name" < "$expected_hotspot_case" ]]; }; }; then
    expected_hotspot_abs="$abs_delta"
    expected_hotspot_case="$case_name"
  fi
done

if [ "$hotspot_case" != "$expected_hotspot_case" ]; then
  echo "$summary_line"
  test_fail "$SMOKE_NAME: hotspot case mismatch actual=$hotspot_case expected=$expected_hotspot_case"
  exit 1
fi
if [ "$hotspot_abs" -ne "$expected_hotspot_abs" ]; then
  echo "$summary_line"
  test_fail "$SMOKE_NAME: hotspot abs mismatch actual=$hotspot_abs expected=$expected_hotspot_abs"
  exit 1
fi

expected_hotspot_significant="0"
if [ "$hotspot_abs" -ge "$threshold" ]; then
  expected_hotspot_significant="1"
fi
if [ "$hotspot_significant" != "$expected_hotspot_significant" ]; then
  echo "$summary_line"
  test_fail "$SMOKE_NAME: hotspot significant mismatch actual=$hotspot_significant expected=$expected_hotspot_significant"
  exit 1
fi

hotspot_case_winner="$(printf '%s\n' "$summary_line" | jq -r ".case_winner[\"${hotspot_case}\"] // \"\"")"
if [ "$hotspot_winner" != "$hotspot_case_winner" ]; then
  echo "$summary_line"
  test_fail "$SMOKE_NAME: hotspot winner mismatch actual=$hotspot_winner expected=$hotspot_case_winner"
  exit 1
fi

test_pass "$SMOKE_NAME"

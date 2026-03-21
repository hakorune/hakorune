#!/bin/bash
# phase21_5_perf_apps_entry_mode_spread_contract_vm.sh
#
# Contract pin:
# - bench_apps_entry_mode_compare.sh --json-lines N outputs:
#   - N sample JSON lines (kind=sample)
#   - 1 summary JSON line (kind=summary)
# - summary contains delta spread fields:
#   delta_ms_min <= delta_ms_median <= delta_ms_max

set -euo pipefail

SMOKE_NAME="phase21_5_perf_apps_entry_mode_spread_contract_vm"

source "$(dirname "$0")/../../../../../../lib/test_runner.sh"
require_env || exit 2

COMPARE="$NYASH_ROOT/tools/perf/bench_apps_entry_mode_compare.sh"
if [ ! -f "$COMPARE" ]; then
  test_fail "$SMOKE_NAME: compare script missing: $COMPARE"
  exit 2
fi

COUNT="${PERF_APPS_ENTRY_MODE_SPREAD_SAMPLES:-3}"
if ! [[ "$COUNT" =~ ^[0-9]+$ ]] || [ "$COUNT" -le 0 ]; then
  test_fail "$SMOKE_NAME: invalid sample count: $COUNT"
  exit 2
fi

OUT="$(
  PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
  bash "$COMPARE" "${PERF_APPS_WARMUP:-1}" "${PERF_APPS_REPEAT:-1}" vm --json-lines "$COUNT" 2>&1
)" || {
  echo "$OUT"
  test_fail "$SMOKE_NAME: compare command failed"
  exit 1
}

sample_lines="$(printf '%s\n' "$OUT" | jq -c 'select(.kind=="sample")' 2>/dev/null || true)"
summary_line="$(printf '%s\n' "$OUT" | jq -c 'select(.kind=="summary")' 2>/dev/null || true)"

if [ -z "$summary_line" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: summary line missing"
  exit 1
fi

sample_count="$(printf '%s\n' "$sample_lines" | sed '/^$/d' | wc -l | tr -d ' ')"
if [ "$sample_count" -ne "$COUNT" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: sample line count mismatch actual=${sample_count} expected=${COUNT}"
  exit 1
fi

backend="$(printf '%s\n' "$summary_line" | jq -r '.backend // ""')"
samples="$(printf '%s\n' "$summary_line" | jq -r '.samples // -1')"
source_total_ms="$(printf '%s\n' "$summary_line" | jq -r '.source_total_ms // -1')"
prebuilt_total_ms="$(printf '%s\n' "$summary_line" | jq -r '.mir_shape_prebuilt_total_ms // -1')"
delta_ms="$(printf '%s\n' "$summary_line" | jq -r '.delta_ms // -999999')"
delta_ms_min="$(printf '%s\n' "$summary_line" | jq -r '.delta_ms_min // -999999')"
delta_ms_median="$(printf '%s\n' "$summary_line" | jq -r '.delta_ms_median // -999999')"
delta_ms_max="$(printf '%s\n' "$summary_line" | jq -r '.delta_ms_max // -999999')"
winner="$(printf '%s\n' "$summary_line" | jq -r '.winner // ""')"

if [ "$backend" != "vm" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: backend must be vm (got: $backend)"
  exit 1
fi

for v in "$samples" "$source_total_ms" "$prebuilt_total_ms" "$delta_ms" "$delta_ms_min" "$delta_ms_median" "$delta_ms_max"; do
  if ! [[ "$v" =~ ^-?[0-9]+$ ]]; then
    echo "$OUT"
    test_fail "$SMOKE_NAME: non-integer summary field ($v)"
    exit 1
  fi
done
if [ "$samples" -ne "$COUNT" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: summary sample count mismatch actual=${samples} expected=${COUNT}"
  exit 1
fi
if [ "$source_total_ms" -le 0 ] || [ "$prebuilt_total_ms" -le 0 ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: totals must be > 0 (source=$source_total_ms prebuilt=$prebuilt_total_ms)"
  exit 1
fi

if [ "$delta_ms_min" -gt "$delta_ms_median" ] || [ "$delta_ms_median" -gt "$delta_ms_max" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: delta spread ordering broken min=${delta_ms_min} median=${delta_ms_median} max=${delta_ms_max}"
  exit 1
fi
expected_delta=$(( prebuilt_total_ms - source_total_ms ))
if [ "$delta_ms" -ne "$expected_delta" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: delta mismatch actual=$delta_ms expected=$expected_delta"
  exit 1
fi

expected_winner="mir_shape_prebuilt"
if [ "$source_total_ms" -le "$prebuilt_total_ms" ]; then
  expected_winner="source"
fi
if [ "$winner" != "$expected_winner" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: winner mismatch actual=$winner expected=$expected_winner"
  exit 1
fi

log_info "$SMOKE_NAME: samples=${samples} delta[min,med,max]=${delta_ms_min},${delta_ms_median},${delta_ms_max} winner=${winner}"
test_pass "$SMOKE_NAME"

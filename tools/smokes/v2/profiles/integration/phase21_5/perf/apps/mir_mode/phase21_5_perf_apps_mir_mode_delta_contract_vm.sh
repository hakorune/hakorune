#!/bin/bash
# phase21_5_perf_apps_mir_mode_delta_contract_vm.sh
#
# Contract pin:
# - bench_apps_mir_mode_compare.sh emits one-line JSON with:
#   emit_total_ms, prebuilt_total_ms, delta_ms, delta_pct, winner.
# - delta_ms must equal prebuilt_total_ms - emit_total_ms.
# - winner must match lower total_ms (emit on tie).

set -euo pipefail

SMOKE_NAME="phase21_5_perf_apps_mir_mode_delta_contract_vm"

source "$(dirname "$0")/../../../../../../lib/test_runner.sh"
require_env || exit 2

COMPARE="$NYASH_ROOT/tools/perf/bench_apps_mir_mode_compare.sh"
if [ ! -f "$COMPARE" ]; then
  test_fail "$SMOKE_NAME: compare script missing: $COMPARE"
  exit 2
fi

OUT="$(
  PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
  "$COMPARE" "${PERF_APPS_WARMUP:-1}" "${PERF_APPS_REPEAT:-1}" vm 2>&1
)" || {
  echo "$OUT"
  test_fail "$SMOKE_NAME: compare command failed"
  exit 1
}

if ! printf '%s\n' "$OUT" | jq -e . >/dev/null 2>&1; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: output is not valid JSON"
  exit 1
fi

backend="$(printf '%s\n' "$OUT" | jq -r '.backend // ""')"
emit_total_ms="$(printf '%s\n' "$OUT" | jq -r '.emit_total_ms // -1')"
prebuilt_total_ms="$(printf '%s\n' "$OUT" | jq -r '.prebuilt_total_ms // -1')"
delta_ms="$(printf '%s\n' "$OUT" | jq -r '.delta_ms // -999999')"
delta_pct="$(printf '%s\n' "$OUT" | jq -r '.delta_pct // null')"
winner="$(printf '%s\n' "$OUT" | jq -r '.winner // ""')"

if [ "$backend" != "vm" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: backend must be vm (got: $backend)"
  exit 1
fi

for v in "$emit_total_ms" "$prebuilt_total_ms" "$delta_ms"; do
  if ! [[ "$v" =~ ^-?[0-9]+$ ]]; then
    echo "$OUT"
    test_fail "$SMOKE_NAME: non-integer numeric field detected ($v)"
    exit 1
  fi
done
if [ "$emit_total_ms" -le 0 ] || [ "$prebuilt_total_ms" -le 0 ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: totals must be > 0 (emit=$emit_total_ms prebuilt=$prebuilt_total_ms)"
  exit 1
fi

if ! [[ "$delta_pct" =~ ^-?[0-9]+(\.[0-9]+)?$ ]]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: delta_pct is not numeric: $delta_pct"
  exit 1
fi

if [ "$winner" != "emit" ] && [ "$winner" != "prebuilt" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: winner must be emit|prebuilt (got: $winner)"
  exit 1
fi

expected_delta=$(( prebuilt_total_ms - emit_total_ms ))
if [ "$delta_ms" -ne "$expected_delta" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: delta mismatch actual=$delta_ms expected=$expected_delta"
  exit 1
fi

expected_winner="prebuilt"
if [ "$emit_total_ms" -le "$prebuilt_total_ms" ]; then
  expected_winner="emit"
fi
if [ "$winner" != "$expected_winner" ]; then
  echo "$OUT"
  test_fail "$SMOKE_NAME: winner mismatch actual=$winner expected=$expected_winner"
  exit 1
fi

log_info "$SMOKE_NAME: emit=${emit_total_ms} prebuilt=${prebuilt_total_ms} delta=${delta_ms} winner=${winner}"
test_pass "$SMOKE_NAME"

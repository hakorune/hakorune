#!/bin/bash
# phase21_5_perf_kilo_micro_machine_lane_contract_vm.sh
#
# Contract pin:
# - fixed machine-code micro ladder exists and runs all 3 kilo/text cases.
# - each case reports [microstat] with aot_status=ok.

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_kilo_micro_machine_lane_contract_vm"
LADDER="$NYASH_ROOT/tools/perf/run_kilo_micro_machine_ladder.sh"

if [ ! -f "$LADDER" ]; then
  test_fail "$SMOKE_NAME: ladder script missing: $LADDER"
  exit 2
fi

set +e
OUT="$(bash "$LADDER" 1 1 2>&1)"
RC=$?
set -e

if [ "$RC" -ne 0 ]; then
  printf '%s\n' "$OUT"
  test_fail "$SMOKE_NAME: ladder failed rc=$RC"
  exit 1
fi

for key in kilo_micro_indexof_line kilo_micro_substring_concat kilo_micro_array_getset; do
  if ! printf '%s\n' "$OUT" | grep -q "\\[microstat\\] name=${key} "; then
    printf '%s\n' "$OUT"
    test_fail "$SMOKE_NAME: missing microstat line for ${key}"
    exit 1
  fi
done

aot_ok_count="$(printf '%s\n' "$OUT" | grep -c 'aot_status=ok' || true)"
if [ "$aot_ok_count" -lt 3 ]; then
  printf '%s\n' "$OUT"
  test_fail "$SMOKE_NAME: expected aot_status=ok for all 3 cases (got $aot_ok_count)"
  exit 1
fi

test_pass "$SMOKE_NAME"

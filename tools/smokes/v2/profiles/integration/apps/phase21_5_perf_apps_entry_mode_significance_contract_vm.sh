#!/bin/bash
# phase21_5_perf_apps_entry_mode_significance_contract_vm.sh
#
# Contract pin:
# - bench_apps_entry_mode_compare summary contains:
#   significance_ms_threshold, delta_ms_abs, significant, winner_significant.
# - threshold=0 forces significant=1 and winner_significant==winner.
# - huge threshold forces significant=0 and winner_significant=tie.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_apps_entry_mode_significance_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

COMPARE="$NYASH_ROOT/tools/perf/bench_apps_entry_mode_compare.sh"
if [ ! -f "$COMPARE" ]; then
  test_fail "$SMOKE_NAME: compare script missing: $COMPARE"
  exit 2
fi

run_and_check() {
  local threshold="$1"
  local expect_significant="$2"
  local expect_winner_sig="$3"
  local out sig_thr sig winner winner_sig delta_ms delta_abs

  out="$(
    PERF_APPS_ENTRY_MODE_SIGNIFICANCE_MS="$threshold" \
    PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
    "$COMPARE" "${PERF_APPS_WARMUP:-1}" "${PERF_APPS_REPEAT:-1}" vm 2>&1
  )" || {
    echo "$out"
    test_fail "$SMOKE_NAME: compare command failed (threshold=$threshold)"
    exit 1
  }

  if ! printf '%s\n' "$out" | jq -e . >/dev/null 2>&1; then
    echo "$out"
    test_fail "$SMOKE_NAME: output is not valid JSON (threshold=$threshold)"
    exit 1
  fi

  sig_thr="$(printf '%s\n' "$out" | jq -r '.significance_ms_threshold // -1')"
  sig="$(printf '%s\n' "$out" | jq -r '.significant // -1')"
  winner="$(printf '%s\n' "$out" | jq -r '.winner // ""')"
  winner_sig="$(printf '%s\n' "$out" | jq -r '.winner_significant // ""')"
  delta_ms="$(printf '%s\n' "$out" | jq -r '.delta_ms // -999999')"
  delta_abs="$(printf '%s\n' "$out" | jq -r '.delta_ms_abs // -999999')"

  for v in "$sig_thr" "$sig" "$delta_ms" "$delta_abs"; do
    if ! [[ "$v" =~ ^-?[0-9]+$ ]]; then
      echo "$out"
      test_fail "$SMOKE_NAME: invalid numeric field ($v)"
      exit 1
    fi
  done
  if [ "$sig_thr" -ne "$threshold" ]; then
    echo "$out"
    test_fail "$SMOKE_NAME: threshold mismatch actual=$sig_thr expected=$threshold"
    exit 1
  fi
  if [ "$winner" != "source" ] && [ "$winner" != "mir_shape_prebuilt" ]; then
    echo "$out"
    test_fail "$SMOKE_NAME: winner must be source|mir_shape_prebuilt (got: $winner)"
    exit 1
  fi

  expected_abs="$delta_ms"
  if [ "$expected_abs" -lt 0 ]; then
    expected_abs=$(( -expected_abs ))
  fi
  if [ "$delta_abs" -ne "$expected_abs" ]; then
    echo "$out"
    test_fail "$SMOKE_NAME: delta abs mismatch actual=$delta_abs expected=$expected_abs"
    exit 1
  fi

  if [ "$sig" -ne "$expect_significant" ]; then
    echo "$out"
    test_fail "$SMOKE_NAME: significant mismatch threshold=$threshold actual=$sig expected=$expect_significant"
    exit 1
  fi

  if [ "$expect_winner_sig" = "__winner__" ]; then
    if [ "$winner_sig" != "$winner" ]; then
      echo "$out"
      test_fail "$SMOKE_NAME: winner_significant must match winner when significant=1 (actual=$winner_sig winner=$winner)"
      exit 1
    fi
  else
    if [ "$winner_sig" != "$expect_winner_sig" ]; then
      echo "$out"
      test_fail "$SMOKE_NAME: winner_significant mismatch actual=$winner_sig expected=$expect_winner_sig"
      exit 1
    fi
  fi
}

# threshold=0 => always significant (abs(delta) >= 0)
run_and_check 0 1 "__winner__"
# huge threshold => always non-significant => tie
run_and_check 1000000 0 "tie"

test_pass "$SMOKE_NAME"

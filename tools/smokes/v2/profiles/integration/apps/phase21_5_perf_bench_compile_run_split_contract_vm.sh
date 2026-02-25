#!/bin/bash
# phase21_5_perf_bench_compile_run_split_contract_vm.sh
#
# Contract pin:
# - bench_compare_compile_run_split emits a stable one-line contract:
#   [bench-split] name=<key> status=<ok|...> emit_route=<route> total_ms=<n> emit_ms=<n> run_prebuilt_ms=<n> ...
# - default emit_route is direct (binary-only stage1 route)
# - default status is ok
# - lock two bench keys:
#   numeric_mixed_medium / method_call_only_small
# - decision is one of:
#   vm_hotpath_priority | json_opt_candidate

set -euo pipefail

SMOKE_NAME="phase21_5_perf_bench_compile_run_split_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SCRIPT="$NYASH_ROOT/tools/perf/bench_compare_compile_run_split.sh"
if [[ ! -x "$SCRIPT" ]]; then
  test_fail "$SMOKE_NAME: script missing or not executable: $SCRIPT"
  exit 2
fi

extract_uint() {
  local key="$1"
  local src="$2"
  printf '%s\n' "$src" | sed -n "s/.* ${key}=\\([0-9][0-9]*\\).*/\\1/p"
}

run_case() {
  local key="$1"
  local out line
  out="$(
    PERF_SPLIT_OUTPUT=text \
    PERF_SPLIT_JSON_OPT_IN_RATIO_PCT="${PERF_SPLIT_JSON_OPT_IN_RATIO_PCT:-40}" \
    PERF_SPLIT_MIN_TOTAL_MS="${PERF_SPLIT_MIN_TOTAL_MS:-100}" \
    bash "$SCRIPT" "$key" 1 1 2>&1
  )" || {
    echo "$out"
    test_fail "$SMOKE_NAME: split bench command failed (key=$key)"
    exit 1
  }

  line="$(printf '%s\n' "$out" | grep -E "^\\[bench-split\\] name=${key} " | tail -n 1 || true)"
  if [[ -z "$line" ]]; then
    echo "$out"
    test_fail "$SMOKE_NAME: missing [bench-split] line for $key"
    exit 1
  fi

  local total_ms emit_ms run_prebuilt_ms threshold_pct min_total_ms emit_route decision status
  total_ms="$(extract_uint total_ms "$line")"
  emit_ms="$(extract_uint emit_ms "$line")"
  run_prebuilt_ms="$(extract_uint run_prebuilt_ms "$line")"
  threshold_pct="$(extract_uint threshold_pct "$line")"
  min_total_ms="$(extract_uint min_total_ms "$line")"
  emit_route="$(printf '%s\n' "$line" | sed -n 's/.* emit_route=\([^ ]*\).*/\1/p')"
  decision="$(printf '%s\n' "$line" | sed -n 's/.* decision=\([^ ]*\).*/\1/p')"
  status="$(printf '%s\n' "$line" | sed -n 's/.* status=\([^ ]*\).*/\1/p')"

  for v in "$total_ms" "$emit_ms" "$run_prebuilt_ms" "$threshold_pct" "$min_total_ms"; do
    if [[ -z "$v" ]]; then
      echo "$out"
      test_fail "$SMOKE_NAME: missing numeric field in output (key=$key)"
      exit 1
    fi
  done

  if [[ "$total_ms" -le 0 || "$emit_ms" -le 0 || "$run_prebuilt_ms" -le 0 ]]; then
    echo "$out"
    test_fail "$SMOKE_NAME: expected positive timings (key=$key total/emit/run_prebuilt)"
    exit 1
  fi
  if [[ "$emit_route" != "direct" ]]; then
    echo "$out"
    test_fail "$SMOKE_NAME: expected emit_route=direct by default (key=$key got=$emit_route)"
    exit 1
  fi
  if [[ "$status" != "ok" ]]; then
    echo "$out"
    test_fail "$SMOKE_NAME: expected status=ok (key=$key got=$status)"
    exit 1
  fi
  if [[ "$decision" != "vm_hotpath_priority" && "$decision" != "json_opt_candidate" ]]; then
    echo "$out"
    test_fail "$SMOKE_NAME: invalid decision=${decision} (key=$key)"
    exit 1
  fi
}

run_case "numeric_mixed_medium"
run_case "method_call_only_small"

test_pass "$SMOKE_NAME"

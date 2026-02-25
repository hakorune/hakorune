#!/bin/bash
# phase21_5_perf_bench_compare_env_contract_vm.sh
#
# Contract pin:
# - bench_compare_c_vs_hako.sh env contracts are fail-fast:
#   - PERF_AOT: 0|1
#   - PERF_SUBTRACT_STARTUP: 0|1
#   - PERF_SKIP_VM_PREFLIGHT: 0|1
#   - PERF_AOT_TIMEOUT_SEC: numeric

set -euo pipefail

SMOKE_NAME="phase21_5_perf_bench_compare_env_contract_vm"
KEY="box_create_destroy_small"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SCRIPT="$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh"
if [ ! -f "$SCRIPT" ]; then
  test_fail "$SMOKE_NAME: missing script: $SCRIPT"
  exit 2
fi

# Case 1: valid env set should pass.
set +e
ok_out="$(
  PERF_AOT=0 \
  PERF_SUBTRACT_STARTUP=0 \
  PERF_SKIP_VM_PREFLIGHT=1 \
  PERF_AOT_TIMEOUT_SEC=20 \
  bash "$SCRIPT" "$KEY" 1 1 2>&1
)"
ok_rc=$?
set -e
if [ "$ok_rc" -ne 0 ]; then
  printf '%s\n' "$ok_out"
  test_fail "$SMOKE_NAME: valid env case failed rc=$ok_rc"
  exit 1
fi
if ! printf '%s\n' "$ok_out" | grep -q "\\[bench\\] name=${KEY}"; then
  printf '%s\n' "$ok_out"
  test_fail "$SMOKE_NAME: valid env case missing bench output"
  exit 1
fi

check_invalid_env() {
  local key="$1"
  local value="$2"
  local expected_msg="$3"
  local out rc
  set +e
  out="$(
    PERF_AOT=0 \
    PERF_SUBTRACT_STARTUP=0 \
    PERF_SKIP_VM_PREFLIGHT=1 \
    PERF_AOT_TIMEOUT_SEC=20 \
    env "$key=$value" \
    bash "$SCRIPT" "$KEY" 1 1 2>&1
  )"
  rc=$?
  set -e
  if [ "$rc" -eq 0 ]; then
    printf '%s\n' "$out"
    test_fail "$SMOKE_NAME: invalid $key unexpectedly succeeded"
    exit 1
  fi
  if ! printf '%s\n' "$out" | grep -q "$expected_msg"; then
    printf '%s\n' "$out"
    test_fail "$SMOKE_NAME: missing fail-fast error for invalid $key"
    exit 1
  fi
}

check_invalid_env "PERF_AOT" "2" "PERF_AOT must be 0|1"
check_invalid_env "PERF_SUBTRACT_STARTUP" "2" "PERF_SUBTRACT_STARTUP must be 0|1"
check_invalid_env "PERF_SKIP_VM_PREFLIGHT" "2" "PERF_SKIP_VM_PREFLIGHT must be 0|1"
check_invalid_env "PERF_AOT_TIMEOUT_SEC" "abc" "PERF_AOT_TIMEOUT_SEC must be numeric seconds"

test_pass "$SMOKE_NAME"

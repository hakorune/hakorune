#!/bin/bash
# phase21_5_perf_fast_regfile_contract_vm.sh
#
# Contract pin:
# - NYASH_VM_FAST_REGFILE=1 時に small perf keys が preflight fail しないこと
# - register write path は fast slot / map を単一路（write_reg/take_reg）で維持すること

set -euo pipefail

SMOKE_NAME="phase21_5_perf_fast_regfile_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

BENCH_COMPARE="$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh"

if [ ! -f "$BENCH_COMPARE" ]; then
  test_fail "$SMOKE_NAME: missing bench script: $BENCH_COMPARE"
  exit 2
fi

run_case() {
  local key="$1"
  local out
  out="$(
    NYASH_VM_FAST_REGFILE=1 \
    PERF_SUBTRACT_STARTUP=1 \
    NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
    bash "$BENCH_COMPARE" "$key" 1 1 2>&1
  )" || {
    echo "$out"
    test_fail "$SMOKE_NAME: bench failed for key=$key"
    exit 1
  }

  if ! printf '%s\n' "$out" | grep -q "\\[bench\\] name=$key "; then
    echo "$out"
    test_fail "$SMOKE_NAME: missing bench line for key=$key"
    exit 1
  fi

  if printf '%s\n' "$out" | grep -q 'VM benchmark preflight failed'; then
    echo "$out"
    test_fail "$SMOKE_NAME: preflight failed for key=$key"
    exit 1
  fi
}

run_case "method_call_only_small"
run_case "box_create_destroy_small"

test_pass "$SMOKE_NAME"

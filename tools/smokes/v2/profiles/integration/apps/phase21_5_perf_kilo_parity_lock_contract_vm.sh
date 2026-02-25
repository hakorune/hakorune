#!/bin/bash
# phase21_5_perf_kilo_parity_lock_contract_vm.sh
#
# Contract pin (LLVM-HOT-20 stop line):
# - kilo bench must keep AOT lane available (`aot_status=ok`)
# - ratio_c_aot must stay above the parity floor (default: 0.95)
# - parse is key-based from [bench4] output (no column dependency)

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../lib/perf_crosslang_contract.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_kilo_parity_lock_contract_vm"
KEY="kilo_kernel_small"
SCRIPT="${NYASH_ROOT}/tools/perf/bench_compare_c_py_vs_hako.sh"
MIN_RATIO="${PERF_KILO_PARITY_MIN_RATIO:-0.95}"

if ! [[ "${MIN_RATIO}" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
  test_fail "${SMOKE_NAME}: PERF_KILO_PARITY_MIN_RATIO must be numeric (got ${MIN_RATIO})"
  exit 2
fi

perf_crosslang_require_inputs "${SMOKE_NAME}" "${SCRIPT}" "${KEY}" || exit 2

set +e
OUTPUT=$(
  PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
  HAKO_VM_MAX_STEPS="${HAKO_VM_MAX_STEPS:-300000000}" \
  "${SCRIPT}" "${KEY}" 1 3 2>&1
)
RC=$?
set -e

if [[ "${RC}" -ne 0 ]]; then
  printf '%s\n' "${OUTPUT}" | tail -n 40 || true
  test_fail "${SMOKE_NAME}: bench compare failed rc=${RC}"
  exit 1
fi

perf_crosslang_assert_output "${SMOKE_NAME}" "${KEY}" "${OUTPUT}" || exit 1

bench_line="$(printf '%s\n' "${OUTPUT}" | grep "\\[bench4\\] name=${KEY} " | tail -n 1 || true)"
if [[ -z "${bench_line}" ]]; then
  printf '%s\n' "${OUTPUT}" | tail -n 40 || true
  test_fail "${SMOKE_NAME}: missing [bench4] line for ${KEY}"
  exit 1
fi

ratio_c_aot="$(printf '%s\n' "${bench_line}" | sed -n 's/.*ratio_c_aot=\([0-9][0-9.]*\).*/\1/p')"
if [[ -z "${ratio_c_aot}" ]]; then
  printf '%s\n' "${OUTPUT}" | tail -n 40 || true
  test_fail "${SMOKE_NAME}: failed to parse ratio_c_aot from bench output"
  exit 1
fi

if ! awk -v ratio="${ratio_c_aot}" -v min="${MIN_RATIO}" 'BEGIN { exit !(ratio + 0 >= min + 0) }'; then
  printf '%s\n' "${OUTPUT}" | tail -n 40 || true
  test_fail "${SMOKE_NAME}: ratio_c_aot=${ratio_c_aot} is below floor ${MIN_RATIO}"
  exit 1
fi

printf '%s\n' "${OUTPUT}"
test_pass "${SMOKE_NAME}: PASS (ratio_c_aot=${ratio_c_aot}, min=${MIN_RATIO})"

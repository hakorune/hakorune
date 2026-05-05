#!/bin/bash
# APP-PERF-01: chip8_kernel_small cross-language contract smoke
#
# Contract:
# 1) bench_compare_c_py_vs_hako.sh exits with code 0
# 2) Output contains [bench4] name=chip8_kernel_small
# 3) while perf lane is parked, aot_status may be ok or skip
# 4) All timing keys are present: c_ms= py_ms= ny_vm_ms= ny_aot_ms=
#
# Note: Parsing uses key-based extraction (not fixed-width)

set -euo pipefail

source "$(dirname "$0")/../../../../../lib/test_runner.sh"
source "$(dirname "$0")/../../../../../lib/perf_crosslang_contract.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_chip8_kernel_crosslang_contract"
SCRIPT="${NYASH_ROOT}/tools/perf/bench_compare_c_py_vs_hako.sh"
KEY="chip8_kernel_small"

# Pre-flight checks
perf_crosslang_require_inputs "${SMOKE_NAME}" "${SCRIPT}" "${KEY}" || exit 2

# Run 4-way comparison (minimal warmup/repeat for smoke)
set +e
OUTPUT=$("${SCRIPT}" "${KEY}" 1 1 2>&1)
RC=$?
set -e

if [[ "${RC}" -ne 0 ]]; then
  echo "${OUTPUT}" | tail -n 40 || true
  test_fail "${SMOKE_NAME}: Script exited with rc=${RC}"
  exit 1
fi

# Verify output format (key-based parsing)
perf_crosslang_assert_output "${SMOKE_NAME}" "${KEY}" "${OUTPUT}" "ok-or-skip" || exit 1

echo "${OUTPUT}"
test_pass "${SMOKE_NAME}: PASS (chip8_kernel_small 4-way cross-language contract pinned)"

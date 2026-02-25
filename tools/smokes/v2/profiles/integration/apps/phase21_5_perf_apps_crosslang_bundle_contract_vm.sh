#!/bin/bash
# APP-PERF-03: chip8/kilo + app wallclock integration contract smoke
#
# Contract:
# 1) bench_crosslang_apps_bundle.sh exits with code 0
# 2) Output contains [bench4-app] marker
# 3) chip8/kilo aot status keys are present and equal to ok
# 4) Numeric keys are present:
#    chip8_ratio_c_aot, chip8_ny_aot_ms, kilo_ratio_c_aot, kilo_ny_aot_ms,
#    apps_total_ms, apps_hotspot_ms, entry_source_total_ms, entry_prebuilt_total_ms, entry_delta_ms
# 5) entry_winner is source|mir_shape_prebuilt

set -euo pipefail

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

SMOKE_NAME="phase21_5_perf_apps_crosslang_bundle_contract_vm"
SCRIPT="${NYASH_ROOT}/tools/perf/bench_crosslang_apps_bundle.sh"

if [[ ! -x "${SCRIPT}" ]]; then
  test_fail "${SMOKE_NAME}: Script not executable: ${SCRIPT}"
  exit 2
fi

set +e
OUTPUT=$(
  NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
  PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
  HAKO_VM_MAX_STEPS="${HAKO_VM_MAX_STEPS:-300000000}" \
  PERF_APPS_ENTRY_MODE_DELTA_SAMPLES="${PERF_APPS_ENTRY_MODE_DELTA_SAMPLES:-1}" \
  "${SCRIPT}" 1 1 1 1 2>&1
)
RC=$?
set -e

if [[ "${RC}" -ne 0 ]]; then
  echo "${OUTPUT}" | tail -n 80 || true
  test_fail "${SMOKE_NAME}: script exited with rc=${RC}"
  exit 1
fi

if ! echo "${OUTPUT}" | grep -q "\[bench4-app\] "; then
  echo "${OUTPUT}" | tail -n 80 || true
  test_fail "${SMOKE_NAME}: missing [bench4-app] marker"
  exit 1
fi

if ! echo "${OUTPUT}" | grep -q "chip8_aot_status=ok"; then
  echo "${OUTPUT}" | tail -n 80 || true
  test_fail "${SMOKE_NAME}: chip8_aot_status is not ok"
  exit 1
fi

if ! echo "${OUTPUT}" | grep -q "kilo_aot_status=ok"; then
  echo "${OUTPUT}" | tail -n 80 || true
  test_fail "${SMOKE_NAME}: kilo_aot_status is not ok"
  exit 1
fi

for key in chip8_ratio_c_aot chip8_ny_aot_ms kilo_ratio_c_aot kilo_ny_aot_ms apps_total_ms apps_hotspot_ms entry_source_total_ms entry_prebuilt_total_ms; do
  if ! echo "${OUTPUT}" | grep -qE "${key}=[0-9]+([.][0-9]+)?"; then
    echo "${OUTPUT}" | tail -n 80 || true
    test_fail "${SMOKE_NAME}: missing numeric key: ${key}="
    exit 1
  fi
done

if ! echo "${OUTPUT}" | grep -qE "entry_delta_ms=-?[0-9]+"; then
  echo "${OUTPUT}" | tail -n 80 || true
  test_fail "${SMOKE_NAME}: missing numeric key: entry_delta_ms="
  exit 1
fi

if ! echo "${OUTPUT}" | grep -qE "entry_winner=(source|mir_shape_prebuilt)"; then
  echo "${OUTPUT}" | tail -n 80 || true
  test_fail "${SMOKE_NAME}: invalid entry_winner"
  exit 1
fi

if ! echo "${OUTPUT}" | grep -qE "apps_hotspot_case=[^[:space:]]+"; then
  echo "${OUTPUT}" | tail -n 80 || true
  test_fail "${SMOKE_NAME}: missing apps_hotspot_case"
  exit 1
fi

echo "${OUTPUT}"
test_pass "${SMOKE_NAME}: PASS (bench4 + app wallclock bundle contract pinned)"

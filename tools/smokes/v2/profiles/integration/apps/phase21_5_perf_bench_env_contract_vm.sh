#!/bin/bash
# phase21_5_perf_bench_env_contract_vm.sh
#
# Contract pin:
# - bench_env timeout resolver is SSOT for fast/heavy defaults.
# - key perf/check scripts consume timeout via resolver path.

set -euo pipefail

SMOKE_NAME="phase21_5_perf_bench_env_contract_vm"

source "$(dirname "$0")/../../../lib/test_runner.sh"
require_env || exit 2

BENCH_ENV="$NYASH_ROOT/tools/perf/lib/bench_env.sh"
BENCH_COMMON="$NYASH_ROOT/tools/perf/lib/bench_compare_common.sh"
BENCH_COMPARE="$NYASH_ROOT/tools/perf/bench_compare_c_vs_hako.sh"
BENCH_COMPARE4="$NYASH_ROOT/tools/perf/bench_compare_c_py_vs_hako.sh"
AOT_HELPERS="$NYASH_ROOT/tools/perf/lib/aot_helpers.sh"
APPS_WALLCLOCK="$NYASH_ROOT/tools/perf/bench_apps_wallclock.sh"
STABILITY="$NYASH_ROOT/tools/perf/record_baseline_stability_21_5.sh"
REG_GUARD="$NYASH_ROOT/tools/checks/phase21_5_perf_regression_guard.sh"
MICRO_STAT="$NYASH_ROOT/tools/perf/bench_micro_c_vs_aot_stat.sh"
MICRO_ASM="$NYASH_ROOT/tools/perf/bench_micro_aot_asm.sh"
MICRO_LADDER="$NYASH_ROOT/tools/perf/run_kilo_micro_machine_ladder.sh"

for f in "$BENCH_ENV" "$BENCH_COMMON" "$BENCH_COMPARE" "$BENCH_COMPARE4" "$AOT_HELPERS" "$APPS_WALLCLOCK" "$STABILITY" "$REG_GUARD" "$MICRO_STAT" "$MICRO_ASM" "$MICRO_LADDER"; do
  if [ ! -f "$f" ]; then
    test_fail "$SMOKE_NAME: missing file: $f"
    exit 2
  fi
done

source "$BENCH_ENV"

if ! declare -F perf_vm_timeout_resolve >/dev/null 2>&1; then
  test_fail "$SMOKE_NAME: perf_vm_timeout_resolve missing"
  exit 1
fi

unset PERF_VM_TIMEOUT || true
if [ "$(perf_vm_timeout_resolve fast)" != "20s" ]; then
  test_fail "$SMOKE_NAME: fast default timeout mismatch"
  exit 1
fi
if [ "$(perf_vm_timeout_resolve heavy)" != "60s" ]; then
  test_fail "$SMOKE_NAME: heavy default timeout mismatch"
  exit 1
fi

PERF_VM_TIMEOUT=77s
if [ "$(perf_vm_timeout_resolve fast)" != "77s" ]; then
  test_fail "$SMOKE_NAME: PERF_VM_TIMEOUT override mismatch"
  exit 1
fi
unset PERF_VM_TIMEOUT || true

set +e
perf_vm_timeout_resolve unknown >/dev/null 2>&1
rc_invalid=$?
set -e
if [ "$rc_invalid" -eq 0 ]; then
  test_fail "$SMOKE_NAME: invalid profile should fail"
  exit 1
fi

if ! grep -q 'perf_vm_timeout_resolve fast' "$BENCH_COMPARE"; then
  test_fail "$SMOKE_NAME: bench_compare does not use resolver fast profile"
  exit 1
fi
if ! grep -q 'source \"${ROOT_DIR}/tools/perf/lib/bench_compare_common.sh\"' "$BENCH_COMPARE"; then
  test_fail "$SMOKE_NAME: bench_compare does not source bench_compare_common"
  exit 1
fi
if ! grep -q 'source \"${ROOT_DIR}/tools/perf/lib/bench_compare_common.sh\"' "$BENCH_COMPARE4"; then
  test_fail "$SMOKE_NAME: bench_compare4 does not source bench_compare_common"
  exit 1
fi
if ! grep -q 'source \"${ROOT_DIR}/tools/perf/lib/bench_compare_common.sh\"' "$MICRO_STAT"; then
  test_fail "$SMOKE_NAME: micro_stat does not source bench_compare_common"
  exit 1
fi
if ! grep -q 'source \"${ROOT_DIR}/tools/perf/lib/aot_helpers.sh\"' "$MICRO_STAT"; then
  test_fail "$SMOKE_NAME: micro_stat does not source aot_helpers"
  exit 1
fi
if ! grep -q 'perf_ny_${KEY}.${BASHPID}.exe' "$BENCH_COMPARE"; then
  test_fail "$SMOKE_NAME: bench_compare does not use pid-suffixed AOT exe path"
  exit 1
fi
if ! grep -q 'perf_vm_timeout_resolve heavy' "$APPS_WALLCLOCK"; then
  test_fail "$SMOKE_NAME: bench_apps_wallclock does not use resolver heavy profile"
  exit 1
fi
if ! grep -q 'perf_vm_timeout_resolve heavy' "$STABILITY"; then
  test_fail "$SMOKE_NAME: record_baseline_stability does not use resolver heavy profile"
  exit 1
fi
if ! grep -q 'perf_vm_timeout_resolve heavy' "$REG_GUARD"; then
  test_fail "$SMOKE_NAME: regression_guard does not use resolver heavy profile"
  exit 1
fi

if ! grep -q 'NYASH_LLVM_FAST=1' "$AOT_HELPERS"; then
  test_fail "$SMOKE_NAME: aot_helpers does not pin NYASH_LLVM_FAST=1"
  exit 1
fi
if ! grep -q 'NYASH_LLVM_FAST_INT="${NYASH_LLVM_FAST_INT:-1}"' "$AOT_HELPERS"; then
  test_fail "$SMOKE_NAME: aot_helpers does not pin FAST_INT default"
  exit 1
fi
if ! grep -q 'NYASH_GC_MODE="${NYASH_GC_MODE:-off}"' "$AOT_HELPERS"; then
  test_fail "$SMOKE_NAME: aot_helpers does not pin AOT GC mode default"
  exit 1
fi
if ! grep -q 'NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}"' "$AOT_HELPERS"; then
  test_fail "$SMOKE_NAME: aot_helpers does not pin AOT safepoint poll default"
  exit 1
fi
if ! grep -q 'NYASH_SCHED_POLL_IN_SAFEPOINT=\${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}' "$BENCH_ENV"; then
  test_fail "$SMOKE_NAME: bench_env does not pin safepoint poll policy default"
  exit 1
fi

test_pass "$SMOKE_NAME"

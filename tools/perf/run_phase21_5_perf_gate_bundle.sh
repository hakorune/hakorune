#!/usr/bin/env bash
set -euo pipefail

# run_phase21_5_perf_gate_bundle.sh
# Single-entry wrapper for phase21.5 perf gate optional toggles.
#
# Usage:
#   tools/perf/run_phase21_5_perf_gate_bundle.sh [quick|hotpath|apps|full]

PROFILE="${1:-quick}"
ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
GATE="${ROOT_DIR}/tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh"

if [[ ! -f "${GATE}" ]]; then
  echo "[perf-gate-bundle] gate script not found: ${GATE}" >&2
  exit 2
fi

declare -a TOGGLES=()

add_toggle() {
  TOGGLES+=("$1=1")
}

load_hotpath_toggles() {
  add_toggle PERF_GATE_FAST_IR_PASSES_CHECK
  add_toggle PERF_GATE_NUMERIC_COMPARE_CHAIN_CHECK
  add_toggle PERF_GATE_NUMERIC_HOT_TRACE_CHECK
  add_toggle PERF_GATE_METHOD_CALL_HOT_TRACE_CHECK
  add_toggle PERF_GATE_COPY_CONST_HOTSPOT_CHECK
  add_toggle PERF_GATE_FAST_NATIVE_CODEGEN_CHECK
  add_toggle PERF_GATE_NUMERIC_ARITH_CSE_CHECK
  add_toggle PERF_GATE_OPT_LEVEL_CHECK
  add_toggle PERF_GATE_COMPARE_EXPR_CSE_CHECK
  add_toggle PERF_GATE_COMPARE_REUSE_AOT_CEILING_CHECK
  add_toggle PERF_GATE_LADDER_EXTRA_MEDIUM_CHECK
}

load_apps_toggles() {
  add_toggle PERF_GATE_APPS_CASE_BREAKDOWN_CHECK
  add_toggle PERF_GATE_MIR_SHAPE_PROFILE_CHECK
  add_toggle PERF_GATE_APPS_SUBTRACT_CHECK
  add_toggle PERF_GATE_APPS_MIR_MODE_CHECK
  add_toggle PERF_GATE_APPS_MIR_MODE_DELTA_CHECK
  add_toggle PERF_GATE_APPS_MIR_MODE_SPREAD_CHECK
  add_toggle PERF_GATE_APPS_MIR_MODE_SIGNIFICANCE_CHECK
  add_toggle PERF_GATE_APPS_MIR_MODE_CASE_HOTSPOT_CHECK
  add_toggle PERF_GATE_APPS_COMPILE_RUN_SPLIT_CHECK
  add_toggle PERF_GATE_APPS_EMIT_ROUTE_CHECK
  add_toggle PERF_GATE_APPS_ENTRY_MODE_CHECK
  add_toggle PERF_GATE_APPS_ENTRY_MODE_DELTA_CHECK
  add_toggle PERF_GATE_APPS_ENTRY_MODE_SIGNIFICANCE_CHECK
  add_toggle PERF_GATE_APPS_ENTRY_MODE_SPREAD_CHECK
  add_toggle PERF_GATE_APPS_ENTRY_MODE_CASE_HOTSPOT_CHECK
}

case "${PROFILE}" in
  quick)
    ;;
  hotpath)
    load_hotpath_toggles
    ;;
  apps)
    load_apps_toggles
    ;;
  full)
    load_hotpath_toggles
    load_apps_toggles
    add_toggle PERF_GATE_REGRESSION_CHECK
    add_toggle PERF_GATE_GUARD_LIB_CHECK
    add_toggle PERF_GATE_BENCH_ENV_CHECK
    add_toggle PERF_GATE_BENCH_COMPARE_ENV_CHECK
    add_toggle PERF_GATE_BENCH_COMPILE_RUN_SPLIT_CHECK
    add_toggle PERF_GATE_AOT_SKIP_BUILD_CHECK
    add_toggle PERF_GATE_AOT_AUTO_SAFEPOINT_ENV_CHECK
    add_toggle PERF_GATE_AOT_LINK_MODE_CHECK
    add_toggle PERF_GATE_KILO_CROSSLANG_CHECK
    add_toggle PERF_GATE_KILO_PARITY_LOCK_CHECK
    add_toggle PERF_GATE_APPS_CROSSLANG_BUNDLE_CHECK
    ;;
  *)
    echo "Usage: $0 [quick|hotpath|apps|full]" >&2
    exit 2
    ;;
esac

echo "[perf-gate-bundle] profile=${PROFILE}"
if [[ "${#TOGGLES[@]}" -eq 0 ]]; then
  echo "[perf-gate-bundle] optional toggles: none"
  bash "${GATE}"
  exit $?
fi

echo "[perf-gate-bundle] optional toggles:"
for pair in "${TOGGLES[@]}"; do
  echo "  - ${pair}"
done

env "${TOGGLES[@]}" bash "${GATE}"

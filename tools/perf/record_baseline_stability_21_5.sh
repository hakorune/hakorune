#!/usr/bin/env bash
set -euo pipefail

# record_baseline_stability_21_5.sh
# Run baseline recorder multiple rounds and print drift summary.
#
# Usage:
#   tools/perf/record_baseline_stability_21_5.sh [rounds] [warmup] [repeat]
# Env:
#   PERF_STABILITY_KEYS="box_create_destroy_small method_call_only_small"
#   PERF_STABILITY_INCLUDE_MEDIUM=1   append numeric_mixed_medium
#   PERF_STABILITY_INCLUDE_APPS=1     record apps/tools wallclock summary
#   PERF_STABILITY_INCLUDE_APPS_ENTRY_MODE=1  record app entry-mode compare summary (source vs prebuilt)
#   PERF_STABILITY_ENTRY_MODE_SAMPLES=1       sample count for entry-mode compare
#   PERF_STABILITY_WRITE_BASELINE=1   write guard baseline json
#   PERF_STABILITY_BASELINE_OUT=...   default: benchmarks/baselines/phase21_5_perf_guard.latest.json
#   PERF_STABILITY_KEEP=1  keep temp output directory
#   PERF_SUBTRACT_STARTUP=1

ROUNDS=${1:-3}
WARMUP=${2:-1}
REPEAT=${3:-3}
KEYS=${PERF_STABILITY_KEYS:-"box_create_destroy_small method_call_only_small"}
INCLUDE_MEDIUM=${PERF_STABILITY_INCLUDE_MEDIUM:-0}
INCLUDE_APPS=${PERF_STABILITY_INCLUDE_APPS:-0}
INCLUDE_APPS_ENTRY_MODE=${PERF_STABILITY_INCLUDE_APPS_ENTRY_MODE:-$INCLUDE_APPS}
ENTRY_MODE_SAMPLES=${PERF_STABILITY_ENTRY_MODE_SAMPLES:-1}
WRITE_BASELINE=${PERF_STABILITY_WRITE_BASELINE:-0}

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
RECORDER="${ROOT_DIR}/tools/perf/record_baselines.sh"
APP_BENCH="${ROOT_DIR}/tools/perf/bench_apps_wallclock.sh"
ENTRY_MODE_COMPARE="${ROOT_DIR}/tools/perf/bench_apps_entry_mode_compare.sh"
REPORTER="${ROOT_DIR}/tools/perf/lib/stability_report.py"
source "${ROOT_DIR}/tools/perf/lib/bench_env.sh"
STABILITY_VM_TIMEOUT="$(perf_vm_timeout_resolve heavy)"
BASELINE_OUT="${PERF_STABILITY_BASELINE_OUT:-${ROOT_DIR}/benchmarks/baselines/phase21_5_perf_guard.latest.json}"
OUT_DIR=$(mktemp -d /tmp/perf_stability_21_5.XXXXXX)

cleanup() {
  if [[ "${PERF_STABILITY_KEEP:-0}" == "1" ]]; then
    echo "[stability] keep out dir: ${OUT_DIR}" >&2
  else
    rm -rf "${OUT_DIR}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

if [[ ! -x "${RECORDER}" ]]; then
  echo "[error] recorder not executable: ${RECORDER}" >&2
  exit 2
fi
if [[ ! -f "${REPORTER}" ]]; then
  echo "[error] stability reporter missing: ${REPORTER}" >&2
  exit 2
fi
if [[ "${INCLUDE_APPS}" == "1" ]] && [[ ! -f "${APP_BENCH}" ]]; then
  echo "[error] app bench runner missing: ${APP_BENCH}" >&2
  exit 2
fi
if [[ "${INCLUDE_APPS_ENTRY_MODE}" == "1" ]] && [[ ! -f "${ENTRY_MODE_COMPARE}" ]]; then
  echo "[error] app entry-mode compare script missing: ${ENTRY_MODE_COMPARE}" >&2
  exit 2
fi
if ! [[ "${ENTRY_MODE_SAMPLES}" =~ ^[0-9]+$ ]] || [[ "${ENTRY_MODE_SAMPLES}" -le 0 ]]; then
  echo "[error] PERF_STABILITY_ENTRY_MODE_SAMPLES must be positive integer: ${ENTRY_MODE_SAMPLES}" >&2
  exit 2
fi

if [[ "${INCLUDE_MEDIUM}" == "1" ]]; then
  case " ${KEYS} " in
    *" numeric_mixed_medium "*) ;;
    *) KEYS="${KEYS} numeric_mixed_medium" ;;
  esac
fi

echo "[stability] rounds=${ROUNDS} warmup=${WARMUP} repeat=${REPEAT} keys=${KEYS} include_apps=${INCLUDE_APPS} include_apps_entry_mode=${INCLUDE_APPS_ENTRY_MODE}"
for r in $(seq 1 "${ROUNDS}"); do
  echo "[stability] round=${r}/${ROUNDS}"
  for key in ${KEYS}; do
    PERF_BASELINE_OUT_DIR="${OUT_DIR}" bash "${RECORDER}" "${key}" "${WARMUP}" "${REPEAT}" >/dev/null
  done
  if [[ "${INCLUDE_APPS}" == "1" ]]; then
    app_log="${OUT_DIR}/apps_vm.round${r}.log"
    app_err="${OUT_DIR}/apps_vm.round${r}.err"
    if ! PERF_VM_TIMEOUT="${STABILITY_VM_TIMEOUT}" PERF_APPS_OUTPUT=json bash "${APP_BENCH}" "${WARMUP}" "${REPEAT}" vm >"${app_log}" 2>"${app_err}"; then
      echo "[error] app bench failed at round ${r}" >&2
      tail -n 40 "${app_err}" >&2 || true
      exit 1
    fi
  fi
  if [[ "${INCLUDE_APPS_ENTRY_MODE}" == "1" ]]; then
    entry_log="${OUT_DIR}/apps_entry_mode.round${r}.json"
    entry_err="${OUT_DIR}/apps_entry_mode.round${r}.err"
    if ! PERF_VM_TIMEOUT="${STABILITY_VM_TIMEOUT}" \
      PERF_APPS_ENTRY_MODE_DELTA_SAMPLES="${ENTRY_MODE_SAMPLES}" \
      bash "${ENTRY_MODE_COMPARE}" "${WARMUP}" "${REPEAT}" vm >"${entry_log}" 2>"${entry_err}"; then
      echo "[error] app entry-mode compare failed at round ${r}" >&2
      tail -n 40 "${entry_err}" >&2 || true
      exit 1
    fi
  fi
done

python3 "${REPORTER}" "${OUT_DIR}" "${WRITE_BASELINE}" "${BASELINE_OUT}" "${ROUNDS}" "${WARMUP}" "${REPEAT}"

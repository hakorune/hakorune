#!/usr/bin/env bash
set -euo pipefail

# bench_crosslang_apps_bundle.sh
# Unified replay harness for APP-PERF-03:
# - micro cross-language baseline (chip8/kilo): C/Python/Nyash VM/Nyash AOT
# - real-app wallclock baseline (apps/tools): Nyash VM
# - app entry mode compare (source vs mir_shape_prebuilt): Nyash VM
#
# Usage:
#   tools/perf/bench_crosslang_apps_bundle.sh [bench_warmup] [bench_repeat] [app_warmup] [app_repeat]
# Env:
#   PERF_BUNDLE_INCLUDE_ENTRY_MODE=1|0  # default: 1
#   PERF_APPS_ENTRY_MODE_DELTA_SAMPLES=<N>  # forwarded to entry-mode compare
# Output (single line):
#   [bench4-app] chip8_aot_status=<ok|skip|fail> chip8_ratio_c_aot=<r> chip8_ny_aot_ms=<n> \
#                kilo_aot_status=<ok|skip|fail> kilo_ratio_c_aot=<r> kilo_ny_aot_ms=<n> \
#                apps_total_ms=<n> apps_hotspot_case=<name> apps_hotspot_ms=<n> \
#                entry_source_total_ms=<n> entry_prebuilt_total_ms=<n> entry_delta_ms=<n> entry_winner=<name>

BENCH_WARMUP=${1:-1}
BENCH_REPEAT=${2:-1}
APP_WARMUP=${3:-1}
APP_REPEAT=${4:-1}
PERF_BUNDLE_INCLUDE_ENTRY_MODE=${PERF_BUNDLE_INCLUDE_ENTRY_MODE:-1}

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
BENCH4="${ROOT_DIR}/tools/perf/bench_compare_c_py_vs_hako.sh"
APPS_BENCH="${ROOT_DIR}/tools/perf/bench_apps_wallclock.sh"
ENTRY_COMPARE="${ROOT_DIR}/tools/perf/bench_apps_entry_mode_compare.sh"

if [[ ! -x "${BENCH4}" ]]; then
  echo "[error] script not executable: ${BENCH4}" >&2
  exit 2
fi
if [[ ! -f "${APPS_BENCH}" ]]; then
  echo "[error] script not found: ${APPS_BENCH}" >&2
  exit 2
fi
if [[ ! -f "${ENTRY_COMPARE}" ]]; then
  echo "[error] script not found: ${ENTRY_COMPARE}" >&2
  exit 2
fi

extract_field() {
  local line="$1"
  local key="$2"
  printf '%s\n' "${line}" | tr ' ' '\n' | sed -n "s/^${key}=//p" | head -n1
}

require_numeric_field() {
  local line="$1"
  local key="$2"
  local value
  value="$(extract_field "${line}" "${key}")"
  if ! [[ "${value}" =~ ^[0-9]+([.][0-9]+)?$ ]]; then
    echo "[error] missing/invalid field: ${key}" >&2
    echo "${line}" >&2
    exit 1
  fi
  printf '%s\n' "${value}"
}

require_status_field() {
  local line="$1"
  local key="$2"
  local value
  value="$(extract_field "${line}" "${key}")"
  case "${value}" in
    ok|skip|fail)
      printf '%s\n' "${value}"
      ;;
    *)
      echo "[error] missing/invalid status field: ${key}" >&2
      echo "${line}" >&2
      exit 1
      ;;
  esac
}

run_bench4_case() {
  local key="$1"
  local out line
  out="$(
    NYASH_LLVM_SKIP_BUILD="${NYASH_LLVM_SKIP_BUILD:-1}" \
    PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
    HAKO_VM_MAX_STEPS="${HAKO_VM_MAX_STEPS:-300000000}" \
    bash "${BENCH4}" "${key}" "${BENCH_WARMUP}" "${BENCH_REPEAT}" 2>&1
  )" || {
    echo "${out}" >&2
    echo "[error] bench4 run failed: key=${key}" >&2
    exit 1
  }
  line="$(printf '%s\n' "${out}" | grep "\[bench4\] name=${key} " | tail -n1 || true)"
  if [[ -z "${line}" ]]; then
    echo "${out}" >&2
    echo "[error] missing bench4 summary line for key=${key}" >&2
    exit 1
  fi
  printf '%s\n' "${line}"
}

chip8_line="$(run_bench4_case "chip8_kernel_small")"
kilo_line="$(run_bench4_case "kilo_kernel_small")"

chip8_ratio_c_aot="$(require_numeric_field "${chip8_line}" "ratio_c_aot")"
chip8_ny_aot_ms="$(require_numeric_field "${chip8_line}" "ny_aot_ms")"
chip8_aot_status="$(require_status_field "${chip8_line}" "aot_status")"

kilo_ratio_c_aot="$(require_numeric_field "${kilo_line}" "ratio_c_aot")"
kilo_ny_aot_ms="$(require_numeric_field "${kilo_line}" "ny_aot_ms")"
kilo_aot_status="$(require_status_field "${kilo_line}" "aot_status")"

apps_json="$(
  PERF_APPS_OUTPUT=json \
  PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
  bash "${APPS_BENCH}" "${APP_WARMUP}" "${APP_REPEAT}" vm 2>&1
)" || {
  echo "${apps_json}" >&2
  echo "[error] apps wallclock failed" >&2
  exit 1
}

if ! printf '%s\n' "${apps_json}" | jq -e . >/dev/null 2>&1; then
  echo "${apps_json}" >&2
  echo "[error] apps wallclock output is not JSON" >&2
  exit 1
fi

apps_total_ms="$(printf '%s\n' "${apps_json}" | jq -r '.total_ms // 0')"
apps_hotspot_case="$(printf '%s\n' "${apps_json}" | jq -r '.hotspot.case // ""')"
apps_hotspot_ms="$(printf '%s\n' "${apps_json}" | jq -r '.hotspot.ms // 0')"

if ! [[ "${apps_total_ms}" =~ ^[0-9]+$ ]] || [[ "${apps_total_ms}" -le 0 ]]; then
  echo "${apps_json}" >&2
  echo "[error] invalid apps total_ms: ${apps_total_ms}" >&2
  exit 1
fi
if [[ -z "${apps_hotspot_case}" ]]; then
  echo "${apps_json}" >&2
  echo "[error] missing apps hotspot.case" >&2
  exit 1
fi
if ! [[ "${apps_hotspot_ms}" =~ ^[0-9]+$ ]] || [[ "${apps_hotspot_ms}" -le 0 ]]; then
  echo "${apps_json}" >&2
  echo "[error] invalid apps hotspot.ms: ${apps_hotspot_ms}" >&2
  exit 1
fi

entry_source_total_ms=0
entry_prebuilt_total_ms=0
entry_delta_ms=0
entry_winner="skip"

if [[ "${PERF_BUNDLE_INCLUDE_ENTRY_MODE}" == "1" ]]; then
  entry_json="$(
    PERF_VM_TIMEOUT="${PERF_VM_TIMEOUT:-60s}" \
    PERF_APPS_ENTRY_MODE_DELTA_SAMPLES="${PERF_APPS_ENTRY_MODE_DELTA_SAMPLES:-1}" \
    bash "${ENTRY_COMPARE}" "${APP_WARMUP}" "${APP_REPEAT}" vm 2>&1
  )" || {
    echo "${entry_json}" >&2
    echo "[error] entry mode compare failed" >&2
    exit 1
  }
  if ! printf '%s\n' "${entry_json}" | jq -e . >/dev/null 2>&1; then
    echo "${entry_json}" >&2
    echo "[error] entry mode output is not JSON" >&2
    exit 1
  fi

  entry_source_total_ms="$(printf '%s\n' "${entry_json}" | jq -r '.source_total_ms // 0')"
  entry_prebuilt_total_ms="$(printf '%s\n' "${entry_json}" | jq -r '.mir_shape_prebuilt_total_ms // 0')"
  entry_delta_ms="$(printf '%s\n' "${entry_json}" | jq -r '.delta_ms // 0')"
  entry_winner="$(printf '%s\n' "${entry_json}" | jq -r '.winner // ""')"

  if ! [[ "${entry_source_total_ms}" =~ ^[0-9]+$ ]] || [[ "${entry_source_total_ms}" -le 0 ]]; then
    echo "${entry_json}" >&2
    echo "[error] invalid entry source_total_ms: ${entry_source_total_ms}" >&2
    exit 1
  fi
  if ! [[ "${entry_prebuilt_total_ms}" =~ ^[0-9]+$ ]] || [[ "${entry_prebuilt_total_ms}" -le 0 ]]; then
    echo "${entry_json}" >&2
    echo "[error] invalid entry mir_shape_prebuilt_total_ms: ${entry_prebuilt_total_ms}" >&2
    exit 1
  fi
  if ! [[ "${entry_delta_ms}" =~ ^-?[0-9]+$ ]]; then
    echo "${entry_json}" >&2
    echo "[error] invalid entry delta_ms: ${entry_delta_ms}" >&2
    exit 1
  fi
  case "${entry_winner}" in
    source|mir_shape_prebuilt)
      ;;
    *)
      echo "${entry_json}" >&2
      echo "[error] invalid entry winner: ${entry_winner}" >&2
      exit 1
      ;;
  esac
elif [[ "${PERF_BUNDLE_INCLUDE_ENTRY_MODE}" != "0" ]]; then
  echo "[error] PERF_BUNDLE_INCLUDE_ENTRY_MODE must be 0|1 (got: ${PERF_BUNDLE_INCLUDE_ENTRY_MODE})" >&2
  exit 2
fi

printf "[bench4-app] chip8_aot_status=%s chip8_ratio_c_aot=%s chip8_ny_aot_ms=%s kilo_aot_status=%s kilo_ratio_c_aot=%s kilo_ny_aot_ms=%s apps_total_ms=%s apps_hotspot_case=%s apps_hotspot_ms=%s entry_source_total_ms=%s entry_prebuilt_total_ms=%s entry_delta_ms=%s entry_winner=%s\n" \
  "${chip8_aot_status}" "${chip8_ratio_c_aot}" "${chip8_ny_aot_ms}" \
  "${kilo_aot_status}" "${kilo_ratio_c_aot}" "${kilo_ny_aot_ms}" \
  "${apps_total_ms}" "${apps_hotspot_case}" "${apps_hotspot_ms}" \
  "${entry_source_total_ms}" "${entry_prebuilt_total_ms}" "${entry_delta_ms}" "${entry_winner}"

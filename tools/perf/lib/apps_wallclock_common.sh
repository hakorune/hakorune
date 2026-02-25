#!/usr/bin/env bash
set -euo pipefail

# Shared helpers for tools/perf/bench_apps_wallclock.sh

apps_wallclock_validate_inputs() {
  if [[ "${PERF_APPS_OUTPUT}" != "text" && "${PERF_APPS_OUTPUT}" != "json" ]]; then
    echo "[error] PERF_APPS_OUTPUT must be text|json: got ${PERF_APPS_OUTPUT}" >&2
    exit 2
  fi
  if [[ "${PERF_APPS_SUBTRACT_STARTUP}" != "0" && "${PERF_APPS_SUBTRACT_STARTUP}" != "1" ]]; then
    echo "[error] PERF_APPS_SUBTRACT_STARTUP must be 0|1: got ${PERF_APPS_SUBTRACT_STARTUP}" >&2
    exit 2
  fi
  if [[ "${PERF_APPS_MIR_SHAPE_INPUT_MODE}" != "emit" && "${PERF_APPS_MIR_SHAPE_INPUT_MODE}" != "prebuilt" ]]; then
    echo "[error] PERF_APPS_MIR_SHAPE_INPUT_MODE must be emit|prebuilt: got ${PERF_APPS_MIR_SHAPE_INPUT_MODE}" >&2
    exit 2
  fi
  if [[ "${PERF_APPS_ENTRY_MODE}" != "source" && "${PERF_APPS_ENTRY_MODE}" != "mir_shape_prebuilt" ]]; then
    echo "[error] PERF_APPS_ENTRY_MODE must be source|mir_shape_prebuilt: got ${PERF_APPS_ENTRY_MODE}" >&2
    exit 2
  fi
}

time_ms() { date +%s%3N; }

measure_cmd_ms_strict() {
  local cmd=("$@")
  local t1 t2 dt
  t1=$(time_ms)
  if ! "${cmd[@]}" >/dev/null 2>&1; then
    return 1
  fi
  t2=$(time_ms)
  dt=$((t2 - t1))
  echo "$dt"
}

median_ms() {
  awk 'NF{print $1}' | sort -n | awk '{ a[NR]=$1 } END { if (NR==0) {print 0; exit} n=int((NR+1)/2); print a[n] }'
}

collect_series_strict() {
  local warmup=$1; shift
  local repeat=$1; shift
  local -a cmd=("$@")
  local sample
  for _ in $(seq 1 "${warmup}"); do
    measure_cmd_ms_strict "${cmd[@]}" >/dev/null
  done
  for _ in $(seq 1 "${repeat}"); do
    sample=$(measure_cmd_ms_strict "${cmd[@]}")
    printf "%s\n" "$sample"
  done
}

split_env_spec() {
  local spec="$1"
  SPLIT_ENV_KV=()
  if [[ -n "${spec}" ]]; then
    IFS=';' read -r -a SPLIT_ENV_KV <<<"${spec}"
  fi
}

find_hotspot_case() {
  local metric="$1"
  HOTSPOT_CASE="${CASE_NAMES[0]}"
  if [[ "${metric}" == "net" ]]; then
    HOTSPOT_MS="${CASE_NET_MS[${HOTSPOT_CASE}]}"
  else
    HOTSPOT_MS="${CASE_MS[${HOTSPOT_CASE}]}"
  fi

  local case_name candidate
  for case_name in "${CASE_NAMES[@]}"; do
    if [[ "${metric}" == "net" ]]; then
      candidate="${CASE_NET_MS[${case_name}]}"
    else
      candidate="${CASE_MS[${case_name}]}"
    fi
    if [[ "${candidate}" -gt "${HOTSPOT_MS}" ]]; then
      HOTSPOT_CASE="${case_name}"
      HOTSPOT_MS="${candidate}"
    fi
  done
}

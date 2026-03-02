#!/usr/bin/env bash
set -euo pipefail

# bench_apps_wallclock.sh
# Measure wall-clock median for small real apps under apps/tools.
#
# Usage:
#   tools/perf/bench_apps_wallclock.sh [warmup] [repeat] [backend]
# Example:
#   tools/perf/bench_apps_wallclock.sh 1 3 vm
# Env:
#   PERF_VM_TIMEOUT=<duration>  per-run timeout (default from bench_env heavy profile)
#   PERF_APPS_OUTPUT=text    text|json (default text)
#   PERF_APPS_SUBTRACT_STARTUP=0|1  subtract startup baseline from each case
#   PERF_APPS_MIR_SHAPE_INPUT_MODE=emit|prebuilt  (default emit)
#   PERF_APPS_MIR_SHAPE_PREBUILT=/path/to/file.mir.json (mode=prebuilt)
#   PERF_APPS_MIR_SHAPE_SOURCE=/path/to/src.hako (mode=emit)
#   PERF_APPS_ENTRY_MODE=source|mir_shape_prebuilt (default source)
# JSON contract extension:
#   timing_ms.prepare       mode setup + MIR input preparation
#   timing_ms.mir_emit      emit helper elapsed (emit mode only; prebuilt=0)
#   timing_ms.startup_probe startup baseline probe elapsed (when subtract=1; else 0)
#   timing_ms.run           equals total_ms (sum of case medians)

WARMUP=${1:-1}
REPEAT=${2:-3}
BACKEND=${3:-vm}
PERF_APPS_OUTPUT=${PERF_APPS_OUTPUT:-text}
PERF_APPS_SUBTRACT_STARTUP=${PERF_APPS_SUBTRACT_STARTUP:-0}
PERF_APPS_MIR_SHAPE_INPUT_MODE=${PERF_APPS_MIR_SHAPE_INPUT_MODE:-emit}
PERF_APPS_MIR_SHAPE_PREBUILT=${PERF_APPS_MIR_SHAPE_PREBUILT:-}
PERF_APPS_MIR_SHAPE_SOURCE=${PERF_APPS_MIR_SHAPE_SOURCE:-}
PERF_APPS_ENTRY_MODE=${PERF_APPS_ENTRY_MODE:-source}

ROOT_DIR=$(cd "$(dirname "$0")/../.." && pwd)
TARGET_DIR="${ROOT_DIR}/target"
HAKORUNE_BIN="${TARGET_DIR}/release/hakorune"
source "${ROOT_DIR}/tools/perf/lib/bench_env.sh"
source "${ROOT_DIR}/tools/perf/lib/apps_wallclock_cases.sh"
source "${ROOT_DIR}/tools/perf/lib/apps_wallclock_common.sh"
PERF_TIMEOUT="$(perf_vm_timeout_resolve heavy)"

apps_wallclock_validate_inputs

if [[ ! -x "${HAKORUNE_BIN}" ]]; then
  echo "[hint] hakorune not built. Run: cargo build --release" >&2
  exit 2
fi

run_case() {
  local name=$1
  local entry_target=$2
  local -a env_kv=("${!3}")
  local -a app_args=("${!4}")
  local use_mir_entry=${5:-0}
  local series med
  local -a run_cmd=()

  if [[ ! -f "${entry_target}" ]]; then
    echo "[error] app entry target not found: ${entry_target}" >&2
    return 1
  fi

  if [[ "${use_mir_entry}" == "1" ]]; then
    run_cmd=("${HAKORUNE_BIN}" --backend "${BACKEND}" --mir-json-file "${entry_target}")
  else
    run_cmd=("${HAKORUNE_BIN}" --backend "${BACKEND}" "${entry_target}" "${app_args[@]}")
  fi

  series=$(collect_series_strict "${WARMUP}" "${REPEAT}" \
    env "${NYASH_VM_BENCH_ENV[@]}" "${env_kv[@]}" \
    timeout "${PERF_TIMEOUT}" "${run_cmd[@]}")
  med=$(printf "%s\n" "${series}" | median_ms)
  if ! [[ "${med}" =~ ^[0-9]+$ ]]; then
    echo "[error] app bench invalid median: name=${name} backend=${BACKEND} ms=${med}" >&2
    return 1
  fi
  RUN_CASE_LAST_MED="${med}"
  if [[ "${PERF_APPS_OUTPUT}" == "text" ]]; then
    printf "[app-bench] name=%-22s backend=%-3s ms=%s\n" "${name}" "${BACKEND}" "${med}"
  fi
}

FIXTURE_LOG="${ROOT_DIR}/apps/tests/gate_log_summarizer/sample_mixed.log"
if [[ -n "${PERF_APPS_MIR_SHAPE_SOURCE}" ]]; then
  MIR_SOURCE="${PERF_APPS_MIR_SHAPE_SOURCE}"
else
  MIR_SOURCE="${ROOT_DIR}/benchmarks/bench_method_call_only_small.hako"
fi
if [[ -n "${PERF_APPS_MIR_SHAPE_PREBUILT}" ]]; then
  MIR_PREBUILT="${PERF_APPS_MIR_SHAPE_PREBUILT}"
else
  MIR_PREBUILT="${ROOT_DIR}/apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json"
fi
EMIT_HELPER="${ROOT_DIR}/tools/hakorune_emit_mir.sh"

if [[ ! -f "${FIXTURE_LOG}" ]]; then
  echo "[error] fixture log not found: ${FIXTURE_LOG}" >&2
  exit 2
fi
TMP_MIR=""
TMP_RET0=$(mktemp /tmp/perf_app_startup.XXXXXX.hako)
TMP_APP_MIRS=()
cleanup() {
  if [[ -n "${TMP_MIR}" ]]; then
    rm -f "${TMP_MIR}" >/dev/null 2>&1 || true
  fi
  for t in "${TMP_APP_MIRS[@]:-}"; do
    rm -f "${t}" >/dev/null 2>&1 || true
  done
  rm -f "${TMP_RET0}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

mir_shape_input_mode="${PERF_APPS_MIR_SHAPE_INPUT_MODE}"
mir_shape_input=""
prepare_ms=0
mir_emit_ms=0
prepare_t0=$(time_ms)
if [[ "${mir_shape_input_mode}" == "emit" ]]; then
  if [[ ! -f "${EMIT_HELPER}" ]]; then
    echo "[error] emit helper not found: ${EMIT_HELPER}" >&2
    exit 2
  fi
  if [[ ! -f "${MIR_SOURCE}" ]]; then
    echo "[error] mir source not found: ${MIR_SOURCE}" >&2
    exit 2
  fi
  TMP_MIR=$(mktemp /tmp/perf_app_mir_shape.XXXXXX.json)
  emit_t0=$(time_ms)
  if ! bash "${EMIT_HELPER}" "${MIR_SOURCE}" "${TMP_MIR}" >/dev/null 2>&1; then
    echo "[error] failed to emit MIR for mir_shape_guard input: ${MIR_SOURCE}" >&2
    exit 1
  fi
  emit_t1=$(time_ms)
  mir_emit_ms=$((emit_t1 - emit_t0))
  if [[ "${mir_emit_ms}" -lt 0 ]]; then
    mir_emit_ms=0
  fi
  mir_shape_input="${TMP_MIR}"
else
  if [[ ! -f "${MIR_PREBUILT}" ]]; then
    echo "[error] prebuilt MIR not found: ${MIR_PREBUILT}" >&2
    exit 2
  fi
  if [[ ! -s "${MIR_PREBUILT}" ]]; then
    echo "[error] prebuilt MIR is empty: ${MIR_PREBUILT}" >&2
    exit 2
  fi
  mir_shape_input="${MIR_PREBUILT}"
fi
prepare_t1=$(time_ms)
prepare_ms=$((prepare_t1 - prepare_t0))
if [[ "${prepare_ms}" -lt 0 ]]; then
  prepare_ms=0
fi

startup_ms=0
startup_probe_ms=0
if [[ "${PERF_APPS_SUBTRACT_STARTUP}" == "1" ]]; then
  cat > "${TMP_RET0}" <<'HAKO'
static box Main { main() { return 0 } }
HAKO
  startup_probe_t0=$(time_ms)
  startup_series=$(collect_series_strict 1 3 \
    env "${NYASH_VM_BENCH_ENV[@]}" \
    timeout "${PERF_TIMEOUT}" "${HAKORUNE_BIN}" --backend "${BACKEND}" "${TMP_RET0}")
  startup_probe_t1=$(time_ms)
  startup_probe_ms=$((startup_probe_t1 - startup_probe_t0))
  if [[ "${startup_probe_ms}" -lt 0 ]]; then
    startup_probe_ms=0
  fi
  startup_ms=$(printf "%s\n" "${startup_series}" | median_ms)
  if ! [[ "${startup_ms}" =~ ^[0-9]+$ ]]; then
    echo "[error] invalid startup median: ${startup_ms}" >&2
    exit 1
  fi
fi

bench_apps_build_case_registry "${ROOT_DIR}" "${FIXTURE_LOG}" "${mir_shape_input}"
declare -A CASE_MS=()
declare -A CASE_NET_MS=()
declare -A CASE_ENTRY_TARGET=()
declare -A CASE_USE_MIR_ENTRY=()

for case_name in "${CASE_NAMES[@]}"; do
  CASE_ENTRY_TARGET["${case_name}"]="${CASE_APP[${case_name}]}"
  CASE_USE_MIR_ENTRY["${case_name}"]="0"
done

if [[ "${PERF_APPS_ENTRY_MODE}" == "mir_shape_prebuilt" ]]; then
  for case_name in "${CASE_NAMES[@]}"; do
    if [[ "${case_name}" != "mir_shape_guard" ]]; then
      continue
    fi
    app_src="${CASE_APP[${case_name}]}"
    if [[ ! -f "${app_src}" ]]; then
      echo "[error] app source not found: ${app_src}" >&2
      exit 2
    fi
    tmp_app_mir=$(mktemp "/tmp/perf_app_entry_${case_name}.XXXXXX.json")
    if ! "${HAKORUNE_BIN}" --emit-mir-json "${tmp_app_mir}" "${app_src}" >/dev/null 2>&1; then
      echo "[error] failed to emit app MIR for entry mode mir_shape_prebuilt: ${app_src}" >&2
      exit 1
    fi
    TMP_APP_MIRS+=("${tmp_app_mir}")
    CASE_ENTRY_TARGET["${case_name}"]="${tmp_app_mir}"
    CASE_USE_MIR_ENTRY["${case_name}"]="1"
  done
fi

for case_name in "${CASE_NAMES[@]}"; do
  split_env_spec "${CASE_ENV_SPEC[${case_name}]}"
  case_env=("${SPLIT_ENV_KV[@]}")
  case_args=()
  run_case "${case_name}" "${CASE_ENTRY_TARGET[${case_name}]}" case_env[@] case_args[@] "${CASE_USE_MIR_ENTRY[${case_name}]}"
  CASE_MS["${case_name}"]="${RUN_CASE_LAST_MED}"
  CASE_NET_MS["${case_name}"]="${RUN_CASE_LAST_MED}"
  if [[ "${PERF_APPS_SUBTRACT_STARTUP}" == "1" ]]; then
    CASE_NET_MS["${case_name}"]=$(( RUN_CASE_LAST_MED > startup_ms ? RUN_CASE_LAST_MED - startup_ms : 0 ))
  fi
done

total_ms=0
net_total_ms=0
for case_name in "${CASE_NAMES[@]}"; do
  total_ms=$(( total_ms + CASE_MS[${case_name}] ))
  net_total_ms=$(( net_total_ms + CASE_NET_MS[${case_name}] ))
done

hotspot_metric="raw"
if [[ "${PERF_APPS_SUBTRACT_STARTUP}" == "1" ]]; then
  hotspot_metric="net"
fi
find_hotspot_case "${hotspot_metric}"
hotspot_case="${HOTSPOT_CASE}"
hotspot_ms="${HOTSPOT_MS}"

if [[ "${PERF_APPS_OUTPUT}" == "text" ]]; then
  printf "[app-bench] total backend=%-3s ms=%s\n" "${BACKEND}" "${total_ms}"
  printf "[app-bench] app_entry mode=%s\n" "${PERF_APPS_ENTRY_MODE}"
  printf "[app-bench] mir_shape_input mode=%s\n" "${mir_shape_input_mode}"
  printf "[app-bench] timing_ms prepare=%s mir_emit=%s startup_probe=%s run=%s\n" \
    "${prepare_ms}" "${mir_emit_ms}" "${startup_probe_ms}" "${total_ms}"
  printf "[app-bench] hotspot metric=%s case=%s ms=%s\n" "${hotspot_metric}" "${hotspot_case}" "${hotspot_ms}"
  if [[ "${PERF_APPS_SUBTRACT_STARTUP}" == "1" ]]; then
    printf "[app-bench] startup_ms=%s net_total_ms=%s\n" "${startup_ms}" "${net_total_ms}"
  fi
else
  {
    for case_name in "${CASE_NAMES[@]}"; do
      printf "raw\t%s\t%s\n" "${case_name}" "${CASE_MS[${case_name}]}"
      if [[ "${PERF_APPS_SUBTRACT_STARTUP}" == "1" ]]; then
        printf "net\t%s\t%s\n" "${case_name}" "${CASE_NET_MS[${case_name}]}"
      fi
    done
  } | python3 -c '
import json
import sys

backend = sys.argv[1]
warmup = int(sys.argv[2])
repeat = int(sys.argv[3])
total_ms = int(sys.argv[4])
subtract_startup = int(sys.argv[5])
startup_ms = int(sys.argv[6])
net_total_ms = int(sys.argv[7])
hotspot_metric = sys.argv[8]
hotspot_case = sys.argv[9]
hotspot_ms = int(sys.argv[10])
mir_shape_input_mode = sys.argv[11]
prepare_ms = int(sys.argv[12])
mir_emit_ms = int(sys.argv[13])
startup_probe_ms = int(sys.argv[14])
app_entry_mode = sys.argv[15]

cases = {}
net_cases = {}
for line in sys.stdin:
    row = line.rstrip("\n")
    if not row:
        continue
    kind, name, value = row.split("\t", 2)
    ivalue = int(value)
    if kind == "raw":
        cases[name] = ivalue
    elif kind == "net":
        net_cases[name] = ivalue

out = {
    "backend": backend,
    "warmup": warmup,
    "repeat": repeat,
    "total_ms": total_ms,
    "app_entry_mode": app_entry_mode,
    "mir_shape_input_mode": mir_shape_input_mode,
    "timing_ms": {
        "prepare": prepare_ms,
        "mir_emit": mir_emit_ms,
        "startup_probe": startup_probe_ms,
        "run": total_ms,
    },
    "cases": cases,
    "hotspot": {
        "metric": hotspot_metric,
        "case": hotspot_case,
        "ms": hotspot_ms,
    },
}
if subtract_startup == 1:
    out["startup_ms"] = startup_ms
    out["net_total_ms"] = net_total_ms
    out["net_cases"] = net_cases

print(json.dumps(out, separators=(",", ":")))
' "$BACKEND" "$WARMUP" "$REPEAT" "$total_ms" \
    "$PERF_APPS_SUBTRACT_STARTUP" "$startup_ms" "$net_total_ms" \
    "$hotspot_metric" "$hotspot_case" "$hotspot_ms" \
    "$mir_shape_input_mode" \
    "$prepare_ms" "$mir_emit_ms" "$startup_probe_ms" \
    "$PERF_APPS_ENTRY_MODE"
fi

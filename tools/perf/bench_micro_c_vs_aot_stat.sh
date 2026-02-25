#!/usr/bin/env bash
set -euo pipefail

# Machine-code focused micro bench:
# - build C reference + Nyash AOT exe for one fixed micro case
# - collect perf stat (instructions/cycles/cache-misses) series
# - report median counters and C/AOT ratios
#
# Usage:
#   tools/perf/bench_micro_c_vs_aot_stat.sh <bench_key> [warmup] [repeat]
#
# Example:
#   tools/perf/bench_micro_c_vs_aot_stat.sh kilo_micro_indexof_line 1 15
#
# Output:
#   [microstat] name=<key> c_instr=<n> c_cycles=<n> c_cache_miss=<n> c_ms=<n> \
#               ny_aot_instr=<n> ny_aot_cycles=<n> ny_aot_cache_miss=<n> ny_aot_ms=<n> \
#               ratio_instr=<r> ratio_cycles=<r> ratio_ms=<r> c_ipc=<r> ny_aot_ipc=<r> aot_status=<ok|skip|fail>

KEY="${1:-}"
WARMUP="${2:-1}"
REPEAT="${3:-11}"

if [[ -z "${KEY}" ]]; then
  echo "Usage: $0 <bench_key> [warmup] [repeat]" >&2
  exit 2
fi

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TARGET_DIR="${ROOT_DIR}/target"
HAKO_PROG="${ROOT_DIR}/benchmarks/bench_${KEY}.hako"
C_SRC="${ROOT_DIR}/benchmarks/c/bench_${KEY}.c"
HAKORUNE_BIN="${TARGET_DIR}/release/hakorune"
C_BIN="${TARGET_DIR}/perf_c_${KEY}.microstat"
NY_AOT_EXE="${TARGET_DIR}/perf_ny_${KEY}.microstat.${BASHPID}.exe"

if [[ ! -x "${HAKORUNE_BIN}" ]]; then
  echo "[error] hakorune binary missing: ${HAKORUNE_BIN}" >&2
  echo "[hint] run: cargo build --release --bin hakorune" >&2
  exit 2
fi
if [[ ! -f "${HAKO_PROG}" ]]; then
  echo "[error] hako benchmark missing: ${HAKO_PROG}" >&2
  exit 2
fi
if [[ ! -f "${C_SRC}" ]]; then
  echo "[error] c benchmark missing: ${C_SRC}" >&2
  exit 2
fi
if ! [[ "${WARMUP}" =~ ^[0-9]+$ && "${REPEAT}" =~ ^[0-9]+$ ]]; then
  echo "[error] warmup/repeat must be unsigned integers" >&2
  exit 2
fi
if [[ "${REPEAT}" -eq 0 ]]; then
  echo "[error] repeat must be >= 1" >&2
  exit 2
fi

mkdir -p "${TARGET_DIR}"
rm -f "${NY_AOT_EXE}" >/dev/null 2>&1 || true

cleanup() {
  rm -f "${NY_AOT_EXE}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

source "${ROOT_DIR}/tools/perf/lib/bench_compare_common.sh"
source "${ROOT_DIR}/tools/perf/lib/aot_helpers.sh"

MICRO_RUN_ENV=(
  NYASH_GC_MODE="${NYASH_GC_MODE:-off}"
  NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}"
  NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}"
  NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}"
)

median_file() {
  local f="$1"
  awk 'NF{print $1}' "${f}" | sort -n | awk '
    { a[NR]=$1 }
    END {
      if (NR == 0) { print 0; exit }
      n = int((NR + 1) / 2)
      print a[n]
    }'
}

ratio_fmt() {
  python3 - "$@" <<'PY'
import sys
num, den = map(float, sys.argv[1:3])
print(f"{(num/den) if den > 0 else 0.0:.2f}")
PY
}

ipc_fmt() {
  python3 - "$@" <<'PY'
import sys
ins, cyc = map(float, sys.argv[1:3])
print(f"{(ins/cyc) if cyc > 0 else 0.0:.2f}")
PY
}

parse_perf_event() {
  local stat_file="$1"
  local event_prefix="$2"
  awk -F';' -v ev="${event_prefix}" '
    $3 ~ ("^" ev) {
      gsub(/ /, "", $1)
      print $1
      exit
    }' "${stat_file}"
}

run_perf_stat_once() {
  local stat_file
  stat_file="$(mktemp --suffix .microstat)"
  set +e
  LC_ALL=C perf stat --no-big-num -x ';' -e instructions,cycles,cache-misses "$@" \
    >/dev/null 2>"${stat_file}"
  local rc=$?
  set -e

  # Benchmarks may intentionally return non-zero.
  # Treat timeout as hard failure only.
  if [[ "${rc}" -eq 124 ]]; then
    rm -f "${stat_file}" >/dev/null 2>&1 || true
    return 124
  fi

  local instr cycles miss
  instr="$(parse_perf_event "${stat_file}" "instructions")"
  cycles="$(parse_perf_event "${stat_file}" "cycles")"
  miss="$(parse_perf_event "${stat_file}" "cache-misses")"
  rm -f "${stat_file}" >/dev/null 2>&1 || true

  if [[ -z "${instr}" || -z "${cycles}" || -z "${miss}" ]]; then
    return 2
  fi
  if ! [[ "${instr}" =~ ^[0-9]+$ && "${cycles}" =~ ^[0-9]+$ && "${miss}" =~ ^[0-9]+$ ]]; then
    return 2
  fi

  printf '%s;%s;%s\n' "${instr}" "${cycles}" "${miss}"
  return 0
}

collect_series_medians() {
  local warmup="$1"
  local repeat="$2"
  shift 2
  local -a cmd=("$@")

  local f_instr f_cycles f_miss f_ms
  f_instr="$(mktemp --suffix .instr)"
  f_cycles="$(mktemp --suffix .cycles)"
  f_miss="$(mktemp --suffix .miss)"
  f_ms="$(mktemp --suffix .ms)"

  local i raw instr cycles miss ms
  for ((i = 0; i < warmup; i++)); do
    run_perf_stat_once "${cmd[@]}" >/dev/null || true
    perf_measure_cmd_ms "${cmd[@]}" >/dev/null || true
  done
  for ((i = 0; i < repeat; i++)); do
    raw="$(run_perf_stat_once "${cmd[@]}")"
    instr="${raw%%;*}"
    raw="${raw#*;}"
    cycles="${raw%%;*}"
    miss="${raw##*;}"
    ms="$(perf_measure_cmd_ms "${cmd[@]}")"
    printf '%s\n' "${instr}" >>"${f_instr}"
    printf '%s\n' "${cycles}" >>"${f_cycles}"
    printf '%s\n' "${miss}" >>"${f_miss}"
    printf '%s\n' "${ms}" >>"${f_ms}"
  done

  local med_instr med_cycles med_miss med_ms
  med_instr="$(median_file "${f_instr}")"
  med_cycles="$(median_file "${f_cycles}")"
  med_miss="$(median_file "${f_miss}")"
  med_ms="$(median_file "${f_ms}")"

  rm -f "${f_instr}" "${f_cycles}" "${f_miss}" "${f_ms}" >/dev/null 2>&1 || true
  printf '%s;%s;%s;%s\n' "${med_instr}" "${med_cycles}" "${med_miss}" "${med_ms}"
}

# Build C baseline.
cc -O3 -march=native -mtune=native -o "${C_BIN}" "${C_SRC}" 2>/dev/null || cc -O3 -o "${C_BIN}" "${C_SRC}"

# Build Nyash AOT exe (AOT helper pins GC=off/poll=off by default).
if ! perf_emit_and_build_aot_exe "${ROOT_DIR}" "${HAKORUNE_BIN}" "${HAKO_PROG}" "${NY_AOT_EXE}"; then
  echo "[error] AOT emit/build failed: status=${PERF_AOT_LAST_STATUS} reason=${PERF_AOT_LAST_REASON} stage=${PERF_AOT_LAST_STAGE}" >&2
  exit 1
fi

C_MEDS="$(collect_series_medians "${WARMUP}" "${REPEAT}" "${C_BIN}")"
NY_MEDS="$(collect_series_medians "${WARMUP}" "${REPEAT}" env \
  "${MICRO_RUN_ENV[@]}" \
  "${NY_AOT_EXE}")"

C_INSTR="${C_MEDS%%;*}"; C_TMP="${C_MEDS#*;}"
C_CYCLES="${C_TMP%%;*}"; C_TMP="${C_TMP#*;}"
C_MISS="${C_TMP%%;*}"; C_MS="${C_TMP##*;}"

NY_INSTR="${NY_MEDS%%;*}"; NY_TMP="${NY_MEDS#*;}"
NY_CYCLES="${NY_TMP%%;*}"; NY_TMP="${NY_TMP#*;}"
NY_MISS="${NY_TMP%%;*}"; NY_MS="${NY_TMP##*;}"

RATIO_INSTR="$(ratio_fmt "${C_INSTR}" "${NY_INSTR}")"
RATIO_CYCLES="$(ratio_fmt "${C_CYCLES}" "${NY_CYCLES}")"
RATIO_MS="$(ratio_fmt "${C_MS}" "${NY_MS}")"
C_IPC="$(ipc_fmt "${C_INSTR}" "${C_CYCLES}")"
NY_IPC="$(ipc_fmt "${NY_INSTR}" "${NY_CYCLES}")"

printf "[microstat] name=%s c_instr=%s c_cycles=%s c_cache_miss=%s c_ms=%s ny_aot_instr=%s ny_aot_cycles=%s ny_aot_cache_miss=%s ny_aot_ms=%s ratio_instr=%s ratio_cycles=%s ratio_ms=%s c_ipc=%s ny_aot_ipc=%s aot_status=%s\n" \
  "${KEY}" \
  "${C_INSTR}" "${C_CYCLES}" "${C_MISS}" "${C_MS}" \
  "${NY_INSTR}" "${NY_CYCLES}" "${NY_MISS}" "${NY_MS}" \
  "${RATIO_INSTR}" "${RATIO_CYCLES}" "${RATIO_MS}" \
  "${C_IPC}" "${NY_IPC}" \
  "${PERF_AOT_LAST_STATUS}"

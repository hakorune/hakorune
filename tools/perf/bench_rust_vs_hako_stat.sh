#!/usr/bin/env bash
set -euo pipefail

# Compare a pure-Rust reference against the Hakorune AOT lane for a matching micro case.
#
# Usage:
#   tools/perf/bench_rust_vs_hako_stat.sh <bench_key> [warmup] [repeat] [variant]
#
# Variants:
#   pure   - safe Rust slice reference (default)
#   clike  - byte-copy, stack-buffer Rust reference closer to C shape
#
# Output:
#   [ruststat] name=<key> rust_instr=<n> rust_cycles=<n> rust_cache_miss=<n> rust_ms=<n> \
#              ny_aot_instr=<n> ny_aot_cycles=<n> ny_aot_cache_miss=<n> ny_aot_ms=<n> \
#              aot_over_rust_instr=<r> aot_over_rust_cycles=<r> aot_over_rust_ms=<r> \
#              rust_ipc=<r> ny_aot_ipc=<r> aot_status=<ok|skip|fail>

KEY="${1:-}"
WARMUP="${2:-1}"
REPEAT="${3:-3}"
VARIANT="${4:-pure}"

if [[ -z "${KEY}" ]]; then
  echo "Usage: $0 <bench_key> [warmup] [repeat]" >&2
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
case "${VARIANT}" in
  pure|clike)
    ;;
  *)
    echo "[error] variant must be pure|clike: got '${VARIANT}'" >&2
    exit 2
    ;;
esac

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TARGET_DIR="${ROOT_DIR}/target"
source "${ROOT_DIR}/tools/perf/lib/bench_key_alias.sh"
BENCH_DATASET_KEY="$(perf_resolve_bench_dataset_key "${KEY}")"
HAKO_PROG="${ROOT_DIR}/benchmarks/bench_${BENCH_DATASET_KEY}.hako"
if [[ "${VARIANT}" == "pure" ]]; then
  RUST_SRC="${ROOT_DIR}/benchmarks/rust/bench_${KEY}.rs"
else
  RUST_SRC="${ROOT_DIR}/benchmarks/rust/bench_${KEY}_clike.rs"
fi
HAKORUNE_BIN="${TARGET_DIR}/release/hakorune"
RUST_BIN="${TARGET_DIR}/perf_rust_${KEY}_${VARIANT}"
NY_AOT_EXE="${TARGET_DIR}/perf_ny_${KEY}.ruststat.${BASHPID}.exe"

if [[ ! -x "${HAKORUNE_BIN}" ]]; then
  echo "[error] hakorune binary missing: ${HAKORUNE_BIN}" >&2
  echo "[hint] run: bash tools/perf/build_perf_release.sh" >&2
  exit 2
fi
if [[ ! -f "${HAKO_PROG}" ]]; then
  echo "[error] benchmark missing: ${HAKO_PROG}" >&2
  exit 2
fi
if [[ ! -f "${RUST_SRC}" ]]; then
  echo "[error] rust benchmark missing: ${RUST_SRC}" >&2
  exit 2
fi

cleanup() {
  rm -f "${RUST_BIN}" "${NY_AOT_EXE}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

source "${ROOT_DIR}/tools/perf/lib/bench_compare_common.sh"
source "${ROOT_DIR}/tools/perf/lib/aot_helpers.sh"

perf_aot_assert_default_release_alignment "${ROOT_DIR}" "${HAKORUNE_BIN}"

if ! command -v "${RUSTC:-rustc}" >/dev/null 2>&1; then
  echo "[error] rustc missing: ${RUSTC:-rustc}" >&2
  exit 2
fi

perf_build_rust_ref() {
  "${RUSTC:-rustc}" \
    --edition=2021 \
    -C opt-level=3 \
    -C codegen-units=1 \
    -C target-cpu=native \
    -C panic=abort \
    -o "${RUST_BIN}" \
    "${RUST_SRC}"
}

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
  stat_file="$(mktemp --suffix .rustrustat)"
  set +e
  LC_ALL=C perf stat --no-big-num -x ';' -e instructions,cycles,cache-misses "$@" \
    >/dev/null 2>"${stat_file}"
  local rc=$?
  set -e

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

perf_build_rust_ref

if ! perf_emit_and_build_aot_exe "${ROOT_DIR}" "${HAKORUNE_BIN}" "${HAKO_PROG}" "${NY_AOT_EXE}"; then
  echo "[error] AOT emit/build failed: status=${PERF_AOT_LAST_STATUS} reason=${PERF_AOT_LAST_REASON} stage=${PERF_AOT_LAST_STAGE}" >&2
  exit 1
fi

MICRO_RUN_ENV=(
  NYASH_GC_MODE="${NYASH_GC_MODE:-off}"
  NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}"
  NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}"
  NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}"
)

RUST_MEDS="$(collect_series_medians "${WARMUP}" "${REPEAT}" "${RUST_BIN}")"
NY_MEDS="$(collect_series_medians "${WARMUP}" "${REPEAT}" env \
  "${MICRO_RUN_ENV[@]}" \
  "${NY_AOT_EXE}")"

RUST_INSTR="${RUST_MEDS%%;*}"; RUST_TMP="${RUST_MEDS#*;}"
RUST_CYCLES="${RUST_TMP%%;*}"; RUST_TMP="${RUST_TMP#*;}"
RUST_MISS="${RUST_TMP%%;*}"; RUST_MS="${RUST_TMP##*;}"

NY_INSTR="${NY_MEDS%%;*}"; NY_TMP="${NY_MEDS#*;}"
NY_CYCLES="${NY_TMP%%;*}"; NY_TMP="${NY_TMP#*;}"
NY_MISS="${NY_TMP%%;*}"; NY_MS="${NY_TMP##*;}"

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

AOT_OVER_RUST_INSTR="$(ratio_fmt "${NY_INSTR}" "${RUST_INSTR}")"
AOT_OVER_RUST_CYCLES="$(ratio_fmt "${NY_CYCLES}" "${RUST_CYCLES}")"
AOT_OVER_RUST_MS="$(ratio_fmt "${NY_MS}" "${RUST_MS}")"
RUST_IPC="$(ipc_fmt "${RUST_INSTR}" "${RUST_CYCLES}")"
NY_IPC="$(ipc_fmt "${NY_INSTR}" "${NY_CYCLES}")"

printf "[ruststat] name=%s rust_instr=%s rust_cycles=%s rust_cache_miss=%s rust_ms=%s ny_aot_instr=%s ny_aot_cycles=%s ny_aot_cache_miss=%s ny_aot_ms=%s aot_over_rust_instr=%s aot_over_rust_cycles=%s aot_over_rust_ms=%s rust_ipc=%s ny_aot_ipc=%s aot_status=%s\n" \
  "${KEY}" \
  "${RUST_INSTR}" "${RUST_CYCLES}" "${RUST_MISS}" "${RUST_MS}" \
  "${NY_INSTR}" "${NY_CYCLES}" "${NY_MISS}" "${NY_MS}" \
  "${AOT_OVER_RUST_INSTR}" "${AOT_OVER_RUST_CYCLES}" "${AOT_OVER_RUST_MS}" \
  "${RUST_IPC}" "${NY_IPC}" \
  "${PERF_AOT_LAST_STATUS}"

#!/usr/bin/env bash
set -euo pipefail

# Three-lane micro bench:
# - C total CLI
# - C kernel-only (resident runner calling bench_main repeatedly)
# - Nyash AOT total CLI / startup baseline / kernel-only (resident runner calling ny_main repeatedly)
#
# Usage:
#   tools/perf/bench_micro_c_vs_aot_lanes.sh <bench_key> [warmup] [repeat] [kernel_inner_runs]

KEY="${1:-}"
WARMUP="${2:-1}"
REPEAT="${3:-7}"
KERNEL_INNER_RUNS="${4:-20}"

if [[ -z "${KEY}" ]]; then
  echo "Usage: $0 <bench_key> [warmup] [repeat] [kernel_inner_runs]" >&2
  exit 2
fi
if ! [[ "${WARMUP}" =~ ^[0-9]+$ && "${REPEAT}" =~ ^[0-9]+$ && "${KERNEL_INNER_RUNS}" =~ ^[0-9]+$ ]]; then
  echo "[error] warmup/repeat/kernel_inner_runs must be unsigned integers" >&2
  exit 2
fi
if [[ "${REPEAT}" -eq 0 || "${KERNEL_INNER_RUNS}" -eq 0 ]]; then
  echo "[error] repeat and kernel_inner_runs must be >= 1" >&2
  exit 2
fi

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TARGET_DIR="${ROOT_DIR}/target"
HAKO_PROG="${ROOT_DIR}/benchmarks/bench_${KEY}.hako"
C_SRC="${ROOT_DIR}/benchmarks/c/bench_${KEY}.c"
HAKORUNE_BIN="${TARGET_DIR}/release/hakorune"
C_BIN="${TARGET_DIR}/perf_c_${KEY}.microstat"
C_KERNEL_OBJ="${TARGET_DIR}/perf_c_${KEY}.kernel.${BASHPID}.o"
C_KERNEL_SRC="${TARGET_DIR}/perf_c_${KEY}.kernel.${BASHPID}.runner.c"
C_KERNEL_BIN="${TARGET_DIR}/perf_c_${KEY}.kernel.${BASHPID}.bin"
NY_AOT_EXE="${TARGET_DIR}/perf_ny_${KEY}.microstat.${BASHPID}.exe"
NY_AOT_RET0="${TARGET_DIR}/perf_ny_ret0.${KEY}.${BASHPID}.exe"
NY_KERNEL_OBJ="${TARGET_DIR}/perf_ny_${KEY}.kernel.${BASHPID}.o"
NY_KERNEL_SRC="${TARGET_DIR}/perf_ny_${KEY}.kernel.${BASHPID}.runner.c"
NY_KERNEL_BIN="${TARGET_DIR}/perf_ny_${KEY}.kernel.${BASHPID}.bin"

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

mkdir -p "${TARGET_DIR}"

cleanup() {
  rm -f \
    "${C_KERNEL_OBJ}" "${C_KERNEL_SRC}" "${C_KERNEL_BIN}" \
    "${NY_AOT_EXE}" "${NY_AOT_RET0}" "${NY_KERNEL_OBJ}" "${NY_KERNEL_SRC}" "${NY_KERNEL_BIN}" \
    >/dev/null 2>&1 || true
}
trap cleanup EXIT

source "${ROOT_DIR}/tools/perf/lib/bench_compare_common.sh"
source "${ROOT_DIR}/tools/perf/lib/aot_helpers.sh"
source "${ROOT_DIR}/tools/perf/lib/microstat_helpers.sh"

export NYASH_LLVM_USE_HARNESS=0

MICRO_RUN_ENV=(
  NYASH_GC_MODE="${NYASH_GC_MODE:-off}"
  NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}"
  NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}"
  NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}"
)

fmt_div() {
  python3 - "$@" <<'PY'
import sys
num = float(sys.argv[1])
den = float(sys.argv[2])
digits = int(sys.argv[3])
value = (num / den) if den > 0 else 0.0
print(f"{value:.{digits}f}")
PY
}

build_c_kernel_runner() {
  local src="$1"
  local out_obj="$2"
  local runner_src="$3"
  local out_bin="$4"
  cc -O3 -march=native -mtune=native -Dmain=bench_main -c -o "${out_obj}" "${src}" 2>/dev/null \
    || cc -O3 -Dmain=bench_main -c -o "${out_obj}" "${src}"
  cat <<'EOF' >"${runner_src}"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

extern int bench_main(void);

int main(int argc, char** argv) {
  long runs = 1;
  char* end = NULL;
  if (argc > 2) {
    fprintf(stderr, "usage: %s [runs]\n", argv[0]);
    return 2;
  }
  if (argc == 2) {
    runs = strtol(argv[1], &end, 10);
    if (!end || *end != '\0' || runs < 1) {
      fprintf(stderr, "invalid runs: %s\n", argv[1]);
      return 2;
    }
  }
  volatile int64_t sink = 0;
  for (long i = 0; i < runs; ++i) {
    sink += (int64_t)bench_main();
  }
  if (sink == INT64_MIN) {
    fprintf(stderr, "unreachable\n");
  }
  return 0;
}
EOF
  cc -O3 -march=native -mtune=native -o "${out_bin}" "${runner_src}" "${out_obj}" 2>/dev/null \
    || cc -O3 -o "${out_bin}" "${runner_src}" "${out_obj}"
}

build_ny_kernel_runner() {
  local root_dir="$1"
  local hako_bin="$2"
  local hako_prog="$3"
  local out_obj="$4"
  local runner_src="$5"
  local out_bin="$6"

  if ! perf_emit_and_build_aot_obj "${root_dir}" "${hako_bin}" "${hako_prog}" "${out_obj}"; then
    echo "[error] AOT object emit/build failed: status=${PERF_AOT_LAST_STATUS} reason=${PERF_AOT_LAST_REASON} stage=${PERF_AOT_LAST_STAGE}" >&2
    return 1
  fi

  cat <<'EOF' >"${runner_src}"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

extern int64_t ny_main(void);

int main(int argc, char** argv) {
  long runs = 1;
  char* end = NULL;
  if (argc > 2) {
    fprintf(stderr, "usage: %s [runs]\n", argv[0]);
    return 2;
  }
  if (argc == 2) {
    runs = strtol(argv[1], &end, 10);
    if (!end || *end != '\0' || runs < 1) {
      fprintf(stderr, "invalid runs: %s\n", argv[1]);
      return 2;
    }
  }
  volatile int64_t sink = 0;
  for (long i = 0; i < runs; ++i) {
    sink += ny_main();
  }
  if (sink == INT64_MIN) {
    fprintf(stderr, "unreachable\n");
  }
  return 0;
}
EOF

  cc -O2 -no-pie -o "${out_bin}" "${runner_src}" "${out_obj}" \
    "${root_dir}/target/release/libnyash_kernel.a" -ldl -lpthread -lm
}

split_medians() {
  local tuple="$1"
  local prefix="$2"
  local first rest second third fourth
  first="${tuple%%;*}"
  rest="${tuple#*;}"
  second="${rest%%;*}"
  rest="${rest#*;}"
  third="${rest%%;*}"
  fourth="${rest##*;}"
  printf -v "${prefix}_INSTR" '%s' "${first}"
  printf -v "${prefix}_CYCLES" '%s' "${second}"
  printf -v "${prefix}_MISS" '%s' "${third}"
  printf -v "${prefix}_MS" '%s' "${fourth}"
}

build_c_kernel_runner "${C_SRC}" "${C_KERNEL_OBJ}" "${C_KERNEL_SRC}" "${C_KERNEL_BIN}"

cc -O3 -march=native -mtune=native -o "${C_BIN}" "${C_SRC}" 2>/dev/null \
  || cc -O3 -o "${C_BIN}" "${C_SRC}"

if ! perf_emit_and_build_aot_exe "${ROOT_DIR}" "${HAKORUNE_BIN}" "${HAKO_PROG}" "${NY_AOT_EXE}"; then
  echo "[error] AOT emit/build failed: status=${PERF_AOT_LAST_STATUS} reason=${PERF_AOT_LAST_REASON} stage=${PERF_AOT_LAST_STAGE}" >&2
  exit 1
fi
NY_TOTAL_STATUS="${PERF_AOT_LAST_STATUS}"

if ! perf_build_ret0_aot_exe "${ROOT_DIR}" "${HAKORUNE_BIN}" "${NY_AOT_RET0}"; then
  echo "[error] AOT ret0 baseline failed: status=${PERF_AOT_LAST_STATUS} reason=${PERF_AOT_LAST_REASON} stage=${PERF_AOT_LAST_STAGE}" >&2
  exit 1
fi

build_ny_kernel_runner "${ROOT_DIR}" "${HAKORUNE_BIN}" "${HAKO_PROG}" "${NY_KERNEL_OBJ}" "${NY_KERNEL_SRC}" "${NY_KERNEL_BIN}"

C_TOTAL_MEDS="$(perf_microstat_collect_series_medians "${WARMUP}" "${REPEAT}" "${C_BIN}")"
C_KERNEL_MEDS="$(perf_microstat_collect_series_medians "${WARMUP}" "${REPEAT}" "${C_KERNEL_BIN}" "${KERNEL_INNER_RUNS}")"
NY_TOTAL_MEDS="$(perf_microstat_collect_series_medians "${WARMUP}" "${REPEAT}" env "${MICRO_RUN_ENV[@]}" "${NY_AOT_EXE}")"
NY_STARTUP_MEDS="$(perf_microstat_collect_series_medians "${WARMUP}" "${REPEAT}" env "${MICRO_RUN_ENV[@]}" "${NY_AOT_RET0}")"
NY_KERNEL_MEDS="$(perf_microstat_collect_series_medians "${WARMUP}" "${REPEAT}" env \
  NYASH_GC_MODE="${NYASH_GC_MODE:-off}" \
  NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}" \
  "${NY_KERNEL_BIN}" "${KERNEL_INNER_RUNS}")"

split_medians "${C_TOTAL_MEDS}" "C_TOTAL"
split_medians "${C_KERNEL_MEDS}" "C_KERNEL_TOTAL"
split_medians "${NY_TOTAL_MEDS}" "NY_TOTAL"
split_medians "${NY_STARTUP_MEDS}" "NY_STARTUP"
split_medians "${NY_KERNEL_MEDS}" "NY_KERNEL_TOTAL"

C_KERNEL_INSTR="$(fmt_div "${C_KERNEL_TOTAL_INSTR}" "${KERNEL_INNER_RUNS}" 0)"
C_KERNEL_CYCLES="$(fmt_div "${C_KERNEL_TOTAL_CYCLES}" "${KERNEL_INNER_RUNS}" 0)"
C_KERNEL_MISS="$(fmt_div "${C_KERNEL_TOTAL_MISS}" "${KERNEL_INNER_RUNS}" 0)"
C_KERNEL_MS="$(fmt_div "${C_KERNEL_TOTAL_MS}" "${KERNEL_INNER_RUNS}" 3)"
NY_KERNEL_INSTR="$(fmt_div "${NY_KERNEL_TOTAL_INSTR}" "${KERNEL_INNER_RUNS}" 0)"
NY_KERNEL_CYCLES="$(fmt_div "${NY_KERNEL_TOTAL_CYCLES}" "${KERNEL_INNER_RUNS}" 0)"
NY_KERNEL_MISS="$(fmt_div "${NY_KERNEL_TOTAL_MISS}" "${KERNEL_INNER_RUNS}" 0)"
NY_KERNEL_MS="$(fmt_div "${NY_KERNEL_TOTAL_MS}" "${KERNEL_INNER_RUNS}" 3)"

RATIO_TOTAL_INSTR="$(perf_microstat_ratio_fmt "${C_TOTAL_INSTR}" "${NY_TOTAL_INSTR}")"
RATIO_TOTAL_CYCLES="$(perf_microstat_ratio_fmt "${C_TOTAL_CYCLES}" "${NY_TOTAL_CYCLES}")"
RATIO_TOTAL_MS="$(perf_microstat_ratio_fmt "${C_TOTAL_MS}" "${NY_TOTAL_MS}")"
RATIO_KERNEL_INSTR="$(perf_microstat_ratio_fmt "${C_KERNEL_INSTR}" "${NY_KERNEL_INSTR}")"
RATIO_KERNEL_CYCLES="$(perf_microstat_ratio_fmt "${C_KERNEL_CYCLES}" "${NY_KERNEL_CYCLES}")"
RATIO_KERNEL_MS="$(perf_microstat_ratio_fmt "${C_KERNEL_MS}" "${NY_KERNEL_MS}")"
C_TOTAL_IPC="$(perf_microstat_ipc_fmt "${C_TOTAL_INSTR}" "${C_TOTAL_CYCLES}")"
NY_TOTAL_IPC="$(perf_microstat_ipc_fmt "${NY_TOTAL_INSTR}" "${NY_TOTAL_CYCLES}")"
C_KERNEL_IPC="$(perf_microstat_ipc_fmt "${C_KERNEL_INSTR}" "${C_KERNEL_CYCLES}")"
NY_KERNEL_IPC="$(perf_microstat_ipc_fmt "${NY_KERNEL_INSTR}" "${NY_KERNEL_CYCLES}")"

printf "[micro-lanes] name=%s inner_runs=%s c_total_instr=%s c_total_cycles=%s c_total_cache_miss=%s c_total_ms=%s c_kernel_instr=%s c_kernel_cycles=%s c_kernel_cache_miss=%s c_kernel_ms=%s ny_total_instr=%s ny_total_cycles=%s ny_total_cache_miss=%s ny_total_ms=%s ny_startup_instr=%s ny_startup_cycles=%s ny_startup_cache_miss=%s ny_startup_ms=%s ny_kernel_instr=%s ny_kernel_cycles=%s ny_kernel_cache_miss=%s ny_kernel_ms=%s ratio_total_instr=%s ratio_total_cycles=%s ratio_total_ms=%s ratio_kernel_instr=%s ratio_kernel_cycles=%s ratio_kernel_ms=%s c_total_ipc=%s ny_total_ipc=%s c_kernel_ipc=%s ny_kernel_ipc=%s aot_status=%s\n" \
  "${KEY}" "${KERNEL_INNER_RUNS}" \
  "${C_TOTAL_INSTR}" "${C_TOTAL_CYCLES}" "${C_TOTAL_MISS}" "${C_TOTAL_MS}" \
  "${C_KERNEL_INSTR}" "${C_KERNEL_CYCLES}" "${C_KERNEL_MISS}" "${C_KERNEL_MS}" \
  "${NY_TOTAL_INSTR}" "${NY_TOTAL_CYCLES}" "${NY_TOTAL_MISS}" "${NY_TOTAL_MS}" \
  "${NY_STARTUP_INSTR}" "${NY_STARTUP_CYCLES}" "${NY_STARTUP_MISS}" "${NY_STARTUP_MS}" \
  "${NY_KERNEL_INSTR}" "${NY_KERNEL_CYCLES}" "${NY_KERNEL_MISS}" "${NY_KERNEL_MS}" \
  "${RATIO_TOTAL_INSTR}" "${RATIO_TOTAL_CYCLES}" "${RATIO_TOTAL_MS}" \
  "${RATIO_KERNEL_INSTR}" "${RATIO_KERNEL_CYCLES}" "${RATIO_KERNEL_MS}" \
  "${C_TOTAL_IPC}" "${NY_TOTAL_IPC}" "${C_KERNEL_IPC}" "${NY_KERNEL_IPC}" \
  "${NY_TOTAL_STATUS}"

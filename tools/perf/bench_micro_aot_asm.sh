#!/usr/bin/env bash
set -euo pipefail

# AOT machine-code probe helper for fixed micro benches.
# - builds AOT exe from benchmarks/bench_<key>.hako
# - records repeated perf samples
# - prints top report + optional annotate/objdump snippet
# - uses a tiny direct C runner instead of a bash loop to reduce shell noise
#
# Usage:
#   tools/perf/bench_micro_aot_asm.sh <bench_key> [symbol] [runs]
#
# Optional env:
#   PERF_MICROASM_RUNNER_MODE=spawn|direct
#     spawn  - default; use a tiny C runner that spawns the exe `runs` times
#     direct - perf the AOT exe directly once; useful to cut spawn/loader noise
#
# Example:
#   tools/perf/bench_micro_aot_asm.sh kilo_micro_indexof_line \
#     'nyash_kernel::exports::string::find_substr_byte_index' 20

KEY="${1:-}"
SYMBOL="${2:-}"
RUNS="${3:-20}"

if [[ -z "${KEY}" ]]; then
  echo "Usage: $0 <bench_key> [symbol] [runs]" >&2
  exit 2
fi
if ! [[ "${RUNS}" =~ ^[0-9]+$ ]] || [[ "${RUNS}" -lt 1 ]]; then
  echo "[error] runs must be >= 1" >&2
  exit 2
fi

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TARGET_DIR="${ROOT_DIR}/target"
source "${ROOT_DIR}/tools/perf/lib/bench_key_alias.sh"
BENCH_DATASET_KEY="$(perf_resolve_bench_dataset_key "${KEY}")"
HAKO_PROG="${ROOT_DIR}/benchmarks/bench_${BENCH_DATASET_KEY}.hako"
HAKORUNE_BIN="${TARGET_DIR}/release/hakorune"
AOT_EXE="${TARGET_DIR}/perf_ny_${KEY}.microasm.${BASHPID}.exe"
PERF_DATA="/tmp/${KEY}.microasm.${BASHPID}.perf.data"
OBJDUMP_TXT="/tmp/${KEY}.microasm.${BASHPID}.objdump.txt"
RUNNER_C="/tmp/${KEY}.microasm.${BASHPID}.runner.c"
RUNNER_BIN="/tmp/${KEY}.microasm.${BASHPID}.runner.bin"
RUNNER_MODE="${PERF_MICROASM_RUNNER_MODE:-spawn}"

if [[ ! -x "${HAKORUNE_BIN}" ]]; then
  echo "[error] hakorune binary missing: ${HAKORUNE_BIN}" >&2
  echo "[hint] run: cargo build --release --bin hakorune" >&2
  exit 2
fi
if [[ ! -f "${HAKO_PROG}" ]]; then
  echo "[error] benchmark missing: ${HAKO_PROG}" >&2
  exit 2
fi

cleanup() {
  if [[ "${KEEP_PERF_MICROASM_ARTIFACTS:-0}" == "1" ]]; then
    return 0
  fi
  rm -f "${AOT_EXE}" "${PERF_DATA}" "${OBJDUMP_TXT}" "${RUNNER_C}" "${RUNNER_BIN}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

source "${ROOT_DIR}/tools/perf/lib/aot_helpers.sh"

# Keep the benchmark-specific AOT executable compact so startup/loader cost stays measurable.
export NYASH_LLVM_LINK_WHOLE_ARCHIVE="${NYASH_LLVM_LINK_WHOLE_ARCHIVE:-0}"
export NYASH_LLVM_LINK_GC_SECTIONS="${NYASH_LLVM_LINK_GC_SECTIONS:-1}"

if ! perf_emit_and_build_aot_exe "${ROOT_DIR}" "${HAKORUNE_BIN}" "${HAKO_PROG}" "${AOT_EXE}"; then
  echo "[error] AOT emit/build failed: status=${PERF_AOT_LAST_STATUS} reason=${PERF_AOT_LAST_REASON} stage=${PERF_AOT_LAST_STAGE}" >&2
  exit 1
fi

if [[ "${RUNNER_MODE}" != "spawn" && "${RUNNER_MODE}" != "direct" ]]; then
  echo "[error] PERF_MICROASM_RUNNER_MODE must be spawn or direct; got '${RUNNER_MODE}'" >&2
  exit 2
fi

if [[ "${RUNNER_MODE}" == "spawn" ]]; then
  if ! command -v "${CC:-cc}" >/dev/null 2>&1; then
    echo "[error] microasm spawn runner requires a C compiler (\$CC or cc)" >&2
    exit 2
  fi

  cat >"${RUNNER_C}" <<'EOF'
#include <errno.h>
#include <spawn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

extern char **environ;

int main(int argc, char **argv) {
  if (argc != 3) {
    fprintf(stderr, "usage: %s <runs> <exe>\n", argv[0]);
    return 2;
  }
  char *end = NULL;
  long runs = strtol(argv[1], &end, 10);
  if (!end || *end != '\0' || runs < 1) {
    fprintf(stderr, "invalid runs: %s\n", argv[1]);
    return 2;
  }

  char *const child_argv[] = { argv[2], NULL };
  for (long i = 0; i < runs; ++i) {
    pid_t pid = 0;
    int rc = posix_spawn(&pid, argv[2], NULL, NULL, child_argv, environ);
    if (rc != 0) {
      fprintf(stderr, "posix_spawn failed: %s\n", strerror(rc));
      return 1;
    }
    int status = 0;
    if (waitpid(pid, &status, 0) < 0) {
      fprintf(stderr, "waitpid failed: %s\n", strerror(errno));
      return 1;
    }
    if (!WIFEXITED(status)) {
      return 1;
    }
  }
  return 0;
}
EOF

  "${CC:-cc}" -O2 -std=c11 -Wall -Wextra -o "${RUNNER_BIN}" "${RUNNER_C}"
  RUNNER_ENV=(
    NYASH_GC_MODE="${NYASH_GC_MODE:-off}"
    NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}"
    NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}"
    NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}"
  )
  if [[ "${KEY}" == kilo_micro_userbox_* ]]; then
    RUNNER_ENV+=(
      HAKO_TYPED_OBJECT_STORE="${HAKO_TYPED_OBJECT_STORE:-direct_slot_exact}"
    )
  fi
  env \
    "${RUNNER_ENV[@]}" \
    perf record -o "${PERF_DATA}" -F 999 -- "${RUNNER_BIN}" "${RUNS}" "${AOT_EXE}" >/dev/null 2>&1 || true
else
  RUNNER_ENV=(
    NYASH_GC_MODE="${NYASH_GC_MODE:-off}"
    NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}"
    NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}"
    NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}"
  )
  if [[ "${KEY}" == kilo_micro_userbox_* ]]; then
    RUNNER_ENV+=(
      HAKO_TYPED_OBJECT_STORE="${HAKO_TYPED_OBJECT_STORE:-direct_slot_exact}"
    )
  fi
  env \
    "${RUNNER_ENV[@]}" \
    perf record -o "${PERF_DATA}" -F 999 -- "${AOT_EXE}" >/dev/null 2>&1 || true
fi

echo "[microasm] key=${KEY} runs=${RUNS} exe=${AOT_EXE} perf_data=${PERF_DATA} runner=${RUNNER_MODE}"
echo "[microasm] top report (--no-children):"
perf report --stdio --no-children -i "${PERF_DATA}" | sed -n '1,80p'

if [[ -n "${SYMBOL}" ]]; then
  echo
  echo "[microasm] annotate symbol=${SYMBOL}:"
  perf annotate --stdio -i "${PERF_DATA}" --symbol "${SYMBOL}" | sed -n '1,220p' || true

  objdump -d --demangle "${AOT_EXE}" >"${OBJDUMP_TXT}"
  line_no="$(grep -nF "${SYMBOL}" "${OBJDUMP_TXT}" | head -n 1 | cut -d: -f1 || true)"
  if [[ -n "${line_no}" ]]; then
    echo
    echo "[microasm] objdump snippet around symbol=${SYMBOL}:"
    sed -n "${line_no},$((line_no + 120))p" "${OBJDUMP_TXT}"
  else
    echo
    echo "[microasm] objdump symbol match not found for '${SYMBOL}'"
  fi
fi

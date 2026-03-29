#!/usr/bin/env bash
set -euo pipefail

# Trace + asm bundle for the current kilo string placement wave.
#
# This bundle keeps timing lanes clean:
#   - bench_compare_* remains timing-only
#   - trace is collected via the probe-only unit contracts
#   - asm is collected from a single built AOT artifact
#
# Usage:
#   tools/perf/run_kilo_string_trace_asm_bundle.sh [--out-dir <dir>] [--bench-key <key>] [--runs <n>] [--symbol <sym> ...]
#
# Defaults:
#   bench key: kilo_kernel_small_hk
#   runs:      20
#   symbols:
#     - nyash_kernel::exports::string::string_handle_from_owned
#     - nyash.string.concat_hh
#     - nyash.string.concat3_hhh
#     - nyash.string.substring_hii
#     - nyash.array.set_his
#     - nyash.array.string_len_hi
#     - nyash_rust::box_trait::BoxBase::new

OUT_DIR=""
BENCH_KEY="kilo_kernel_small_hk"
RUNS=20
declare -a SYMBOLS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir)
      OUT_DIR="${2:-}"
      shift 2
      ;;
    --bench-key)
      BENCH_KEY="${2:-}"
      shift 2
      ;;
    --runs)
      RUNS="${2:-}"
      shift 2
      ;;
    --symbol)
      SYMBOLS+=("${2:-}")
      shift 2
      ;;
    -h|--help)
      sed -n '1,140p' "$0"
      exit 0
      ;;
    *)
      echo "[error] unknown arg: $1" >&2
      sed -n '1,140p' "$0" >&2
      exit 2
      ;;
  esac
done

if ! [[ "${RUNS}" =~ ^[0-9]+$ ]] || [[ "${RUNS}" -lt 1 ]]; then
  echo "[error] runs must be >= 1" >&2
  exit 2
fi

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
source "${ROOT_DIR}/tools/perf/lib/bench_key_alias.sh"
source "${ROOT_DIR}/tools/perf/lib/bench_env.sh"
source "${ROOT_DIR}/tools/perf/lib/aot_helpers.sh"

if ! perf_is_supported_bench4_key "${BENCH_KEY}"; then
  echo "[error] unsupported bench key for string trace bundle: ${BENCH_KEY}" >&2
  exit 2
fi

BENCH_DATASET_KEY="$(perf_resolve_bench_dataset_key "${BENCH_KEY}")"
HAKO_PROG="${ROOT_DIR}/benchmarks/bench_${BENCH_DATASET_KEY}.hako"
HAKORUNE_BIN="${ROOT_DIR}/target/release/hakorune"

if [[ ! -x "${HAKORUNE_BIN}" ]]; then
  echo "[error] hakorune binary missing: ${HAKORUNE_BIN}" >&2
  echo "[hint] run: cargo build --release --bin hakorune" >&2
  exit 2
fi
if [[ ! -f "${HAKO_PROG}" ]]; then
  echo "[error] benchmark missing: ${HAKO_PROG}" >&2
  exit 2
fi

if [[ -z "${OUT_DIR}" ]]; then
  ts="$(date +%Y%m%d-%H%M%S)"
  OUT_DIR="${ROOT_DIR}/target/trace_logs/kilo-string-trace-asm/${ts}"
fi

TRACE_DIR="${OUT_DIR}/trace"
ASM_DIR="${OUT_DIR}/asm"
mkdir -p "${TRACE_DIR}" "${ASM_DIR}"

if [[ "${#SYMBOLS[@]}" -eq 0 ]]; then
  SYMBOLS=(
    'nyash_kernel::exports::string::string_handle_from_owned'
    'nyash.string.concat_hh'
    'nyash.string.concat3_hhh'
    'nyash.string.substring_hii'
    'nyash.array.set_his'
    'nyash.array.string_len_hi'
    'nyash_rust::box_trait::BoxBase::new'
  )
fi

TRACE_STDOUT="${OUT_DIR}/trace_probe.stdout.log"
TRACE_SUMMARY="${TRACE_DIR}/summary.txt"
ASM_STDOUT="${OUT_DIR}/asm_probe.stdout.log"
ASM_PERF_DATA="${ASM_DIR}/bundle.perf.data"
ASM_REPORT="${ASM_DIR}/perf_report.txt"
ASM_OBJDUMP="${ASM_DIR}/objdump.txt"
SUMMARY="${OUT_DIR}/summary.txt"
MANIFEST="${OUT_DIR}/bundle_manifest.txt"

: > "${SUMMARY}"

{
  printf 'bench_key=%s\n' "${BENCH_KEY}"
  printf 'bench_dataset=%s\n' "${BENCH_DATASET_KEY}"
  printf 'runs=%s\n' "${RUNS}"
  printf 'trace_dir=%s\n' "${TRACE_DIR}"
  printf 'asm_dir=%s\n' "${ASM_DIR}"
  printf 'symbols=%s\n' "${SYMBOLS[*]}"
} > "${MANIFEST}"

echo "[bundle] trace=probe-only out_dir=${TRACE_DIR}" | tee -a "${SUMMARY}"
if ! "${ROOT_DIR}/tools/perf/run_kilo_string_trace_probe.sh" --out-dir "${TRACE_DIR}" >"${TRACE_STDOUT}" 2>&1; then
  echo "[error] string trace probe failed" >&2
  tail -n 40 "${TRACE_STDOUT}" >&2 || true
  exit 1
fi

{
  echo "[bundle] trace_stdout=${TRACE_STDOUT}"
  echo "[bundle] trace_summary=${TRACE_SUMMARY}"
  printf '\n'
  cat "${TRACE_SUMMARY}"
} | tee -a "${SUMMARY}" >/dev/null

if ! command -v "${CC:-cc}" >/dev/null 2>&1; then
  echo "[error] asm bundle requires a C compiler (\$CC or cc)" >&2
  exit 2
fi

if ! perf_emit_and_build_aot_exe "${ROOT_DIR}" "${HAKORUNE_BIN}" "${HAKO_PROG}" "${ASM_DIR}/bundle.exe"; then
  echo "[error] AOT emit/build failed: status=${PERF_AOT_LAST_STATUS} reason=${PERF_AOT_LAST_REASON} stage=${PERF_AOT_LAST_STAGE}" >&2
  exit 1
fi

cat >"${ASM_DIR}/runner.c" <<'EOF'
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

"${CC:-cc}" -O2 -std=c11 -Wall -Wextra -o "${ASM_DIR}/runner.bin" "${ASM_DIR}/runner.c"

env \
  NYASH_GC_MODE="${NYASH_GC_MODE:-off}" \
  NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}" \
  NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}" \
  perf record -o "${ASM_PERF_DATA}" -F 999 -- "${ASM_DIR}/runner.bin" "${RUNS}" "${ASM_DIR}/bundle.exe" >/dev/null 2>&1

{
  echo "[bundle] asm_perf_data=${ASM_PERF_DATA}"
  echo "[bundle] asm_report=${ASM_REPORT}"
  echo "[bundle] asm_objdump=${ASM_OBJDUMP}"
} | tee -a "${SUMMARY}" >/dev/null

perf report --stdio --no-children -i "${ASM_PERF_DATA}" > "${ASM_REPORT}"
objdump -d --demangle "${ASM_DIR}/bundle.exe" > "${ASM_OBJDUMP}"

resolve_report_symbol() {
  local query="$1"
  local line
  line="$(grep -F "${query}" "${ASM_REPORT}" | head -n 1 || true)"
  if [[ -z "${line}" ]]; then
    return 1
  fi
  printf '%s\n' "${line}" | sed -E 's/^[[:space:]]*[0-9.]+%[[:space:]]+[^[:space:]]+[[:space:]]+[^[:space:]]+[[:space:]]+\[\.\][[:space:]]+//'
}

for symbol in "${SYMBOLS[@]}"; do
  safe_name="$(printf '%s' "${symbol}" | tr '/ :' '__' | tr -cd 'A-Za-z0-9._-')"
  annotate_out="${ASM_DIR}/${safe_name}.annotate.txt"
  snippet_out="${ASM_DIR}/${safe_name}.objdump.txt"
  resolved_symbol="$(resolve_report_symbol "${symbol}" || true)"
  {
    printf '== query=%s ==\n' "${symbol}"
    if [[ -n "${resolved_symbol}" ]]; then
      printf 'resolved=%s\n' "${resolved_symbol}"
      perf annotate --stdio -i "${ASM_PERF_DATA}" --symbol "${resolved_symbol}" | sed -n '1,220p' || true
    else
      printf '[warn] symbol not found in perf report: %s\n' "${symbol}"
    fi
    printf '\n'
  } > "${annotate_out}"
  line_no=""
  if [[ -n "${resolved_symbol}" ]]; then
    line_no="$(grep -nF "${resolved_symbol}" "${ASM_OBJDUMP}" | head -n 1 | cut -d: -f1 || true)"
  fi
  if [[ -n "${line_no}" ]]; then
    sed -n "${line_no},$((line_no + 120))p" "${ASM_OBJDUMP}" > "${snippet_out}"
  else
    printf '[warn] objdump symbol match not found: %s\n' "${symbol}" > "${snippet_out}"
  fi
  {
    printf '== %s ==\n' "${symbol}"
    if [[ -n "${resolved_symbol}" ]]; then
      printf 'resolved=%s\n' "${resolved_symbol}"
    fi
    printf 'annotate=%s\n' "${annotate_out}"
    printf 'snippet=%s\n' "${snippet_out}"
    printf '\n'
  } | tee -a "${SUMMARY}" >/dev/null
done

{
  printf '\n[bundle] summary trace=%s\n' "${TRACE_SUMMARY}"
  printf '[bundle] summary asm=%s\n' "${ASM_REPORT}"
  printf '[bundle] out_dir=%s\n' "${OUT_DIR}"
  printf '[bundle] note=bench_compare_c_py_vs_hako.sh remains timing-only and suppresses trace output\n'
} | tee -a "${SUMMARY}" >/dev/null

printf '[bundle] out_dir=%s\n' "${OUT_DIR}"
printf '[bundle] summary=%s\n' "${SUMMARY}"

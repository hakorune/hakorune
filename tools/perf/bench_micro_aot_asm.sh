#!/usr/bin/env bash
set -euo pipefail

# AOT machine-code probe helper for fixed micro benches.
# - builds AOT exe from benchmarks/bench_<key>.hako
# - records repeated perf samples
# - prints top report + optional annotate/objdump snippet
#
# Usage:
#   tools/perf/bench_micro_aot_asm.sh <bench_key> [symbol] [runs]
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
HAKO_PROG="${ROOT_DIR}/benchmarks/bench_${KEY}.hako"
HAKORUNE_BIN="${TARGET_DIR}/release/hakorune"
AOT_EXE="${TARGET_DIR}/perf_ny_${KEY}.microasm.${BASHPID}.exe"
PERF_DATA="/tmp/${KEY}.microasm.${BASHPID}.perf.data"
OBJDUMP_TXT="/tmp/${KEY}.microasm.${BASHPID}.objdump.txt"
RUNNER="/tmp/${KEY}.microasm.${BASHPID}.runner.sh"

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
  rm -f "${AOT_EXE}" "${PERF_DATA}" "${OBJDUMP_TXT}" "${RUNNER}" >/dev/null 2>&1 || true
}
trap cleanup EXIT

source "${ROOT_DIR}/tools/perf/lib/aot_helpers.sh"

if ! perf_emit_and_build_aot_exe "${ROOT_DIR}" "${HAKORUNE_BIN}" "${HAKO_PROG}" "${AOT_EXE}"; then
  echo "[error] AOT emit/build failed: status=${PERF_AOT_LAST_STATUS} reason=${PERF_AOT_LAST_REASON} stage=${PERF_AOT_LAST_STAGE}" >&2
  exit 1
fi

cat >"${RUNNER}" <<EOF
#!/usr/bin/env bash
set +e
for i in \$(seq 1 ${RUNS}); do
  NYASH_GC_MODE="${NYASH_GC_MODE:-off}" \
  NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}" \
  NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}" \
  "${AOT_EXE}" >/dev/null 2>&1 || true
done
EOF
chmod +x "${RUNNER}"

perf record -o "${PERF_DATA}" -F 999 -- "${RUNNER}" >/dev/null 2>&1

echo "[microasm] key=${KEY} runs=${RUNS} exe=${AOT_EXE} perf_data=${PERF_DATA}"
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

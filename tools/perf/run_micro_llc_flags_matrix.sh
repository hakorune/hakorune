#!/usr/bin/env bash
set -euo pipefail

# Run a small LLC flag A/B matrix against the micro benchmark lane.
# Each candidate is measured through bench_micro_c_vs_aot_stat.sh with NYASH_NY_LLVM_LLC_FLAGS set.
#
# Usage:
#   tools/perf/run_micro_llc_flags_matrix.sh <bench_key> [--warmup N] [--repeat N] [--flags label=flags ...]

KEY="${1:-}"
shift || true

if [[ -z "${KEY}" ]]; then
  echo "Usage: $0 <bench_key> [--warmup N] [--repeat N] [--flags label=flags ...]" >&2
  exit 2
fi

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
STAT="${ROOT_DIR}/tools/perf/bench_micro_c_vs_aot_stat.sh"

WARMUP=1
REPEAT=7
OUT_DIR=""
declare -a MATRIX_LABELS=()
declare -a MATRIX_FLAGS=()

parse_label_and_flags() {
  local spec="$1"
  local label="${spec%%=*}"
  local flags="${spec#*=}"
  if [[ "${label}" == "${flags}" ]]; then
    echo "[error] --flags must use label=flags syntax: ${spec}" >&2
    exit 2
  fi
  printf '%s\n%s\n' "${label}" "${flags}"
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --warmup)
      WARMUP="${2:-}"
      shift 2
      ;;
    --repeat)
      REPEAT="${2:-}"
      shift 2
      ;;
    --out-dir)
      OUT_DIR="${2:-}"
      shift 2
      ;;
    --flags)
      spec="${2:-}"
      mapfile -t parsed < <(parse_label_and_flags "${spec}")
      MATRIX_LABELS+=("${parsed[0]}")
      MATRIX_FLAGS+=("${parsed[1]}")
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

if ! [[ "${WARMUP}" =~ ^[0-9]+$ ]] || ! [[ "${REPEAT}" =~ ^[0-9]+$ ]]; then
  echo "[error] warmup/repeat must be unsigned integers" >&2
  exit 2
fi
if [[ "${REPEAT}" -eq 0 ]]; then
  echo "[error] repeat must be >= 1" >&2
  exit 2
fi

label_safe() {
  printf '%s' "$1" | tr '/ ' '__' | tr -cd 'A-Za-z0-9._-'
}

if [[ -z "${OUT_DIR}" ]]; then
  ts="$(date +%Y%m%d-%H%M%S)"
  OUT_DIR="${ROOT_DIR}/target/perf_state/optimization_bundle/llc-flags-matrix/${ts}-$(label_safe "${KEY}")"
fi
mkdir -p "${OUT_DIR}"

if [[ "${#MATRIX_FLAGS[@]}" -eq 0 ]]; then
  MATRIX_LABELS=(
    "o3-native"
    "o3"
    "o2-native"
  )
  MATRIX_FLAGS=(
    "-O3 -mcpu=native"
    "-O3"
    "-O2 -mcpu=native"
  )
fi

summary_file="${OUT_DIR}/matrix.txt"
: > "${summary_file}"

printf 'bench_key=%s\nwarmup=%s\nrepeat=%s\nout_dir=%s\n' "${KEY}" "${WARMUP}" "${REPEAT}" "${OUT_DIR}" | tee -a "${summary_file}"
printf 'label\tflags\tc_ms\tny_aot_ms\tratio_instr\tratio_cycles\tratio_ms\tstatus\n' | tee -a "${summary_file}"

parse_microstat_line() {
  python3 - "$1" <<'PY'
import sys
line = sys.argv[1]
fields = {}
for token in line.strip().split():
    if '=' in token:
        k, v = token.split('=', 1)
        fields[k] = v
keys = ["c_ms", "ny_aot_ms", "ratio_instr", "ratio_cycles", "ratio_ms"]
print("\t".join(fields.get(k, "") for k in keys))
PY
}

for idx in "${!MATRIX_FLAGS[@]}"; do
  label="${MATRIX_LABELS[$idx]}"
  flags="${MATRIX_FLAGS[$idx]}"
  safe_label="$(label_safe "${label}")"
  case_log="${OUT_DIR}/${idx}-${safe_label}.log"
  set +e
  NYASH_NY_LLVM_LLC_FLAGS="${flags}" bash "${STAT}" "${KEY}" "${WARMUP}" "${REPEAT}" > "${case_log}" 2>&1
  rc=$?
  set -e
  micro_line="$(grep '^\[microstat\]' "${case_log}" | tail -n 1 || true)"
  if [[ ${rc} -eq 0 && -n "${micro_line}" ]]; then
    read -r c_ms ny_aot_ms ratio_instr ratio_cycles ratio_ms < <(parse_microstat_line "${micro_line}")
    printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\tok\n' "${label}" "${flags}" "${c_ms}" "${ny_aot_ms}" "${ratio_instr}" "${ratio_cycles}" "${ratio_ms}" | tee -a "${summary_file}"
  else
    printf '%s\t%s\t-\t-\t-\t-\t-\tfail(rc=%s)\n' "${label}" "${flags}" "${rc}" | tee -a "${summary_file}"
  fi
done

echo "[matrix] summary=${summary_file}"

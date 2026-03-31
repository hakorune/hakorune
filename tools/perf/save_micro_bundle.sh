#!/usr/bin/env bash
set -euo pipefail

# Convenience wrapper for trace_optimization_bundle.sh with a stable micro-lane default.
# Saves lowered.ll, objdump, perf_top evidence, and route traces under a per-benchmark bundle dir.
#
# Usage:
#   tools/perf/save_micro_bundle.sh <bench_key> [--out-dir <dir>] [--route direct|hako-mainline|hako-helper] [--microasm-runs N] [--symbol <sym> ...]

KEY="${1:-}"
shift || true

if [[ -z "${KEY}" ]]; then
  echo "Usage: $0 <bench_key> [--out-dir <dir>] [--route direct|hako-mainline|hako-helper] [--microasm-runs N] [--symbol <sym> ...]" >&2
  exit 2
fi

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TRACE="${ROOT_DIR}/tools/perf/trace_optimization_bundle.sh"

ROUTE="direct"
MICROASM_RUNS=0
OUT_DIR=""
declare -a SYMBOLS=()
SKIP_INDEXOF_LINE_SEED=0
TIMEOUT_SECS=60

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir)
      OUT_DIR="${2:-}"
      shift 2
      ;;
    --route)
      ROUTE="${2:-}"
      shift 2
      ;;
    --microasm-runs)
      MICROASM_RUNS="${2:-}"
      shift 2
      ;;
    --symbol)
      SYMBOLS+=("${2:-}")
      shift 2
      ;;
    --skip-indexof-line-seed)
      SKIP_INDEXOF_LINE_SEED=1
      shift
      ;;
    --timeout-secs)
      TIMEOUT_SECS="${2:-}"
      shift 2
      ;;
    -h|--help)
      sed -n '1,120p' "$0"
      exit 0
      ;;
    *)
      echo "[error] unknown arg: $1" >&2
      sed -n '1,120p' "$0" >&2
      exit 2
      ;;
  esac
done

if ! [[ "${MICROASM_RUNS}" =~ ^[0-9]+$ ]] || [[ "${MICROASM_RUNS}" -lt 0 ]]; then
  echo "[error] --microasm-runs must be >= 0" >&2
  exit 2
fi
if ! [[ "${TIMEOUT_SECS}" =~ ^[0-9]+$ ]] || [[ "${TIMEOUT_SECS}" -lt 0 ]]; then
  echo "[error] --timeout-secs must be >= 0" >&2
  exit 2
fi

label_safe() {
  printf '%s' "$1" | tr '/ ' '__' | tr -cd 'A-Za-z0-9._-'
}

if [[ -z "${OUT_DIR}" ]]; then
  ts="$(date +%Y%m%d-%H%M%S)"
  OUT_DIR="${ROOT_DIR}/target/perf_state/optimization_bundle/${ts}-$(label_safe "${KEY}")"
fi

mkdir -p "${OUT_DIR}"

trace_args=(
  --input "${KEY}"
  --route "${ROUTE}"
  --microasm-runs "${MICROASM_RUNS}"
  --out-dir "${OUT_DIR}"
  --timeout-secs "${TIMEOUT_SECS}"
)
if [[ "${SKIP_INDEXOF_LINE_SEED}" -eq 1 ]]; then
  trace_args+=(--skip-indexof-line-seed)
fi
for symbol in "${SYMBOLS[@]}"; do
  trace_args+=(--symbol "${symbol}")
done

bash "${TRACE}" "${trace_args[@]}"

if [[ -f "${OUT_DIR}/bundle.exe" && ! -f "${OUT_DIR}/objdump.txt" ]] && command -v objdump >/dev/null 2>&1; then
  objdump -d --demangle "${OUT_DIR}/bundle.exe" > "${OUT_DIR}/objdump.txt"
fi

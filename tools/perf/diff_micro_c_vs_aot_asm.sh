#!/usr/bin/env bash
set -euo pipefail

# Compare C vs Nyash AOT assembly for a fixed micro benchmark.
# Saves the full optimization bundle, C objdump, AOT objdump, symbol snippets, and a unified diff.
#
# Usage:
#   tools/perf/diff_micro_c_vs_aot_asm.sh <bench_key> [--out-dir <dir>] [--c-symbol main] [--aot-symbol ny_main]

KEY="${1:-}"
shift || true

if [[ -z "${KEY}" ]]; then
  echo "Usage: $0 <bench_key> [--out-dir <dir>] [--c-symbol main] [--aot-symbol ny_main]" >&2
  exit 2
fi

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
source "${ROOT_DIR}/tools/perf/lib/bench_key_alias.sh"
source "${ROOT_DIR}/tools/perf/lib/aot_helpers.sh"

DATASET_KEY="$(perf_resolve_bench_dataset_key "${KEY}")"
HAKO_PROG="${ROOT_DIR}/benchmarks/bench_${DATASET_KEY}.hako"
C_SRC="${ROOT_DIR}/benchmarks/c/bench_${DATASET_KEY}.c"
HAKORUNE_BIN="${ROOT_DIR}/target/release/hakorune"

C_SYMBOL="main"
AOT_SYMBOL="ny_main"
OUT_DIR=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir)
      OUT_DIR="${2:-}"
      shift 2
      ;;
    --c-symbol)
      C_SYMBOL="${2:-}"
      shift 2
      ;;
    --aot-symbol)
      AOT_SYMBOL="${2:-}"
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
if ! command -v objdump >/dev/null 2>&1; then
  echo "[error] objdump missing" >&2
  exit 2
fi
if ! command -v "${CC:-cc}" >/dev/null 2>&1; then
  echo "[error] C compiler missing: ${CC:-cc}" >&2
  exit 2
fi

label_safe() {
  printf '%s' "$1" | tr '/ ' '__' | tr -cd 'A-Za-z0-9._-'
}

if [[ -z "${OUT_DIR}" ]]; then
  ts="$(date +%Y%m%d-%H%M%S)"
  OUT_DIR="${ROOT_DIR}/target/perf_state/optimization_bundle/asm-diff/${ts}-$(label_safe "${KEY}")"
fi
mkdir -p "${OUT_DIR}"

bundle_out="${OUT_DIR}/bundle"
mkdir -p "${bundle_out}"

bash "${ROOT_DIR}/tools/perf/save_micro_bundle.sh" "${KEY}" --out-dir "${bundle_out}" >/dev/null

C_BIN="${OUT_DIR}/c.bin"
C_OBJDUMP="${OUT_DIR}/c.objdump.txt"
AOT_OBJDUMP="${bundle_out}/objdump.txt"
C_SNIPPET="${OUT_DIR}/c.${C_SYMBOL}.txt"
AOT_SNIPPET="${OUT_DIR}/aot.${AOT_SYMBOL}.txt"
ASM_DIFF="${OUT_DIR}/asm.diff.txt"
SUMMARY="${OUT_DIR}/summary.txt"

cc -O3 -march=native -mtune=native -o "${C_BIN}" "${C_SRC}" 2>/dev/null || cc -O3 -o "${C_BIN}" "${C_SRC}"
objdump -d --demangle "${C_BIN}" > "${C_OBJDUMP}"

extract_symbol_snippet() {
  local objdump_path="$1"
  local symbol="$2"
  local out_path="$3"
  local line_no=""
  line_no="$(grep -nF "<${symbol}>:" "${objdump_path}" | head -n 1 | cut -d: -f1 || true)"
  if [[ -n "${line_no}" ]]; then
    sed -n "${line_no},$((line_no + 120))p" "${objdump_path}" > "${out_path}"
  else
    printf '[warn] symbol not found: %s\n' "${symbol}" > "${out_path}"
  fi
}

extract_symbol_snippet "${C_OBJDUMP}" "${C_SYMBOL}" "${C_SNIPPET}"
extract_symbol_snippet "${AOT_OBJDUMP}" "${AOT_SYMBOL}" "${AOT_SNIPPET}"
diff -u "${C_SNIPPET}" "${AOT_SNIPPET}" > "${ASM_DIFF}" || true

{
  printf 'bench_key=%s\n' "${KEY}"
  printf 'dataset_key=%s\n' "${DATASET_KEY}"
  printf 'out_dir=%s\n' "${OUT_DIR}"
  printf 'bundle_out=%s\n' "${bundle_out}"
  printf 'c_objdump=%s\n' "${C_OBJDUMP}"
  printf 'aot_objdump=%s\n' "${AOT_OBJDUMP}"
  printf 'c_snippet=%s\n' "${C_SNIPPET}"
  printf 'aot_snippet=%s\n' "${AOT_SNIPPET}"
  printf 'asm_diff=%s\n' "${ASM_DIFF}"
  printf 'symbols=%s -> %s\n' "${C_SYMBOL}" "${AOT_SYMBOL}"
} > "${SUMMARY}"

cat "${SUMMARY}"
echo "[diff] asm diff saved to ${ASM_DIFF}"

#!/usr/bin/env bash
set -euo pipefail

# Thin orchestration wrapper for the current kilo/string split read order.
# Keeps measurement primitives separate and only sequences the accepted gates:
#   1. mixed accept gate
#   2. substring-only split exact
#   3. len-only split exact
#   4. whole kilo strict
#
# Usage:
#   tools/perf/run_kilo_string_split_pack.sh [warmup] [repeat] [asm_runs]
#
# Example:
#   tools/perf/run_kilo_string_split_pack.sh 1 3
#   tools/perf/run_kilo_string_split_pack.sh 1 3 20

WARMUP="${1:-1}"
REPEAT="${2:-3}"
ASM_RUNS="${3:-0}"

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
STAT="${ROOT_DIR}/tools/perf/bench_micro_c_vs_aot_stat.sh"
WHOLE="${ROOT_DIR}/tools/perf/run_kilo_hk_bench.sh"
ASM="${ROOT_DIR}/tools/perf/bench_micro_aot_asm.sh"

for script in "${STAT}" "${WHOLE}"; do
  if [[ ! -f "${script}" ]]; then
    echo "[error] missing script: ${script}" >&2
    exit 2
  fi
done

cases=(
  kilo_micro_substring_only
  kilo_micro_substring_views_only
  kilo_micro_len_substring_views
)

for key in "${cases[@]}"; do
  bash "${STAT}" "${key}" "${WARMUP}" "${REPEAT}"
done

bash "${WHOLE}" strict "${WARMUP}" "${REPEAT}"

if ! [[ "${ASM_RUNS}" =~ ^[0-9]+$ ]] || [[ "${ASM_RUNS}" -lt 0 ]]; then
  echo "[error] asm_runs must be >= 0" >&2
  exit 2
fi

if [[ "${ASM_RUNS}" -gt 0 ]]; then
  if [[ ! -f "${ASM}" ]]; then
    echo "[error] missing script: ${ASM}" >&2
    exit 2
  fi
  bash "${ASM}" kilo_micro_len_substring_views 'nyash.string.len_h' "${ASM_RUNS}"
fi

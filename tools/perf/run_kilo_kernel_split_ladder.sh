#!/usr/bin/env bash
set -euo pipefail

# Contract-oriented split ladder for kilo_kernel_small_hk.
# Read the whole benchmark through the main lifecycle-heavy corridors first,
# then confirm the assembled whole at the end.
#
# Usage:
#   tools/perf/run_kilo_kernel_split_ladder.sh [warmup] [repeat]

WARMUP="${1:-1}"
REPEAT="${2:-15}"

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
STAT="${ROOT_DIR}/tools/perf/bench_micro_c_vs_aot_stat.sh"
WHOLE="${ROOT_DIR}/tools/perf/run_kilo_hk_bench.sh"

if [[ ! -f "${STAT}" ]]; then
  echo "[error] missing executable: ${STAT}" >&2
  exit 2
fi
if [[ ! -f "${WHOLE}" ]]; then
  echo "[error] missing executable: ${WHOLE}" >&2
  exit 2
fi

cases=(
  kilo_micro_concat_hh_len
  kilo_micro_array_string_store
  kilo_meso_substring_concat_len
  kilo_meso_substring_concat_array_set_loopcarry
)

for key in "${cases[@]}"; do
  bash "${STAT}" "${key}" "${WARMUP}" "${REPEAT}"
done

bash "${WHOLE}" strict "${WARMUP}" "${REPEAT}"

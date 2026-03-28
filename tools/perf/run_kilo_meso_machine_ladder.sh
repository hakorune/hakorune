#!/usr/bin/env bash
set -euo pipefail

# Run meso C vs AOT stats between kilo micro leafs and full kilo mainline.
#
# Usage:
#   tools/perf/run_kilo_meso_machine_ladder.sh [warmup] [repeat]

WARMUP="${1:-1}"
REPEAT="${2:-15}"

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
STAT="${ROOT_DIR}/tools/perf/bench_micro_c_vs_aot_stat.sh"

if [[ ! -x "${STAT}" ]]; then
  echo "[error] missing executable: ${STAT}" >&2
  exit 2
fi

cases=(
  kilo_meso_substring_concat_len
  kilo_meso_substring_concat_array_set
  kilo_meso_substring_concat_array_set_loopcarry
)

for key in "${cases[@]}"; do
  bash "${STAT}" "${key}" "${WARMUP}" "${REPEAT}"
done

#!/usr/bin/env bash
set -euo pipefail

# Run meso C vs AOT stats between kilo micro leafs and full kilo mainline.
#
# Judgment policy:
#   - repeat < 3 is probe-only
#   - keep/reject decisions require at least 3 runs plus a quick ASM probe
#   - WSL / allocator-like noisy lanes should be rechecked with repeat=20 before closing
#
# Usage:
#   tools/perf/run_kilo_meso_machine_ladder.sh [warmup] [repeat]

WARMUP="${1:-1}"
REPEAT="${2:-15}"

if (( REPEAT < 3 )); then
  echo "[warn] repeat=${REPEAT} is below the perf judgment minimum (3); treat this run as probe-only" >&2
fi

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
STAT="${ROOT_DIR}/tools/perf/bench_micro_c_vs_aot_stat.sh"

if [[ ! -f "${STAT}" ]]; then
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

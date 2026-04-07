#!/usr/bin/env bash
set -euo pipefail

# Run machine-code focused micro stat suite for kilo/text hotspots.
#
# Usage:
#   tools/perf/run_kilo_micro_machine_ladder.sh [warmup] [repeat]
#
# Example:
#   tools/perf/run_kilo_micro_machine_ladder.sh 1 15

WARMUP="${1:-1}"
REPEAT="${2:-15}"

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
STAT="${ROOT_DIR}/tools/perf/bench_micro_c_vs_aot_stat.sh"

if [[ ! -x "${STAT}" ]]; then
  echo "[error] missing executable: ${STAT}" >&2
  exit 2
fi

cases=(
  kilo_micro_concat_const_suffix
  kilo_micro_array_string_store
  kilo_micro_indexof_line
  kilo_micro_substring_only
  kilo_micro_substring_concat
  kilo_micro_array_getset
)

for key in "${cases[@]}"; do
  bash "${STAT}" "${key}" "${WARMUP}" "${REPEAT}"
done

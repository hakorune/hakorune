#!/usr/bin/env bash
set -euo pipefail

# Run leaf-proof micro stat suite before broader kilo micro / main checks.
#
# Usage:
#   tools/perf/run_kilo_leaf_proof_ladder.sh [warmup] [repeat]

WARMUP="${1:-1}"
REPEAT="${2:-15}"

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
STAT="${ROOT_DIR}/tools/perf/bench_micro_c_vs_aot_stat.sh"

if [[ ! -x "${STAT}" ]]; then
  echo "[error] missing executable: ${STAT}" >&2
  exit 2
fi

cases=(
  kilo_leaf_array_rmw_add1
  kilo_leaf_array_string_len
  kilo_leaf_array_string_indexof_const
)

for key in "${cases[@]}"; do
  bash "${STAT}" "${key}" "${WARMUP}" "${REPEAT}"
done

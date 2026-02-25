#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "$0")/.." && pwd)
WARMUP=${1:-2}
REPEAT=${2:-7}

run_one() {
  local key=$1
  local warmup=$2
  local repeat=$3
  bash "${ROOT_DIR}/perf/bench_compare_c_vs_hako.sh" "$key" "${warmup}" "${repeat}" || true
}

echo "[phase-21.5] Comparing C vs Hakorune VM (median of ${REPEAT}, warmup ${WARMUP})"
run_one box_create_destroy_small "${WARMUP}" "${REPEAT}"
run_one method_call_only_small "${WARMUP}" "${REPEAT}"

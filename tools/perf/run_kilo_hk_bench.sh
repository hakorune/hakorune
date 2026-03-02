#!/usr/bin/env bash
set -euo pipefail

# run_kilo_hk_bench.sh
# Stable wrapper for kilo hk bench4 route.
#
# Usage:
#   tools/perf/run_kilo_hk_bench.sh [mode] [warmup] [repeat]
#   mode: strict|diagnostic (default: strict)
#
# Behavior:
#   strict:
#     PERF_VM_FORCE_NO_FALLBACK=1
#     PERF_REQUIRE_AOT_RESULT_PARITY=1
#   diagnostic:
#     PERF_VM_FORCE_NO_FALLBACK=1
#     PERF_REQUIRE_AOT_RESULT_PARITY=0

MODE="${1:-strict}"
WARMUP="${2:-1}"
REPEAT="${3:-3}"
KEY="kilo_kernel_small_hk"

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BENCH4="${ROOT_DIR}/tools/perf/bench_compare_c_py_vs_hako.sh"

if [[ ! -x "${BENCH4}" ]]; then
  echo "[error] bench4 script not executable: ${BENCH4}" >&2
  exit 2
fi

if ! [[ "${WARMUP}" =~ ^[0-9]+$ ]] || ! [[ "${REPEAT}" =~ ^[0-9]+$ ]]; then
  echo "[error] warmup/repeat must be non-negative integers: warmup=${WARMUP} repeat=${REPEAT}" >&2
  exit 2
fi

case "${MODE}" in
  strict)
    exec env \
      PERF_VM_FORCE_NO_FALLBACK=1 \
      PERF_REQUIRE_AOT_RESULT_PARITY=1 \
      bash "${BENCH4}" "${KEY}" "${WARMUP}" "${REPEAT}"
    ;;
  diagnostic)
    exec env \
      PERF_VM_FORCE_NO_FALLBACK=1 \
      PERF_REQUIRE_AOT_RESULT_PARITY=0 \
      bash "${BENCH4}" "${KEY}" "${WARMUP}" "${REPEAT}"
    ;;
  *)
    echo "[error] mode must be strict|diagnostic (got: ${MODE})" >&2
    exit 2
    ;;
esac

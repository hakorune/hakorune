#!/usr/bin/env bash
# run_all.sh — perf harness entry (manual). Example:
#   NYASH_LLVM_PERF=1 tools/perf/run_all.sh 2000 3
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SIZE="${1:-2000}"
REPEATS="${2:-3}"

echo "[perf] binop_chain size=$SIZE repeats=$REPEATS"

echo "[perf] crate..."
bash "$ROOT/perf/binop_chain_perf.sh" crate "$SIZE" "$REPEATS"

echo "[perf] native..."
bash "$ROOT/perf/binop_chain_perf.sh" native "$SIZE" "$REPEATS"

if [[ "${NYASH_LLVM_RUN_LLVMLITE:-0}" == "1" ]]; then
  echo "[perf] llvmlite (opt-in)..."
  bash "$ROOT/perf/binop_chain_perf.sh" llvmlite "$SIZE" "$REPEATS"
fi

echo "[perf] done"


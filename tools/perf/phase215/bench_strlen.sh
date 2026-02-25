#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
RUNS=5; N=${N:-10000000}; TIMEOUT=${TIMEOUT:-120}; FAST=${FAST:-0}
while [[ $# -gt 0 ]]; do case "$1" in --runs) RUNS="$2"; shift 2;; --n) N="$2"; shift 2;; --timeout) TIMEOUT="$2"; shift 2;; --fast) FAST="$2"; shift 2;; *) shift;; esac; done
NYASH_LLVM_BACKEND=crate NYASH_LLVM_FAST="$FAST" \
  "$ROOT/perf/microbench.sh" --case strlen --n "$N" --runs "$RUNS" --backend llvm --exe

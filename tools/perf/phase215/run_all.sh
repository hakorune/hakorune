#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
RUNS=${RUNS:-5}; TIMEOUT=${TIMEOUT:-120}
"$ROOT/bench_loop.sh" --runs "$RUNS"
"$ROOT/bench_strlen.sh" --runs "$RUNS" --fast 1
"$ROOT/bench_box.sh" --runs "$RUNS"

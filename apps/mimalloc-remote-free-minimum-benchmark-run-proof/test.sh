#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

bash tools/checks/impl/phase295x_mimalloc_remote_free_minimum_benchmark_run_guard.sh

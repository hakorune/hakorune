#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"
exec bash tools/checks/k2_wide_phase295x_reuse_cycle_small_workload_implementation_guard.sh

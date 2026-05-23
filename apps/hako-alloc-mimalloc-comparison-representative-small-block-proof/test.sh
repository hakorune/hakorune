#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"
exec bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_same_workload_memory_report_guard.sh

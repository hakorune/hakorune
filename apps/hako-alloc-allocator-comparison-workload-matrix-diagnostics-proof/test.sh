#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"
exec bash tools/checks/k2_wide_hako_alloc_allocator_comparison_workload_matrix_diagnostics_guard.sh --level L2

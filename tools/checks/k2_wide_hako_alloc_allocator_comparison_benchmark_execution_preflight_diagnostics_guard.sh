#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec bash "$ROOT_DIR/tools/checks/impl/k2_wide_hako_alloc_allocator_comparison_benchmark_execution_preflight_diagnostics_guard.sh" "$@"

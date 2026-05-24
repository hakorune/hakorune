#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

exec bash tools/checks/k2_wide_phase295x_realloc_aligned_hako_exe_acceptance_guard.sh

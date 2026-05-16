#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard.sh

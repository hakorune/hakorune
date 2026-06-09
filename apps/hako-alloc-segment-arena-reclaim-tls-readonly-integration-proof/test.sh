#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

exec bash "$ROOT_DIR/tools/checks/k2_wide_hako_alloc_segment_arena_reclaim_tls_readonly_integration_guard.sh" "$@"

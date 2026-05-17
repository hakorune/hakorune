#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec "$ROOT_DIR/tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_ledger_release_apply_guard.sh"

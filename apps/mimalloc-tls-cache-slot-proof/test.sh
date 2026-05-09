#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

exec "$ROOT_DIR/tools/checks/k2_wide_mimalloc_tls_cache_slot_exe_guard.sh"

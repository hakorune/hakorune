#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec bash "$ROOT_DIR/tools/checks/impl/k2_wide_mimalloc_remote_free_retry_loop_exe_guard.sh" "$@"

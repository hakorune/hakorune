#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

exec "$ROOT_DIR/tools/checks/k2_wide_mimalloc_tls_ptr_remote_free_exe_guard.sh"

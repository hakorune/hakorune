#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

# Compatibility entry for 293x-052. The current hako.mem runtime-decl guard
# lives in k2_wide_hako_mem_runtime_decl_guard.sh and covers alloc + realloc.
exec "$ROOT_DIR/tools/checks/k2_wide_hako_mem_runtime_decl_guard.sh"

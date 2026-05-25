#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
exec bash "$ROOT_DIR/tools/checks/impl/k2_wide_mimalloc_facade_huge_unregister_failfast_exe_guard.sh" "$@"
